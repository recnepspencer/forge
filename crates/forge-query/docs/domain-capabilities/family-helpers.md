# Family Helpers

## What This Feature Is

Family helpers are the domain-native front door for declaration families that
already have a stable generic Query path underneath.

Use them when you want the call site to read like the domain operation you are
actually performing, without rebuilding declaration entry, continuation
preparation, contribution composition, ordinary outcomes, or recovery yourself.

The current shipped slice is the geometry helper surface. It gives admitted
handles one family-gated helper lane for active-face selection continuation
preparation and declaration-scoped material-attachment composition.

## Why You Use It

- write domain-native handle calls instead of assembling generic helper input
  by hand
- keep family gating at compile time instead of relying on helper naming
  conventions
- lower onto the same generic signal-orchestration and
  contribution-composed-orchestration surfaces the rest of Query uses
- keep ordinary, checked, proof-visible, and recovery lanes available through
  the same canonical artifacts
- discover helper verbs through the same orchestration inventory and audit
  boundary as the generic public surfaces

## Stable Entry Points

Admitted-handle entry points:

- `family_helpers() -> ForgeQueryFamilyHelpers<'_, D, C>`
- `geometry_helpers() -> ForgeQueryGeometryFamilyHelpers<'_, D, C>`

Geometry helper types:

- `ForgeQueryFamilyHelpers<'a, D, C>`
- `ForgeQueryGeometryFamilyHelpers<'a, D, C>`
- `ForgeQueryGeometryMaterialAttachmentInput<D, I>`
- `ForgeQueryGeometryActiveFaceSelectionHelperFamily<D>`
- `ForgeQueryGeometryMaterialAttachmentHelperFamily<D>`

Geometry helper verbs:

- `progress_active_face_selection(...)`
- `prepare_preview_for_active_face_selection(...)`
- `prepare_preview_for_active_face_selection_outcome(...)`
- `prepare_preview_for_active_face_selection_checked(...)`
- `prepare_preview_for_active_face_selection_proof(...)`
- `prepare_runtime_route_for_active_face_selection(...)`
- `prepare_runtime_route_for_active_face_selection_outcome(...)`
- `prepare_runtime_route_for_active_face_selection_checked(...)`
- `prepare_runtime_route_for_active_face_selection_proof(...)`
- `prepare_current_truth_view_for_active_face_selection(...)`
- `prepare_current_truth_view_for_active_face_selection_outcome(...)`
- `prepare_current_truth_view_for_active_face_selection_checked(...)`
- `prepare_current_truth_view_for_active_face_selection_proof(...)`
- `prepare_historical_truth_view_for_active_face_selection(...)`
- `prepare_historical_truth_view_for_active_face_selection_outcome(...)`
- `prepare_historical_truth_view_for_active_face_selection_checked(...)`
- `prepare_historical_truth_view_for_active_face_selection_proof(...)`
- `orchestrate_material_attachment_for_active_face_selection(...)`
- `orchestrate_material_attachment_for_active_face_selection_outcome(...)`
- `orchestrate_material_attachment_for_active_face_selection_checked(...)`
- `orchestrate_material_attachment_for_active_face_selection_proof(...)`

Good to know:

- the current shipped family slice is geometry only
- helper-family marker traits are part of the stable boundary
- helpers return canonical Query artifacts, not helper-local result types

## Core Mental Model

Think of family helpers as typed request builders plus typed projections over
the generic Query engines.

They do two jobs:

1. narrow the public surface to one real declaration family
2. lower onto the canonical generic lane that already owns the semantics

That means the helper surface is intentionally narrower than the generic one.

For geometry helpers:

- `progress_active_face_selection(...)` still performs normal declaration
  progression
- continuation-style helpers lower onto
  `orchestrate_signal_compatibility(...)`
- material-attachment helpers lower onto
  `orchestrate_declaration_with_contributions(...)`

The important rule is:

- helpers improve expression, not meaning

If the generic path would produce `WrongWorld`, `Deferred`, `Prepared`,
`ContributionDenied`, or another typed result, the helper path should produce
the same posture through the same canonical surface.

## How It Executes

The current geometry helper lifecycle is:

1. start from an admitted configured handle
2. choose the family helper facade with `geometry_helpers()`
3. progress one family-gated declaration input with
   `progress_active_face_selection(...)`
4. choose the next helper lane:
   - preview continuation preparation
   - runtime-route continuation preparation
   - current truth-view preparation
   - historical truth-view preparation
   - material-attachment contribution composition
5. Query lowers that helper call into the generic canonical lane
6. ordinary, checked, proof-visible, and recovery behavior stay owned by the
   generic lane

The continuation-oriented helpers deliberately stop at signal-facing
orchestration. They do not skip directly to continuation execution, and they do
not hide declaration-entry failure behind one helper-local union type. You
still progress the declaration first, then ask for the next step explicitly.

## Small Example

```rust
let progressed = handle
    .geometry_helpers()
    .progress_active_face_selection(
        geometry_session.prepare_preview_for_active_face_selection()?,
    )?;

let outcome = handle
    .geometry_helpers()
    .prepare_preview_for_active_face_selection_outcome(progressed);

match outcome {
    ForgeQueryOrdinaryOutcome::Bound(orchestration) => {
        let _ = orchestration.class();
        let _ = orchestration.signal_execution_family();
    }
    other => {
        let _ = handle.recover_from_outcome(&other);
    }
}
```

This is the smallest honest helper example because it shows the real boundary:
you still progress the declaration first, then ask the helper to lower into the
signal-facing next-step lane.

## Real Example

```rust
let input = ForgeQueryGeometryMaterialAttachmentInput::new(
    geometry_session.attach_material_for_active_face_selection()?,
)
.with_support_contribution(
    ForgeQuerySupportContributionAuthoring::declaration_traceability(
        "geometry.trace",
        "track the face selection through material attachment",
    ),
)
.with_workflow_contribution(
    ForgeQueryWorkflowContributionAuthoring::preview_only(
        "geometry.preview",
        "keep this attachment in preview-only workflow posture",
    ),
);

let outcome = handle
    .geometry_helpers()
    .orchestrate_material_attachment_for_active_face_selection_outcome(input);

match outcome {
    ForgeQueryOrdinaryOutcome::Bound(composed) => {
        let _ = composed.envelope().envelope_digest();
        let _ = composed.contribution_composition().contribution_digest();
        let _ = composed.contributions()[0].semantic_posture();
    }
    other => {
        let brief = handle.recover_from_outcome(&other);
        let _ = brief.map(|value| value.recommended_action());
    }
}
```

What this example is showing:

- the helper input is only a family-native wrapper over the composed
  orchestration input
- support and workflow contributions still use the canonical contribution
  authoring types
- the bound result is still the canonical contribution-composed artifact
- recovery still goes through the shared recovery boundary

## How It Relates To Other Features

- [Configured Domain Handles](./configured-domain-handles.md) own the admitted
  handle that exposes `family_helpers()` and `geometry_helpers()`.
- [Signal Compatibility Orchestration](./signal-compatibility-orchestration.md)
  owns the continuation-oriented helper results. The geometry preview,
  runtime-route, and truth-view helper verbs are projections onto that surface.
- [Contribution-Composed Orchestration](./contribution-composed-orchestration.md)
  owns the material-attachment helper result. The helper input is a
  family-native wrapper around the generic composed input.
- [Ordinary Outcomes](./ordinary-outcomes.md) own the compact helper outcome
  lane when you choose the `..._outcome(...)` variants.
- [Recovery Boundary](./recovery-boundary.md) owns helper repair guidance when
  a helper lane stops.
- [Orchestration Inventory](./orchestration-inventory.md) owns the registry and
  audit coverage for helper verbs just like it does for the generic public
  orchestration surface.

Use the generic orchestration docs when you need the full cross-family model.
Use family helpers when you already know the declaration family and want the
call site to match that family directly.

## Inspection And Debugging

Use the helper facade to discover the family-scoped surface:

- `family_helpers()`
- `geometry_helpers()`

Use the returned canonical artifacts for the real inspection surface:

- signal-facing helper results:
  - `class()`
  - `signal_execution_family()`
  - `basis_families()`
  - `linked_artifacts()`
- material-attachment helper results:
  - `envelope()`
  - `contribution_composition()`
  - `contributions()`
  - `materialized_artifacts()`

Use the shared recovery boundary when a helper lane stopped:

- `recover_from_outcome(...)`
- `recover_from_signal_compatibility_checked(...)`
- `recover_from_signal_compatibility_proof(...)`
- `recover_from_contribution_composed_checked(...)`
- `recover_from_contribution_composed_proof(...)`

Use the orchestration inventory when you need to audit the shipped helper verbs
as data:

- `ForgeQueryOrchestrationSurfaceInventory::current()`
- `ForgeQueryOrchestrationInventoryAudit::current()`

## Anti-Patterns

- treating helper verbs as a second execution model instead of projections onto
  generic Query surfaces
- assuming helper-friendly naming means the helper can be called on any
  declaration family
- expecting helper progression to skip declaration-entry stop posture
- assuming the preview helper returns a prepared continuation directly
- inventing helper-local retry or denial handling instead of using ordinary
  outcomes and the recovery boundary
- using family helpers when you need a generic cross-family declaration tool

## Current Limits

- the current shipped helper slice is geometry-only
- the current geometry helpers cover active-face selection continuation
  preparation and material-attachment composition
- helper verbs still require real family marker traits at compile time
- continuation-oriented helpers currently stop at signal-compatibility
  orchestration; they do not execute continuation or Signal work
- grouped or multi-member family helper flows are not part of this slice yet

## Related Docs

- [Configured Domain Handles](./configured-domain-handles.md)
- [Signal Compatibility Orchestration](./signal-compatibility-orchestration.md)
- [Contribution-Composed Orchestration](./contribution-composed-orchestration.md)
- [Ordinary Outcomes](./ordinary-outcomes.md)
- [Recovery Boundary](./recovery-boundary.md)
- [Orchestration Inventory](./orchestration-inventory.md)
- [Declaration Family Taxonomy](./declaration-family-taxonomy.md)
- [Domain Capabilities](./README.md)
