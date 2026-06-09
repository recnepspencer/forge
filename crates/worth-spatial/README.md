# worth-spatial

`worth-spatial` owns spatial vocabulary and semantic interpretation for Worth.
It does not present itself as the final runtime operating surface.

`forge-query` owns the runtime-facing lifecycle:

- declaration entry
- readiness and inspection
- receipts and retained artifacts
- workflow, preview, and recovery routing

## Public Surface

Use the namespaced facade for public semantic vocabulary and family-owned
Query-native entry surfaces:

- `worth_spatial::facade::refs`
- `worth_spatial::facade::refs`
- `worth_spatial::facade::anchor_binding`
- `worth_spatial::facade::binding`
- `worth_spatial::facade::rebinding`
- `worth_spatial::facade::placement`
- `worth_spatial::facade::bindings`
  This now carries only shared binding-site and anchor-carrier vocabulary.
- `worth_spatial::facade::neighborhood`
- `worth_spatial::facade::continuation`
- `worth_spatial::facade::inspection`
- `worth_spatial::facade::projection`
- `worth_spatial::facade::recovery`
- `worth_spatial::facade::support`
- `worth_spatial::facade::tolerance`

- `facade::anchor_selection` admits motion and constraint semantics
- `facade::placement` applies admitted motion or constraint semantics to
  placement and exposes placement admission/application helpers
- frame admission, witness resolution, birth scaffolding, arbitration, and
  direct binding/rebinding helpers are no longer public support entrypoints

## Ownership Split

`worth-spatial` owns:

- authored reference vocabulary
- witness and frame meaning
- declarative placement vocabulary
- placement semantics and family-owned application of admitted motion or
  constraint semantics
- spatial conflict and continuity meaning
- primitive-birth assessment meaning

`forge-query` owns:

- Query declaration entry
- runtime inspection and eligibility
- retained artifact and workflow progression
- recovery and continuation posture

## Example

```rust
use worth_spatial::facade::placement;

let placement_spec = placement::SpatialPlacementSpec::world().at([10.0, 0.0, 3.0]);

assert_eq!(placement_spec.origin(), [10.0, 0.0, 3.0]);
```

The important boundary is that public facade modules expose semantic vocabulary
and family-owned Query-native entry surfaces grouped by runtime responsibility.
Direct arbitration internals and witness-resolution internals are not public
runtime entry surfaces.

