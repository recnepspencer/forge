# Artifact Access Matrix

This is the shortest honest answer to "what can I expect to exist right now?"

`forge-signal` has three different ideas that are easy to conflate:

- canonical semantic truth
- eagerly retained rich artifacts
- reconstructed rich artifacts

They are not the same thing.

## The hard guarantee

These are the authoritative runtime facts in every supported policy:

- stable task IDs
- stable semantic segment IDs
- execution report summaries
- replay artifacts
- failure and rollback diagnostics
- enough compact semantic state to check deterministic equivalence

If you need the answer to "what really happened?", start here.

## Artifact behavior by policy

| Surface | `Operational` | `Development` | `Forensic` |
| --- | --- | --- | --- |
| Replay events | Retained | Retained | Retained |
| Execution report / stage records | Retained | Retained | Retained |
| Failure / rollback diagnostics | Retained | Retained | Retained |
| Explanation artifact | Usually reconstructed on demand | Retained eagerly | Retained eagerly |
| Provenance artifact | Usually reconstructed on demand | Retained eagerly | Retained eagerly |
| Deep history detail | Minimal | Richer | Richest |
| Forensic failure context | Minimal | Moderate | Maximal |

If you override retention manually, that override wins over the preset.

## Which API to call

If you know exactly what you want, use the explicit accessors.

### On `SignalGraph`

- `graph.retained_explanation_artifact(node)`
- `graph.reconstruct_explanation_artifact(node)`
- `graph.retained_provenance_artifact(node)`
- `graph.reconstruct_provenance_artifact(node)`

### On `SignalRuntime`

- `runtime.retained_explanation_artifact(node)`
- `runtime.reconstruct_explanation_artifact(node)`
- `runtime.retained_provenance_artifact(node)`
- `runtime.reconstruct_provenance_artifact(node)`

### Convenience calls

- `graph.explain(node)`
- `graph.explain_artifact(node)`
- `graph.provenance_artifact(node)`
- `runtime.explain(node)`

These are useful when you want "give me the best available answer", not "tell me exactly how this artifact was materialized."

## Recommended usage

### Production hot path

Prefer:

- retained accessors
- replay and report summaries

Do not make your fast path accidentally depend on expensive reconstruction.

### Debugging / incident response

Prefer:

- reconstructed accessors when retained artifacts are absent
- replay plus stable IDs as the source of truth

### Harness / CI / certification

Prefer:

- artifact helpers that report materialization mode
- canonical JSON capture of replay, report, explanation, and provenance

That way "retained" versus "reconstructed" becomes data, not folklore.

## Semantic equivalence

Two runs can still be semantically equivalent if:

- one retained explanation eagerly
- the other reconstructed the same explanation on demand

They are not equivalent because the retention mode matches. They are equivalent because the canonical semantic truth and derived artifact content match.

## Practical rule

If you are unsure:

1. trust replay plus stable IDs first
2. treat explanation/provenance as retained-or-derived views over that truth
3. use retained access only when you care about overhead
