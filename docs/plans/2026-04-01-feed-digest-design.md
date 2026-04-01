# Feed Digest — Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use moinsenpowers:subagent-driven-development (recommended) or moinsenpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** A Claude Code skill (`/feed-digest`) that collects LinkedIn feed posts every 30 minutes via cron, deduplicates by URN, and after 24 hours generates an AI summary of themes, trends, and highlights.

**Architecture:** Single skill file with two modes (collect/summarize) selected by checking `started_at.txt` age. Uses `linkedin-cli feed list --json` for data, JSONL file for accumulation, line-based URN set for dedup. Cron scheduling via `/schedule`.

**Tech Stack:** Claude Code skill (Markdown), Bash (linkedin-cli invocation, jq for JSON parsing), `/schedule` for cron triggers.

---

## File Structure

| File | Action | Responsibility |
|------|--------|----------------|
| `.claude/skills/feed-digest/SKILL.md` | Create | Skill definition — mode selection, collect logic, summarize logic |
| `secrets/feed-collector/` | Create (runtime) | Storage dir for posts.jsonl, seen_urns.txt, started_at.txt, digest files |

The skill is a single file. All runtime data lives in `secrets/feed-collector/` (already gitignored via `secrets/` in `.gitignore`).

---

### Task 1: Create the skill file with frontmatter and mode selection

**Files:**
- Create: `.claude/skills/feed-digest/SKILL.md`

- [ ] **Step 1: Create skill directory**

```bash
mkdir -p .claude/skills/feed-digest
```

- [ ] **Step 2: Write skill file with frontmatter and mode selection logic**

Create `.claude/skills/feed-digest/SKILL.md` with:

```markdown
---
name: feed-digest
description: "Collect LinkedIn feed posts every 30min and generate a 24h AI summary. Modes: collect (periodic), summarize (after 24h). Start with /feed-digest start, check with /feed-digest status, stop with /feed-digest stop."
user-invocable: true
---

# LinkedIn Feed Digest

Collect unique LinkedIn feed posts over 24 hours and generate an AI-powered daily summary.

## Commands

- `/feed-digest start` — Start a new 24h collection cycle (sets up cron)
- `/feed-digest stop` — Stop collection and generate summary immediately
- `/feed-digest status` — Show current cycle info (posts collected, time remaining)

## Prerequisites

1. `linkedin-cli` must be on PATH
2. Active LinkedIn session: `linkedin-cli auth status --local`

## Start Command

When the user runs `/feed-digest start`:

1. Check auth:

   ```bash
   linkedin-cli auth status --local
   ```

   If no session, tell user to authenticate and stop.

2. Create storage directory and initialize cycle:

   ```bash
   mkdir -p secrets/feed-collector
   date -u +"%Y-%m-%dT%H:%M:%SZ" > secrets/feed-collector/started_at.txt
   touch secrets/feed-collector/posts.jsonl
   touch secrets/feed-collector/seen_urns.txt
   ```

3. Run the first collect pass (see Collect section below).

4. Set up cron to run collect every 30 minutes. Use the `/schedule` skill to create a trigger:

   Schedule a remote trigger with cron `"*/30 * * * *"` and prompt:

   ```
   Run /feed-digest collect
   ```

5. Report to user:

   ```
   Feed-Digest gestartet. Sammle alle 30 Min neue Posts.
   Erster Durchlauf: N neue Posts gefunden.
   Zusammenfassung wird in ~24h automatisch generiert.
   ```

## Collect Command (cron-triggered)

When triggered by cron or manually via `/feed-digest collect`:

### Step 1: Check if 24h cycle is complete

```bash
cat secrets/feed-collector/started_at.txt
```

Compare the timestamp to now. If more than 24 hours have passed, run the **Summarize** flow instead, then start a new cycle and collect.

If `started_at.txt` does not exist, tell the user to run `/feed-digest start` first and stop.

### Step 2: Auth check

```bash
linkedin-cli auth status --local
```

If no session, log error and stop. Do not retry.

### Step 3: Fetch feed

```bash
linkedin-cli feed list --count 50 --json
```

### Step 4: Extract and deduplicate posts

Parse the JSON output. For each element in the `elements` array:

**URN extraction (CRITICAL):**
LinkedIn feed items wrap the UpdateV2 payload inside:
```
element.value["com.linkedin.voyager.feed.render.UpdateV2"]
```

Extract the dedup URN — prefer `socialDetail.urn` (original post), fallback to top-level `entityUrn`:
- Path: `element.value["com.linkedin.voyager.feed.render.UpdateV2"].socialDetail.urn`
- Fallback: `element.entityUrn`

This handles both direct posts and mediated posts (someone liked/reposted). See `re/feed_urn_resolution.md`.

**For each element:**

1. Extract URN as described above
2. Check if URN is already in `secrets/feed-collector/seen_urns.txt` — if yes, skip
3. Extract author: `update.actor.name.text` (where update = the unwrapped UpdateV2)
4. Extract text: `update.commentary.text.text`
5. If both author and text are present, append to `secrets/feed-collector/posts.jsonl`:
   ```json
   {"urn":"urn:li:activity:123","author":"Jane Smith","text":"Post content here...","collected_at":"2026-04-01T10:30:00Z"}
   ```
6. Append URN to `secrets/feed-collector/seen_urns.txt`

### Step 5: Log result

Count how many new posts were added. No user-visible output needed (cron runs silently).

## Status Command

When the user runs `/feed-digest status`:

1. Check if `secrets/feed-collector/started_at.txt` exists. If not: "Kein aktiver Cycle. Starte mit `/feed-digest start`."

2. Read and display:
   ```bash
   cat secrets/feed-collector/started_at.txt
   wc -l < secrets/feed-collector/posts.jsonl
   wc -l < secrets/feed-collector/seen_urns.txt
   ```

3. Calculate time remaining until 24h mark.

4. Report:
   ```
   Feed-Digest Status:
   - Gestartet: YYYY-MM-DD HH:MM UTC
   - Gesammelte Posts: N (unique)
   - Verbleibend: Xh Ym bis zur Zusammenfassung
   ```

## Stop Command / Summarize

When the user runs `/feed-digest stop` OR when the 24h cycle expires (detected during collect):

### Step 1: Read collected posts

```bash
cat secrets/feed-collector/posts.jsonl
```

Read all lines. Each line is a JSON object with `urn`, `author`, `text`, `collected_at`.

### Step 2: Read cycle metadata

```bash
cat secrets/feed-collector/started_at.txt
```

Count total posts and collection duration.

### Step 3: Generate AI summary

Analyze all collected posts and write a summary. The summary should identify:

- **Themes**: Group posts by topic. Name each theme, give a brief description, and how many posts belong to it.
- **Trends**: Patterns across the feed — what topics are hot, what shifted.
- **Highlights**: 3-5 standout posts worth revisiting (author + brief summary of why).
- **Stats**: Total unique posts, collection period, number of collection runs.

### Step 4: Write digest file

Write the summary to `secrets/feed-collector/digest-YYYY-MM-DD.md` using today's date.

Format:

```markdown
# LinkedIn Feed Digest — YYYY-MM-DD

## Themes
- **Theme 1** — brief description, N posts
- **Theme 2** — brief description, N posts

## Trends
- Notable patterns observed across the feed

## Highlights
- Standout posts worth revisiting (author + brief summary)

## Stats
- Total unique posts collected: N
- Collection period: start — end
- Collection runs: N
```

### Step 5: Clean up cycle files

```bash
rm secrets/feed-collector/posts.jsonl
rm secrets/feed-collector/seen_urns.txt
rm secrets/feed-collector/started_at.txt
```

Do NOT delete the digest file — it persists.

### Step 6: Cancel the cron trigger

Remove the scheduled trigger that was set up during start.

### Step 7: Report to user

```
Feed-Digest abgeschlossen!
N Posts ueber Xh gesammelt.
Zusammenfassung: secrets/feed-collector/digest-YYYY-MM-DD.md
```

Then display the digest content inline.

## Safety

- This skill only uses READ operations (`feed list`). No WRITE operations.
- All data stored in `secrets/` (gitignored, contains PII like author names).
- Single API call per cron run — minimal rate-limit risk.
- If auth fails during collect, the run is skipped silently. No retry loop.
- Inherits all rules from the `/linkedin` skill.
```

- [ ] **Step 3: Verify skill file exists and has correct frontmatter**

```bash
head -6 .claude/skills/feed-digest/SKILL.md
```

Expected: the YAML frontmatter block with `name: feed-digest`.

- [ ] **Step 4: Commit**

```bash
git add .claude/skills/feed-digest/SKILL.md
git commit -m "feat: add feed-digest skill for 24h LinkedIn feed collection and AI summary"
```

---

### Task 2: Verify skill is recognized by Claude Code

**Files:**
- Read: `.claude/skills/feed-digest/SKILL.md`

- [ ] **Step 1: Check skill appears in skill list**

The skill should now appear when listing skills. Verify by checking that the file is in the expected location:

```bash
ls -la .claude/skills/feed-digest/SKILL.md
```

Expected: file exists with the content from Task 1.

- [ ] **Step 2: Verify secrets directory is gitignored**

```bash
grep -n "secrets" .gitignore
```

Expected: a line containing `secrets/` — confirms feed-collector data won't be committed.

---

### Task 3: Manual smoke test — collect mode

**Files:**
- Read: `.claude/skills/feed-digest/SKILL.md`

- [ ] **Step 1: Verify auth**

```bash
linkedin-cli auth status --local
```

Expected: session found.

- [ ] **Step 2: Initialize storage**

```bash
mkdir -p secrets/feed-collector
date -u +"%Y-%m-%dT%H:%M:%SZ" > secrets/feed-collector/started_at.txt
touch secrets/feed-collector/posts.jsonl
touch secrets/feed-collector/seen_urns.txt
```

- [ ] **Step 3: Fetch feed and verify JSON output**

```bash
linkedin-cli feed list --count 5 --json
```

Expected: JSON with `elements` array. Inspect the structure to confirm URN paths match the skill's extraction logic.

- [ ] **Step 4: Manually verify URN extraction paths**

Using the JSON from Step 3, confirm these paths resolve:
- `element.value["com.linkedin.voyager.feed.render.UpdateV2"].socialDetail.urn`
- `element.value["com.linkedin.voyager.feed.render.UpdateV2"].actor.name.text`
- `element.value["com.linkedin.voyager.feed.render.UpdateV2"].commentary.text.text`
- Fallback: `element.entityUrn`

If any paths differ from what the skill documents, update the skill file accordingly.

- [ ] **Step 5: Clean up test data**

```bash
rm -rf secrets/feed-collector
```

---

### Task 4: Update settings.local.json for feed-digest permissions

**Files:**
- Modify: `.claude/settings.local.json`

- [ ] **Step 1: Check current permissions**

```bash
cat .claude/settings.local.json
```

Expected: only `Bash(linkedin-cli feed:*)` is allowed. This already covers `feed list` which is all the skill needs. No changes required unless the pattern doesn't match.

- [ ] **Step 2: Verify the permission pattern matches**

The skill runs `linkedin-cli feed list --count 50 --json`. The existing `Bash(linkedin-cli feed:*)` pattern should match. If it doesn't (test by running the command), add an explicit allow:

```json
"Bash(linkedin-cli feed list:*)"
```

- [ ] **Step 3: Commit if changed**

Only if settings were modified:

```bash
git add .claude/settings.local.json
git commit -m "chore: update permissions for feed-digest skill"
```
