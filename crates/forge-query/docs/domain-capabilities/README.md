# Domain Capabilities

These docs cover the public `forge-query` surfaces you use when a downstream
domain wants Query to own the artifact, orchestration, grouping, and recovery
model instead of rebuilding a domain-local copy.

The goal of this tree is simple: start from the kind of domain work you are
trying to do, not from the internal module name you happen to remember.

## Start Here

- [Configured Domain Handles](./configured-domain-handles.md)
  Enter one admitted operating world and get the public handle most domain work
  starts from.
- [Typed Binding Pipeline](./typed-binding-pipeline.md)
  Turn current context or one retained artifact into the next explicit Query
  input.
- [Ordinary Outcomes](./ordinary-outcomes.md)
  Use the compact result lane shared by binding, orchestration, and
  continuation-facing surfaces.
- [Recovery Boundary](./recovery-boundary.md)
  Get one machine-readable answer for what stopped, who owns the fix, and what
  to do next.
- [Choosing The Right Surface](./choosing/README.md)
  Start here when neighboring Query docs feel close together and you want the
  shortest chooser path.

## Choosing The Right Surface

- [Choosing The Right Surface](./choosing/README.md)
  Landing page for cross-surface chooser docs.
- [Binding Vs Orchestration Vs Helpers](./choosing/binding-vs-orchestration-vs-helpers.md)
  Choose between selecting the next input, lowering a declaration, and using a
  family-native helper.
- [Inspection Vs Readiness Vs Recovery](./choosing/inspection-vs-readiness-vs-recovery.md)
  Choose between support posture, retained truth inspection, and next-step
  repair guidance.
- [Grouped Authoring Vs Grouped Products Vs Grouped Contributions](./choosing/grouped-authoring-vs-grouped-products-vs-grouped-contributions.md)
  Choose between defining one grouped meaning, reading grouped artifacts, and
  composing grouped contributions.
- [Signal Compatibility Vs Continuation Pipeline](./choosing/signal-compatibility-vs-continuation-pipeline.md)
  Choose between freezing signal-facing eligibility and preparing the next
  continuation step.

## Workflow Guides

- [Workflow Guides](./workflow/README.md)
  Landing page for task-first multi-surface Query jobs.
- [Single Declaration To Envelope](./workflow/single-declaration-to-envelope.md)
  Shortest path from one declaration input to one public envelope artifact.
- [Retained Artifact To Next Step](./workflow/retained-artifact-to-next-step.md)
  Move progression, route, receipt, or envelope truth forward without
  rebuilding earlier declaration work.
- [Envelope To Signal Or Continuation](./workflow/envelope-to-signal-or-continuation.md)
  Choose the next runtime-facing step after envelope truth exists.
- [Grouped Neighborhood Workflow](./workflow/grouped-neighborhood-workflow.md)
  Run one neighborhood-shaped job across grouped authoring, products, and
  contributions.
- [Stop To Recovery](./workflow/stop-to-recovery.md)
  Turn one stop into one next-step repair answer.

## Recipes

- [Recipes](./recipes/README.md)
  Copy-oriented examples for common Query jobs.
- [Prepare Preview From An Active Face Selection](./recipes/prepare-preview-from-active-face-selection.md)
  Progress one active-face declaration and ask Query for the preview-facing
  next step.
- [Attach Material With Declaration-Scoped Contributions](./recipes/attach-material-with-declaration-scoped-contributions.md)
  Run one material-attachment declaration plus contribution meaning in one
  helper-driven call.
- [Author A Grouped Neighborhood With Contributions](./recipes/author-a-grouped-neighborhood-with-contributions.md)
  Build one neighborhood-shaped operation with shared and member-local grouped
  meaning.
- [Turn A Stop Into A Recovery Action](./recipes/turn-a-stop-into-a-recovery-action.md)
  Convert one stop into a typed repair answer.

## Core Domain Work

- [Platform Entry](./platform-entry.md)
  Typed domain front door and admission posture.
- [Configured Domain Handles](./configured-domain-handles.md)
  One admitted operating world plus the public handle-centered entry points.
- [Typed Binding Pipeline](./typed-binding-pipeline.md)
  Candidate selection and retained-target binding for the next explicit input.
- [Ordinary Outcomes](./ordinary-outcomes.md)
  Compact result vocabulary over checked topology.
- [Recovery Boundary](./recovery-boundary.md)
  Next-step action planning after ordinary, checked, or proof-visible stops.

## Helpers And Grouped Work

- [Family Helpers](./family-helpers.md)
  Family-native ergonomics that still lower onto canonical Query lanes.
- [Grouped Authoring](./grouped-authoring.md)
  Define one grouped neighborhood meaning when the group itself is part of the
  operation.
- [Grouped Products](./grouped-products.md)
  Read grouped route, receipt, and envelope truth.
- [Grouped Contributions](./grouped-contributions.md)
  Compose shared and member-local contributions over grouped meaning.
- [Grouped Support And Readiness](./grouped-support-readiness.md)
  Check whether an admitted grouped declaration can support stronger grouped
  claims before later grouped execution or projection.

## Declaration Pipeline

- [Canonical Domain Declarations](./canonical-domain-declarations.md)
  Author one Query-owned declaration artifact.
- [Declaration Family Taxonomy](./declaration-family-taxonomy.md)
  Understand how Query classifies domain declaration families.
- [Declaration Family Capability Matrix](./declaration-family-capability-matrix.md)
  Family-level support and capability posture.
- [Declaration Legality](./declaration-legality.md)
  Structural legality review for one admitted declaration.
- [Declaration Progression](./declaration-progression.md)
  Carry legality-cleared declarations into proof-bearing admitted progression.
- [Declaration Foundational Evidence](./declaration-foundational-evidence.md)
  Describe retained truth through shared foundational evidence.
- [Declaration Route Plans](./declaration-route-plan.md)
  Build one route plan over progression and foundational evidence.
- [Declaration Boundary Receipts](./declaration-boundary-receipts.md)
  Materialize the crossing artifact that records what followed from route
  truth.
- [Declaration Boundary Envelopes](./declaration-boundary-envelopes.md)
  Carry retained evidence, route truth, and receipt truth forward together.
- [Declaration Entry Orchestration](./declaration-entry-orchestration.md)
  Lower one declaration input through the declaration-entry pipeline.
- [Declaration Entry Inspection](./declaration-entry-inspection.md)
  Read retained seam truth after a run.
- [Declaration Entry Readiness](./declaration-entry-readiness.md)
  Check family-level seam posture before one concrete run.

## Lower-Authority And Continuation

- [Declaration Relational Truth Routing](./declaration-relational-truth-routing.md)
  Bind envelope-backed truth into one relational authority lane.
- [Declaration Bridge Continuation Routing](./declaration-bridge-continuation-routing.md)
  Bind envelope-backed truth into one bridge continuation lane.
- [Declaration Signal Compatibility](./declaration-signal-compatibility.md)
  Freeze later Signal-backed eligibility from retained declaration truth.
- [Signal Compatibility Orchestration](./signal-compatibility-orchestration.md)
  Answer the next signal-facing question directly: `Compatible`, `Prepared`, or
  one typed stop.
- [Continuation Pipeline](./continuation-pipeline.md)
  Prepare one continuation artifact and optionally execute it.

## Composition, Support, And Certification

- [Contribution-Composed Orchestration](./contribution-composed-orchestration.md)
  Lower one declaration plus declaration-scoped contributions in one public
  call.
- [Orchestration Inventory](./orchestration-inventory.md)
  Registry of public orchestration and helper surfaces.
- [Public Doc Coverage](./public-doc-coverage.md)
  Registry of feature-page, README, golden, and journey coverage.
- [Platform Entry Closeout](./platform-entry-closeout.md)
  Machine-checkable certification bundle over the public product surface.
- `admission/`
  - [Advisory And Violation Contributions](./admission/advisory-and-violation-contributions.md)
  - [Declaration Vs Admitted-Plan Targets](./admission/declaration-vs-admitted-plan-targets.md)
- `support/`
  - [Declaration-Scoped Support And Traceability](./support/declaration-scoped-support-and-traceability.md)
  - [Admission-Local Support Reports](./support/admission-local-support-reports.md)
  - [Lower-Runtime Support And Boundary Traceability](./support/lower-runtime-support-and-boundary-traceability.md)
- `invariants/`
  - [Registering Domain Invariants Through Query](./invariants/registering-domain-invariants-through-query.md)
  - [Capability Gaps And Invariant Denials](./invariants/capability-gaps-and-invariant-denials.md)
- `workflow/`
  - [Workflow Guides](./workflow/README.md)
    Task-first entry point for common multi-surface Query jobs.
  - [Single Declaration To Envelope](./workflow/single-declaration-to-envelope.md)
    Shortest path from one declaration input to one public envelope artifact.
  - [Retained Artifact To Next Step](./workflow/retained-artifact-to-next-step.md)
    Move progression, route, receipt, or envelope truth forward without
    rebuilding earlier declaration work.
  - [Envelope To Signal Or Continuation](./workflow/envelope-to-signal-or-continuation.md)
    Choose the next runtime-facing step after envelope truth exists.
  - [Grouped Neighborhood Workflow](./workflow/grouped-neighborhood-workflow.md)
    Run one neighborhood-shaped job across grouped authoring, products, and
    contributions.
  - [Stop To Recovery](./workflow/stop-to-recovery.md)
    Turn one stop into one next-step repair answer.
  - [Preview Inspection And Mutation Planning](./workflow/preview-inspection-and-mutation-planning.md)
  - [Runtime-Preflight Workflow Contributions](./workflow/runtime-preflight-workflow-contributions.md)
  - [Workflow Lanes: Common, Checked, Proof, And Raw](./workflow/workflow-lanes-common-checked-proof-raw.md)
- `continuity/`
  - [Continuity Contributions And Authoritative Successors](./continuity/continuity-contributions-and-authoritative-successors.md)
  - [Continuity Vs Correspondence](./continuity/continuity-vs-correspondence.md)
- `aftermath/`
  - [Projection Contract Consumption](./aftermath/projection-contract-consumption.md)
  - [Aftermath Review, Support, Eligibility, And Materialization](./aftermath/aftermath-review-support-eligibility-and-materialization.md)
- `explanation/`
  - [Lower-Runtime Explanation Contributions](./explanation/lower-runtime-explanation-contributions.md)
  - [Cross-Runtime Fallback Vs Store-Backed Replay Gap](./explanation/cross-runtime-fallback-vs-store-backed-replay-gap.md)
- `certification/`
  - [Certification Surface And Closeout Bundle](./certification/certification-surface-and-closeout-bundle.md)
  - [Goldens, Boundaries, And Hostile Certification](./certification/goldens-boundaries-and-hostile-certification.md)

## Reading Order

1. [Configured Domain Handles](./configured-domain-handles.md)
2. [Choosing The Right Surface](./choosing/README.md)
3. [Typed Binding Pipeline](./typed-binding-pipeline.md)
4. [Ordinary Outcomes](./ordinary-outcomes.md)
5. [Recovery Boundary](./recovery-boundary.md)
6. the one feature page that matches your task

## Related Docs

- [Forge Query Docs Home](../README.md)
- [Family Helpers](./family-helpers.md)
- [Grouped Authoring](./grouped-authoring.md)
- [Continuation Pipeline](./continuation-pipeline.md)
