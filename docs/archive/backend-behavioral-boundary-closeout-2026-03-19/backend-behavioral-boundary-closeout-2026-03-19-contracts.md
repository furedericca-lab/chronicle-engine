---
description: API and schema contracts for backend-behavioral-boundary-closeout-2026-03-19.
---

# backend-behavioral-boundary-closeout-2026-03-19 Contracts

## Contract A: canonical naming target

After this scope, active code and active docs should converge on **behavioral-guidance** terminology across:

- backend HTTP routes
- backend-client DTO names
- debug/trace naming exposed to active tools
- internal temporary/session naming used by active runtime code

The scope should remove or justify any remaining active `reflection` terminology outside archive material.

## Contract B: compatibility policy frozen in Phase 1

Phase 1 chooses a **bounded dual-acceptance policy for HTTP routes plus storage-internal retention**:

1. active/canonical backend-client/tool/doc naming moves to behavioral-facing names;
2. old reflection-named HTTP routes may remain temporarily as thin compatibility aliases to the new canonical behavioral routes;
3. persisted field/category names may remain internal in this scope if active surfaces stop treating them as the canonical external contract.

No silent half-state is allowed.

## Contract C: data semantics cannot regress

Behavioral-guidance recall must continue to preserve current semantics:

- durable/invariant vs adaptive/derived selection behavior
- manual writes to backend-managed behavioral rows remain blocked
- generic recall exclusion/filter behavior remains correct
- stats/reporting still expose behavioral row counts accurately
- debug recall remains inspectable and internally consistent

## Contract D: active-path cleanup targets

At minimum, this scope must review and decide the fate of these active-path surfaces:

- `backend/src/lib.rs`
- `backend/src/models.rs`
- `backend/src/state.rs`
- `src/backend-client/client.ts`
- `src/backend-client/types.ts`
- `src/backend-tools.ts`
- `index.ts`
- `src/context/auto-recall-orchestrator.ts`
- `docs/runtime-architecture.md`
- `README.md`
- `README_CN.md`
- backend and Node integration tests that still assert `/v1/recall/reflection`, `reflection_kind`, `reflectionCount`, or related semantics

## Shared Types / Schemas

This scope must make an explicit decision for each currently active schema surface:

- route names: `recall/reflection`, `debug/recall/reflection`
- DTO fields: `reflectionCount`
- trace kind values: `reflection`
- persisted field names: `reflection_kind`
- persisted category values: `reflection`
- temporary/internal keys: `temp:memory-reflection`

For each item, the scope must classify it as:

- renamed
- retained intentionally
- supported as alias temporarily
- rejected as removed legacy syntax

## Error Model

If any old route/field/name is removed, failure behavior must be explicit and tested.

Examples:
- old route returns not found / invalid request by deliberate policy
- old config/runtime alias throws targeted removal guidance
- manual mutation of backend-managed behavioral rows remains rejected with behavioral-facing messaging

## Validation and Compatibility Rules

- Active docs must match the chosen compatibility policy.
- Archive docs may preserve historical reflection naming but must not be mistaken for current guidance.
- New tests must prove the chosen rename/compatibility behavior instead of relying on implied translation.
- If data migration is required, the scope must define rollback and re-run behavior before implementation starts.
