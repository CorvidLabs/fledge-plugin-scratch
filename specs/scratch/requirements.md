---
spec: scratch.spec.md
---

## User Stories

- As a developer, I want disposable notes associated with my current repository but stored outside version control.

## Acceptance Criteria

### REQ-scratch-001

The default command SHALL resume the newest repository scratch or create one when none exists.

### REQ-scratch-002

New scratches SHALL be timestamped Markdown files stored under the repository's sanitized Fledge scratch bucket.

### REQ-scratch-003

List SHALL include only Markdown scratches in newest-first order.

### REQ-scratch-004

Open SHALL use a one-based recent index and reject zero or unavailable entries explicitly.

### REQ-scratch-005

Editor launch and failure SHALL be surfaced accurately without reporting an unsaved note as saved.

## Constraints

- Buckets use the nearest Git root's directory name rather than a remote repository identity.

## Out of Scope

- Synchronization, repository commits, rich-note indexing, and collaborative editing.
