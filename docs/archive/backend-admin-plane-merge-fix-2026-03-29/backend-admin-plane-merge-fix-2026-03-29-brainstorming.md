---
description: Decision notes for repairing the merged admin-plane implementation without widening scope.
---

# backend-admin-plane-merge-fix-2026-03-29 Brainstorming

## Problem

- The merged admin-plane implementation is partially functional but regressed existing verification and drifted from the active contract in routes, DTOs, and settings semantics.

## Scope

- Repair the merged implementation back to the intended contract and restore the backend/web verification line.

## Constraints

- Keep the bundled admin-plane architecture.
- Keep TOML as the backend config source of truth.
- Avoid adding new feature scope under the cover of bug fixing.

## Options

- Repair toward the parent `backend-admin-plane-2026-03-27` contract.
- Revise the parent contract downward to match the merged implementation.

## Decision

- Repair toward the parent `backend-admin-plane-2026-03-27` contract.
- Keep TOML end to end for settings.
- Fix route/DTO mismatches directly instead of preserving both drifted and canonical names.

## Risks

- If route names change without synchronized frontend/test/doc updates, the tree can stay partially broken.
- If Settings is left half-implemented, operators will continue to see a misleading success path.

## Open Questions

- Whether this repair scope implements atomic config write now or explicitly downgrades settings to read-only remains an implementation decision, but the merged fake-success path is not allowed to remain.
