---
description: Single-contract scope for removing remaining public reflection compatibility surfaces from the backend API.
---

# backend-public-reflection-surface-removal-2026-03-25 Contract

## Context

The current runtime architecture keeps `behavioral` as the canonical public concept, but the backend still exposes temporary public compatibility surfaces under `reflection` naming. The user requested full removal of those public aliases while preserving the storage-internal schema for data stability.

## Findings

- `backend/src/lib.rs` still mounts `/v1/recall/reflection` and `/v1/debug/recall/reflection`.
- `backend/src/models.rs` still accepts `"reflection"` as a public category value and `excludeReflection` as a public request-field alias.
- Current canonical runtime docs still describe reflection routes as temporary aliases.
- Storage internals still intentionally persist behavioral rows as `category=reflection` plus `reflection_kind`.

## Goals / Non-goals

Goals:
- Remove public reflection route aliases from the backend.
- Remove public request-schema aliases that still accept reflection naming.
- Update active docs and tests to match the cleaned public contract.

Non-goals:
- Migrating persisted storage from `category=reflection` / `reflection_kind`.
- Renaming LanceDB columns or rebuilding stored data.
- Broad backend retrieval redesign.

## Target Files / Modules

- `backend/src/lib.rs`
- `backend/src/models.rs`
- `backend/tests/phase2_contract_semantics.rs`
- `docs/runtime-architecture.md`
- `docs/remote-memory-backend-2026-03-18/remote-memory-backend-contracts.md`

## Constraints

- Keep storage-internal reflection naming unchanged in this scope.
- Preserve canonical behavioral routes and DTOs.
- Make failure behavior for removed public aliases explicit in tests.

## Verification Plan

- `cargo test --manifest-path backend/Cargo.toml --test phase2_contract_semantics -- --nocapture`
- `npm test`
- `git diff --check`
- `bash /root/.openclaw/workspace/skills/repo-task-driven/scripts/doc_placeholder_scan.sh docs/archive/backend-public-reflection-surface-removal-2026-03-25`
- `bash /root/.openclaw/workspace/skills/repo-task-driven/scripts/post_refactor_text_scan.sh docs/archive/backend-public-reflection-surface-removal-2026-03-25 README.md`

## Rollback

- Restore the removed routes and request aliases from Git history if a downstream runtime still depends on them.
- Keep storage-internal naming untouched so rollback does not require data migration.

## Open Questions

- Whether the top-level remote-backend design snapshot should continue documenting historical reflection route names after the canonical runtime contract removes them.

## Implementation Updates

- Removed `/v1/recall/reflection` and `/v1/debug/recall/reflection` from the backend router.
- Removed the public request aliases that accepted `"reflection"` as a category value and `excludeReflection` as a request field.
- Updated active top-level runtime/docs to keep only behavioral-named public recall routes.
- Kept storage-internal `category=reflection` and `reflection_kind` unchanged in this scope.

## Evidence

- `cargo test --manifest-path backend/Cargo.toml --test phase2_contract_semantics -- --nocapture`
- `npm test`
- `git diff --check`
- `bash /root/.openclaw/workspace/skills/repo-task-driven/scripts/doc_placeholder_scan.sh docs/archive/backend-public-reflection-surface-removal-2026-03-25`
- `bash /root/.openclaw/workspace/skills/repo-task-driven/scripts/post_refactor_text_scan.sh docs/archive/backend-public-reflection-surface-removal-2026-03-25 README.md`

## Outcome

- Public reflection-named backend recall surfaces are removed.
- Storage-internal reflection naming remains intentionally unchanged for data compatibility.
