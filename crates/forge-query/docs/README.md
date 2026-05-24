# Forge Query Docs

`forge-query` docs are organized by category instead of as one flat folder.
Each category should contain one capability doc per public feature surface,
with examples folded into the owning capability page instead of split into
parallel example-only siblings.

## Categories

- `foundations/`
  - workspace and runtime-orientation docs
  - support posture and state surfaces
  - preview and branch session context
- `authoring/`
  - query authoring, composition, and reusable shape docs
- `runtime-surfaces/`
  - retained live and derived runtime surfaces plus their consumption paths
- `execution/`
  - execution-time runtime capabilities such as writes, effects, and intent
    admission
- `capabilities/`
  - higher-level capability surfaces such as inspection, projection
    consumption, existing-truth handling, historical basis work, lineage, and
    subscription diagnostics
- `modeling/`
  - aspect, authority-lane, and schema/modeling guidance
- `domain-capabilities/`
  - downstream domain entry and contribution surfaces for admission, support,
    invariants, workflow, continuity, aftermath, explanation, and
    certification
  - typed domain front doors and later domain-shaped Query surfaces

## Current Layout

- `foundations/workspace-overview.md`
- `foundations/downstream-runtime-integration.md`
- `foundations/support-matrix-and-admission.md`
- `foundations/state.md`
- `foundations/branches-and-previews.md`
- `authoring/query-expressions-and-result-shapes.md`
- `authoring/scopes-templates-saved-queries-and-view-shapes.md`
- `authoring/read-composition.md`
- `authoring/graph-composition-authoring.md`
- `runtime-surfaces/live-views.md`
- `runtime-surfaces/computed.md`
- `runtime-surfaces/reads-observe-materialize.md`
- `execution/writes-and-intents.md`
- `execution/effects.md`
- `execution/intent-admission.md`
- `capabilities/existing-truth.md`
- `capabilities/inspection.md`
- `capabilities/projection-consumption.md`
- `capabilities/historical-diff-and-basis.md`
- `capabilities/lineage-and-correspondence.md`
- `capabilities/subscription-selection-and-diagnostics.md`
- `modeling/aspects-and-authority-lanes.md`
- `modeling/schema-validation.md`
- `domain-capabilities/README.md`
- `domain-capabilities/platform-entry.md`
- `domain-capabilities/configured-domain-handles.md`
- `domain-capabilities/canonical-domain-declarations.md`


