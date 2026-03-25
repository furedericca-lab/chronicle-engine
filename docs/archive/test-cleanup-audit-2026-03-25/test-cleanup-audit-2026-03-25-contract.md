---
description: Single-contract cleanup scope for dead and overexposed test assets.
---

# test-cleanup-audit-2026-03-25 Contract

## Context

The current `test/` tree passes, but it contains at least two test assets with no execution-path references and one oversized helper module that exports more surface than the active tests consume.

## Findings

- `test/benchmark-fixtures.json` is only referenced by archived docs and is not loaded by any active test command or helper.
- `test/helpers/openclaw-extension-api-stub.mjs` is only referenced by archived docs and is not imported by active tests.
- `test/helpers/behavioral-guidance-reference.ts` exposes multiple parsing helpers that active tests do not import; some of those helpers are not used even internally.
- Multiple test files still duplicate fetch/mock/setup helpers, but that is a maintainability issue rather than dead code and can remain out of scope for this cleanup.

## Goals / Non-goals

Goals:
- Remove dead test-only files with no active execution references.
- Reduce the exported surface of `test/helpers/behavioral-guidance-reference.ts` to the symbols active tests actually use.
- Keep active docs and tests truthful after cleanup.

Non-goals:
- Broad test harness refactoring.
- Reorganizing active test topology or renaming test files.
- Rewriting archived historical scope docs beyond what is needed to avoid obviously stale file references.

## Target Files / Modules

- `test/benchmark-fixtures.json`
- `test/helpers/openclaw-extension-api-stub.mjs`
- `test/helpers/behavioral-guidance-reference.ts`
- `docs/archive/rust-rag-completion/*`
- `docs/archive/autorecall-governance-unification-2026-03-18/*`

## Constraints

- Keep the cleanup bounded and low-risk.
- Do not remove helpers still imported by active tests.
- Preserve archive value while avoiding misleading references to deleted active test assets.

## Verification Plan

- `npm test`
- `git diff --check`
- `bash /root/.openclaw/workspace/skills/repo-task-driven/scripts/doc_placeholder_scan.sh docs/archive/test-cleanup-audit-2026-03-25`
- `bash /root/.openclaw/workspace/skills/repo-task-driven/scripts/post_refactor_text_scan.sh docs/archive/test-cleanup-audit-2026-03-25 README.md`
- `rg -n "benchmark-fixtures\\.json|openclaw-extension-api-stub\\.mjs" .`

## Rollback

- Restore the deleted files from Git history.
- Re-export removed helper functions if a later test or fixture genuinely depends on them.

## Open Questions

- Whether the duplicated fetch/setup helpers should be consolidated into shared test utilities in a later scope.

## Implementation Updates

- Deleted `test/benchmark-fixtures.json` after confirming it had no active execution references.
- Deleted `test/helpers/openclaw-extension-api-stub.mjs` after confirming it had no active execution references.
- Reduced the exported surface of `test/helpers/behavioral-guidance-reference.ts` by converting externally unused helpers to internal functions and deleting fully unused slice/open-loop helpers.
- Updated archived docs that named the deleted assets so they now refer to historical benchmark fixtures / stubs generically rather than pointing at removed active paths.

## Evidence

- `npm test`
- `git diff --check`
- `bash /root/.openclaw/workspace/skills/repo-task-driven/scripts/doc_placeholder_scan.sh docs/archive/test-cleanup-audit-2026-03-25`
- `bash /root/.openclaw/workspace/skills/repo-task-driven/scripts/post_refactor_text_scan.sh docs/archive/test-cleanup-audit-2026-03-25 README.md`
- `rg -n "benchmark-fixtures\\.json|openclaw-extension-api-stub\\.mjs|extractBehavioralGuidanceOpenLoops|extractBehavioralGuidanceMappedMemoryItems|extractBehavioralGuidanceSlices|extractBehavioralGuidanceSliceItems" .`

## Outcome

- Cleanup completed with all active tests passing.
- Remaining follow-up is optional maintainability work: consolidate repeated test harness helpers into shared utilities.
