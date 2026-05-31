# Aspect Truth Flow

This note is the local developer map for Milestone 2 aspect semantics. Use it
when changing commit, history, query, or diagnostics code.

## Semantic center

The runtime owns one one-way aspect-truth pipeline:

`KindAspectDeclarations`
-> `LoweredAspectPlan`
-> `CanonicalRecordAspectDelta`
-> durable patch/commit encoding
-> history/query/lineage consumers
-> diagnostics, traces, and reports derived from those same artifacts

If a helper needs raw external document access after `CanonicalRecordAspectDelta`
exists, treat that as suspect. History, query, and diagnostics must not
re-derive aspect meaning from JSON or compatibility documents.

## Core types

Schema truth:

- `schema::data::KindAspectDeclarations`
- `schema::data::DeclaredAspect`
- `schema::data::AspectBinding`
- `forge_foundational::facade::AspectContract`
- `forge_foundational::facade::AspectKey`

Executable truth:

- `schema::data::LoweredAspectPlan`
- `schema::data::LoweredAspectBinding`
- `schema::data::AspectPlanCatalog`

Commit-time truth:

- `authority::mutation::canonical_deltas::CanonicalRecordAspectDelta`
- `authority::mutation::canonical_deltas::EvaluatedAspectBinding`
- ordered `forge_foundational::facade::AspectKey` lists
- `publication::patch::data::RecordStructuralChange`

Durable/public consumption:

- `publication::patch::data::PatchRecord`
- `transactions::data::CommitAspectSummary`
- `history::data::AspectHistoryOrigin`
- `history::data::AspectResolutionContext`
- `history::data::AspectHistoryEntry`

Trace/report views:

- `schema::data::AspectDeclarationTrace`
- `schema::data::AspectLoweringTrace`
- `transactions::data::AspectEvaluationTrace`
- `transactions::data::AspectEmissionTrace`
- `transactions::data::PatchVsTruthDeltaReport`
- `transactions::data::AspectTagAccuracyReport`
- `history::data::AspectHistoryResolutionTrace`
- `history::data::AspectHistoryDigest`
- `history::data::LineageAspectResolutionDigest`

## Guardrails

Keep these invariants intact:

- no post-delta external document rescans
- no second aspect-set construction path
- no diagnostics builder with raw external document access
- no lineage mutation of origin semantics
- no noncanonical emitted aspect collections
- no replay or patch parity path that ignores structural or degraded-precision flags

The test helpers in `tests/support.rs` encode the current Milestone 2 invariants
and should be reused instead of open-coding new semantic checks.
