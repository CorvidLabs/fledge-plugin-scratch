---
module: scratch
version: 1
status: active
files:
  - src/main.rs

db_tables: []
depends_on: []
---

# Scratch

## Purpose

Create, resume, list, and open temporary Markdown notes scoped to the current repository without adding files to the repository itself.

## Public API

| Surface | Behavior |
|---------|----------|
| default | Resume the newest repository scratch or create one when none exists. |
| new | Create a timestamped Markdown scratch and open it. |
| list | List repository scratches newest first. |
| open | Open a one-based recent scratch index. |
| path | Print the repository's scratch directory. |

## Invariants

1. Scratch files live under the user's Fledge data directory, not in the repository.
2. Repository buckets derive from the nearest Git root name and unsafe filename characters become underscores.
3. Outside Git, scratches use the global bucket.
4. Only Markdown files participate in listing and resume.
5. Scratch ordering is newest modification time first.
6. Open indices are one-based and zero is rejected.
7. The configured editor is launched as a program plus arguments, with vi as fallback.
8. Editor failure is surfaced and never reported as a saved note.

## Behavioral Examples

```
Given a repository with an existing scratch note
When the developer runs scratch without a subcommand
Then the newest Markdown scratch is opened and its path is reported
```

## Error Cases

| Error | When | Behavior |
|-------|------|----------|
| HOME missing | The user data directory cannot be resolved | Report the missing environment requirement and exit non-zero. |
| Scratch directory unreadable | Existing notes cannot be enumerated | Surface the filesystem error. |
| Invalid index | Open uses zero or exceeds available notes | Explain the one-based range and exit non-zero. |
| Editor unavailable | The configured editor cannot launch | Surface the launch failure. |
| Editor failure | The editor exits non-zero | Report failure without printing a saved confirmation. |

## Dependencies

- Rust 1.89 or later
- User HOME directory
- Configured text editor or vi
- Git worktree for repository scoping, with global fallback

## Change Log

| Version | Date | Changes |
|---------|------|---------|
| 1 | 2026-07-12 | Document existing repository-scoped scratch lifecycle for SpecSync 5 adoption. |
