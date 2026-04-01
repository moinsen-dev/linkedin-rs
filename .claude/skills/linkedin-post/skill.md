---
name: linkedin-post
description: "Analyze a source code project and generate 3 German-language LinkedIn post variants promoting the idea. Two-phase workflow: analyze project -> generate variants -> user selects -> post via linkedin-cli."
user-invocable: true
---

# LinkedIn Post Generator

Analyze a source code project directory and generate 3 LinkedIn post variants in German that promote the project idea without revealing too much.

## Input

The user provides a directory path as argument: `/linkedin-post <directory>`

## Workflow

Follow these steps in order:

### Step 1: Validate Directory

Check that the provided directory exists and contains a software project (look for config files like package.json, pubspec.yaml, Cargo.toml, pyproject.toml, go.mod, or at least source code files).

If invalid, tell the user and stop.

### Step 2: Auth Check

```bash
linkedin-cli auth status --local
```

If no session found, tell the user to authenticate with `linkedin-cli auth login` and stop.

### Step 3: Analyze Project (Sub-Agent)

Spawn an **Explore sub-agent** to analyze the project directory. Give it this exact task:

> Analyze the project at `<directory>`. Read README, CLAUDE.md, PID/PRD docs, config files (package.json, pubspec.yaml, Cargo.toml, pyproject.toml, etc.), directory structure (max 2 levels), and 2-3 central files showing core logic.
>
> Produce a briefing in EXACTLY this format:
>
> ```
> PROBLEM: <What problem does this project solve? 1-2 sentences.>
> SOLUTION: <Solution approach at conceptual level. 2-3 sentences.>
> TECH_HIGHLIGHT: <One specific technical detail that would make developers curious. Be concrete — name a pattern, architecture choice, or clever mechanism.>
> STATUS: <One of: Early idea / Prototype / MVP / Production-ready>
> STACK: <Tech stack in one line, e.g. "FastAPI + Next.js + PostgreSQL + Claude API">
> KEY_INSIGHT: <The unexpected or clever insight behind the project — what makes this approach different from the obvious solution?>
> ```
>
> Be specific and concrete. No filler. No marketing language.

### Step 4: Generate 3 Post Variants

Using the briefing from Step 3, generate 3 LinkedIn post variants. Each variant follows the SAME structure but uses a DIFFERENT style.

#### Fixed Post Structure (all variants must follow this)

```
1. HOOK (1-2 sentences)
   → Problem as question or provocative thesis
   → MUST be compelling in the first 210 characters (LinkedIn collapse point)

2. CONTEXT (1 sentence)
   → Brief bridge: working on AI agent tooling for over a year,
     many ideas explored along the way, this is one of them
   → Keep it to ONE sentence, weave it naturally into the flow

3. PROBLEM DEEPENING (3-5 sentences)
   → Make the problem concrete with examples and pain points
   → Reader should recognize the problem from their own experience

4. SOLUTION APPROACH (3-5 sentences)
   → Conceptually what the project does
   → NOT the full feature scope — just the core idea
   → Enough to understand the direction, not enough to rebuild it

5. TECH HOOK (2-3 sentences)
   → One specific technical detail that sparks curiosity
   → Enough for technical people to sense real substance behind the idea

6. STATUS + OUTLOOK (1-2 sentences)
   → Where the project stands, what's next

7. CTA (1-2 sentences)
   → Varies per variant (see style matrix)
```

#### Style Variation Matrix

| Aspect | Variant 1 | Variant 2 | Variant 3 |
|--------|-----------|-----------|-----------|
| Hook | Provocative thesis | Personal experience | Rhetorical question |
| Tone | Direct, opinionated | Reflective, thoughtful | Pragmatic, hands-on |
| CTA | Open-inviting ("Schreib mir wenn dich das Thema interessiert") | Concrete ("Ich suche 1-2 Leute die Lust haben, das mitzubauen") | Discussion ("Was meint ihr, braucht das jemand?") |

#### Style Rules (NON-NEGOTIABLE)

1. **German language.** Du-Form, collegial tone. Like talking to a fellow developer over coffee.
2. **English technical terms stay English.** API, Clean Architecture, SSE, MVP, CI/CD — don't translate these.
3. **Target ~3000 characters.** Use the full LinkedIn limit. The post should feel substantial, not padded.
4. **No emojis.** Exception: if the post genuinely benefits from 1-2 structural bullets.
5. **No LinkedIn cliches.** NEVER write: "Ich bin dankbar", "Lasst uns connecten", "Wer kennt es nicht", "Agree?", "Thoughts?", or any variation.
6. **No Broetry.** No artificial line break after every sentence. Write flowing paragraphs.
7. **Sound human.** Like an experienced developer casually talking about a side project. Informal shortenings are OK ("grad" statt "gerade", "ne" statt "eine"). Sometimes skip the final period.
8. **Be specific.** Reference concrete details from the project. Never generic.
9. **Vary sentence structure.** Mix short and long sentences. Don't start consecutive sentences the same way.

### Step 5: Present Variants

Present all 3 variants clearly separated:

```
## Variante 1 (Provokante These / Offen-einladend)

<full post text>

---

## Variante 2 (Persoenliche Erfahrung / Konkret)

<full post text>

---

## Variante 3 (Rhetorische Frage / Diskussion)

<full post text>
```

Then ask: **"Welche Variante soll ich posten? Du kannst auch Teile kombinieren oder editieren."**

Wait for explicit user choice before proceeding.

### Step 6: Post

After the user selects (or provides an edited version):

```bash
linkedin-cli feed post "<SELECTED_TEXT>" --yes --json
```

Report the result. If the response contains a permalink, show it.

If posting fails, show the error and offer to retry or save the text to a file instead.

## Safety

- This skill inherits ALL rules from the `/linkedin` skill (credentials, rate limits, sequential calls).
- The variant selection is the confirmation gate. Nothing is posted without the user's explicit choice.
- The user can edit any variant, combine parts, or reject all and ask for regeneration.
- Default visibility is `ANYONE` (public). The user can request `CONNECTIONS_ONLY` before posting.
