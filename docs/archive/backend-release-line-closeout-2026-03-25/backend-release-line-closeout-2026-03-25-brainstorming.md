---
description: Brainstorming and decision framing for backend-release-line-closeout-2026-03-25.
---

# backend-release-line-closeout-2026-03-25 Brainstorming

## Problem

The backend was functionally green but not at release line:

- `cargo clippy --all-targets --all-features -D warnings` failed.
- production code still contained panic-style invariant checks in trace wrappers.
- `backend/tests/contract_semantics.rs` had grown into one large contract file with mixed concerns and shared scaffolding embedded inline.

## Scope

- Fix current backend release blockers without changing the public backend/runtime contract.
- Restructure `backend/tests/contract_semantics.rs` by feature area while preserving the same test target and coverage.
- Keep existing reflection-removal and behavioral storage boundary decisions intact.

## Constraints

- No reintroduction of any reflection-named public runtime surface.
- No weakening of auth/idempotency checks.
- Preserve the existing `cargo test --manifest-path backend/Cargo.toml --test contract_semantics -- --nocapture` entrypoint.

## Options

- Option A: patch only the current Clippy errors and leave the monolithic contract test file intact.
- Option B: split the contract tests into multiple integration-test files under `backend/tests/`, which would require a larger helper extraction and test target changes.
- Option C: keep the existing integration-test target file but split it into feature-scoped modules, with shared helper scaffolding remaining centralized at the top of the file.

## Decision

Choose Option C.

Reasoning:

- It reaches release line without changing the test target name or CI command shape.
- It lowers reader load by grouping tests into feature-scoped modules.
- It avoids a larger filesystem refactor while still making the contract suite maintainable.

## Risks

- Clippy fixes in `backend/src/state.rs` could accidentally alter retrieval behavior if done mechanically.
- Contract-test regrouping could break helper visibility or async mock setup ordering.

## Open Questions

- None for this scope. Archive/commit/push can happen after the user reviews the final state.
