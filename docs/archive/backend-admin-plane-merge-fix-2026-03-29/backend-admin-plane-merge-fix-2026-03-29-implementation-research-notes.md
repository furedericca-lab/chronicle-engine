---
description: Implementation research notes for repairing the merged admin-plane regressions.
---

# backend-admin-plane-merge-fix-2026-03-29 Implementation Research Notes

## Baseline (Current State)

- `backend/tests/admin_plane.rs` passes, so the new surface has partial coverage.
- `backend/tests/contract_semantics.rs` does not compile after the merge because the test harness still initializes `ServerConfig` without `admin_assets_path`.
- `backend/web` builds, but the SPA still contains request/format mismatches versus the backend DTOs and handlers.

## Gap Analysis

- `backend/src/admin/routes.rs` `update_settings` parses TOML, emits audit, and returns a response, but it does not persist the config file.
- `backend/web/src/pages/Settings.tsx` loads backend config as JSON and sends the edited string back as `config_toml`, which is incompatible with the backend parser.
- `backend/web/src/api.ts` sends `mode_override` for admin recall simulation while `backend/src/admin/dto.rs` requires `mode`.
- `backend/src/admin/routes.rs` and `backend/src/state.rs` do not apply `Idempotency-Key` protection to governance review/promote mutations even though the active contract says admin mutations keep idempotency semantics.
- `backend/src/lib.rs` route names drifted from the parent admin-plane contract by using `_` and shortened nouns instead of the frozen path families.

## Candidate Designs and Trade-offs

- Repair toward the parent `backend-admin-plane-2026-03-27` contract:
  - pro: restores one canonical admin surface
  - con: requires route renames and test/doc updates now
- Revise the parent contract downward to match the merged implementation:
  - pro: fewer immediate edits
  - con: bakes accidental drift and fake-success behavior into the official contract

## Selected Design

- Treat `docs/backend-admin-plane-2026-03-27/` as the canonical contract and repair the merged implementation toward it.
- Keep TOML as the only editable runtime-config format.
- Prefer direct convergence over keeping dual field names or dual route names unless a shim is strictly required to recover the merged tree safely.

## Validation Plan

- Backend:
  - `cargo test --manifest-path backend/Cargo.toml --test contract_semantics -- --nocapture`
  - `cargo test --manifest-path backend/Cargo.toml --test admin_plane -- --nocapture`
  - `cargo clippy --manifest-path backend/Cargo.toml --all-targets --all-features -- -D warnings`
- Web:
  - `npm --prefix backend/web ci`
  - `npm --prefix backend/web run build`
- Docs:
  - `bash /root/.openclaw/workspace/skills/repo-task-driven/scripts/doc_placeholder_scan.sh docs/backend-admin-plane-merge-fix-2026-03-29`
  - `bash /root/.openclaw/workspace/skills/repo-task-driven/scripts/post_refactor_text_scan.sh docs/backend-admin-plane-merge-fix-2026-03-29 README.md`

## Risks and Assumptions

- If the team chooses not to implement atomic config writes in this repair scope, the page and API must be explicitly downgraded to truthful read-only behavior instead of preserving the merged fake-success state.
- Route renames may require synchronized backend, web, test, and doc edits in one batch to avoid transient drift.
