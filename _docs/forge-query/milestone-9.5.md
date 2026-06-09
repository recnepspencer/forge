# Milestone 9.5 Engineering Spec: Query Productization Debt Cleanup For Reuse, View Shapes, And Typed Consumption

> **Status:** Draft
>
> **Roadmap parent:** [forge_query_roadmap.md](./forge_query_roadmap.md)
>
> **Vision parent:** [forge_query_vision.md](./forge_query_vision.md)
>
> **Primary predecessors:** [milestone-9.4.md](./milestone-9.4.md), [milestone-9.3.4.md](./milestone-9.3.4.md), [milestone-9.3.8.md](./milestone-9.3.8.md)
>
> **Purpose:** close the remaining runtime-backed Query productization debt in reusable scope/template composition, core view-shape families, grouped composition, retained-artifact projection consumption, preserved temporal/async reuse, and raw runtime bootstrap before store-backed and durable milestones freeze those lanes as public truth.

## Goal

Turn the remaining admitted-but-debt Query productization lanes into honest
runtime-backed production surfaces so ordinary Query consumers can rely on
scope/template reuse, core view families, grouped composition, retained
artifact fact consumption, preserved temporal/async reuse, and a simple valid
read-runtime bootstrap without falling back to special-case seams,
custom bridge-backed scaffolding, or
support-profile debt markers.

## Why This Milestone Exists

The old roadmap split temporal and async work across `9.4` through `9.7`, but
that semantic closure now lives inside one merged [milestone-9.4.md](./milestone-9.4.md).
What remains is a different kind of problem: Query's ordinary productization
surface still exposes several lanes that are admitted, documented, and useful,
but still marked as `debt` or left structurally unfinished.

The concrete evidence is already visible:

- `application/tests.rs` still marks `named_scope_expansion` and
  `template_instantiation` as `debt`
- the same support/profile tests still mark the core view families `table`,
  `detail`, `inspector_detail_observed`, `inspector_detail_focused`, and
  `kanban_grouped` as `debt`
- grouped composition docs still call out grouped template/composition profile
  debt
- projection consumption still says retained derived-artifact and live-artifact
  bindings are not yet first-class source families
- scopes/templates/saved-query/view-shape docs still show that runtime-backed
  temporal/async reuse does not yet carry through the full preserved
  inspector/grouped reuse surface
- hostile Phase 28 read/runtime work still needed a custom minimal
  bridge-backed runtime/testing seam because the raw runtime layer does not yet
  expose a simple public "give me a valid read runtime" bootstrap

Without this milestone:

- store-backed and durable milestones will inherit half-hardened reuse and
  view-shape semantics as if they were final
- product consumers will keep using special-case pack/bind/decode seams where
  Query claims a typed fact-consumption lane
- hostile runtime-backed read tests will keep spending their effort on bridge
  and adapter assembly instead of the read-seam behavior they are supposed to
  certify
- support/profile output will keep advertising core product surfaces as
  admitted-but-debt instead of actually closed

## Governing Summaries

- `MENTALITY.md`: the hard part is not adding one more helper. The hard part is
  preventing public Query product lanes from claiming closure while still
  depending on special-case seams or structurally unfinished neighbors.
- `arch_laws.md`: composition, planning, reuse, support, and certification must
  each have one explicit authority path. Query cannot close debt by smearing
  logic into docs, helpers, and tests separately.
- `composition_laws.md`: scopes, templates, view families, grouped planning,
  retained-artifact consumption, preserved reuse, and raw runtime bootstrap are
  different products and need separate boundary homes.
- `domain_structure_laws.md`: reusable composition, view-shape meaning,
  retained-artifact fact extraction, and temporal/async reuse posture must stay
  explicit instead of disappearing into generic runtime support or helper bags.
- `perf_laws.md`: debt closure cannot hide broad rescans, reopen lower-source
  artifacts, or make retained-artifact fact extraction more expensive just to
  look cleaner. The common path must remain bounded and honest, and hostile
  runtime testing must not require large assembly tax just to reach an
  ordinary valid read-runtime state.
- `AI_README.md`: Query is the ordinary runtime and product facade. Reusable
  composition, inspection, support/admission, projection consumption, and
  continuation/reuse must extend the existing Query categories rather than
  inventing sidecar semantics.
- `forge_query_roadmap.md`: merged `9.4` closes temporal/async semantics first.
  This milestone belongs immediately after it so productization debt is removed
  before store-backed and durable milestones freeze it.
- `milestone-9.4.md`: preserved temporal/async meaning is now closed at the
  runtime-backed semantic layer. This milestone must keep that meaning intact
  when composition, view-shape reuse, and retained artifacts participate.
- `test-requirements.md`: Query closes only when hostile certification proves
  canonical meaning, typed rejection, exact counters, and narrow artifact
  comparison. This milestone must add full-build certification rather than
  treating docs or support markers as self-certifying.

## Adversarial Constraint

For the same canonical query declaration, scope/template expansion,
view-shape family, grouped composition posture, retained result artifact,
basis/remask posture, and preserved temporal/async reuse posture, Query must
produce the same canonical declaration identity, the same support/admission
posture, the same typed fact-consumption contract, and the same delivery/reuse
meaning regardless of whether the caller reaches the lane through direct
composition, grouped composition, inspector/grouped reuse, or retained
artifact consumption.

This milestone fails if any covered path:

- treats scope or template composition as string substitution instead of
  canonical declaration composition
- treats view shape as display-only sugar instead of planning, delivery, and
  reuse semantics
- forces consumers back onto special-case runtime-owned pack/bind/decode seams
  where Query claims a typed projection-consumption lane
- preserves temporal/async vocabulary but erases the runtime-backed meaning
  closed in [milestone-9.4.md](./milestone-9.4.md)
- or leaves any covered public lane unfinished at milestone close

## Product Decision Lock

- This is a debt-cleanup milestone, not a capability-expansion milestone.
- The goal is to close runtime-backed productization debt in already-visible
  Query lanes, not to invent store-backed or durable semantics early.
- Scope/template expansion remains canonical query composition, not host-local
  rewrite folklore.
- View-shape families remain part of planning, delivery, identity, and reuse
  semantics, not presentation-only sugar.
- Projection consumption remains the typed fact lane. Retained derived/live
  artifacts must participate through that ordinary product path for the covered
  families in this milestone.
- Preserved temporal/async reuse must carry forward merged `9.4` semantics; it
  may not erase time/async posture during saved-query, scope/template, or
  view-shape reuse.
- Raw runtime bootstrap must expose a simple public ordinary lane for a valid
  bridge-backed read runtime. Hostile certification should not need custom
  one-off runtime assembly just to reach the read seam.
- This milestone fully builds the runtime-backed product surface it names.
  Milestones `10` and `11` own additional store-backed and durable capabilities;
  they do not excuse unfinished runtime-backed work here.

## Phase Plan

### Phase 1: Scope And Template Composition Debt Closure Boundary

Close the explicit `named_scope_expansion:debt` and
`template_instantiation:debt` posture so reusable composition becomes a fully
admitted runtime-backed product lane rather than a public lane that still
claims debt in support/profile output.

**Relevant subsystems**
- `composition::scopes`
- `composition::templates`
- `composition::report`
- `runtime` read-composition support reporting
- `application` support/profile reporting

**Relevant Query docs**
- [Scopes, Templates, Saved Queries, And View Shapes](../../crates/forge-query/docs/authoring/scopes-templates-saved-queries-and-view-shapes.md)
- [Read Composition](../../crates/forge-query/docs/authoring/read-composition.md)
- [Support Matrix And Admission](../../crates/forge-query/docs/foundations/support-matrix-and-admission.md)

**Documentation follow-through**
- Revise the composition-facing docs in the same phase so they stop teaching
  scope/template composition as admitted debt.

**Relevant Query source surfaces**
- [composition/scopes/descriptor.rs](../../crates/forge-query/src/composition/scopes/descriptor.rs)
- [composition/scopes/expansion.rs](../../crates/forge-query/src/composition/scopes/expansion.rs)
- [composition/templates/descriptor.rs](../../crates/forge-query/src/composition/templates/descriptor.rs)
- [composition/templates/instantiation.rs](../../crates/forge-query/src/composition/templates/instantiation.rs)
- [composition/report/support_profile.rs](../../crates/forge-query/src/composition/report/support_profile.rs)
- [runtime/workspace_read_composition_support.rs](../../crates/forge-query/src/runtime/workspace_read_composition_support.rs)
- [runtime/read_composition_support_report.rs](../../crates/forge-query/src/runtime/read_composition_support_report.rs)
- [application/support/report.rs](../../crates/forge-query/src/application/support/report.rs)

**Relevant APIs and product surfaces**
- `ForgeQueryApplicationFacade::runtime_backed_default()`
- `facade.support_matrix()`
- `profile.admitted_scope_families()`
- `profile.admitted_template_families()`
- `profile.composition_statuses()`

**Shared crate usage**
- `forge-proof`: None.
- `forge-foundational`: None.

**Warnings**
- Do not close debt by hiding composition statuses from the public profile.
- Do not let host-local template binding or scope expansion rewrite canonical
  declaration meaning.

**Test requirements**
- Add a `Scope And Template Productization Debt Closure Test` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Prove that scope expansion and template instantiation normalize to the same
  canonical declaration identity as equivalent direct construction.
- Prove that support/profile output moves from `debt` to closed runtime-backed
  product readiness for the covered families.

**Engineering decisions**
- Scope and template reuse stay in the ordinary composition lane.
- Public support/profile output is part of the product surface and must be
  treated as authoritative.

**Open questions**
- None.

### Phase 2: Core View-Shape Productization Closure Boundary

Close the admitted core runtime-backed view families still marked `debt` so
`table`, `detail`, `inspector_detail_observed`, `inspector_detail_focused`,
and `kanban_grouped` become honest product lanes with runtime-backed planning,
delivery, and reuse posture.

**Relevant subsystems**
- `view_shape`
- `view_shape_live`
- identity-evolution inspector support
- `application` support/profile reporting

**Relevant Query docs**
- [Scopes, Templates, Saved Queries, And View Shapes](../../crates/forge-query/docs/authoring/scopes-templates-saved-queries-and-view-shapes.md)
- [Collections, Cursors, Ordering, And Aggregations](../../crates/forge-query/docs/authoring/collections-cursors-ordering-and-aggregations.md)

**Documentation follow-through**
- Revise the view-shape docs in the same phase so admitted core families stop
  being described as product debt.

**Relevant Query source surfaces**
- [view_shape/family.rs](../../crates/forge-query/src/view_shape/family.rs)
- [view_shape/planning.rs](../../crates/forge-query/src/view_shape/planning.rs)
- [view_shape/delivery.rs](../../crates/forge-query/src/view_shape/delivery.rs)
- [view_shape/grouped_planning.rs](../../crates/forge-query/src/view_shape/grouped_planning.rs)
- [view_shape/grouped_binding.rs](../../crates/forge-query/src/view_shape/grouped_binding.rs)
- [view_shape_live/family.rs](../../crates/forge-query/src/view_shape_live/family.rs)
- [view_shape_live/grouped_execution.rs](../../crates/forge-query/src/view_shape_live/grouped_execution.rs)
- [identity_evolution/inspector.rs](../../crates/forge-query/src/identity_evolution/inspector.rs)
- [application/support/report.rs](../../crates/forge-query/src/application/support/report.rs)

**Relevant APIs and product surfaces**
- `profile.admitted_view_families()`
- `profile.view_shape_statuses()`

**Shared crate usage**
- `forge-proof`: None.
- `forge-foundational`: None.

**Warnings**
- Do not mark a family closed just because it renders rows.
- Do not let focused-inspector and grouped lanes drift into separate local
  semantics under live or retained use.

**Test requirements**
- Add a `Core View Shape Productization Closure Test` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Prove that each admitted core view family has canonical planning, delivery,
  and support/profile parity rather than a single happy-path rendering demo.
- Prove exact status movement from `debt` to closed runtime-backed product
  readiness for every covered family.

**Engineering decisions**
- Core view families are ordinary product surfaces, not examples.
- View-shape closure includes support/profile honesty, not just planner output.

**Open questions**
- None.

### Phase 3: Grouped Composition Closure Boundary

Close the explicit grouped template/composition profile debt so grouped
planning and grouped reuse become a fully built productization lane.

**Relevant subsystems**
- `grouped_authoring`
- grouped view-shape planning/binding
- composition support/profile reporting

**Relevant Query docs**
- [Collections, Cursors, Ordering, And Aggregations](../../crates/forge-query/docs/authoring/collections-cursors-ordering-and-aggregations.md)
- [Scopes, Templates, Saved Queries, And View Shapes](../../crates/forge-query/docs/authoring/scopes-templates-saved-queries-and-view-shapes.md)

**Documentation follow-through**
- Remove the grouped template/composition debt wording from the authoring docs
  in the same phase that closes the implementation debt.

**Relevant Query source surfaces**
- [grouped_authoring/declaration.rs](../../crates/forge-query/src/grouped_authoring/declaration.rs)
- [grouped_authoring/orchestration.rs](../../crates/forge-query/src/grouped_authoring/orchestration.rs)
- [grouped_authoring/posture.rs](../../crates/forge-query/src/grouped_authoring/posture.rs)
- [grouped_authoring/products.rs](../../crates/forge-query/src/grouped_authoring/products.rs)
- [grouped_authoring/support.rs](../../crates/forge-query/src/grouped_authoring/support.rs)
- [view_shape/grouped_planning.rs](../../crates/forge-query/src/view_shape/grouped_planning.rs)
- [view_shape/grouped_policy.rs](../../crates/forge-query/src/view_shape/grouped_policy.rs)

**Relevant APIs and product surfaces**
- grouped authoring declarations and grouped support/profile outputs carried
  through the existing Query authoring facade

**Shared crate usage**
- `forge-proof`: None.
- `forge-foundational`: None.

**Warnings**
- Do not close grouped debt by routing grouped composition through hidden
  one-off ordinary collection plans.
- Do not let grouped reuse erase view-family identity or grouping posture.

**Test requirements**
- Add a `Grouped Composition Debt Closure Test` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Prove grouped declarations preserve canonical grouped identity across direct
  and reusable composition paths.
- Prove grouped product docs and support/profile surfaces no longer describe
  the admitted lane as composition debt.

**Engineering decisions**
- Grouped composition is its own productization boundary, not just a view-shape
  subcase.
- Documentation wording counts as part of the closure surface.

**Open questions**
- None.

### Phase 4: Retained-Artifact Projection-Consumption Source Closure Boundary

Close the gap where retained derived-artifact bindings and live-artifact
bindings still fall back to special-case runtime-owned seams instead of
participating as first-class projection-consumption source families.

**Relevant subsystems**
- `projection_consumption`
- retained/live artifact binding seams
- authorized projection and fact extraction

**Relevant Query docs**
- [Projection Consumption](../../crates/forge-query/docs/capabilities/projection-consumption.md)
- [Reads, Observation, and Materialization](../../crates/forge-query/docs/runtime-surfaces/reads-observe-materialize.md)

**Documentation follow-through**
- Update projection-consumption docs in the same phase so retained derived/live
  artifact bindings are documented as first-class source families in the
  covered ordinary product lane.

**Relevant Query source surfaces**
- [projection_consumption/source.rs](../../crates/forge-query/src/projection_consumption/source.rs)
- [projection_consumption/contracts.rs](../../crates/forge-query/src/projection_consumption/contracts.rs)
- [projection_consumption/facts.rs](../../crates/forge-query/src/projection_consumption/facts.rs)
- [projection_consumption/extraction/mod.rs](../../crates/forge-query/src/projection_consumption/extraction/mod.rs)
- [projection_consumption/extraction/grouped.rs](../../crates/forge-query/src/projection_consumption/extraction/grouped.rs)
- [projection_consumption/extraction/query_context.rs](../../crates/forge-query/src/projection_consumption/extraction/query_context.rs)
- [projection_consumption/receipt.rs](../../crates/forge-query/src/projection_consumption/receipt.rs)
- [projection_consumption/envelope.rs](../../crates/forge-query/src/projection_consumption/envelope.rs)

**Relevant APIs and product surfaces**
- `workspace.read(...)`
- `workspace.observe(...)`
- `workspace.materialize(...)`
- runtime-owned seams that must stop being the ordinary product path:
  - `consume_scalar_fields(...)`
  - `decode_row_pair(...)`
  - `decode_row_triple(...)`
  - `verify_scalar_alignment(...)`
  - `read_live_artifact_bundle(...)`
  - `bind_live_artifact(...)`
  - `read_live_artifact_binding(...)`

**Shared crate usage**
- `forge-proof`: None.
- `forge-foundational`: None.

**Warnings**
- Do not "close" this by wrapping the same runtime-owned escape hatch in a new
  name.
- Do not reopen lower-source artifacts by hand where Query already claims a
  typed fact lane.

**Test requirements**
- Add a `Retained Artifact Projection Consumption Closure Test` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Prove retained derived-artifact and live-artifact fact extraction flows
  through one typed declaration, contract, extraction, receipt, and envelope
  path when admitted.
- Prove ordinary product code no longer needs the special-case runtime-owned
  seams above for the covered families.

**Engineering decisions**
- Retained derived/live artifacts become real source families in the covered
  ordinary product lane.
- Projection consumption stays receipt/envelope based rather than turning into
  row-bag reinterpretation.

**Open questions**
- None.

### Phase 5: Preserved Temporal/Async Reuse Neighbor Closure Boundary

Close the preserved inspector/grouped temporal/async reuse neighbors so
runtime-backed reuse keeps the merged `9.4` meaning intact across the full
covered reuse surface.

**Relevant subsystems**
- saved-query reuse
- policy-basis saved reuse
- preview/runtime temporal-async preserved reuse
- view-shape and grouped preserved reuse

**Relevant Query docs**
- [Scopes, Templates, Saved Queries, And View Shapes](../../crates/forge-query/docs/authoring/scopes-templates-saved-queries-and-view-shapes.md)
- [Historical Basis, Diff, And Comparison Queries](../../crates/forge-query/docs/capabilities/historical-diff-and-basis.md)
- [Automatic Subscription Family Selection And Diagnostics](../../crates/forge-query/docs/capabilities/subscription-selection-and-diagnostics.md)

**Documentation follow-through**
- Replace the unfinished wording with built runtime-backed preserved reuse
  semantics that carry the merged `9.4` meaning all the way through.

**Relevant Query source surfaces**
- [saved_query/reuse/matrix.rs](../../crates/forge-query/src/saved_query/reuse/matrix.rs)
- [saved_query/future_support.rs](../../crates/forge-query/src/saved_query/future_support.rs)
- [policy_basis/saved_reuse.rs](../../crates/forge-query/src/policy_basis/saved_reuse.rs)
- [runtime/tests/preview/temporal_async.rs](../../crates/forge-query/src/runtime/tests/preview/temporal_async.rs)
- [query_context/scoped.rs](../../crates/forge-query/src/query_context/scoped.rs)
- [view_shape/family.rs](../../crates/forge-query/src/view_shape/family.rs)
- [grouped_authoring/posture.rs](../../crates/forge-query/src/grouped_authoring/posture.rs)

**Relevant APIs and product surfaces**
- preserved saved-query and view-shape reuse posture carried through the
  existing Query saved/reuse and preview surfaces

**Shared crate usage**
- `forge-proof`:
  - `forge_proof::facade::TransitionReadiness`
  - `forge_proof::facade::TransitionOutcome`
  Use these only where preserved reuse and readmission already cross typed
  reuse/admission progression boundaries. Do not introduce a second local
  result family.
- `forge-foundational`: None.

**Warnings**
- Do not preserve the vocabulary while erasing the merged `9.4` temporal/async
  meaning.
- Do not let inspector/grouped preserved reuse remain partially built at
  milestone close.

**Test requirements**
- Add a `Preserved Temporal Async Reuse Closure Test` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Prove preserved runtime-backed reuse keeps canonical temporal/async posture
  through the covered inspector and grouped reuse lanes.
- Prove grouped and inspector preserved neighbors do not silently downcast to
  plain non-temporal/non-async reuse.

**Engineering decisions**
- Runtime-backed preserved reuse must be semantically strict, not
  vocabulary-preserving only.
- The covered preserved neighbors are part of the build target, not optional
  future follow-on work.

**Open questions**
- None.

### Phase 6: Raw Runtime Read Bootstrap Closure Boundary

Close the raw runtime bootstrap debt so hostile read/runtime certification can
reach a valid bridge-backed read runtime through one simple public ordinary
lane instead of building custom minimal bridge-backed harnesses around backend
parts and adapter assembly.

**Relevant subsystems**
- `runtime` builder and backend-parts assembly
- raw runtime bridge-backed support seams
- public bridge-backed runtime test support

**Relevant Query docs**
- [Workspace Overview](../../crates/forge-query/docs/foundations/workspace-overview.md)
- [Reads, Observation, and Materialization](../../crates/forge-query/docs/runtime-surfaces/reads-observe-materialize.md)
- any raw runtime/bootstrap docs or examples that currently force full backend
  part assembly for ordinary read-lane work

**Documentation follow-through**
- Document the simple valid read-runtime bootstrap in the same phase so
  downstream hostile tests and examples stop teaching custom assembly as the
  ordinary path.

**Relevant Query source surfaces**
- [runtime/builder.rs](../../crates/forge-query/src/runtime/builder.rs)
- [runtime/backend/parts.rs](../../crates/forge-query/src/runtime/backend/parts.rs)
- [runtime/error.rs](../../crates/forge-query/src/runtime/error.rs)
- [tests/support/public_bridge_runtime/mod.rs](../../crates/forge-query/tests/support/public_bridge_runtime/mod.rs)
- [runtime/tests/support/bridge/runtime_support.rs](../../crates/forge-query/src/runtime/tests/support/bridge/runtime_support.rs)
- [runtime/tests/support/stateful_bridge_runtime/mod.rs](../../crates/forge-query/src/runtime/tests/support/stateful_bridge_runtime/mod.rs)

**Relevant APIs and product surfaces**
- `ForgeQueryRuntime::builder()`
- `runtime_bridge(...)`
- `build_backend_from_parts()`
- the raw runtime ordinary lane that should become the simple public valid
  read-runtime bootstrap for bridge-backed hostile testing

**Shared crate usage**
- `forge-proof`: None.
- `forge-foundational`: None.

**Warnings**
- Do not "solve" this only inside test support. The debt is at the raw runtime
  productization boundary.
- Do not add a magical one-off helper that bypasses the real runtime authority
  path or hides support posture.
- Do not make write, preview, or mutation authority an implicit requirement for
  a simple valid read-runtime bootstrap if the read seam does not actually need
  them.

**Test requirements**
- Add a `Raw Runtime Read Bootstrap Simplicity Test` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Prove hostile runtime-backed read tests can obtain a valid bridge-backed read
  runtime through one simple public path without custom backend-part assembly.
- Prove the bootstrap still preserves typed support posture and does not create
  a second authority path beside the real runtime builder.

**Engineering decisions**
- A valid read-runtime bootstrap is part of Query productization, not just test
  ergonomics.
- The ordinary raw runtime path must be simple enough that hostile tests can
  focus on read behavior instead of bridge assembly.

**Open questions**
- None.

### Phase 7: Support/Profile, Docs, And Debt-Marker Eradication Boundary

Remove the remaining public debt wording across support/profile and docs so the
ordinary Query product surface stops advertising admitted core lanes as debt.

**Relevant subsystems**
- `application` support/profile reporting
- runtime support/profile output
- public documentation coverage

**Relevant Query docs**
- [Support Matrix And Admission](../../crates/forge-query/docs/foundations/support-matrix-and-admission.md)
- [Workspace Overview](../../crates/forge-query/docs/foundations/workspace-overview.md)
- the authoring and capability docs touched in Phases 1 through 5

**Documentation follow-through**
- This phase is the doc closeout pass. Every remaining `debt` marker or
  unfinished wording in the covered productization lanes must be removed
  because the implementation is closed in the earlier phases.

**Relevant Query source surfaces**
- [application/support/report.rs](../../crates/forge-query/src/application/support/report.rs)
- [application/support/registry.rs](../../crates/forge-query/src/application/support/registry.rs)
- [runtime/support/profile.rs](../../crates/forge-query/src/runtime/support/profile.rs)
- [public_doc_coverage/tests/support.rs](../../crates/forge-query/src/public_doc_coverage/tests/support.rs)

**Relevant APIs and product surfaces**
- `facade.support_matrix()`
- public support/profile reports and documentation coverage surfaces that teach
  the ordinary Query product lanes

**Shared crate usage**
- `forge-proof`: None.
- `forge-foundational`:
  - `forge_foundational::facade::DiagnosticRichnessProfile`
  Use this for debt-closeout reporting and support/profile publication richness.
  Do not introduce a Query-local diagnostic richness taxonomy for the same job.

**Warnings**
- Do not remove `debt` text before the underlying lane is actually built.
- Do not let docs, support profiles, and product surfaces disagree about the
  same lane.

**Test requirements**
- Add a `Debt Marker Eradication And Support Profile Honesty Test` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Prove the covered support/profile rows and public docs agree exactly on the
  runtime-backed posture of scopes, templates, core views, grouped composition,
  retained-artifact projection consumption, and preserved temporal/async reuse.

**Engineering decisions**
- Public docs and support/profile output are authority surfaces here.
- This milestone is incomplete if wording and implementation drift.

**Open questions**
- None.

### Phase 8: Hostile Debt-Close Certification Boundary

Close the milestone with one hostile certification program proving that the
covered productization lanes are truly closed rather than merely reworded.

**Relevant subsystems**
- application support/profile certification
- view-shape certification
- projection-consumption certification
- saved-query/reuse certification
- public documentation coverage

**Relevant Query docs**
- [test-requirements.md](./test-requirements.md)
- all phase-local docs touched in Phases 1 through 7

**Documentation follow-through**
- The milestone closes only when the docs, support/profile output, and hostile
  certification all agree. There is no later documentation cleanup phase.

**Relevant Query source surfaces**
- [application/tests.rs](../../crates/forge-query/src/application/tests.rs)
- [projection_consumption/certification/mod.rs](../../crates/forge-query/src/projection_consumption/certification/mod.rs)
- [view_shape/tests.rs](../../crates/forge-query/src/view_shape/tests.rs)
- [saved_query/tests.rs](../../crates/forge-query/src/saved_query/tests.rs)
- [public_doc_coverage/tests/support.rs](../../crates/forge-query/src/public_doc_coverage/tests/support.rs)
- [tests/support/public_bridge_runtime/mod.rs](../../crates/forge-query/tests/support/public_bridge_runtime/mod.rs)
- [runtime/tests/support/bridge/runtime_support.rs](../../crates/forge-query/src/runtime/tests/support/bridge/runtime_support.rs)

**Relevant APIs and product surfaces**
- the support/profile, declaration, projection-consumption, view-shape, and
  preserved reuse certification bundles emitted by the covered Query surfaces
- the raw runtime valid-read bootstrap surface used by hostile runtime-backed
  read certification

**Shared crate usage**
- `forge-proof`: None.
- `forge-foundational`:
  - `forge_foundational::facade::FoundationalBoundaryArtifactCompileFailBoundary`
  - `forge_foundational::facade::FoundationalBoundaryEvidenceCompileFailBoundary`
  - `forge_foundational::facade::FoundationalPerformanceCompileFailBoundary`
  - if milestone-close artifacts are emitted, the matching
    `...ProductionTestReadyArtifact` surfaces

**Warnings**
- Do not close the milestone on broad support-report equality.
- Do not certify only one axis at a time; the hostile matrix must combine
  composition path variation, view-family variation, retained-artifact
  extraction, temporal/async preserved reuse pressure, and raw runtime
  bootstrap pressure.

**Test requirements**
- Add a `Milestone 9.5 Debt-Close Hostile Certification Matrix` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Require narrow canonical artifacts for:
  - composition digest
  - view-family digest
  - projection-consumption contract digest
  - preserved temporal/async reuse digest
  - raw runtime read-bootstrap digest
  - support-profile digest
- Require exact zero assertions for forbidden fallback to the special-case
  runtime-owned retained-artifact seams in the ordinary product path.
- Require exact zero assertions for forbidden custom bootstrap scaffolding in
  the ordinary hostile read-runtime path.

**Engineering decisions**
- This milestone closes on hostile proof, not on removal of `debt` strings.
- Certification must prove ordinary product consumers can stay on the intended
  Query path without hand-written glue.

**Open questions**
- None.

## Must Ship

- runtime-backed closure for named-scope expansion and template instantiation
- runtime-backed closure for the admitted core view families `table`, `detail`,
  `inspector_detail_observed`, `inspector_detail_focused`, and
  `kanban_grouped`
- grouped composition closure with no remaining explicit composition debt in
  the admitted public lane
- first-class projection-consumption source-family closure for retained
  derived-artifact bindings and live-artifact bindings
- preserved inspector/grouped temporal/async reuse closure across the covered
  runtime-backed reuse surface
- simple public raw runtime bootstrap for a valid bridge-backed read runtime so
  hostile tests can focus on read-seam behavior instead of custom assembly
- support/profile, docs, and hostile certification closure for all covered debt
  families

## Must Preserve

- canonical declaration identity across direct construction and reusable
  composition
- view-family meaning as planning, delivery, identity, and reuse semantics
- projection consumption as the typed fact lane rather than row-bag folklore
- merged `9.4` runtime-backed temporal/async meaning across preserved reuse
- honest runtime-backed-first posture without leaking fake durable/store-backed
  claims early

## Acceptance Evidence

This milestone is complete only when `forge-query` can prove:

- the relevant Milestone `9.5` certification suites added to
  [test-requirements.md](./test-requirements.md) all pass with narrow
  machine-checkable artifacts
- `application/tests.rs` no longer reports the covered scope/template and
  core view-family rows as `debt`
- grouped composition docs and support/profile surfaces no longer describe the
  admitted grouped planning lane as composition debt
- retained derived-artifact and live-artifact ordinary product paths no longer
  require the special-case runtime-owned seams listed in Phase 4
- preserved inspector/grouped temporal/async reuse neighbors are built with
  canonical runtime-backed semantics preserved across the covered reuse surface
- hostile runtime-backed read tests no longer need a custom minimal
  bridge-backed runtime harness just to obtain a valid raw runtime read lane

## Sequencing Notes

- This milestone belongs immediately after the merged
  [milestone-9.4.md](./milestone-9.4.md) closure so runtime-backed
  temporal/async semantics are already frozen before reuse and productization
  work tries to preserve them.
- It belongs before Milestones `10` and `11` because store-backed and durable
  milestones should not inherit half-hardened reuse, view-shape, and retained
  fact-consumption surfaces as if they were final.
- It should be implemented as an explicit per-family build program, not as
  scattered opportunistic cleanup.
