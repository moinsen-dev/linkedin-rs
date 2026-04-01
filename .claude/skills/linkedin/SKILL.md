---
name: linkedin
description: "Interact with LinkedIn via linkedin-cli. Read profiles, messages, feed, connections, search, notifications. Write operations (posting, messaging, reacting) require explicit user confirmation."
user-invocable: true
---

# LinkedIn CLI

Wraps the `linkedin-cli` binary to let you interact with LinkedIn on behalf of the user. All API responses are consumed as JSON. Write operations that create visible content or contact other people require explicit user confirmation before execution.

## Prerequisites

- `linkedin-cli` must be on PATH. Install with `just install` from project root.
- The user must authenticate themselves: `linkedin-cli auth login --li-at <cookie>`. Never handle the `li_at` cookie value yourself.

## Rules

1. **Always use `--json`** on every command except `auth login`, `auth logout`, and `auth status --local`.
2. **Never echo credentials.** Never include `li_at` cookie values, JSESSIONID tokens, or session file contents in your output.
3. **Confirm before every WRITE and SOFT-WRITE operation.** Describe what you are about to do, show the exact command, and wait for the user's explicit approval. Do not infer consent from earlier conversation.
4. **One API call at a time.** Do not chain commands with `&&` or `;`. Run them as separate Bash invocations.
5. **Respect rate limits.** If a call returns a rate-limit error (429, CAPTCHA, "too many requests"), stop and tell the user. Do not retry automatically.
6. **Auth check first.** Before the first API call in a conversation, run `linkedin-cli auth status --local` to verify a session exists.

## Command Classification

| Tier       | Command                                                   | Notes                                |
| ---------- | --------------------------------------------------------- | ------------------------------------ |
| READ       | `auth status [--local]`                                   |                                      |
| READ       | `auth logout`                                             | Clears local session file            |
| READ       | `profile me --json`                                       |                                      |
| READ       | `profile view <id> --json`                                |                                      |
| READ       | `profile viewers --json`                                  |                                      |
| READ       | `messages list [--count N] [--before TS] --json`          | Cursor pagination (epoch ms)         |
| READ       | `messages read <id> [--before TS] --json`                 | Cursor pagination (epoch ms)         |
| READ       | `feed list [--count N] [--start N] --json`                |                                      |
| READ       | `connections list [--count N] [--start N] --json`         |                                      |
| READ       | `connections invitations [--count N] [--start N] --json`  |                                      |
| READ       | `search people <keywords> [--count N] [--start N] --json` |                                      |
| READ       | `search jobs <keywords> [--count N] [--start N] --json`   |                                      |
| READ       | `notifications list [--count N] [--start N] --json`       |                                      |
| SOFT-WRITE | `profile visit <id> --json`                               | Registers in "who viewed my profile" |
| WRITE      | `messages send <recipient> <text> --json`                 | Sends a real DM                      |
| WRITE      | `feed react <urn> [--type TYPE] --json`                   | Visible reaction                     |
| WRITE      | `feed unreact <urn> [--type TYPE] --json`                 | Removes a reaction                   |
| WRITE      | `feed comment <urn> <text> --yes --json`                  | Creates a real comment               |
| WRITE      | `feed post <text> [--visibility V] --yes --json`          | Creates a real post                  |
| WRITE      | `connections invite <id> [--message M] --json`            | Sends connection request             |
| WRITE      | `connections accept <id> --secret <s> --json`             | Accepts invitation                   |
| SETUP      | `auth login [--li-at <cookie>]`                           | User runs this themselves            |

## Command Reference

### Auth

```bash
linkedin-cli auth login --li-at <LI_AT>     # or set LINKEDIN_LI_AT env var
linkedin-cli auth status --local             # check locally, no API call
linkedin-cli auth status                     # live API check
linkedin-cli auth logout                     # clear session
```

### Profile

```bash
linkedin-cli profile me --json
linkedin-cli profile view <PUBLIC_ID> --json
linkedin-cli profile visit <PUBLIC_ID> --json          # SOFT-WRITE
linkedin-cli profile viewers --json
```

### Messages

```bash
linkedin-cli messages list [--count 10] [--before <EPOCH_MS>] --json
linkedin-cli messages read <CONVERSATION_ID> [--before <EPOCH_MS>] --json
linkedin-cli messages send <PUBLIC_ID> "<TEXT>" --json  # WRITE
```

### Feed

```bash
linkedin-cli feed list [--count 10] [--start 0] --json
linkedin-cli feed react <POST_URN> [--type LIKE] --json             # WRITE
linkedin-cli feed unreact <POST_URN> [--type LIKE] --json           # WRITE
linkedin-cli feed comment <POST_URN> "<TEXT>" --yes --json          # WRITE
linkedin-cli feed post "<TEXT>" [--visibility ANYONE] --yes --json  # WRITE
```

Reaction types: `LIKE`, `PRAISE`, `EMPATHY`, `INTEREST`, `APPRECIATION`, `ENTERTAINMENT`, `CELEBRATION`

Visibility: `ANYONE` (default), `CONNECTIONS_ONLY`

### Connections

```bash
linkedin-cli connections list [--count 10] [--start 0] --json
linkedin-cli connections invitations [--count 10] [--start 0] --json
linkedin-cli connections invite <PUBLIC_ID_OR_URN> [--message "<TEXT>"] --json  # WRITE
linkedin-cli connections accept <INVITATION_ID> --secret <SECRET> --json       # WRITE
```

The `--secret` value comes from `connections invitations --json` output.

### Search

```bash
linkedin-cli search people "<KEYWORDS>" [--count 10] [--start 0] --json
linkedin-cli search jobs "<KEYWORDS>" [--count 10] [--start 0] --json
```

### Notifications

```bash
linkedin-cli notifications list [--count 10] [--start 0] --json
```

## Write Operation Confirmation Protocol

Before any WRITE or SOFT-WRITE command:

1. State in plain language what you are about to do
2. Show the exact command
3. Wait for the user's explicit approval

Example:

> I'm about to send a message to **john-doe-123** saying "Thanks for connecting!".
>
> Command: `linkedin-cli messages send john-doe-123 "Thanks for connecting!" --json`
>
> Shall I proceed?

For `feed post`, always include the visibility setting in the confirmation.

## Error Handling

| Error                        | Action                                     |
| ---------------------------- | ------------------------------------------ |
| "No session found"           | Tell user to run `linkedin-cli auth login` |
| "session invalid or expired" | Tell user to re-authenticate               |
| Rate limit / 429 / CAPTCHA   | Stop immediately, inform user              |
| Network error                | Report error, do not retry                 |
| Missing `--yes` error        | Bug in invocation — add `--yes` flag       |

## Example Workflows

**Check messages and reply:**

1. `linkedin-cli auth status --local`
2. `linkedin-cli messages list --json`
3. `linkedin-cli messages read <id> --json`
4. Confirm with user -> `linkedin-cli messages send <recipient> "<text>" --json`

**Browse feed and react:**

1. `linkedin-cli auth status --local`
2. `linkedin-cli feed list --count 20 --json`
3. Confirm with user -> `linkedin-cli feed react <urn> --json`

**Review connection requests:**

1. `linkedin-cli auth status --local`
2. `linkedin-cli connections invitations --json`
3. Confirm with user -> `linkedin-cli connections accept <id> --secret <secret> --json`

**Research a person:**

1. `linkedin-cli auth status --local`
2. `linkedin-cli search people "Jane Smith" --json`
3. `linkedin-cli profile view <public_id> --json`
4. Optionally, confirm with user -> `linkedin-cli profile visit <public_id> --json`
