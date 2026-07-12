---
spec: scratch.spec.md
---

## Context

Scratch provides fast project-adjacent notes without polluting working trees or requiring a note service.

## Related Modules

- Git root discovery.
- User Fledge data directory and configured editor.

## Design Decisions

- Store outside the repository so notes remain disposable and untracked.
- Reuse modification time for intuitive resume ordering.
