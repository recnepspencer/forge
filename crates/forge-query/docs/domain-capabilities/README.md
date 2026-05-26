# Domain Capabilities

`forge-query` domain capabilities let downstream domains contribute semantic
runtime posture through Query-owned public surfaces while Query keeps canonical
artifact authority.

This docs tree is organized by capability area so you can start from the kind
of domain work you are trying to do:

- [Platform Entry](./platform-entry.md)
- [Configured Domain Handles](./configured-domain-handles.md)
- [Canonical Domain Declarations](./canonical-domain-declarations.md)
- [Declaration Family Taxonomy](./declaration-family-taxonomy.md)
- [Declaration Family Capability Matrix](./declaration-family-capability-matrix.md)
- [Declaration Legality](./declaration-legality.md)
- [Declaration Progression](./declaration-progression.md)
- [Declaration Foundational Evidence](./declaration-foundational-evidence.md)
- [Declaration Route Plans](./declaration-route-plan.md)
- [Declaration Boundary Receipts](./declaration-boundary-receipts.md)
- [Declaration Boundary Envelopes](./declaration-boundary-envelopes.md)
- [Declaration Relational Truth Routing](./declaration-relational-truth-routing.md)
- [Declaration Bridge Continuation Routing](./declaration-bridge-continuation-routing.md)
- [Declaration Signal Compatibility](./declaration-signal-compatibility.md)
- [Declaration Entry Orchestration](./declaration-entry-orchestration.md)
- [Declaration Entry Inspection](./declaration-entry-inspection.md)
- [Declaration Entry Readiness](./declaration-entry-readiness.md)
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

Use these docs when you are building domain-specific behavior on top of the
public Query runtime, especially when your domain needs typed admission,
support, workflow, continuity, projection aftermath, or explanation artifacts
without rebuilding a pseudo-Query layer locally.

Start with [Platform Entry](./platform-entry.md) when you need the typed
facade-first domain front door where the downstream domain supplies its own
marker type rather than relying on separate string-authored contribution
surfaces.

Move next to [Configured Domain Handles](./configured-domain-handles.md) when
you need an admitted operating world, then to
[Canonical Domain Declarations](./canonical-domain-declarations.md) when that
admitted world needs to express declaration-local meaning through one retained
Query-owned declaration artifact. Use
[Declaration Family Taxonomy](./declaration-family-taxonomy.md) when you need
to understand how Query classifies downstream declaration families and carries
that classification forward without owning the family nouns themselves. Use
[Declaration Family Capability Matrix](./declaration-family-capability-matrix.md)
when you need family-scoped support reports, checked family admission, or
structural witness surfaces on canonical declarations. Use
[Declaration Legality](./declaration-legality.md) when you need to review an
already admitted canonical declaration for structural legality inside one
admitted operating world. Use
[Declaration Progression](./declaration-progression.md) when you need to carry
that legality-cleared declaration into a proof-bearing admitted progression or
one typed deferred/denied/stale/rebind/failed outcome. Use
[Declaration Foundational Evidence](./declaration-foundational-evidence.md)
when you need to describe retained legality or progression truth through shared
foundational provenance, support, receipt, and attachment-bundle artifacts.
Use [Declaration Route Plans](./declaration-route-plan.md) when you need one
Query-owned route plan over admitted progression proof plus matching
foundational evidence, with explicit route sets, typed caller route intent,
and plan-local explanations. Use
[Declaration Boundary Receipts](./declaration-boundary-receipts.md) when you
need the Query-owned operational crossing artifact that records what followed
from that route truth. Use
[Declaration Boundary Envelopes](./declaration-boundary-envelopes.md) when you
need the one public artifact that carries retained evidence, route truth, and
receipt truth forward together. Use
[Declaration Relational Truth Routing](./declaration-relational-truth-routing.md)
when that public crossing story needs to bind into one real relational
truth-authority family. Use
[Declaration Bridge Continuation Routing](./declaration-bridge-continuation-routing.md)
when that same public crossing story needs to bind into one real bridge
continuation family. Use
[Declaration Signal Compatibility](./declaration-signal-compatibility.md)
when you need to freeze whether that retained declaration story is later
eligible for Signal-backed derived execution.
[Declaration Entry Orchestration](./declaration-entry-orchestration.md)
when you want one admitted-handle front door over the declaration-entry
pipeline through the current envelope ceiling, with ordinary, checked, and
proof-visible visibility levels over the same canonical lowering path, one
locked automation sequence, and one inspectable materialization/cost policy.
[Declaration Entry Inspection](./declaration-entry-inspection.md)
when you need one unified read surface over retained seam artifacts after that
lowering. Use
[Declaration Entry Readiness](./declaration-entry-readiness.md) when you need
family-level seam support posture and executable crossing inventory rows rather
than one concrete orchestration or lowering run.

## Declaration Pipeline Surface Map

The declaration-side public lane is handle-centered and progresses in this
order:

- configured-handle admission:
  - `with_operating_context(...)`
  - `validate()`
  - `admit()`
- family support and declaration authoring:
  - `family_support::<F>()`
  - `family_support_checked::<F>()`
  - `declare(...)`
  - `declare_checked(...)`
  - `declare_with_version(...)`
- legality review:
  - `review_legality(...)`
  - `review_legality_checked(...)`
  - `declare_and_review(...)`
- proof-bearing progression:
  - `declaration_progression_recipe(...)`
  - `progress_declaration(...)`
  - `progress_declaration_checked(...)`
  - `progress_declaration_recipe(...)`
  - `progress_declaration_recipe_checked(...)`
  - `declare_review_and_progress(...)`
- foundational description:
  - `describe_foundational(...)`
  - `describe_foundational_checked(...)`
  - `describe_foundational_with_profile(...)`
- route planning:
  - `plan_routes(...)`
  - `plan_routes_checked(...)`
  - `plan_routes_from_progressed(...)`
  - `plan_routes_from_progressed_with_intent(...)`
  - `declare_review_progress_describe_and_plan(...)`
- boundary receipts:
  - `receipt_routes(...)`
  - `receipt_routes_checked(...)`
  - `receipt_routes_from_progressed(...)`
  - `receipt_routes_from_progressed_with_intent(...)`
  - `declare_review_progress_describe_plan_and_receipt(...)`
- boundary envelopes:
  - `envelope_routes(...)`
  - `envelope_routes_checked(...)`
  - `envelope_routes_from_progressed(...)`
  - `envelope_routes_from_progressed_with_intent(...)`
  - `declare_review_progress_describe_plan_receipt_and_envelope(...)`
- relational truth routing:
  - `route_relational_truth(...)`
  - `route_relational_truth_checked(...)`
  - `route_relational_truth_from_progressed(...)`
  - `route_relational_truth_from_progressed_with_intent(...)`
  - `declare_review_progress_describe_plan_receipt_envelope_and_route_relational_truth(...)`
  - `relational_truth_support::<I>()`
- bridge continuation routing:
  - `route_bridge_continuation(...)`
  - `route_bridge_continuation_checked(...)`
  - `route_bridge_continuation_from_progressed(...)`
  - `route_bridge_continuation_from_progressed_with_intent(...)`
  - `declare_review_progress_describe_plan_receipt_envelope_and_route_bridge_continuation(...)`
  - `bridge_continuation_support::<I>()`
- signal compatibility:
  - `signal_compatibility(...)`
  - `signal_compatibility_checked(...)`
  - `signal_compatibility_from_progressed(...)`
  - `signal_compatibility_from_progressed_with_intent(...)`
  - `declare_review_progress_describe_plan_receipt_envelope_and_check_signal_compatibility(...)`
  - `signal_compatibility_support::<I>()`
- declaration-entry orchestration:
  - `orchestrate_declaration_entry(...)`
  - `orchestrate_declaration_entry_checked(...)`
  - `orchestrate_declaration_entry_proof(...)`
  - `orchestrate_routes_from_progressed(...)`
  - `orchestrate_receipt_from_progressed(...)`
  - `orchestrate_envelope_from_progressed(...)`
- seam-ledger projections:
  - `declaration_entry_crossing_inventory::<I>()`
  - `declaration_entry_readiness::<I>()`
  - `inspect_declaration_entry(...)`

The main retained public artifacts introduced along that path are:

- `ForgeQueryCanonicalDeclarationArtifact`
- `ForgeQueryDeclarationLegalityEvidence`
- `ForgeQueryAdmittedDeclarationProgression`
- `ForgeQueryDeclarationFoundationalEvidence`
- `ForgeQueryDeclarationRoutePlan`
- `ForgeQueryDeclarationReceipt`
- `ForgeQueryDeclarationEnvelope`
- `ForgeQueryDeclarationRelationalRouting`
- `ForgeQueryDeclarationBridgeRouting`
- `ForgeQueryDeclarationSignalCompatibility`
- `ForgeQueryDeclarationEntryOrchestrationInput`
- `ForgeQueryDeclarationEntryOrchestrationPlan`
- `ForgeQueryDeclarationEntryOrchestrationOutcome`
- `ForgeQueryDeclarationEntryOrchestrationTranscript`
- `ForgeQueryDeclarationEntryOrchestrationChecked`
- `ForgeQueryDeclarationEntryOrchestrationProof`
- `ForgeQueryDeclarationEntryCrossingInventory`
- `ForgeQueryDeclarationEntryInspection`
- `ForgeQueryDeclarationEntryReadinessReport`

The main retained binding targets carried by those artifacts are:

- `ForgeQueryAdmittedDeclarationProgressionBindingTarget`
- `ForgeQueryDeclarationRoutePlanBindingTarget`
- `ForgeQueryDeclarationReceiptBindingTarget`
- `ForgeQueryDeclarationEnvelopeBindingTarget`

These belong to the shared retained target-binding seam that now connects
`9.3.7` contribution authoring and `9.3.8` declaration-entry/product
orchestration. They are not the same subsystem as the older query
canonicalization/slot-fulfillment binding code under `src/binding/`.

Use this quick chooser when the declaration-entry docs feel close together:

- choose orchestration when Query should own the current declaration-entry
  lowering path and default materialization policy for one already-assembled
  declaration input
- choose envelopes when you already have retained receipt-backed crossing truth
  and want the public crossing artifact directly
- choose inspection when you need one read artifact over retained seam truth
- choose readiness when you need family-level seam posture before or beside one
  concrete run

The current public orchestration grammar is intentionally layered:

- `orchestrate_declaration_entry(...)`
- `orchestrate_declaration_entry_checked(...)`
- `orchestrate_declaration_entry_proof(...)`
- `orchestrate_routes_from_progressed(...)`
- `orchestrate_receipt_from_progressed(...)`
- `orchestrate_envelope_from_progressed(...)`

Treat the trio as the generic declaration-input front door.
Treat the progressed route/receipt/envelope methods as compact product-target
projections over the same retained pipeline.
Treat the earlier declaration, legality, progression, foundational, route,
receipt, and envelope methods as the advanced explicit path.
If you need the locked grammar as data, use
`ForgeQueryDeclarationEntryOrchestrationVerbInventory::current()` and inspect
its rows instead of inferring the surface from scattered examples.

If you need the locked publication story as data, inspect the orchestration
plan and transcript instead of guessing from the current receipt or envelope
shape:

- `materialization_policy()`
- `materialization_tier()`
- `cost_posture()`
- `materialization_gate()`
- `foundational_evidence_profile()`

The orchestration front door also has one locked sequencing boundary:

- Query starts after your session or tool resolved the user's targets
- Query automates admitted handle through envelope only
- checked and proof-visible lanes keep stop posture and boundary honesty typed
- `Refused` means automation stopped intentionally, not that Query flattened
  every non-success posture into one error

It also has one locked materialization boundary:

- the default orchestration lane uses lean foundational evidence publication
- receipt and envelope publication stay support-ready by default
- richer publication changes descriptive breadth, not declaration-entry truth
- cost posture and publication gate are inspectable through the orchestration
  plan and transcript

The main checked and denied families are:

- `ForgeQueryDeclaredFamilyChecked`
- `ForgeQueryDeclarationAdmissionError`
- `ForgeQueryDeclarationLegalityChecked`
- `ForgeQueryDeclarationLegalityDenial`
- `ForgeQueryDeclarationAdmissionOrLegalityError`
- `ForgeQueryDeclarationProgressionChecked`
- `ForgeQueryDeclarationProgressionTerminalError`
- `ForgeQueryDeclarationEntryProgressionError`
- `ForgeQueryDeclarationFoundationalEvidenceChecked`
- `ForgeQueryDeclarationFoundationalEvidenceDenial`
- `ForgeQueryDeclarationRoutePlanChecked`
- `ForgeQueryDeclarationRoutePlanTerminalError`
- `ForgeQueryDeclarationEntryRoutePlanError`
- `ForgeQueryDeclarationReceiptChecked`
- `ForgeQueryDeclarationReceiptTerminalError`
- `ForgeQueryDeclarationEntryReceiptError`
- `ForgeQueryDeclarationEnvelopeChecked`
- `ForgeQueryDeclarationEnvelopeTerminalError`
- `ForgeQueryDeclarationEntryEnvelopeError`
- `ForgeQueryDeclarationRelationalRoutingChecked`
- `ForgeQueryDeclarationRelationalRoutingTerminalError`
- `ForgeQueryDeclarationEntryRelationalRoutingError`
- `ForgeQueryDeclarationBridgeRoutingChecked`
- `ForgeQueryDeclarationBridgeRoutingTerminalError`
- `ForgeQueryDeclarationEntryBridgeRoutingError`
- `ForgeQueryDeclarationSignalCompatibilityChecked`
- `ForgeQueryDeclarationSignalCompatibilityTerminalError`
- `ForgeQueryDeclarationEntrySignalCompatibilityError`
- `ForgeQueryDeclarationEntryOrchestrationTerminalError`

Start here if:

- you need ordinary Query-facing invariants: [Registering Domain Invariants Through Query](./invariants/registering-domain-invariants-through-query.md)
- you need declaration-preview workflow planning: [Preview Inspection And Mutation Planning](./workflow/preview-inspection-and-mutation-planning.md)
- you need successor truth across topology changes: [Continuity Contributions And Authoritative Successors](./continuity/continuity-contributions-and-authoritative-successors.md)
- you need lower-runtime causal explanation: [Lower-Runtime Explanation Contributions](./explanation/lower-runtime-explanation-contributions.md)
- you need projection aftermath contracts: [Projection Contract Consumption](./aftermath/projection-contract-consumption.md)
- you need to audit the proof surface itself: [Certification Surface And Closeout Bundle](./certification/certification-surface-and-closeout-bundle.md)
