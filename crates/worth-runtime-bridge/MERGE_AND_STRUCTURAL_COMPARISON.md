# Merge And Structural Comparison

This guide covers the bridge's advanced identity and comparison surfaces for
non-trivial history.

These are not everyday APIs.
They are public because some integrations genuinely need them.

## Merge-Aware Bridge Work

When truth history includes multiple parents, structural contradictions, or
authoritative lineage decisions, the bridge must remain explicit about:

- what merge shape it is interpreting
- which authority basis is in force
- whether structural advisory is consistent
- whether replay preserves the same merge meaning

Relevant public surfaces include:

- `MergeHistoryDeclaration`
- `AdmittedMergeHistoryContract`
- `BridgeMergeAuthorityBasis`
- `BridgeMergeConsumptionClass`
- `BridgeMergeDenialClass`
- `PublishedMergeContinuityArtifact`
- `PublishedMergeRemapArtifact`

## Structural Comparison

Structural comparison exists for jobs like:

- compare branch state without identity fusion
- detect exact versus ambiguous structural match
- support advisory remap
- keep branch comparison replay-safe

Relevant surfaces include:

- `StructuralIdentityDeclaration`
- `AdmittedStructuralComparisonContract`
- `StructuralComparisonMode`
- `StructuralFingerprint`
- `PublishedBranchComparisonArtifact`
- `PublishedStructuralRemapArtifact`

## Why This Layer Stays Advanced

Most readers should never need to think about merge ontology lowering or
structural fingerprint normalization on day one.

But when the job is branch comparison, merge interpretation, or remap under
history pressure, these surfaces are the honest public contract.

## Product Rule

The bridge should never flatten:

- merge semantics
- structural ambiguity
- remap decisions

into vague "history changed" behavior.

These are typed bridge decisions and should stay that way.
