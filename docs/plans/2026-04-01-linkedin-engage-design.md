# Design: /linkedin:engage — Feed Engagement Skill

## Context

The `linkedin-cli` already supports reading the feed, reacting to posts, and commenting. This skill adds an AI-driven workflow on top: analyze the feed, filter for relevant posts, draft human-sounding comments, and execute after user approval.

## Approach

Pure skill — no new Rust code. Claude reads the raw feed JSON, applies filtering heuristics, generates a batch engagement plan, and executes after user confirmation.

## Workflow

1. **Auth check** — `linkedin-cli auth status --local`
2. **Load feed** — `linkedin-cli feed list --count 30 --json`
3. **Analyze** — Claude evaluates each post against interest criteria
4. **Engagement plan** — Present a table: Post -> Action (Like/Comment/Skip) -> Draft comment
5. **User approval** — User reviews, edits, approves the batch
6. **Execute** — Sequential API calls (one per post, like then comment)
7. **Summary** — Report what was done

## Interest Criteria

**Relevant topics (Branchen-Mix):**
- Tech/Engineering: software, AI/ML, DevOps, open source, Rust, Flutter, cloud, infrastructure
- Business/Leadership: entrepreneurship, strategy, product management, team leadership, startups

**Skip signals:**
- Pure self-promotion without substance
- Motivational quotes / engagement bait ("Agree?", "Like if you...")
- Recruiting spam
- Posts the user already liked (`liked: true` in socialDetail)
- Reshares without commentary

**Boost signals (prefer these):**
- Posts from actual connections (vs. algorithmic suggestions)
- Low engagement count (commenting early = more visibility)
- Controversial or original theses
- Posts asking genuine questions

## Engagement Plan Format

```
## Engagement Plan (X von Y Posts relevant)

| # | Autor | Post (Preview) | Aktion | Kommentar |
|---|-------|----------------|--------|-----------|
| 1 | Jane Smith | "Wir haben unser Monitoring..." | LIKE + Kommentar | "Spannend, welches Tool nutzt ihr dafuer?" |
| 2 | Max M. | "Rust im Embedded-Bereich..." | LIKE | - |

Skipped: N Posts (reasons breakdown)

Soll ich den Plan so ausfuehren?
```

## Comment Style

- **Language:** Match the post's language (German post -> German comment, English -> English)
- **Tone:** Casual-collegial. Like a colleague quickly typing a response
- **Length:** 1-3 sentences, never more
- **Authenticity:** Occasional small imperfections (informal contractions like "grad" instead of "gerade", sometimes missing final punctuation, casual phrasing). Not artificially error-laden — just natural
- **Content:** Always reference the post's actual content. Add own experience or ask a question. Never generic ("Great post!", "Thanks for sharing!")
- **Reaction choice:** Default LIKE. Use CELEBRATION for achievements/milestones, EMPATHY for struggles/layoffs, PRAISE for impressive work

## Safety

- Inherits all rules from `/linkedin` skill (credentials, rate limits, sequential calls)
- The engagement plan IS the confirmation gate — nothing executes without explicit "go"
- User can edit any comment in the plan before approval
- User can change any action (upgrade Like to Comment, downgrade to Skip)

## File

`.claude/skills/linkedin/engage.md`
