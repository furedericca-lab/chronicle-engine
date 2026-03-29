---
description: Scope boundaries and milestones for the admin-plane merge-fix closeout.
---

# backend-admin-plane-merge-fix-2026-03-29 Scope and Milestones

## In Scope

- Restore the broken backend verification baseline after the merged admin-plane PR.
- Fix backend/frontend contract mismatches that make merged admin-plane routes unusable.
- Converge implemented route names and settings semantics back to the frozen `backend-admin-plane-2026-03-27` contract.
- Make `/admin` static asset serving work coherently in local runs, tests, and the runtime container image.
- Update active docs and task plans so future execution follows the repaired contract instead of the drifted merged state.

## Out of Scope

- New admin pages or dashboard expansions.
- Re-architecting the admin plane away from the bundled backend model.
- Replacing bearer-token admin auth with a new auth product.
- Major redesign of governance workflow beyond adding the missing idempotency and contract alignment.

## Milestones

- M1: Reproduce and document the current merged regressions with concrete commands and touched modules.
- M2: Restore backend/runtime verification by fixing config-schema fallout, route/DTO mismatches, and stubbed settings behavior.
- M3: Align the admin web client, deploy/runtime asset path, and active docs with the repaired route/config contract.
- M4: Re-run backend/web quality gates and record closeout evidence for archive-or-continue decision making.

## Dependencies

- Depends on the merged admin-plane implementation in `backend/src/admin/*`, `backend/web/*`, `backend/src/lib.rs`, `backend/src/config.rs`, and `deploy/Dockerfile`.
- Must stay aligned with the still-active parent scope under `docs/backend-admin-plane-2026-03-27/`.

## Exit Criteria

- `contract_semantics` and `admin_plane` both compile and pass.
- Settings write behavior is either fully implemented for the TOML source of truth or explicitly reduced to a truthful read-only contract; the merged fake-success state is removed.
- Recall Lab request/response payloads match across backend and frontend.
- Governance review/promote writes require idempotency and preserve audit logging.
- Active docs describe the repaired route names, settings behavior, and deploy asset-path behavior without contradiction.
