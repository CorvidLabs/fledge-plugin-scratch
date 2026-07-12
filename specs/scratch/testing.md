---
spec: scratch.spec.md
---

## Test Plan

### Unit Tests

- Sanitize repository bucket names.
- Return no scratches for a missing directory.
- Order Markdown scratches newest first and ignore other files.

### Integration Tests

- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test`
- `cargo build --release`
- Verify the help surface.
