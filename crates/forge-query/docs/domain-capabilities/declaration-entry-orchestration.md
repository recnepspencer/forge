# Declaration Entry Orchestration

## What This Feature Is

Declaration entry orchestration is the generic Query front door for the
declaration-entry pipeline.

Give an admitted configured domain handle one already-assembled declaration
intent, and Query will lower it through the locked public sequence up to the
current ceiling: envelope construction.

This feature does not resolve geometry targets for you, and it does not
continue into runtime execution, workspace entry, signal execution, or
declaration-scoped contribution composition. It starts after your tool or
session has already decided what the
user is trying to do.

If that same declaration-entry run also needs declaration-scoped contribution
authoring, use the separate contribution-composed orchestration surface instead
of trying to treat the generic declaration-entry front door as if it already
owned contribution posture.

Query also exposes product-target orchestration for callers who already hold
progression proof and want one compact route, receipt, or envelope surface
without rebuilding route-plan, receipt, or envelope inputs by hand.

Those progressed product lanes share the same retained target-binding story as
declaration-scoped contribution authoring. Progression, route, receipt, and
envelope artifacts can expose typed binding targets without turning Query into
a target-resolution or ambient-DI layer.

The separate typed binding pipeline sits on top of that retained target seam.
Use binding when you need Query to choose or deny the next
explicit input from current context or a retained artifact. Use orchestration
when you already know which declaration-entry lowering run should happen.

## Why You Use It

- run one declaration-entry sequence without manually stitching multiple
  surfaces together
- keep ordinary, checked, and proof-visible lanes on one canonical pipeline
- preserve deferred, denied, stale, rebind-required, failed, and refused
  posture as typed results
- inspect the exact stop boundary when automation cannot continue
- inspect the default materialization and cost policy for the run
- compare equivalent runs through stable orchestration and outcome digests

## Stable Entry Points

- `ForgeQueryAdmittedConfiguredDomainHandle::orchestrate_declaration_entry(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::orchestrate_declaration_entry_outcome(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::orchestrate_declaration_entry_checked(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::orchestrate_declaration_entry_proof(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::orchestrate_routes_from_progressed(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::orchestrate_receipt_from_progressed(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::orchestrate_envelope_from_progressed(...)`
- `ForgeQueryDeclarationEntryOrchestrationInput`
- `ForgeQueryDeclarationEntryOrchestrationPlan`
- `ForgeQueryDeclarationEntryOrchestrationOutcome`
- `ForgeQueryDeclarationEntryOrchestrationTranscript`
- `ForgeQueryDeclarationRouteOrchestrationTranscript`
- `ForgeQueryDeclarationReceiptOrchestrationTranscript`
- `ForgeQueryDeclarationEnvelopeOrchestrationTranscript`
- `ForgeQueryDeclarationEntryOrchestrationExposureLevel`
- `ForgeQueryDeclarationEntryOrchestrationArtifactPolicy`
- `ForgeQueryDeclarationEntryOrchestrationStepRecord`
- `ForgeQueryDeclarationEntryOrchestrationStepDisposition`
- `ForgeQueryDeclarationEntryOrchestrationAutomationBoundary`
- `ForgeQueryDeclarationEntryOrchestrationAutomationStep`
- `ForgeQueryDeclarationEntryOrchestrationAutomationRefusal`
- `ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass`
- `ForgeQueryDeclarationEntryOrchestrationMaterializationPolicy`
- `ForgeQueryDeclarationEntryOrchestrationMaterializationTier`
- `ForgeQueryDeclarationEntryOrchestrationCostPosture`
- `ForgeQueryDeclarationEntryOrchestrationMaterializationGate`
- `ForgeQueryDeclarationEntryOrchestrationVerbInventory`
- `ForgeQueryOrchestrationSurfaceInventory`
- `ForgeQueryOrchestrationInventoryAudit`

Good to know:

- `ForgeQueryDeclarationEntryOrchestrationChecked` is the checked-lane
  compatibility alias for `ForgeQueryDeclarationEntryOrchestrationOutcome`
- `ForgeQueryDeclarationEntryOrchestrationProof` is the proof-visible
  compatibility alias for `ForgeQueryDeclarationEntryOrchestrationTranscript`
- Query owns construction of orchestration inputs, plans, refusals, outcomes,
  and transcripts

## Core Mental Model

There is one generic orchestration feature and one public sequence:

1. admitted handle
2. canonical declaration
3. legality
4. progression
5. foundational evidence
6. route plan
7. receipt
8. envelope

The public grammar over that sequence is intentionally narrow:

- `orchestrate_declaration_entry(...)`
- `orchestrate_declaration_entry_checked(...)`
- `orchestrate_declaration_entry_proof(...)`

Those are not three helper stacks. They are three visibility levels over the
same run:

- ordinary: give me the envelope or one typed terminal error
- checked: give me the typed stop posture
- proof-visible: give me the same posture plus the exact automation transcript

The ordinary lane is one shared public outcome family:

- `ForgeQueryOrdinaryOutcome<T>`

That ordinary surface is still a projection over the same checked topology. It
does not introduce a second orchestration engine or a second stop-boundary
model.

The important system boundary is this:

- your session or tool resolves the user's targets and assembles declaration
  intent
- Query lowers that intent through the retained declaration-entry pipeline

Do not teach your app team that Query is the target-resolution layer. It is
the orchestration layer after intent already exists.

That distinction matters even more now that declaration-entry surfaces retain
semantic slices explicitly. The
declaration-entry pipeline is now trying to preserve semantic slices from
active geometry or workflow context, not teach callers to pass around raw
geometry ids as the main DX model.

## Product Targets

There is also a second declarative ladder for callers who already hold
`ForgeQueryAdmittedDeclarationProgression`:

- `orchestrate_routes_from_progressed(...)`
- `orchestrate_routes_from_progressed_with_intent(...)`
- `orchestrate_receipt_from_progressed(...)`
- `orchestrate_receipt_from_progressed_with_intent(...)`
- `orchestrate_envelope_from_progressed(...)`
- `orchestrate_envelope_from_progressed_with_intent(...)`

Each family also has checked and proof-visible variants.

These are not a new orchestration engine. They are compact product-target
projections over the same retained declaration-entry path:

1. progression
2. foundational evidence
3. route plan
4. receipt when needed
5. envelope when needed

Use this request ladder when you want stronger DX without giving up control:

- shortest path: `...from_progressed(...)`
- narrowed path: `...from_progressed_with_intent(...)`
- checked path: `..._checked(...)`
- proof path: `..._proof(...)`
- explicit substrate: direct route/receipt/envelope input families still exist

## How It Executes

The public ceiling is `EnvelopeCeiling`.

That means the orchestration plan may automate only these steps:

- `AdmittedHandle`
- `CanonicalDeclaration`
- `Legality`
- `Progression`
- `FoundationalEvidence`
- `RoutePlan`
- `Receipt`
- `Envelope`

Query derives one plan at the start of the run and then carries retained proof
forward step by step. The public surfaces do not recompute the declaration
meaning differently for ordinary, checked, and proof-visible lanes.

The plan exposes that sequencing law directly:

- `ceiling_stage()`
- `automation_boundary()`
- `automation_steps()`
- `explicit_caller_handoff_steps()`
- `orchestration_identity_digest()`

If a declaration-orchestration step touches graph-shaped authority, the graph
obligation selector remains Query-owned. The orchestration plan may carry touch
descriptor, operating world descriptor, dispatch plan, executor verdict, and
budget evidence, but it must not replace graph touch obligation authority with
local validator dispatch.

The plan also exposes the materialization and cost policy directly:

- `materialization_policy()`
- `materialization_tier()`
- `cost_posture()`
- `materialization_gate()`
- `foundational_evidence_profile()`
- `descriptive_materialization_cost()`
- `aspect_contract()`
- `aspect_coverage()`
- `aspect_coverage_basis()`
- `foundational_aspect_publication()`
- `receipt_aspect_publication()`
- `envelope_aspect_publication()`
- `relational_authority_summary()`
- `bridge_authority_summary()`
- `signal_authority_summary()`

`explicit_caller_handoff_steps()` is currently empty because the public front
door stops at envelope construction instead of
advertising a second automated continuation step.

That ceiling is now paired with retained authority posture. The plan and proof
surfaces do not automate relational routing, bridge routing, or signal
compatibility, but they do freeze what the retained envelope publication
already implies for those later authority consumers.

That implication remains a retained projection, not an executed lower stage.
The transcript may tell you that the retained envelope publication would later
leave the bridge slice missing, the signal dependency slice unsupported, or the
relational slice only partially covered, while the orchestration run itself
still honestly stops successfully at `EnvelopeConstructed`.

## Sequencing Law

The locked execution order is:

1. admitted handle
2. canonical declaration
3. legality
4. progression
5. foundational evidence
6. route plan
7. receipt
8. envelope

Public orchestration may not:

- skip directly to a later step
- invent a second planning abstraction
- widen into runtime, workspace, basis, continuation, or signal work
- imply that later execution happened because earlier preparation succeeded

Step records expose the real reached sequence through:

- `stage()`
- `automation_step()`
- `disposition()`
- `retained_digest()`
- `reason()`
- `is_reached()`
- `is_stop()`
- `is_terminal()`

## Materialization And Cost Policy

Sequencing and publication are separate public axes.

Keep these concepts distinct:

- exposure level: ordinary, checked, proof-visible
- sequencing law: which declaration-entry steps Query may automate
- materialization policy: how lean or rich the descriptive artifacts are
- cost posture: whether that publication shape is an ordinary default, an
  explicit rich request, or a prepared-but-not-executed boundary

The default orchestration policy is intentionally conservative:

- foundational evidence uses
  `FoundationalBoundaryEvidenceMaterializationProfile::ElideSupportAndDiagnostics`
- receipt publication uses the `SupportReady` tier
- envelope publication uses the `SupportReady` tier
- the plan reports `OrdinaryDefault` cost posture and `AdmittedByDefault` gate

That keeps the generic orchestration trio cheap-looking in truth. Proof-visible
inspection exposes more metadata, but it does not silently widen declaration
truth or claim later execution happened.

That policy is now semantically inspectable rather than only profile-shaped:

- the plan retains the declaration-entry `aspect_contract()`
- the plan retains `aspect_coverage()` plus `aspect_coverage_basis()` so callers
  can tell whether the publication policy is operating on declared family
  coverage or reviewed retained coverage
- `foundational_aspect_publication()` tells you what the foundational profile
  actually publishes
- `receipt_aspect_publication()` and `envelope_aspect_publication()` tell you
  how the ordinary, checked, and proof lanes widen or elide semantic slices for
  later retained artifacts, using the route-scoped crossing contract rather
  than the broader declaration-published slice
- `relational_authority_summary()` tells you which relational slice is already
  visible at the envelope boundary and whether it is exact, a compatible
  superset, partial, missing, or conflicting
- `bridge_authority_summary()` keeps the available envelope slice separate from
  the narrower mapped continuation slice later bridge routing would still need
- `signal_authority_summary()` freezes dependency and produced-aspect posture so
  later execution surfaces do not have to rediscover it from the envelope

The basis split is part of the public honesty contract:

- `DeclaredFamilyCoverage` means the generic admitted-handle entry lane is
  publishing from the family-declared semantic coverage it knows before later
  retained proof exists
- `ReviewedRetainedCoverage` means the progressed-product lane is publishing
  from real reviewed retained proof, not just the family's declared semantic
  promise
- in both lanes, masked or conflicting slices stay masked instead of being
  promoted into visible publication by richer transcript policy

Later lower-authority surfaces now depend on that publication honesty directly:

- relational routing consumes the envelope's published relational slice
- bridge routing consumes the envelope's published bridge slice and freezes the
  narrower mapped slice
- signal compatibility consumes the envelope's published dependency slice plus
  basis-family posture

Orchestration now exposes those downstream consequences directly without
pretending the lower-authority surfaces already ran. A bridge or signal summary
can legitimately report `MissingRequired` while the orchestration proof still
ends successfully at `Envelope`.

## Where Automation Stops

The current front door stops in one of two broad ways.

First, the retained declaration-entry pipeline can land in an ordinary
non-success posture:

- `Deferred`
- `Denied`
- `Stale`
- `RebindRequired`
- `Failed`

Second, Query can refuse to automate farther even though the declaration-entry
story still has a meaningful typed stop boundary:

- `Refused`

That distinction matters.

`Deferred`, `Denied`, `Stale`, `RebindRequired`, and `Failed` describe what the
pipeline proved. `Refused` describes where Query intentionally stopped
automation.

Use `is_automation_refusal()` when you only need the quick branch. Use the
typed refusal accessors when you need the exact reason:

- `refusal_class()`
- `automation_refusal_class()`
- `stop_stage()`
- `reason()`
- `retained_digest()`
- `orchestration_identity_digest()`
- `automation_boundary()`

The sequencing-specific refusal classes are:

- `ExplicitIntentRequired`
- `ExpensiveAutomationForbidden`
- `AuthorityTransitionRequired`
- `PreparedButNotExecuted`
- `UnsupportedAutomation`
- `StrongerProofRequired`

## Boundary Honesty

The transcript and checked outcome must report the farthest crossed public
boundary honestly.

Examples:

- if route posture lowers into a deferred receipt before stopping, the stop
  stage is `ReceiptIssued`
- if receipt kind support is the blocker, the stop stage is `ReceiptIssued`,
  not `RoutePlanned`
- if the route boundary truly stopped before any receipt crossing, the stop stage
  remains `RoutePlanned`

This is why proof-visible orchestration is more than debug logging. It shows
the exact automation story and publication policy Query is willing to claim.

## Public Grammar

The grammar is fixed for this feature:

- base verb: `orchestrate_declaration_entry`
- checked suffix: `_checked`
- proof-visible suffix: `_proof`

That trio is still the generic declaration-input front door. The grammar
inventory now also reports the public route/receipt/envelope product-target
orchestration verbs for progressed declarations.

Do not introduce parallel names like:

- `_transcript`
- `_trace`
- `_debug`
- family-specific orchestration aliases

Inspect the locked grammar directly when you need it:

```rust
let inventory = ForgeQueryDeclarationEntryOrchestrationVerbInventory::current();

for verb in inventory.verbs() {
    let _ = verb.public_name();
    let _ = verb.family();
    let _ = verb.exposure_level();
    let _ = verb.ceiling();
    let _ = verb.canonical_base_name();
}
```

The larger cross-family orchestration inventory is also available. Use that
when you need the declaration-entry rows in the context of continuation,
signal-facing orchestration, and contribution-composed orchestration instead of
as a declaration-entry-only list:

```rust
let inventory = ForgeQueryOrchestrationSurfaceInventory::current();

let row = inventory
    .row_for_public_name("orchestrate_declaration_entry")
    .expect("declaration-entry row should exist");

let _ = row.family();
let _ = row.visibility();
let _ = row.proof_contract().transcript_family();
```

## Small Example

This example shows the intended boundary: the editing session resolved the
targets already, then Query orchestrated the declaration-entry pipeline.

```rust
let trim_request = geometry_session.trim_segment_at_active_intersection()?;
let envelope = handle.orchestrate_declaration_entry(trim_request)?;

let _ = envelope.envelope_digest();
```

Progressed-product orchestration is the compact route/receipt/envelope ladder:

```rust
let progressed = handle.declare_review_and_progress(trim_request)?;
let route_plan = handle.orchestrate_routes_from_progressed(progressed.clone())?;
let receipt = handle.orchestrate_receipt_from_progressed(progressed.clone())?;
let envelope = handle.orchestrate_envelope_from_progressed(progressed)?;

let _ = route_plan.route_plan_digest();
let _ = receipt.receipt_digest();
let _ = envelope.envelope_digest();
```

## Real Example

Use the checked lane when the UI or collaboration workflow needs to know
exactly why automation stopped.

```rust
let trim_request = geometry_session
    .selection()
    .single_segment()
    .trim_at_highlighted_intersection()?;

match handle.orchestrate_declaration_entry_checked(trim_request) {
    ForgeQueryDeclarationEntryOrchestrationOutcome::Enveloped(envelope) => {
        let _ = envelope.envelope_digest();
    }
    ForgeQueryDeclarationEntryOrchestrationOutcome::Refused(refusal) => {
        let _ = refusal.automation_refusal_class();
        let _ = refusal.stop_stage();
        let _ = refusal.reason();
    }
    other => {
        let _ = other.stop_stage();
        let _ = other.outcome_identity_digest();
    }
}
```

Use the proof-visible lane when you need the exact reached sequence and the
declared publication policy:

```rust
let trim_request = geometry_session.trim_segment_at_active_intersection()?;
let transcript = handle.orchestrate_declaration_entry_proof(trim_request);

let _ = transcript.plan().automation_boundary();
let _ = transcript.plan().automation_steps();
let _ = transcript.plan().materialization_policy();
let _ = transcript.plan().foundational_evidence_profile();
let _ = transcript.plan().descriptive_materialization_cost();
let _ = transcript.plan().foundational_aspect_publication();
let _ = transcript.plan().receipt_aspect_publication();
let _ = transcript.plan().envelope_aspect_publication();
let _ = transcript.relational_authority_summary().aspect_fit();
let _ = transcript.bridge_authority_summary().mapping_fit();
let _ = transcript.signal_authority_summary().produced_aspects();
let _ = transcript.outcome().stop_stage();

for record in transcript.step_records() {
    let _ = record.stage();
    let _ = record.automation_step();
    let _ = record.disposition();
    let _ = record.materialization_tier();
    let _ = record.retained_digest();
    let _ = record.reason();
}
```

## How It Relates To Other Features

- Use [Configured Domain Handles](./configured-domain-handles.md) to create the
  admitted world that owns this orchestration run.
- Use [Canonical Domain Declarations](./canonical-domain-declarations.md) when
  you need to author or inspect declaration intent without lowering it.
- Use [Declaration Progression](./declaration-progression.md) when you want the
  proof-bearing progression artifact directly.
- Use [Declaration Foundational Evidence](./declaration-foundational-evidence.md)
  when you want to control foundational descriptive richness explicitly outside
  the orchestration default policy.
- Use [Declaration Boundary Envelopes](./declaration-boundary-envelopes.md)
  when you already have retained receipt-backed crossing truth and want the
  envelope artifact directly.
- Use [Declaration Entry Inspection](./declaration-entry-inspection.md) when
  you need a read surface over retained seam artifacts after a run.
- Use [Declaration Entry Readiness](./declaration-entry-readiness.md) when you
  need family-level seam support posture instead of one concrete run.
- Use [Typed Binding Pipeline](./typed-binding-pipeline.md) when the main job
  is selecting or denying the next explicit input from context or retained
  targets rather than lowering through the declaration-entry ceiling.
- Use [Ordinary Outcomes](./ordinary-outcomes.md) when you want the compact
  public result vocabulary shared by binding, continuation, and orchestration.
- Use [Contribution-Composed Orchestration](./contribution-composed-orchestration.md)
  when the declaration-entry run also needs declaration-scoped contribution
  authoring or contribution summary materialization in the same public call.
- Use [Continuation Pipeline](./continuation-pipeline.md) when the next job is
  to turn retained envelope or continuation-ready truth into one prepared
  continuation artifact and explicit execution step.
- Use [Signal Compatibility Orchestration](./signal-compatibility-orchestration.md)
  when the next job is signal-facing composition over retained compatibility
  and optional continuation preparation, not the generic envelope-ceiling
  declaration-entry run.

## Inspection And Debugging

Use the checked lane when you need:

- the typed stop posture
- the exact stop stage
- the canonical outcome identity
- the exact automation refusal class when the stop is `Refused`

Use the proof-visible lane when you need:

- the declared automation boundary
- the exact sequence of reached stages
- the farthest crossed public boundary
- retained digests attached to each reached or stopped stage
- parity evidence across equivalent runs
- the declared materialization tier and cost posture for the run
- the exact semantic slices the foundational, receipt, and envelope tiers
  published or elided
- the downstream relational, bridge, and signal posture implied by the retained
  envelope without widening the execution ladder

Useful accessors:

- `plan().orchestration_identity_digest()`
- `outcome().outcome_identity_digest()`
- `plan().automation_boundary()`
- `plan().automation_steps()`
- `plan().materialization_policy()`
- `plan().cost_posture()`
- `plan().foundational_evidence_profile()`
- `plan().descriptive_materialization_cost()`
- `plan().aspect_contract()`
- `plan().aspect_coverage()`
- `plan().aspect_coverage_basis()`
- `plan().foundational_aspect_publication()`
- `plan().receipt_aspect_publication()`
- `plan().envelope_aspect_publication()`
- `plan().relational_authority_summary()`
- `plan().bridge_authority_summary()`
- `plan().signal_authority_summary()`
- `proof.relational_authority_summary()`
- `proof.bridge_authority_summary()`
- `proof.signal_authority_summary()`
- `step_records()`
- `step_record.automation_step()`
- `step_record.materialization_tier()`
- `ForgeQueryOrchestrationSurfaceInventory::current()`
- `ForgeQueryOrchestrationInventoryAudit::current()`

## Anti-Patterns

- treating ordinary orchestration as "best effort farther lowering"
- assuming a successful envelope implies later runtime or signal work is ready
- assuming proof-visible automatically means full descriptive publication
- assuming richer descriptive publication means more declaration-entry progress
- passing raw geometry IDs as the main public mental model for app usage docs
- teaching that Query resolves user selections or highlights for you
- flattening `Stale`, `RebindRequired`, `Denied`, `Failed`, and `Refused` into
  one generic error path
- treating proof-visible transcripts as the same thing as retained seam
  inspection artifacts
- inventing parallel orchestration verb families around the locked trio

## Current Limits

- success still stops at the envelope ceiling
- the current front door does not automate relational routing, bridge routing,
  signal compatibility, runtime entry, workspace entry, or continuation
  preparation
- proof-visible is intentionally the only transcript-bearing public lane
- the automation boundary is currently single-valued: `EnvelopeCeiling`
- the grammar is intentionally generic only; family-specific orchestration
  aliases are out of scope here

## Aspect Semantics

Orchestration is one of the first user-visible Query surfaces where semantic
aspect fit matters more than source-order folklore.

The main geometry story here is dynamic context binding such as active
intersection trimming or active-selection material edits, not raw identifier
passing. Plans and transcripts therefore explain:

- which semantic slices were required
- which retained coverage Query used
- what was published at the foundational, receipt, and envelope boundaries
- what remained masked, partial, or missing when the run stopped

## Related Docs

- [Graph Touch Obligation Authority](../authoring/graph-touch-obligation-authority.md)
- [Configured Domain Handles](./configured-domain-handles.md)
- [Typed Binding Pipeline](./typed-binding-pipeline.md)
- [Ordinary Outcomes](./ordinary-outcomes.md)
- [Orchestration Inventory](./orchestration-inventory.md)
- [Contribution-Composed Orchestration](./contribution-composed-orchestration.md)
- [Continuation Pipeline](./continuation-pipeline.md)
- [Signal Compatibility Orchestration](./signal-compatibility-orchestration.md)
- [Canonical Domain Declarations](./canonical-domain-declarations.md)
- [Declaration Progression](./declaration-progression.md)
- [Declaration Foundational Evidence](./declaration-foundational-evidence.md)
- [Declaration Boundary Envelopes](./declaration-boundary-envelopes.md)
- [Declaration Entry Inspection](./declaration-entry-inspection.md)
- [Declaration Entry Readiness](./declaration-entry-readiness.md)
