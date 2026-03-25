---
description: Scope boundaries and milestones for backend-release-line-closeout-2026-03-25.
---

# backend-release-line-closeout-2026-03-25 Scope and Milestones

## In Scope

- Eliminate current backend Clippy release blockers.
- Clean up small runtime invariant handling issues in the backend.
- Reorganize `backend/tests/contract_semantics.rs` into feature-scoped modules.
- Preserve and rerun full backend and repo-level verification.

## Out of Scope

- No new backend features.
- No route or config surface expansion.
- No new storage migration work.
- No archive/commit/push in this scope unless explicitly requested later.

## Milestones

1. Baseline the current blockers and scope shape.
2. Remove all `clippy -D warnings` blockers from `backend/`.
3. Split the contract suite into feature modules while preserving the same test target.
4. Re-run backend tests, repo tests, and build hygiene checks.

## Dependencies

- `backend/src/error.rs`
- `backend/src/config.rs`
- `backend/src/lib.rs`
- `backend/src/state.rs`
- `backend/tests/contract_semantics.rs`

## Exit Criteria

- `cargo clippy --manifest-path backend/Cargo.toml --all-targets --all-features -- -D warnings` passes.
- `cargo test --manifest-path backend/Cargo.toml -- --nocapture` passes.
- `npm test` passes.
- `backend/target/` is removed from the worktree.
- Scope docs contain concrete execution and verification evidence.
