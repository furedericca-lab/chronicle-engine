---
description: Single-contract scope for removing storage-internal reflection field naming from the backend memory table.
---

# storage-reflection-field-cleanup-2026-03-25 Contract

## Context

The public backend/runtime contract has already been cleaned up to behavioral-facing naming. The remaining reflection-era residue is now limited to storage-internal LanceDB field names and values such as:

- `category=reflection`
- `reflection_kind`
- `strict_key` values prefixed with `reflection:`

The user requested a new scope to clean up these storage-internal historical fields and then clarified the desired compatibility posture: do not preserve automatic reads of old reflection-named storage, and do not perform automatic rebuild migration for legacy reflection-era tables.

## Findings

- `backend/src/models.rs` still serialized behavioral category rows to storage as `reflection`.
- `backend/src/state.rs` still defined and wrote the LanceDB schema with a `reflection_kind` column and `reflection:` strict-key prefixes.
- Contract tests still seeded behavioral rows by mutating stored rows to legacy `reflection` names.
- Active docs still described storage-internal reflection names as retained implementation details, which no longer matches the desired cleanup target.

## Goals / Non-goals

Goals:

- Change storage schema and writes to use behavioral-facing internal names only.
- Reject legacy reflection-era LanceDB schemas instead of auto-reading or auto-migrating them.
- Preserve public behavioral-facing API behavior and current route names.
- Update active docs and backend tests to match the new storage boundary.

Non-goals:

- No new public HTTP routes or request-shape changes.
- No change to the current public behavioral recall lane naming.
- No attempt to redesign behavioral recall semantics beyond storage/internal naming cleanup.
- No in-place or automatic migration of legacy reflection-era LanceDB tables.

## Target files / modules

- `backend/src/models.rs`
- `backend/src/state.rs`
- `backend/tests/phase2_contract_semantics.rs`
- `docs/runtime-architecture.md`
- `docs/remote-memory-backend-2026-03-25/*`

## Constraints

- Legacy LanceDB tables containing reflection-era schema must fail clearly with a manual reset/migration requirement.
- Do not silently coerce persisted `category=reflection`, `reflection_kind`, or `reflection:*` strict keys into the new schema.
- Do not reintroduce any public reflection-named runtime surface.

## Verification plan

- `cargo test --manifest-path backend/Cargo.toml --test phase2_contract_semantics -- --nocapture`
- `npm test`
- `bash /root/.openclaw/workspace/skills/repo-task-driven/scripts/doc_placeholder_scan.sh docs/storage-reflection-field-cleanup-2026-03-25`
- `bash /root/.openclaw/workspace/skills/repo-task-driven/scripts/post_refactor_text_scan.sh docs/storage-reflection-field-cleanup-2026-03-25 README.md`
- `git diff --check`

## Rollback

- Restore the prior storage schema field names in code.
- Revert the legacy-schema rejection changes and updated tests/docs.
- Manual rollback would continue to require restoring or recreating any previously existing reflection-era LanceDB tables out of band.

## Open questions

- Keep public `kind` values as-is in this scope. If invariant/derived naming is ever revisited, that should be a separate contract because it is no longer a pure storage-field cleanup.
