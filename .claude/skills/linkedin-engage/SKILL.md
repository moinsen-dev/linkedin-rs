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

**Language filter (MANDATORY):**

- **ONLY engage with German-language posts.** This is the primary filter.
- Posts written in English or any other non-German language are SKIPPED, regardless of topic relevance.
- Mixed-language posts (e.g., German text with English technical terms) count as German.
- Posts from German-speaking authors that are written entirely in English are still SKIPPED.

**Include posts about:**

- Software engineering, AI/ML, DevOps, open source, Rust, Flutter, cloud, infrastructure
- Entrepreneurship, strategy, product management, team leadership, startups
- Original takes, genuine questions, interesting technical challenges

**Skip posts that are:**

- Written in English or any non-German language (see language filter above)
- Pure self-promotion without substance
- Motivational quotes or engagement bait ("Agree?", "Like if you...")
- Recruiting spam or job ads
- Already liked by the user (`liked: true`)
- Reshares without own commentary
- Generic corporate announcements

**Prefer posts with:**

- Low engagement (commenting early increases visibility)
- Original or controversial theses
- Genuine questions from the author

### Step 4: Create Engagement Plan

Present a table with ALL proposed actions at once:

```
## Engagement Plan (X von Y Posts relevant)

| # | Autor | Post (Preview) | Aktion | Kommentar |
|---|-------|----------------|--------|-----------|
| 1 | Jane Smith | "Wir haben unser Monitoring..." | LIKE + Kommentar | "Spannend, welches Tool nutzt ihr dafuer?" |
| 2 | Max M. | "Rust im Embedded-Bereich..." | LIKE | - |
| 3 | Sarah K. | "After 3 months of..." | LIKE + Kommentar | "Nice, congrats! How did the onboarding go?" |

Skipped: N Posts (Xx self-promo, Xx already liked, Xx not relevant)
```

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
