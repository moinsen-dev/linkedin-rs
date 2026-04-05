---
name: linkedin-engage
description: "Analyze LinkedIn feed, filter for interesting German-language posts, draft human-sounding comments, and execute engagement after user approval. Batch workflow: load feed -> analyze -> plan -> approve -> execute."
user-invocable: true
---

# LinkedIn Feed Engagement

Analyze the user's LinkedIn feed, identify interesting **German-language** posts, draft reactions and comments, and execute after batch approval.

## Workflow

Follow these steps in order:

### Step 1: Auth Check

```bash
linkedin-cli auth status --local
```

If no session found, tell the user to authenticate with `linkedin-cli auth login` and stop.

### Step 2: Load Feed

```bash
linkedin-cli feed list --count 30 --json
```

### Step 3: Analyze Posts

For each post, extract: author name, post text, engagement counts (likes/comments/shares), whether already liked, and the **correct interaction URN**.

**CRITICAL: URN extraction rule:**

The feed contains two types of posts:
- **Direct posts**: The top-level `urn` and `socialDetail.urn` are the same.
- **Mediated posts** (someone in your network liked/commented/reposted): The top-level `urn` is a feed-routing event URN that CANNOT be used for reactions/comments. The `socialDetail.urn` (or `socialDetail.threadId`) contains the **original post's activity URN** that MUST be used.

**Always extract the URN from `socialDetail.urn`** (path: `element.value["com.linkedin.voyager.feed.render.UpdateV2"].socialDetail.urn`). This works correctly for both direct and mediated posts.

You can detect mediated posts by the presence of a `header` block in the feed item (showing who liked/reposted/commented).

**Filter 1 — Language (MANDATORY, applied first):**

- **ONLY engage with German-language posts.**
- Posts written in English or any other non-German language are SKIPPED, regardless of topic relevance.
- Mixed-language posts (e.g., German text with English technical terms) count as German.
- Posts from German-speaking authors that are written entirely in English are still SKIPPED.

**Filter 2 — Builder filter (MANDATORY, applied second):**

The core question for every post: **"Hat diese Person etwas gebaut, geloest, oder dabei geholfen?"**

A post PASSES the builder filter if the author:
- **Built something**: shipped a feature, product, tool, library, side project, prototype, or open-source contribution
- **Solved a concrete problem**: debugged an issue, found a workaround, fixed a production incident, optimized performance
- **Shared hands-on experience**: lessons from actual implementation work, architecture decisions they made, migration stories they lived through
- **Asked a genuine technical question**: stuck on a real problem they're working on, seeking specific advice (not rhetorical engagement bait)

A post FAILS the builder filter if:
- The author **commentates without building** — opinions about industry trends, hot takes on other companies' decisions, "AI will change everything" style posts
- The author **curates or summarizes** — "5 tools every developer needs", listicles, link roundups without own experience
- The author **teaches from authority, not experience** — generic advice posts ("Here's how to be a better developer") without referencing their own recent work
- The post is **meta-LinkedIn content** — posts about posting, engagement strategy, personal branding, "what I learned from going viral"
- The author's **profile signals influencer over builder** — titles like "LinkedIn Top Voice", "Content Creator", "Thought Leader", "Keynote Speaker" without a clear technical role. Check the author description field for these signals.

**Borderline cases:** If the post mixes opinion with genuine build experience (e.g., a hot take backed by "we hit this exact problem last month when..."), it PASSES. The bar is: does the post contain evidence that the author actually does the work?

**Filter 3 — Topic relevance (applied after builder filter passes):**

Include posts about:
- Software engineering, AI/ML, DevOps, open source, Rust, Flutter, cloud, infrastructure
- Entrepreneurship with substance (building a company, shipping product, hiring decisions with context)
- Concrete technical challenges, architecture decisions, debugging stories

**Skip posts that are (in addition to builder filter failures):**

- Written in English or any non-German language (see Filter 1)
- Already liked by the user (`liked: true`)
- Sponsored posts
- Reshares without own commentary
- Recruiting spam or job ads
- Motivational quotes or engagement bait ("Agree?", "Like if you...")
- Generic corporate announcements
- News articles or press releases without personal experience added

**Prefer posts with:**

- Low engagement (commenting early increases visibility)
- Specific technical details (code snippets, architecture diagrams, error messages, metrics)
- First-person experience ("Wir haben...", "Ich hab letzte Woche...", "Nach 3 Monaten mit X...")
- Genuine questions from someone stuck on a real problem

### Step 4: Create Engagement Plan

Present a table with ALL proposed actions at once:

```
## Engagement Plan (X von Y Posts relevant)

| # | Autor | Was gebaut/geloest? | Post (Preview) | Aktion | Kommentar |
|---|-------|---------------------|----------------|--------|-----------|
| 1 | Jane S. | Monitoring-Stack migriert | "Wir haben unser Monitoring..." | LIKE + Kommentar | "Spannend, welches Tool nutzt ihr dafuer?" |
| 2 | Max M. | Rust auf Embedded deployed | "Rust im Embedded-Bereich..." | LIKE | - |

Skipped: N Posts (Xx influencer/opinion-only, Xx English, Xx already liked, Xx sponsored, Xx not relevant)
```

The "Was gebaut/geloest?" column is mandatory. If you cannot fill it with a concrete builder action, the post does not pass Filter 2.

Then ask: **"Soll ich den Plan so ausfuehren? Du kannst einzelne Zeilen aendern oder streichen."**

Wait for explicit approval before proceeding.

### Step 5: Execute

For each approved action, run sequentially (one Bash call per command). **Wait 30 seconds between every API call** to avoid LinkedIn rate-limiting (which manifests as 400 errors after rapid successive calls). Speed does not matter here -- the user can wait.

For each post with LIKE + COMMENT, execute them as two separate calls with 30s between them:

1. First: React (like)
2. Wait 30s
3. Then: Comment

**Like:**

```bash
sleep 30 && linkedin-cli feed react <URN> --json
```

**Comment (always add `--yes`):**

```bash
sleep 30 && linkedin-cli feed comment <URN> "<COMMENT_TEXT>" --yes --json
```

The very first call of the entire batch does not need the delay.

Use the appropriate reaction type:

- Default: `LIKE`
- Achievements/milestones: `--type CELEBRATION`
- Struggles/layoffs: `--type EMPATHY`
- Impressive work: `--type PRAISE`

### Step 6: Summary

After execution, report what was done:

```
Fertig! X Posts geliked, Y Kommentare geschrieben.
```

## Comment Style Rules

All comments are written in **German** since only German posts are engaged with. These rules are NON-NEGOTIABLE:

1. **Always write in German.** English technical terms are fine inline (API, DevOps, CI/CD, etc.) but the comment itself is German.
2. **1-3 sentences max.** Never longer.
3. **Casual-collegial tone.** Like a colleague quickly typing a response during lunch. Du-Form, not Sie.
4. **Reference the actual content.** Always mention something specific from the post. Never generic.
5. **Add value.** Ask a follow-up question, share a brief related experience, or offer a concrete thought.
6. **Sound human.** Use informal shortenings occasionally ("grad" statt "gerade", "ne" statt "eine"). Sometimes skip the final period. Don't overdo it — natural, not artificially sloppy.
7. **NEVER write** "Toller Post!", "Danke fuers Teilen!", "So wahr!", "Sehe ich genauso!" or any variation. These are engagement-bait cliches.
8. **NEVER use emojis** unless the original post is emoji-heavy.
9. **Vary the structure.** Don't start every comment with a question. Mix: observation, then question. Or: brief experience, then thought. Or: direct disagreement with reasoning.

## Safety

- This skill inherits ALL rules from the `/linkedin` skill (credentials, rate limits, sequential calls).
- The engagement plan is the confirmation gate. Nothing executes without the user's explicit "go".
- The user can edit any comment, change any action, or skip any post before approval.
- If any API call fails during execution, report the error and continue with the remaining actions.
