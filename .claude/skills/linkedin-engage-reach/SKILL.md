---
name: linkedin-engage-reach
description: "Visibility-optimized LinkedIn engagement. Targets high-reach posts with smart comments to maximize profile exposure. Complements linkedin-engage (community) with a reach strategy."
user-invocable: true
---

# LinkedIn Reach Engagement

Find high-visibility posts in the user's feed and place smart, value-adding comments that maximize profile exposure. Unlike `/linkedin-engage` (community-building with builders), this skill targets **popular posts where a good comment gets seen by thousands**.

## Workflow

Follow these steps in order:

### Step 1: Auth Check

```bash
linkedin-cli auth status --local
```

If no session found, tell the user to authenticate with `linkedin-cli auth login` and stop.

### Step 2: Load Feed

```bash
linkedin-cli feed list --count 50 --json
```

Load more posts (50 instead of 30) to have a larger pool of high-reach candidates.

### Step 3: Analyze Posts for Reach Potential

For each post, extract: author name, author description, post text, engagement counts (likes/comments/shares), whether already liked, post age if available, and the **correct interaction URN**.

**CRITICAL: URN extraction rule:**

The feed contains two types of posts:
- **Direct posts**: The top-level `urn` and `socialDetail.urn` are the same.
- **Mediated posts** (someone in your network liked/commented/reposted): The top-level `urn` is a feed-routing event URN that CANNOT be used for reactions/comments. The `socialDetail.urn` (or `socialDetail.threadId`) contains the **original post's activity URN** that MUST be used.

**Always extract the URN from `socialDetail.urn`** (path: `element.value["com.linkedin.voyager.feed.render.UpdateV2"].socialDetail.urn`). This works correctly for both direct and mediated posts.

You can detect mediated posts by the presence of a `header` block in the feed item (showing who liked/reposted/commented).

### Step 4: Score & Filter for Reach

**Reach Score** (compute for every post):

```
reach_score = likes / max(comments, 1)
```

A high reach_score means many eyeballs but few comments — the comment section is uncrowded, giving maximum visibility.

**Filter pipeline (apply in order):**

**Filter 1 — Hard skip (applied first):**

- Already liked by the user (`liked: true`) — SKIP
- Sponsored posts — SKIP
- Reshares without own commentary — SKIP
- Job ads / recruiting — SKIP
- Posts with > 80 comments — SKIP (comment will drown)
- Posts with < 10 likes — SKIP (not enough reach to matter)

**Filter 2 — Language:**

- **German AND English posts are both eligible.** (Unlike `/linkedin-engage`, reach matters more than language.)
- Comments are written in the **same language as the post**.
- Mixed-language posts: comment in whichever language dominates.

**Filter 3 — Topic relevance (MANDATORY):**

The post MUST be in one of these domains (broadly interpreted):
- Software engineering, architecture, DevOps, cloud, infrastructure
- AI/ML, LLMs, agents, automation
- Open source, Rust, Flutter, mobile development
- Tech entrepreneurship, product building, startup engineering
- Developer tools, productivity, engineering culture
- Technical leadership, CTO/VP Eng perspectives

SKIP posts about:
- Pure marketing / corporate PR without substance
- Motivational quotes, engagement bait ("Agree?", "Like if you...")
- Meta-LinkedIn content (posting tips, algorithm hacks, personal branding)
- Politics, news commentary, lifestyle
- Finance/investing without tech angle

**Filter 4 — Builder filter (RELAXED compared to /linkedin-engage):**

Unlike the strict builder filter in `/linkedin-engage`, this skill accepts:
- **Thought leaders IF they have real technical background** — a CTO sharing opinions about architecture is fine; a "LinkedIn Top Voice" with no technical role is not.
- **Curated insights IF the author adds substantial own perspective** — not just "5 tools" listicles, but "I evaluated 5 tools and here's what we picked and why"
- **Experience-based teaching** — "Here's how to approach X" is fine IF backed by the author's own work

Still SKIP:
- Pure influencer accounts (Content Creator, Keynote Speaker, LinkedIn Top Voice with no technical role)
- Engagement bait disguised as insights
- Recycled platitudes

**Ranking (after filtering):**

Sort remaining posts by:
1. **reach_score** (higher = better opportunity) — primary sort
2. **Freshness** (newer posts = comment grows with the post) — tiebreaker
3. **Topic alignment** with user's expertise (AI, Rust, Flutter, DevOps) — bonus

Select **top 5 posts** maximum. Quality over quantity — fewer, better comments.

### Step 5: Create Engagement Plan

Present a table with ALL proposed actions at once:

```
## Reach Engagement Plan (X von Y Posts selected)

| # | Autor | Reach Score | Likes/Comments | Thema | Post (Preview) | Sprache | Kommentar-Entwurf |
|---|-------|-------------|----------------|-------|----------------|---------|-------------------|
| 1 | Sarah K. | 45.0 | 90/2 | K8s Migration | "We just migrated our..." | EN | "We ran into similar issues with..." |
| 2 | Thomas B. | 28.5 | 57/2 | AI Agents | "Unsere Erfahrung mit..." | DE | "Spannend - wir bauen grad was aehnliches..." |

Skipped: N Posts (Xx too many comments, Xx too few likes, Xx off-topic, Xx already liked, Xx sponsored)
```

Then ask: **"Soll ich den Plan so ausfuehren? Du kannst einzelne Zeilen aendern oder streichen."**

Wait for explicit approval before proceeding.

### Step 6: Execute

For each approved action, run sequentially. **Wait 30 seconds between every API call** to avoid LinkedIn rate-limiting.

Every selected post gets LIKE + COMMENT (this is a reach skill — commenting is the whole point).

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

### Step 7: Summary

After execution, report what was done:

```
Fertig! X Posts geliked, X Kommentare geschrieben.
Gesamt-Reach der kommentierten Posts: ~Y Likes (potenzielle Sichtbarkeit).
```

## Comment Style Rules (Reach-Optimized)

These rules are NON-NEGOTIABLE:

1. **Match the post's language.** German post = German comment. English post = English comment. Technical terms stay as-is in both.
2. **2-4 sentences.** Slightly longer than `/linkedin-engage` comments because the goal is to demonstrate expertise, not just connect.
3. **Lead with substance, not praise.** The first sentence must contain a concrete insight, experience, or perspective. NEVER open with a compliment.
4. **Show expertise.** Reference your own experience building things. Use phrases like "We built...", "In our case...", "Wir haben bei uns...", "Nach unserer Erfahrung mit...". This positions the profile as a doer.
5. **Add a dimension the post missed.** Extend the conversation — mention a related challenge, an alternative approach, a tradeoff the author didn't cover. This makes the comment worth reading on its own.
6. **Constructive disagreement is gold.** If you genuinely see it differently, say so with reasoning. "Interesting, though in practice I've found X works better because Y" gets far more engagement than agreement.
7. **NO questions as the primary move.** Unlike `/linkedin-engage`, don't default to asking follow-up questions — it reads as subordinate on high-reach posts. Questions are fine as a secondary element after establishing your point.
8. **NEVER write** "Great post!", "Thanks for sharing!", "So true!", "Couldn't agree more!", "This resonates!", "Love this!" or ANY variation in any language. These are invisible noise.
9. **NEVER use emojis** unless the original post is emoji-heavy.
10. **Vary the structure.** Don't start every comment the same way. Mix: own experience first. Or: counterpoint then agreement. Or: specific technical detail then broader implication.

### Comment Templates (for inspiration, not copy-paste):

**Expertise comment (DE):**
"Wir haben bei uns aehnlich angefangen, aber dann gemerkt dass [specific detail]. Am Ende war [alternative approach] deutlich robuster — vor allem weil [reason]."

**Expertise comment (EN):**
"We hit the same wall last quarter. What made the difference for us was [specific approach] — cut our [metric] by [amount]. The tricky part was [nuance the post didn't mention]."

**Constructive disagreement (DE):**
"Interessanter Ansatz, wobei ich in der Praxis eher das Gegenteil sehe: [counterpoint]. Bei uns hat [alternative] besser funktioniert, weil [concrete reason]."

**Extending the conversation (EN):**
"One thing worth adding: [related challenge]. We solved it by [approach], which also helped with [secondary benefit] — might be relevant for your setup too."

## Safety

- This skill inherits ALL rules from the `/linkedin` skill (credentials, rate limits, sequential calls).
- The engagement plan is the confirmation gate. Nothing executes without the user's explicit "go".
- The user can edit any comment, change any action, or skip any post before approval.
- If any API call fails during execution, report the error and continue with the remaining actions.
- Maximum 5 comments per run to avoid looking spammy.
