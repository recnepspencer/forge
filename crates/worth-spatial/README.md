# worth-spatial

`worth-spatial` owns spatial vocabulary and semantic interpretation for Worth.
It does not present itself as the final runtime operating surface.

`forge-query` owns the runtime-facing lifecycle:

- declaration entry
- readiness and inspection
- receipts and retained artifacts
- workflow, preview, and recovery routing

## Public Surface

Use the namespaced facade instead of a flat top-level API:

- `worth_spatial::facade::refs`
- `worth_spatial::facade::witness_catalog`
- `worth_spatial::facade::witness_resolution`
- `worth_spatial::facade::frames`
- `worth_spatial::facade::placement`
- `worth_spatial::facade::motion`
- `worth_spatial::facade::constraints`
- `worth_spatial::facade::lowering`
- `worth_spatial::facade::arbitration`
- `worth_spatial::facade::bindings`

## Ownership Split

`worth-spatial` owns:

- authored reference vocabulary
- witness and frame meaning
- placement, motion, and constraint semantics
- spatial conflict and continuity meaning
- primitive-birth planning and consequence meaning

`forge-query` owns:

- Query declaration entry
- runtime inspection and eligibility
- retained artifact and workflow progression
- recovery and continuation posture

## Example

```rust
use worth_spatial::facade::{arbitration, lowering, motion, placement};

let admitted_move = motion::admit_spatial_move(
    motion::SpatialMoveSpec::shape_origin().to([10.0, 0.0, 3.0]),
)?;

let declaration = lowering::lower_admitted_move_intent(
    placement::SpatialPlacementSpec::world(),
    &admitted_move,
)?;

assert_eq!(declaration.name(), "worth.spatial.lowered.move");

let analysis = arbitration::analyze_spatial_intent_conflict(
    arbitration::SpatialAuthoredActKind::Move,
    &[],
);

assert_eq!(
    analysis.preview_commit_disposition(),
    arbitration::SpatialIntentPreviewCommitDisposition::WouldAutoResolve(
        arbitration::SpatialIntentCandidate::MoveOnly
    )
);
```

The important boundary is that lowering hands off to Query declarations and
arbitration carries preview and continuity meaning without building a second
runtime platform.
