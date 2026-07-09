# Cross-Runtime Causal Inspection

## What This Feature Is

Cross-runtime causal inspection is the **`CausalInspection` lane**: admit and request **cross-runtime causal explanations** with explicit richness and explanation-family support. It is **not** the same as `workspace.inspect`, which returns **per-target retained evidence** on the workspace inspection surface.

Domain **explanation contributions** declare how domains attach explanation posture—they do not replace this runtime inspection lane.

## Why You Use It

- explain causality across runtime boundaries with `CrossRuntimeCausalExplanation`
- explain temporal wakes, async completions, mixed-cause suppressions,
  preview remasks, replay drift, and resume mismatches without importing lower
  runtime bridge or signal types in product code
- choose reference-only vs materialized detail with honest advisory/deferred rows
- avoid overloading `workspace.inspect` for full cross-runtime envelopes
- align agent/docs language with `CausalInspection::support()` postures

## Core Mental Model

Three distinct surfaces:

| Surface | What you get |
|---------|----------------|
| `workspace.inspect` | Per-target retained inspection evidence ([inspection](inspection.md)) |
| `CausalInspection` | Cross-runtime causal explanation families, admission + request pipeline |
| Explanation contributions | Domain declaration posture ([explanation/](../domain-capabilities/explanation/)) |

Pipeline (facade: `admit_causal_inspection`, `request_causal_inspection`, materialization helpers):

```text
CausalInspection plan
  → admit_causal_inspection
  → request_causal_inspection (family + richness)
  → [optional] materialized detail (advisory)
  → artifacts + decision trace
```

Temporal and async-rich explanations now stay Query-owned on the materialized
artifact itself:

- `QueryCausalTemporalAsyncExplanation`
- `QueryCausalTemporalAsyncExplanationKind`

That summary is projected from the anchored observation reason plus retained
causal evidence families. It is not reconstructed by downstream code.

## Main Entry Points

`exports_runtime.rs` (representative):

- `CausalInspection`, `CausalInspectionPlan`, `CausalInspectionRequest`
- `admit_causal_inspection`, `request_causal_inspection`
- `CausalInspectionExplanationFamily`, `CausalInspectionRichness`
- `CausalInspection::support()` — `builder_support.rs` row postures
- builder helpers such as `why_temporal_wake()`, `why_async_completion()`,
  `why_remasked()`, and `why_resume_mismatch()`
- Materialization errors/policy types when narrowing to detail

Tests: `src/runtime/tests/causal_inspection/`.

## Typical Flow

1. Build a causal inspection plan for the cross-runtime question (targets, families).
2. `admit_causal_inspection` → admission receipt and counters.
3. `request_causal_inspection` with `CrossRuntimeCausalExplanation` and **ReferenceOnly** richness (common supported path).
4. If you need materialized detail, check advisory posture and materialization policy—may narrow until bridge envelope materialization.
5. Use `resolve_causal_evidence_references` when following reference sets—not durable archive replay.

When the question is specifically temporal or async:

- use the ordinary builder helpers when they fit the question
- inspect `artifact.temporal_async_explanation()` on admitted, advisory, or
  denied causal artifacts
- use the richer evidence reference set only when the compact Query-owned
  temporal/async summary is not enough

## How It Relates

- [Inspection](inspection.md) — retained per-target evidence; “what this is not”
- [Choosing: inspection vs cross-runtime explanation](../domain-capabilities/choosing/inspection-vs-cross-runtime-explanation.md)
- [Explanation contributions](../domain-capabilities/explanation/) — domain posture, not runtime causal lane
- [Lower-runtime capability routing](../domain-capabilities/lower-runtime-capability-routing.md) — bridge envelopes for materialization neighbors

## Good to Know

- `CrossRuntimeCausalExplanation` + **ReferenceOnly** is **Supported**.
- Same family + **MaterializedDetail** is **Advisory** (“narrows until bridge envelope materialization”).
- **DurableCausalArchive** and **StoreBackedReplayReconstruction** are **Deferred** for all richness levels.
- Temporal/async causal richness does not turn `workspace.inspect(...)` into a
  causal lane clone; it stays on `CausalInspection` artifacts.

## Anti-Patterns

- Calling `workspace.inspect` and documenting it as “cross-runtime causal inspection.”
- Expecting store-backed replay reconstruction from causal inspection APIs today.
- Using explanation contribution registration instead of admitting/requesting causal inspection when you need the runtime envelope.

## Current Limits

`CausalInspection::support()` (`builder_support.rs`):

| Family | Richness | Posture |
|--------|----------|---------|
| CrossRuntimeCausalExplanation | ReferenceOnly | **Supported** |
| CrossRuntimeCausalExplanation | MaterializedDetail | **Advisory** |
| DurableCausalArchive | ReferenceOnly | **Deferred** |
| StoreBackedReplayReconstruction | ReferenceOnly | **Deferred** |

## Related Docs

- [Inspection](inspection.md)
- [Inspection vs cross-runtime explanation (chooser)](../domain-capabilities/choosing/inspection-vs-cross-runtime-explanation.md)
- [Support matrix and admission](../foundations/support-matrix-and-admission.md)
- [Authoritative mutation evidence](authoritative-mutation-evidence.md) — causality on writes vs causal inspection reads
