# LinkedIn Post Skill — Design

**Date:** 2026-04-01
**Status:** Approved

## Overview

A skill (`/linkedin-post <directory>`) that analyzes a source code project and generates 3 German-language LinkedIn post variants promoting the project idea. After user selection, the chosen post is published via `linkedin-cli feed post`.

## Workflow

```
1. Validate directory (exists? has code?)
2. Analyze project → structured briefing (via Explore sub-agent)
3. Generate 3 post variants from briefing (~3000 chars each)
4. Present variants to user
5. User selects one (or edits)
6. After approval: post via `linkedin-cli feed post`
```

## Phase 1: Project Analysis

An Explore sub-agent reads the project directory and produces a structured briefing.

**What it reads:**
- README, CLAUDE.md, PID/PRD docs
- Config files (package.json, pubspec.yaml, Cargo.toml, pyproject.toml, etc.)
- Directory structure (max 2 levels deep)
- 2-3 central files that show the core logic

**Briefing output fields:**

| Field | Description |
|-------|-------------|
| `PROBLEM` | What problem does the project solve? (1-2 sentences) |
| `SOLUTION` | Solution approach at conceptual level (2-3 sentences) |
| `TECH_HIGHLIGHT` | One specific technical detail as curiosity hook |
| `STATUS` | Early idea / Prototype / MVP / Production-ready |
| `STACK` | Short tech stack summary (1 line) |
| `KEY_INSIGHT` | The unexpected or clever insight behind the project |

## Phase 2: Post Generation

3 variants are generated from the briefing, each following the same structure but with different style and CTA.

### Fixed Post Structure (all variants)

```
1. HOOK (1-2 sentences)
   → Problem as question or provocative thesis
   → Must compel reading (LinkedIn collapses after ~210 chars)

2. CONTEXT (1 sentence)
   → "Ich arbeite seit ueber einem Jahr an AI-Agent-Tooling und dabei..."
   → Bridge from personal work to the idea

3. PROBLEM DEEPENING (3-5 sentences)
   → Make the problem concrete, examples, pain points
   → Reader should nod: "Ja, kenne ich"

4. SOLUTION APPROACH (3-5 sentences)
   → Conceptually what the project does
   → NOT the full feature scope, just the core

5. TECH HOOK (2-3 sentences)
   → One specific technical detail that sparks curiosity
   → Enough for tech people to sense substance

6. STATUS + OUTLOOK (1-2 sentences)
   → Where the project stands, where it's headed

7. CTA (1-2 sentences)
   → Varies per variant (see style matrix below)
```

### Style Variation Matrix

| Aspect | Variant 1 | Variant 2 | Variant 3 |
|--------|-----------|-----------|-----------|
| Hook style | Provocative thesis | Personal experience | Rhetorical question |
| Tonality | Direct, opinionated | Reflective, thoughtful | Pragmatic, hands-on |
| CTA | Open-inviting ("Schreib mir wenn...") | Concrete ("Suche 1-2 Leute die...") | Discussion ("Braucht das jemand?") |

### Style Rules (NON-NEGOTIABLE)

- German language, Du-Form, collegial tone
- No emojis (unless post needs structural bullets, then sparingly)
- No LinkedIn cliches ("Ich bin dankbar...", "Lasst uns connecten")
- No artificial line breaks after every sentence (no "Broetry")
- English technical terms stay English (API, Clean Architecture, SSE, etc.)
- Sounds like an experienced developer casually talking about a side project
- Target length: ~3000 characters (LinkedIn limit)

## Phase 3: Posting

After user selects a variant (or edits one):

```bash
linkedin-cli feed post "<TEXT>" --yes --json
```

Confirmation with permalink after posting.

## Safety

- Nothing is posted without explicit user approval
- User can edit any variant before posting
- `--visibility ANYONE` is default (can be overridden)
- Inherits all rules from `/linkedin` base skill (credentials, rate limits)

## Skill Location

`.claude/skills/linkedin-post/skill.md` — user-invocable as `/linkedin-post <directory>`
