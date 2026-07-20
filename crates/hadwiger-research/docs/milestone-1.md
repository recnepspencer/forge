# Milestone 1: Query-First Aspect-Aware Proof Artifact Pipeline

## Goal

Build the first Hadwiger-Nelson research pipeline as one downstream WORTH
domain crate that enters through `worth-query` from the first operation.

The milestone must produce a durable, queryable, replayable, aspect-aware, and
invalidation-aware artifact pipeline for finite graph lower-bound search. It
must not attempt to solve Hadwiger-Nelson by chat, and it must not let AI text,
unchecked solver output, floating-point geometry, visual inspection, or
whole-node validity flags become theorem authority.

The first slice prioritizes lower-bound witness search:

- admit real finite lower-bound claims only to the strength justified by the
  checked witness; a `chi(plane) >= 6` claim remains a frontier target because
  it would require an admitted finite unit-distance graph that is not
  5-colorable
- preserve the later path toward `chi(plane) = 7` by modeling 6-colorability
  with the same authority rules
- verify whole-plane coloring construction candidates through the same
  authority discipline rather than postponing upper-bound evidence as prose

## Why This Milestone Exists

The Hadwiger-Nelson problem is a perfect stress case for WORTH because useful
research artifacts are rarely globally valid or globally invalid. A candidate
graph can be admitted as an abstract obstruction, rejected as an exact
unit-distance witness, retained as a gadget source, and preserved as negative
evidence at the same time.

A naive implementation would store one candidate status and lose the research
value of failed attempts. A worse implementation would let a plausible AI
explanation, sampled geometry, or unchecked SAT result leak into theorem
authority.

WORTH Query already provides the platform-entry substrate this domain needs:

- typed domain entry through `WORTHQueryApplicationFacade::domain(...)`,
  `domain_checked(...)`, and `domain_proof_root(...)`
- configured domain handles through
  `WORTHQueryDomainEntryRoot::with_operating_context(...)`,
  `WORTHQueryConfiguredDomainHandleDraft::validate(...)`, and
  `WORTHQueryValidatedConfiguredDomainHandle::admit(...)`
- pre-runtime declaration entry
- canonical declarations through `WORTHQueryAdmittedConfiguredDomainHandle::declare(...)`
  and `declare_checked(...)`
- declaration-entry orchestration through
  `orchestrate_declaration_entry(...)`,
  `orchestrate_declaration_entry_outcome(...)`,
  `orchestrate_declaration_entry_checked(...)`, and
  `orchestrate_declaration_entry_proof(...)`
- aspect contracts through `WORTHQueryDeclarationAspectContract`,
  `WORTHQueryDeclarationAspectCoverage`, and retained declaration artifact
  aspect accessors
- retained binding through `WORTHQueryBindingOutcome<T>`,
  `WORTHQueryBindingChecked<T>`, and `WORTHQueryBindingTranscript<T>`
- ordinary outcomes through `WORTHQueryOrdinaryOutcome<T>`
- domain capability contributions for admission, support, workflow,
  continuity, aftermath, and explanation through
  `WORTHQueryContributionIntent::{admission,support,explanation,workflow,continuity}`
- contribution-composed orchestration through
  `orchestrate_declaration_with_contributions(...)`,
  `orchestrate_declaration_with_contributions_outcome(...)`,
  `orchestrate_declaration_with_contributions_checked(...)`, and
  `orchestrate_declaration_with_contributions_proof(...)`
- recovery briefs through `recover_from_outcome(...)` and the matching
  `recover_from_..._checked(...)` / `recover_from_..._proof(...)` methods

This crate must consume those Query surfaces instead of rebuilding a local
pseudo-Query runtime.

## Adversarial Constraint

A candidate graph with mixed admitted, rejected, deferred, heuristic, and
advisory evidence must never allow a theorem-like plane chromatic claim to
become admitted unless every required dependency is admitted by its owning
checker/runtime authority. Rejection of one aspect must invalidate only the
sound dependency closure, or conservatively escalate when closure is incomplete,
while preserving reusable abstract, solver, gadget, and advisory evidence.

If any path can:

- admit a proof claim from AI advisory text
- admit a lower-bound theorem from floating-only geometry
- treat a SAT solver result as authority without independent verification
- discard abstract obstruction evidence because geometry failed
- invalidate only one edge when a shared coordinate parameter makes local
  invalidation unsafe
- bypass Query declaration/progression/contribution posture
- collapse admitted, rejected, deferred, heuristic, and advisory posture into a
  boolean
- generate candidate graphs without recording hypotheses, equivalence basis,
  counterexample obligations, and expected information gain

then this milestone has failed.

## Governing Source Summary

- `MENTALITY.md`: the hard problem is false theorem authority under partial
  evidence. The spec must solve that before convenience search features.
- `arch_laws.md`: proof-bearing phases, typed errors, self-describing
  envelopes, and authority/derivation separation are mandatory.
- `composition_laws.md`: graph artifacts, checker admission, invalidation,
  proof claims, AI advisory, and certification must remain named
  responsibilities.
- `domain_structure_laws.md`: the tree must make source truth, derived aspects,
  speculative proposals, external observations, diagnostics, and explanations
  physically distinct.
- `perf_laws.md`: invalidation breadth must be bounded by semantic dependency
  scope, and conservative escalation must expose counters.
- `worth_query_vision.md`: Query is the typed, aspect-aware public entry for
  asking for and shaping truth; Hadwiger domain meaning stays downstream.
- `worth_query_roadmap.md`: Query-owned platform entry, retained binding,
  contribution-composed orchestration, ordinary outcomes, and recovery are the
  intended substrate for serious downstream domains.
- `worth-query` Milestone 9.3.7: domain-authored support, admission,
  workflow, continuity, aftermath, and explanation posture must enter through
  Query-owned contribution surfaces.
- `worth-query` Milestone 9.3.8: serious downstream domains must enter through
  Query declaration entry and progress through Query-owned declaration,
  legality, proof, route, envelope, binding, readiness, orchestration, and
  certification seams.
- `worth-runtime-bridge` roadmap: external solvers and checkers are controlled
  external engines with routed, receipt-backed, replayable boundary artifacts;
  the bridge sits underneath Query as the causal protocol layer between
  authoritative truth and derived computation, and must not own Hadwiger math
  semantics.
- `worth-relational` architecture: relation integrity and domain-specific
  invariants are phase-typed, cataloged, lowered, and executed at authority
  boundaries; Hadwiger may use that lower layer to make illegal research-graph
  states uncommittable, but not to promote mathematical conjectures to theorem
  authority.
- `worth-signal` temporal/async roadmap: later async checker execution and
  reactive invalidation should consume Signal-owned derived execution
  semantics instead of inventing a Hadwiger-local async lifecycle.

## Existing Query Surfaces To Consume

This section is a locked dependency map. If a later implementation uses a
different Query surface, the spec must be updated first.

### Platform Entry

Docs:

- `crates/worth-query/docs/domain-capabilities/platform-entry.md`

Exported APIs:

- `WORTHQueryApplicationFacade`
- `WORTHQueryApplicationFacade::domain_entry_support_snapshot()`
- `WORTHQueryApplicationFacade::domain(marker)`
- `WORTHQueryApplicationFacade::domain_checked(marker)`
- `WORTHQueryApplicationFacade::domain_proof_root(marker)`
- `WORTHQueryDomainEntryMarker`
- `WORTHQueryDomainEntryChecked`
- `WORTHQueryDomainEntryRoot`
- `WORTHQueryDomainEntryProofRoot`

Hadwiger use:

- define `HadwigerResearchDomainEntry`
- require the Query capability families needed for declaration entry,
  orchestration, contribution composition, and later bridge/signal preparation
- enter Query before constructing authoritative Hadwiger research artifacts

### Configured Domain Handles

Docs:

- `crates/worth-query/docs/domain-capabilities/configured-domain-handles.md`

Exported APIs:

- `WORTHQueryDomainOperatingContext`
- `WORTHQueryDomainEntryRoot::with_operating_context(...)`
- `WORTHQueryDomainEntryProofRoot::with_operating_context(...)`
- `WORTHQueryDomainEntryChecked::with_operating_context(...)`
- `WORTHQueryConfiguredDomainHandleDraft`
- `WORTHQueryConfiguredDomainHandleDraft::validate()`
- `WORTHQueryValidatedConfiguredDomainHandle`
- `WORTHQueryValidatedConfiguredDomainHandle::admit()`
- `WORTHQueryAdmittedConfiguredDomainHandle`
- `WORTHQueryConfiguredDomainHandleChecked`

Hadwiger use:

- define a stable research operating context that captures assumption regime,
  checker support regime, and conservative-invalidation regime
- keep candidate-specific graph meaning out of the operating context
- start all declaration work from an admitted configured handle

### Canonical Domain Declarations

Docs:

- `crates/worth-query/docs/domain-capabilities/canonical-domain-declarations.md`

Exported APIs:

- `WORTHQueryDeclarationInput`
- `WORTHQueryDeclarationFamilyMarker`
- `WORTHQueryDeclarationCanonicalEntry`
- `WORTHQueryDeclarationCanonicalValue`
- `WORTHQueryAdmittedConfiguredDomainHandle::declare(...)`
- `WORTHQueryAdmittedConfiguredDomainHandle::declare_checked(...)`
- `WORTHQueryAdmittedConfiguredDomainHandle::declare_with_version(...)`
- `WORTHQueryCanonicalDeclarationArtifact`
- `WORTHQueryDeclaredFamilyChecked`

Hadwiger use:

- model candidate graph proposals, embedding proposals, colorability requests,
  lower-bound witness requests, advisory notes, and explanation requests as
  declaration inputs
- derive canonical Hadwiger artifact identities from retained Query
  declaration identity rather than host-local hashing

### Declaration Legality And Progression

Docs:

- `crates/worth-query/docs/domain-capabilities/declaration-legality.md`
- `crates/worth-query/docs/domain-capabilities/declaration-progression.md`

Exported APIs:

- `review_legality(...)`
- `review_legality_checked(...)`
- `declare_and_review(...)`
- `progress_declaration(...)`
- `progress_declaration_checked(...)`
- `progress_declaration_recipe(...)`
- `progress_declaration_recipe_checked(...)`
- `declare_review_and_progress(...)`
- `WORTHQueryDeclarationLegalityContract`
- `WORTHQueryDeclarationLegalityChecked`
- `WORTHQueryDeclarationProgressionChecked`
- `WORTHQueryAdmittedDeclarationProgression`

Hadwiger use:

- use legality/progression as the Query-owned proof chain before Hadwiger
  artifact admission
- never let a raw domain declaration skip directly to an admitted checker or
  proof-claim artifact

### Declaration Entry Orchestration And Seam Inspection

Docs:

- `crates/worth-query/docs/domain-capabilities/declaration-entry-orchestration.md`
- `crates/worth-query/docs/domain-capabilities/declaration-entry-inspection.md`
- `crates/worth-query/docs/domain-capabilities/declaration-entry-readiness.md`
- `crates/worth-query/docs/domain-capabilities/workflow/single-declaration-to-envelope.md`

Exported APIs:

- `declaration_entry_crossing_inventory::<I>()`
- `declaration_entry_readiness::<I>()`
- `inspect_declaration_entry(subject)`
- `orchestrate_declaration_entry(input)`
- `orchestrate_declaration_entry_outcome(input)`
- `orchestrate_declaration_entry_checked(input)`
- `orchestrate_declaration_entry_proof(input)`
- `orchestrate_routes_from_progressed(...)`
- `orchestrate_receipt_from_progressed(...)`
- `orchestrate_envelope_from_progressed(...)`
- `WORTHQueryDeclarationEntryOrchestrationInput`
- `WORTHQueryDeclarationEntryOrchestrationOutcome`
- `WORTHQueryDeclarationEntryOrchestrationTranscript`

Hadwiger use:

- use this as the ordinary pre-runtime path from Hadwiger declaration to Query
  envelope
- use readiness and inspection to expose whether the current Query build admits
  the research declaration family before invoking real checker adapters

### Foundational Evidence, Routes, Receipts, And Envelopes

Docs:

- `crates/worth-query/docs/domain-capabilities/declaration-foundational-evidence.md`
- `crates/worth-query/docs/domain-capabilities/declaration-route-plan.md`
- `crates/worth-query/docs/domain-capabilities/declaration-boundary-receipts.md`
- `crates/worth-query/docs/domain-capabilities/declaration-boundary-envelopes.md`
- `crates/worth-query/docs/domain-capabilities/workflow/retained-artifact-to-next-step.md`

Exported APIs:

- `WORTHQueryDeclarationFoundationalEvidenceInput`
- `WORTHQueryDeclarationFoundationalEvidenceChecked`
- `WORTHQueryDeclarationFoundationalEvidence`
- `WORTHQueryAdmittedConfiguredDomainHandle::describe_foundational(...)`
- `WORTHQueryAdmittedConfiguredDomainHandle::describe_foundational_checked(...)`
- `WORTHQueryDeclarationRouteIntent`
- `WORTHQueryDeclarationRoutePlanInput`
- `WORTHQueryDeclarationRoutePlanChecked`
- `WORTHQueryAdmittedConfiguredDomainHandle::plan_routes(...)`
- `WORTHQueryAdmittedConfiguredDomainHandle::plan_routes_checked(...)`
- `WORTHQueryAdmittedConfiguredDomainHandle::plan_routes_from_progressed(...)`
- `WORTHQueryDeclarationReceiptInput`
- `WORTHQueryDeclarationReceiptChecked`
- `WORTHQueryAdmittedConfiguredDomainHandle::receipt_routes(...)`
- `WORTHQueryAdmittedConfiguredDomainHandle::receipt_routes_checked(...)`
- `WORTHQueryAdmittedConfiguredDomainHandle::receipt_routes_from_progressed(...)`
- `WORTHQueryDeclarationEnvelopeInput`
- `WORTHQueryDeclarationEnvelopeChecked`
- `WORTHQueryAdmittedConfiguredDomainHandle::envelope_routes(...)`
- `WORTHQueryAdmittedConfiguredDomainHandle::envelope_routes_checked(...)`
- `WORTHQueryAdmittedConfiguredDomainHandle::envelope_routes_from_progressed(...)`
- `bind_receipt_from_target(...)`
- `bind_envelope_from_target(...)`
- `bind_continuation_request_from_context(...)`
- `bind_continuation_from_target(...)`

Hadwiger use:

- treat Query declaration envelopes as the public crossing artifact for admitted,
  deferred, denied, or failed Hadwiger declaration truth
- preserve foundational evidence, route intent, receipt posture, and envelope
  digest on Hadwiger artifacts instead of recomputing boundary meaning locally
- model future external checker participation as route/receipt/envelope truth
  before bridge or signal execution is introduced
- reject implementations that construct receipts directly from legality or
  foundational evidence without route truth

### Aspect Contracts And Retained Binding

Docs:

- `crates/worth-query/docs/modeling/aspects-and-authority-lanes.md`
- `crates/worth-query/docs/domain-capabilities/typed-binding-pipeline.md`

Exported APIs:

- `WORTHQueryDeclarationAspectContract`
- `WORTHQueryDeclarationAspectCoverage`
- `WORTHQueryDeclarationAspectFit`
- `WORTHQueryDeclarationAspectPublication`
- `WORTHQueryBindingOutcome<T>`
- `WORTHQueryBindingChecked<T>`
- `WORTHQueryBindingTranscript<T>`
- `WORTHQueryBindingSourceKind`
- `WORTHQueryBindingSpecificity`
- `WORTHQueryBindingWrongWorld`
- `WORTHQueryBindingWrongHandle`
- `WORTHQueryBindingStale`
- `WORTHQueryBindingRebindRequired`
- `WORTHQueryBindingMissingRequiredAspect`
- `WORTHQueryBindingAspectConflict`
- `WORTHQueryBindingAuthorityMismatch`
- `WORTHQueryBindingBasisMismatch`

Hadwiger use:

- represent Hadwiger aspect requirements through Query aspect contracts where
  declaration entry needs public semantic slices
- use retained binding posture for stale, rebind-required, missing-aspect, and
  authority-mismatch explanations rather than inventing local generic errors

### Ordinary Outcomes And Recovery

Docs:

- `crates/worth-query/docs/domain-capabilities/ordinary-outcomes.md`
- `crates/worth-query/docs/domain-capabilities/recovery-boundary.md`
- `crates/worth-query/docs/domain-capabilities/workflow/stop-to-recovery.md`

Exported APIs:

- `WORTHQueryOrdinaryOutcome<T>`
- `WORTHQueryOrdinaryPosture`
- `WORTHQueryOrdinaryPostureKind`
- `WORTHQueryOrdinaryNextStep`
- `WORTHQueryAdmittedConfiguredDomainHandle::recover_from_outcome(...)`
- `recover_from_declaration_entry_checked(...)`
- `recover_from_declaration_entry_proof(...)`
- `recover_from_contribution_composed_checked(...)`
- `recover_from_contribution_composed_proof(...)`

Hadwiger use:

- return concise public outcomes without flattening denied, deferred,
  unsupported, stale, rebind-required, refused, or failed topology
- map rejection/partial-admission stops into Query recovery briefs wherever the
  stop occurred in Query-owned declaration or contribution posture

### Domain Capability Contributions And Contribution Composition

Docs:

- `crates/worth-query/docs/domain-capabilities/contribution-composed-orchestration.md`
- `crates/worth-query/docs/domain-capabilities/admission/advisory-and-violation-contributions.md`
- `crates/worth-query/docs/domain-capabilities/support/declaration-scoped-support-and-traceability.md`
- `crates/worth-query/docs/domain-capabilities/explanation/lower-runtime-explanation-contributions.md`
- `crates/worth-query/docs/domain-capabilities/aftermath/aftermath-review-support-eligibility-and-materialization.md`

Exported APIs:

- `WORTHQueryContributionComposedOrchestrationInput<D, I>`
- `WORTHQueryContributionIntent`
- `WORTHQueryContributionIntent::admission(...)`
- `WORTHQueryContributionIntent::support(...)`
- `WORTHQueryContributionIntent::explanation(...)`
- `WORTHQueryContributionIntent::workflow(...)`
- `WORTHQueryContributionIntent::continuity(...)`
- `orchestrate_declaration_with_contributions(...)`
- `orchestrate_declaration_with_contributions_outcome(...)`
- `orchestrate_declaration_with_contributions_checked(...)`
- `orchestrate_declaration_with_contributions_proof(...)`
- `WORTHQueryAdmissionContributionAuthoring`
- `WORTHQuerySupportContributionAuthoring`
- `WORTHQueryExplanationContributionAuthoring`
- `WORTHQueryAftermathContributionAuthoring`
- `WORTHQueryWorkflowContributionAuthoring`
- `WORTHQueryContinuityContributionAuthoring`

Hadwiger use:

- attach AI advisory, support traceability, rejection aftermath, and
  explanation posture to declaration-scoped research runs
- preserve partial contribution admission when one advisory/support contribution
  admits and another contribution denies
- keep AI advisory text in contribution/explanation posture, never in proof
  authority

### Grouped Authoring, Grouped Products, And Family Helpers

Docs:

- `crates/worth-query/docs/domain-capabilities/grouped-authoring.md`
- `crates/worth-query/docs/domain-capabilities/grouped-products.md`
- `crates/worth-query/docs/domain-capabilities/grouped-contributions.md`
- `crates/worth-query/docs/domain-capabilities/grouped-support-readiness.md`
- `crates/worth-query/docs/domain-capabilities/family-helpers.md`
- `crates/worth-query/docs/domain-capabilities/choosing/grouped-authoring-vs-grouped-products-vs-grouped-contributions.md`

Exported APIs:

- `WORTHQueryGroupedDeclarationInput::local_neighborhood(...)`
- `with_atomicity(...)`
- `with_grouping_intent(...)`
- `with_continuity_assumption(...)`
- `with_shared_posture_claim(...)`
- `with_shared_rationale(...)`
- `WORTHQueryGroupedAtomicity`
- `WORTHQueryGroupedIntent`
- `WORTHQueryGroupedContinuityAssumption`
- `WORTHQueryGroupedSharedPostureClaim`
- `declare_grouped(...)`
- `orchestrate_grouped_outcome(...)`
- `grouped_route_checked(...)`
- `grouped_receipt_checked(...)`
- `grouped_envelope_checked(...)`
- `grouped_support_report(...)`
- `grouped_contributions_checked(...)`
- `with_shared_support_contribution(...)`
- `with_shared_explanation_contribution(...)`
- `with_shared_workflow_contribution(...)`
- `recover_from_grouped_orchestration_checked(...)`
- `recover_from_grouped_orchestration_proof(...)`
- `family_helpers()`
- `geometry_helpers()`

Hadwiger use:

- use grouped authoring only when the group itself is part of the mathematical
  meaning, such as a gadget neighborhood, reduction neighborhood, or
  composition port bundle
- use grouped products when a retained grouped declaration needs per-member
  route, receipt, or envelope truth
- use grouped contributions when shared advisory/support/explanation posture
  must be distinguished from member-local posture
- do not loop over single declarations when grouped atomicity, continuity, or
  shared posture changes the claim's meaning
- do not reuse the existing geometry helper families as Hadwiger APIs; define
  Hadwiger-native helpers later only as thin projections onto canonical Query
  grouped and declaration surfaces

### Lower-Authority Routing And Signal Compatibility

Docs:

- `crates/worth-query/docs/domain-capabilities/declaration-relational-truth-routing.md`
- `crates/worth-query/docs/domain-capabilities/declaration-bridge-continuation-routing.md`
- `crates/worth-query/docs/domain-capabilities/declaration-signal-compatibility.md`
- `crates/worth-query/docs/domain-capabilities/choosing/signal-compatibility-vs-continuation-pipeline.md`

Exported APIs:

- `WORTHQueryDeclarationRelationalRoutingInput`
- `WORTHQueryDeclarationRelationalRoutingChecked`
- `WORTHQueryAdmittedConfiguredDomainHandle::route_relational_truth_checked(...)`
- `WORTHQueryAdmittedConfiguredDomainHandle::relational_truth_support::<I>()`
- `WORTHQueryDeclarationBridgeRoutingInput`
- `WORTHQueryDeclarationBridgeRoutingChecked`
- `WORTHQueryAdmittedConfiguredDomainHandle::route_bridge_continuation_checked(...)`
- `WORTHQueryAdmittedConfiguredDomainHandle::bridge_continuation_support::<I>()`
- `WORTHQueryDeclarationSignalCompatibilityInput`
- `WORTHQueryDeclarationSignalCompatibilityChecked`
- `WORTHQueryAdmittedConfiguredDomainHandle::signal_compatibility_checked(...)`
- `WORTHQueryAdmittedConfiguredDomainHandle::signal_compatibility_from_progressed(...)`
- `WORTHQueryAdmittedConfiguredDomainHandle::signal_compatibility_support::<I>()`

Hadwiger use:

- Milestone 1 should preserve compatibility with these lower-authority lanes but
  stop before real relational, bridge, or signal execution
- future checker-backed Hadwiger claims should enter relational/bridge/signal
  lanes from envelope-backed truth, not from raw Hadwiger artifacts
- signal compatibility is eligibility posture only; it must not be treated as
  live `worth-signal` execution or recomputation authority

### Continuation And Signal Compatibility For Later Milestones

Docs:

- `crates/worth-query/docs/domain-capabilities/continuation-pipeline.md`
- `crates/worth-query/docs/domain-capabilities/signal-compatibility-orchestration.md`
- `crates/worth-query/docs/domain-capabilities/workflow/envelope-to-signal-or-continuation.md`

Exported APIs:

- `prepare_continuation_from_target(...)`
- `prepare_continuation_from_target_outcome(...)`
- `prepare_continuation_from_target_checked(...)`
- `prepare_continuation_from_target_proof(...)`
- `prepare_continuation_from_context(...)`
- `execute_prepared_continuation(...)`
- `execute_prepared_continuation_outcome(...)`
- `orchestrate_signal_compatibility(...)`
- `orchestrate_signal_compatibility_outcome(...)`
- `orchestrate_signal_compatibility_checked(...)`
- `orchestrate_signal_compatibility_proof(...)`

Hadwiger use:

- keep real external checker execution and later reactive recomputation out of
  Milestone 1
- when those arrive, enter through prepared continuation and signal
  compatibility surfaces instead of a Hadwiger-local execution lifecycle

### Support, Inventory, Public Docs, And Certification

Docs:

- `crates/worth-query/docs/AI_README.md`
- `crates/worth-query/docs/foundations/support-matrix-and-admission.md`
- `crates/worth-query/docs/domain-capabilities/orchestration-inventory.md`
- `workspaces/worth-query/crates/worth-query/docs/domain-capabilities/runtime-installed-domains.md`
- `workspaces/worth-query/crates/worth-query/docs/domain-capabilities/conditional-installed-operations.md`
- `workspaces/worth-query/crates/worth-query/docs/domain-capabilities/public-doc-coverage.md`
- `workspaces/worth-query/crates/worth-query/docs/domain-capabilities/platform-entry-closeout.md`
- `workspaces/worth-query/crates/worth-query/docs/domain-capabilities/certification/certification-surface-and-closeout-bundle.md`
- `workspaces/worth-query/crates/worth-query/docs/domain-capabilities/certification/goldens-boundaries-and-hostile-certification.md`
- `crates/worth-query/docs/domain-capabilities/workflow/retained-artifact-to-next-step.md`
- `crates/worth-query/docs/domain-capabilities/choosing/binding-vs-orchestration-vs-helpers.md`
- `crates/worth-query/docs/domain-capabilities/recovery/README.md`
- `crates/worth-query/docs/domain-capabilities/grouped-authoring.md`
- `crates/worth-query/docs/domain-capabilities/grouped-products.md`
- `crates/worth-query/docs/domain-capabilities/grouped-contributions.md`
- `crates/worth-query/docs/capabilities/projection-consumption.md`

Exported APIs:

- `WORTHQueryRuntimeFacadeFamily`
- `WORTHQueryRuntimeFamilySupportStatus`
- `WORTHQueryOrchestrationSurfaceInventory::current()`
- `WORTHQueryOrchestrationInventoryAudit::current()`
- `WORTHQueryPublicDocCoverageInventory::current()`
- `WORTHQueryPublicDocCoverageAudit::current()`
- `WORTHQueryPlatformEntryCloseoutBundle`
- `WORTHQueryPlatformEntryAlignmentAudit`
- `WORTHQueryPlatformEntryParityAudit`
- `WORTHQueryPlatformEntryHostileAudit`

Hadwiger use:

- use `AI_README.md` as the first Query navigation document before choosing a
  Query lane; use linked docs for exact APIs and then verify signatures in code
- expose support/readiness posture before each stronger Query lane is attempted
- keep Hadwiger docs, golden transcripts, hostile lanes, and parity lanes
  aligned with the Query public-surface certification pattern
- do not claim a new Hadwiger public helper is stable until its generic Query
  path, helper path, ordinary path, checked path, and recovery path have parity
- preserve retained Query artifacts as next-step inputs rather than flattening
  them into Hadwiger-local queues or status enums
- use Query binding/resolver surfaces when a retained declaration, envelope,
  checker artifact, blocked proof claim, failure record, or experiment plan is
  consumed by the next operation
- use Query grouped authoring, grouped products, and grouped contributions when
  a gadget, reduction, graph composition, motif neighborhood, or failure scope
  has shared posture or neighborhood meaning
- use Query recovery briefs and recovery boundaries for Query-owned stops; local
  Hadwiger explanations may add domain meaning but must not replace the Query
  recovery shape
- use projection-consumption receipts when discovery or proof admission consumes
  materialized Query facts, so later operations do not reopen authority or
  scrape raw artifact payloads

### Query Invariant Registration And Denial Surfaces

Docs:

- `crates/worth-query/docs/domain-capabilities/invariants/registering-domain-invariants-through-query.md`
- `crates/worth-query/docs/domain-capabilities/invariants/capability-gaps-and-invariant-denials.md`

Exported APIs:

- `worth_query_domain(...).for_intent(...).register_invariant_catalog(...).because(...).materialize()`
- `WORTHQueryRuntime::builder().invariant_catalog(...)`
- `WORTHQueryRuntime::builder().custom_invariant(...)`
- `WORTHQueryRuntime::builder().register_invariant(...)`
- `WORTHQueryRuntime::builder().invariant_registration_artifact(...)`
- `WORTHQueryInvariantCapabilityContributionAuthoring::graph_capability_gap(...)`
- `WORTHQueryInvariantCapabilityContributionAuthoring::graph_invariant_denial(...)`
- `materialize_graph_composition_capability_support_row(...)`
- `materialize_graph_composition_domain_invariant_denial(...)`

Hadwiger use:

- AI-discovered invariant candidates are not runtime invariants when proposed;
  they are Hadwiger hypothesis artifacts with advisory or speculative posture
- only admitted, checked invariant definitions may be registered through Query
  invariant registration surfaces
- research-graph legality invariants are different from mathematical
  conjecture invariants: they may block malformed graph-memory, suppression,
  failure-residency, branch-promotion, or experiment-admission states without
  claiming anything about the chromatic number of the plane
- invariant-denial artifacts are used when a graph, gadget, composition, or
  reduction operation must be blocked because a named admitted invariant would
  be broken
- capability gaps and invariant denials must remain distinct digest families so
  "we cannot check this yet" never looks like "this violates the invariant"

## Runtime Bridge Contract To Preserve

Runtime Bridge is not the public entry point for this crate. Query is. The
bridge is the lower causal protocol layer Query can route toward when
Hadwiger checker execution needs external-process routing, replay, diagnostics,
or long-running lifecycle support.

Bridge source docs reviewed:

- `_docs/worth-runtime-bridge/worth_runtime_bridge_roadmap.md`
- `crates/worth-runtime-bridge/API_OVERVIEW.md`
- `crates/worth-runtime-bridge/CAUSAL_BUNDLES_AND_GUARANTEES.md`
- `crates/worth-runtime-bridge/CERTIFICATION_AND_HARNESS.md`

Actual bridge public surface concepts to preserve compatibility with:

- `worth_runtime_bridge::facade`
- `RuntimeBridge::builder()`
- `RuntimeBridgeBuilder::with_truth_source(...)`
- `RuntimeBridgeBuilder::with_compute_sink(...)`
- `RuntimeBridgeBuilder::build()`
- `bridge.route(...)`
- `bridge.evaluate_current(...)`
- `bridge.evaluate(...)`
- `bridge.speculate(...)`
- `bridge.diagnostics()`
- `BridgeTruthViewEvaluationRequest::for_branch_head(...)`
- `BridgeTruthViewEvaluationRequest::for_branch_snapshot(...)`
- `BridgeTruthViewEvaluationRequest::for_historical_commit(...)`
- `BridgeSpeculativeSessionHandle`
- retained/certification records such as `BridgeCanonicalRouteRecord`,
  `BridgeCanonicalHistoricalEvaluationRecord`, `BridgePreviewReplayBundle`,
  `BridgeWritebackReplayBundle`, `BridgeRouteContractProof`, and
  `CanonicalBridgeWorkloadRequest`

Hadwiger implications:

- Query owns the Hadwiger-facing declaration, contribution, ordinary outcome,
  recovery, and later continuation surfaces.
- Runtime Bridge owns deterministic routing, truth-view basis binding,
  snapshot-backed evaluation, speculation isolation, promotion/discard
  boundaries, replay-safe retained records, and bridge diagnostics.
- Bridge coordinates between authoritative truth and derived compute; it does
  not own Hadwiger proof semantics, solver semantics, geometry semantics,
  scheduling semantics, or theorem admission.
- Real checker boundary outputs in Milestone 1 must already look like future
  bridge-consumable causal evidence: stable input basis, route/evaluation
  identity, result digest, replay metadata, diagnostics, and authority crossing
  posture.
- External solver/checker integration must eventually enter below Query through
  bridge-style route/evaluate/speculate/diagnose contracts, not through
  Hadwiger-local host adapters that invent their own lifecycle.
- Speculative candidate generation, branch-local embedding exploration, and
  repair previews must remain non-authoritative until an explicit Query and
  bridge promotion boundary admits the result.
- Certification must compare causal bundles, not just final theorem-like
  outputs. If final values match but route, truth basis, speculation residue,
  promotion, replay, or diagnostics drift, the bridge-shaped obligation has
  failed.

## Query-Routed Relational And Signal Utilization

Hadwiger should exploit the lower runtimes without making them alternate public
entry points. Query remains the domain-facing facade, but Query-routed
declarations, contributions, readiness, recovery, and continuation artifacts may
carry lower-runtime-shaped evidence.

Relational-shaped use:

- graph identity, graph versions, gadget neighborhoods, embedding constraint
  scopes, failure records, hypothesis records, and suppression records should
  be modeled as authoritative research graph truth when their meaning is
  structural and replay-worthy
- graph-attached failure records should live at the narrowest graph scope that
  can honestly suppress future work: whole graph, induced subgraph, motif,
  gadget boundary, embedding region, coordinate-parameter family,
  colorability-encoding family, or composition port
- structural fingerprints should exist for graph shape, induced subgraph shape,
  gadget boundary shape, embedding-constraint shape, colorability encoding
  shape, and failure basis
- branch-local research worlds should be represented as speculative or
  branch-scoped truth through Query-owned declaration/progression posture, with
  promotion only through explicit Query and future bridge admission boundaries
- committed failures with structural recurrence potential should become
  queryable graph relations/aspects, not only diagnostic records

Relational invariant use:

- research-graph legality should eventually be enforced by relational
  extensible invariants registered through Query, not by Hadwiger-local
  best-effort validators
- invariant catalog work must stay below graph truth: it governs whether a
  research graph state may be committed, not whether a Hadwiger theorem is true
- invariant registration must use Query-owned registration surfaces such as
  `WORTHQueryRuntime::builder().invariant_catalog(...)`,
  `WORTHQueryRuntime::builder().custom_invariant(...)`,
  `WORTHQueryRuntime::builder().register_invariant(...)`, or
  `WORTHQueryRuntime::builder().invariant_registration_artifact(...)`
- invariant denials for graph operations must use Query-owned denial surfaces
  such as
  `WORTHQueryInvariantCapabilityContributionAuthoring::graph_invariant_denial(...)`
  and
  `materialize_graph_composition_domain_invariant_denial(...)`
- Milestone 1 defines the Hadwiger research-graph invariant catalog and proves
  its intended blocking behavior; always-on registration should wait until the
  graph-memory schema, failure-scope model, suppression semantics, and branch
  promotion rules are stable
- no AI-authored `InvariantHypothesis`, example-supported pattern, or
  discovery score may become a relational invariant unless a checked Hadwiger
  authority path admits it as a graph-legality rule rather than a theorem claim

Signal-shaped use:

- discovery frontier ranking, stale frontier marking, repeated-shape
  suppression, next-experiment priority, and checker-budget pressure are
  derived state over authoritative research graph truth
- signal-style derived state may make planning fast and explainable, but it is
  rebuildable from graph truth plus Query/bridge evidence and must never become
  theorem authority
- frontier recomputation should be dirty or maybe-stale when graph-attached
  failures, structural fingerprints, hypothesis scopes, checker support, or
  equivalence contracts change
- signal diagnostics are useful as future explanation/provenance shape, but
  Hadwiger must preserve the underlying Query and graph evidence needed to
  rebuild them

Query-routed lower-runtime rule:

- public Hadwiger operations enter through Query
- Query artifacts carry the handles, receipts, envelopes, support posture,
  contribution posture, recovery posture, and future continuation posture
- relational-shaped records and signal-shaped derived records are consumed or
  exposed through those Query surfaces
- no public Hadwiger API may require callers to import `worth-relational`,
  `worth-signal`, or `worth-runtime-bridge` directly for ordinary research work

## Product Decision Locks

- This milestone creates one crate: `hadwiger-research`.
- `worth-query` is the entry point, including pre-runtime artifact authoring.
- The crate may define Hadwiger-specific domain declarations and helper APIs,
  but those helpers must compile onto Query declaration entry handles.
- Hadwiger owns mathematical domain meaning; Query owns declaration identity,
  progression, foundational evidence, route plans, boundary receipts,
  envelopes, support/readiness, contribution posture, ordinary outcomes,
  recovery, and later continuation entry.
- Retained Hadwiger artifacts, checker outputs, blocked proof claims, failures,
  and experiment plans must be usable as Query-shaped next-step inputs; do not
  build a Hadwiger-local research queue when Query binding/resolver surfaces can
  preserve the lifecycle.
- Query recovery and ordinary outcome posture dominate local blocked-status
  enums whenever a stop is Query-owned. Hadwiger may attach domain explanation
  and authority-chain detail, but must preserve the Query stop/recovery shape.
- Discovery and proof admission should consume typed Query projection facts or
  retained artifact references instead of reopening lower authority or scraping
  raw payloads.
- External checkers own mathematical authority for their checked domains.
- AI is advisory only and may contribute explanation/support/aftermath posture,
  never proof admission.
- Proof claims may never be admitted above the weakest required dependency
  posture.
- Rejection is aspect-scoped, not node-scoped.
- Graph/gadget/reduction neighborhoods that have atomicity, continuity, or
  shared posture meaning must use grouped Query surfaces rather than scalar
  declaration loops.
- Milestone 1 requires real lower-authority execution for SAT colorability,
  exact geometry unit-distance checking, and whole-plane coloring construction
  verification. Mock checker output is not an implementation lane and may not
  satisfy, approximate, or stand in for any checker authority surface.
- Hadwiger public APIs must not expose `RuntimeBridge` as a second entry point;
  bridge-compatible data appears as retained causal evidence underneath Query
  declaration and envelope posture.
- Hadwiger public APIs must not expose `worth-relational` or `worth-signal` as
  second entry points; relational-shaped graph truth and signal-shaped derived
  frontier state must be reached through Query-shaped operations.
- Checked failures with structural recurrence potential must live on the
  research graph at the narrowest honest scope so future planning can suppress
  repeated dead ends before generating work.
- Whole-plane coloring construction verification is in scope as upper-bound
  evidence. It may be admitted only by a construction checker that verifies the
  construction contract; visual inspection, sampled rendering, or AI prose
  remain advisory.

## Public Shape

The primary public surface should be Query-facing, not a raw domain-core API.

Required public concepts:

- `HadwigerResearchDomainEntry`
- Hadwiger declaration families for:
  - candidate graph proposal
  - graph version proposal
  - embedding proposal
  - unit-distance verification request
  - k-colorability verification request
  - whole-plane coloring construction verification request
  - lower-bound witness admission request
  - AI advisory note
  - rejection explanation request
  - partial-admission explanation request
- Query helper methods or family helpers that lower into the same generic Query
  declaration, contribution, ordinary outcome, and recovery surfaces.

Required Query API alignment:

- domain entry must implement `WORTHQueryDomainEntryMarker`
- stable research context must implement `WORTHQueryDomainOperatingContext`
- every research operation that introduces domain-local meaning must implement
  `WORTHQueryDeclarationInput`
- each operation family must implement `WORTHQueryDeclarationFamilyMarker`
- the ordinary declaration path must be reachable through
  `WORTHQueryAdmittedConfiguredDomainHandle::declare(...)` and
  `orchestrate_declaration_entry_outcome(...)`
- envelope-producing paths must preserve `describe_foundational(...)`,
  `plan_routes_checked(...)`, `receipt_routes_checked(...)`, and
  `envelope_routes_checked(...)` truth when the operation crosses that seam
- advisory/support/explanation/workflow/continuity posture must use
  `WORTHQueryContributionIntent` and
  `orchestrate_declaration_with_contributions_outcome(...)`
- retained-artifact-to-next-step flows must use Query binding/resolver
  surfaces where the next operation consumes a retained declaration, envelope,
  checker artifact, proof claim, failure record, or experiment plan
- grouped Hadwiger helper paths must lower to `declare_grouped(...)`,
  `orchestrate_grouped_outcome(...)`, `grouped_*_checked(...)`, or
  `grouped_contributions_checked(...)` when group semantics are part of the
  claim
- discovery and proof-admission operations that consume materialized Query facts
  must preserve projection-consumption receipts instead of treating payload
  fields as ambient local data
- lower-authority compatibility must be represented through
  `route_relational_truth_checked(...)`, `route_bridge_continuation_checked(...)`,
  or `signal_compatibility_checked(...)` when a later milestone crosses those
  seams
- aftermath posture must use the standalone aftermath contribution authoring
  and materialization APIs until a composed aftermath intent exists
- recoverable stops must expose or project to `WORTHQueryRecoveryBrief`
  through existing `recover_from_...` methods when the stop is Query-owned

Forbidden public shape:

- public constructors that mint admitted theorem claims directly
- public booleans such as `is_valid_graph` or `is_unit_distance_graph`
- public APIs that run checkers outside Query-shaped declaration/progression
  posture
- public APIs that create receipt, envelope, grouped, bridge, or signal posture
  through Hadwiger-local status enums when Query already owns that posture
- public APIs that accept retained artifact digests as raw strings when a Query
  binding, projection-consumption receipt, or typed Hadwiger artifact reference
  is the honest input
- public APIs that ask callers to build or drive `RuntimeBridge` directly for
  ordinary Hadwiger research operations
- public APIs that ask callers to mutate lower-runtime graph truth or signal
  derived state directly for ordinary Hadwiger research operations
- public AI advisory APIs that return theorem authority

## Internal Responsibility Topology

The crate should use one crate with internal modules that preserve ownership:

- `query_entry`
  - Query domain marker, declaration families, configured handle integration,
    family helpers, grouped projections, and lower-authority seam adapters
- `domain_artifacts`
  - canonical Hadwiger artifacts such as graph identity, graph version, vertex,
    edge, embedding, encoding, solver run, checker result, gadget, reduction,
    and proof claim records
- `aspect_authority`
  - aspect kinds, authority postures, dependency edges, promotion rules,
    source artifacts, invalidation references, and aspect digests
- `mathematical_verification`
  - Query-facing real checker operations and local deterministic math
    subsystems for exact unit-distance verification, finite SAT colorability,
    solver model/refutation replay, and hexagonal whole-plane coloring
    construction verification
- `invalidation`
  - dependency closure, stale marking, surviving aspect reports, conservative
    escalation, repair obligations, and branch suggestions
- `proof_claims`
  - sealed theorem-like claim admission, weakest-dependency posture
    calculation, retained authority chains, blocked claims, background theorem
    retention, and exportable proof package metadata
- `proof_recovery`
  - Query-shaped recovery projections for blocked claims, Query-owned stops,
    surviving authority-chain evidence, and repair obligations
- `projection_consumption`
  - typed consumption records for materialized Query facts used by proof
    admission, discovery, failure attachment, and experiment planning
- `agent_advisory`
  - externally supplied agent proposals, explanations, repair notes, transcript
    digests, source artifacts, source aspects, and promotion path descriptors
- `discovery_frontier`
  - evidence corpora, motif observations, pattern signatures, invariant
    hypotheses, experiment plans, scorecards, frontier updates, equivalence
    contracts, and discovery counters
- `graph_memory`
  - graph-resident failure records, structural fingerprints, suppression
    relations, branch-local research scopes, failure-to-hypothesis links, and
    reactivation conditions exposed through Query-routed operations
- `research_graph_invariants`
  - Query-routed invariant catalog drafts, graph-legality rule descriptors,
    invariant violation records, invariant denial bindings, and certification
    cases for the relational invariant layer
- `derived_frontier`
  - signal-shaped derived planning state such as frontier priority,
    repeated-shape suppression, stale planning surfaces, and checker-budget
    pressure; this state is rebuildable from graph memory and Query evidence
- `research_harness`
  - certification cases, hostile lanes, parity lanes, canonical bundles, and
    counter assertions

No module named `common`, `helpers`, `manager`, or `service` should be used.

## Canonical Domain Artifacts

Milestone 1 must define typed canonical artifacts for the finite lower-bound
pipeline.

Required artifact families:

- `GraphIdentity`
- `GraphVersion`
- `VertexIdentity`
- `EdgeIdentity`
- `EmbeddingCandidate`
- `UnitDistanceVerification`
- `ColorabilityEncoding`
- `SolverRun`
- `ColorabilityVerification`
- `UnsatCoreArtifact`
- `GadgetDefinition`
- `GadgetContract`
- `GraphComposition`
- `ReductionTrace`
- `ProofClaim`
- `AIAdvisoryArtifact`
- `ResearchEvidenceCorpus`
- `MotifObservation`
- `PatternSignature`
- `InvariantHypothesis`
- `InvariantCandidate`
- `ExperimentPlan`
- `ExperimentResult`
- `DiscoveryScorecard`
- `DiscoveryFrontier`
- `GraphResidentFailure`
- `FailureScope`
- `FailureBasisFingerprint`
- `SuppressionRelation`
- `ReactivationCondition`
- `BranchLocalResearchScope`
- `HadwigerResearchInvariantCatalog`
- `ResearchGraphInvariantRule`
- `ResearchGraphInvariantViolation`
- `ResearchGraphInvariantDenial`
- `ResearchGraphInvariantRegistrationPlan`
- `DerivedFrontierState`

Each artifact must expose:

- stable identity
- source operation or declaration reference
- parent artifact references
- digest
- authority owner
- Query declaration/progression reference where applicable

The first implementation must produce checker-shaped authority artifacts from
the real checker paths required by this milestone. Hostile and unsupported
lanes should be exercised with real malformed inputs, incomplete contracts,
unsupported real capability declarations, or typed Query stops, not mock
checker artifacts.

## Aspect Model

Every candidate graph is represented as one conceptual node with multiple
aspect records. Aspect posture is the unit of admission and invalidation.

Required aspect kinds:

- `AbstractGraphStructure`
- `GeometricEmbedding`
- `UnitDistanceVerification`
- `FiveColorability`
- `SixColorability`
- `UnsatCore`
- `GadgetContract`
- `CompositionPort`
- `ReductionLineage`
- `ProofClaim`
- `Visualization`
- `AIAdvisory`

Each aspect record must include:

- aspect identity
- parent candidate identity
- aspect kind
- authority posture
- dependency edges
- source artifacts
- invalidation references
- recompute policy
- promotion rules
- digest

Authority postures:

- `Admitted`
- `Rejected`
- `Deferred`
- `Unsupported`
- `Heuristic`
- `Speculative`
- `Advisory`
- `Stale`

`Stale` is distinct from `Rejected`: stale means the aspect may have been true
under an older dependency basis and needs re-checking.

## Dependency Model

Dependency edges must be explicit and typed.

Required dependency edge families:

- graph version depends on graph identity
- aspect depends on aspect
- vertex depends on coordinate assignment
- edge depends on vertex positions
- unit-distance verification depends on edge lengths
- colorability encoding depends on graph version
- solver run depends on encoding
- colorability verification depends on solver run
- proof claim depends on verifications
- proof claim depends on aspects
- composition depends on gadget contracts
- reduction depends on source graph
- theorem claim depends on foundational assumptions
- AI advisory depends on source artifacts and aspects
- motif observation depends on source graph, grouped neighborhood, aspects,
  and observation method
- invariant hypothesis depends on motif observations and counterexample
  obligations
- dead-end signature depends on falsifying evidence, equivalence contract, and
  suppression scope
- graph-resident failure depends on failure scope, failure basis fingerprint,
  owning graph or neighborhood, and Query declaration/progression evidence
- suppression relation depends on graph-resident failure, pattern signature,
  experiment family, and reactivation condition
- derived frontier state depends on graph-resident failures, hypothesis
  posture, support/readiness posture, checker budgets, and structural
  fingerprints
- retired hypothesis record depends on hypothesis, retirement cause, and
  reactivation condition
- experiment suppression proof depends on proposed experiment and a prior
  dead-end, retired hypothesis, duplicate pattern, or budget basis
- experiment plan depends on hypothesis, equivalence contract, checker
  obligations, and search budget
- discovery scorecard depends on experiment results, negative evidence,
  surviving aspects, and novelty basis

The invalidation engine must carry enough dependency detail to distinguish:

- directly invalidated aspects
- downstream stale aspects
- surviving aspects
- blocked theorem claims
- reusable artifacts
- repeat obligations
- repair branches
- conservative escalation basis

## Discovery Operating Model

The system must not treat research as random candidate generation plus later
filtering. It must operate as a closed loop over retained evidence:

1. observe admitted, rejected, deferred, stale, and advisory artifacts
2. extract typed motif observations and structural measurements
3. propose hypothesis and invariant candidates with explicit basis evidence
4. plan bounded experiments that would confirm, refine, or falsify each
   candidate
5. execute those experiments through Query-facing declaration/checker posture
6. update candidate confidence, survival scope, falsification basis, and next
   obligations

AI may participate in steps 2 through 4 and in explanation of step 6. AI may
not collapse that loop by turning a pattern into authority.

Required discovery records:

- `ResearchEvidenceCorpus`
- `MotifObservation`
- `PatternSignature`
- `InvariantHypothesis`
- `InvariantCandidate`
- `CounterexampleObligation`
- `DeadEndSignature`
- `GraphResidentFailure`
- `FailureScope`
- `FailureBasisFingerprint`
- `SuppressionRelation`
- `ReactivationCondition`
- `RetiredHypothesisRecord`
- `ExperimentSuppressionProof`
- `ExperimentPlan`
- `ExperimentBatch`
- `ExperimentResult`
- `DiscoveryScorecard`
- `DiscoveryFrontier`
- `DerivedFrontierState`

Every `MotifObservation` must record:

- source graph or grouped neighborhood
- source aspects
- source posture at observation time
- canonical feature vector or structural signature
- observation method
- digest

Every `InvariantHypothesis` must record:

- statement
- scope
- basis observations
- required checker families
- expected failure modes
- counterexample obligations
- current posture
- confidence score
- digest

Hypothesis postures:

- `Proposed`
- `SupportedByExamples`
- `Falsified`
- `BlockedByMissingCapability`
- `PromotedToCheckedCandidate`
- `AdmittedInvariant`
- `Retired`

The only path from `InvariantHypothesis` to `AdmittedInvariant` is through
checked Hadwiger authority and, where runtime registration is required, Query
invariant registration. Example-supported hypotheses must remain advisory or
speculative until then.

Every `ExperimentPlan` must record:

- target hypothesis or pattern
- expected information gain
- mutation family or construction family
- required Query declaration family
- checker obligations
- stopping condition
- search budget
- equivalence contract
- novelty basis
- digest

Experiment planning must prefer falsifiable plans over volume. A plan that
generates many near-duplicates without an equivalence contract is invalid.

Every dead end must become operational state, not prose. When an experiment
falsifies a hypothesis, exhausts a construction family, or proves a candidate
class duplicate under the current equivalence contract, the system must emit a
`DeadEndSignature` or `RetiredHypothesisRecord` that future planning consumes.

`DeadEndSignature` must record:

- failed pattern signature
- invalidated hypothesis or construction family
- counterexample artifact or rejection basis
- equivalence contract used to classify recurrence
- scope of suppression
- expiry or revalidation condition
- digest

`GraphResidentFailure` must record:

- owning graph identity, graph version, or grouped neighborhood
- narrowest honest `FailureScope`
- failure basis fingerprint
- checker or Query posture that produced the failure
- affected experiment families
- suppression relations created from the failure
- reactivation condition
- digest

`FailureScope` must distinguish:

- whole graph
- induced subgraph
- motif
- gadget boundary
- composition port
- embedding region
- coordinate-parameter family
- colorability encoding family
- reduction lineage
- theorem-claim dependency slice

`SuppressionRelation` must connect a graph-resident failure to the experiment
or hypothesis families it blocks. It is a planning input, not a diagnostic
string.

`DerivedFrontierState` must record:

- source graph-memory digest
- source Query support/readiness digest
- source checker-budget digest
- prioritized experiment families
- stale frontier entries
- suppressed frontier entries
- recomputation cause
- digest

`RetiredHypothesisRecord` must record:

- retired hypothesis
- retirement cause
- falsifying evidence or exhaustion evidence
- surviving reusable motifs, if any
- future reactivation condition
- digest

`ExperimentSuppressionProof` must record:

- proposed experiment
- matching dead-end signature, retired hypothesis, duplicate pattern, or
  exhausted budget basis
- suppression reason
- what new evidence would be required to make the experiment admissible again
- digest

Experiment planning must consume these proof-bearing records before emitting an
`ExperimentPlan`. If a suppressed or retired path can be reissued without new
evidence that satisfies its reactivation condition, the discovery loop has
failed.

Graph-resident failure residency law:

- any checked failure that can recur structurally must be attached to the
  research graph as `GraphResidentFailure`
- the attachment scope must be the narrowest honest scope that can suppress
  future work without over-suppressing unrelated candidates
- broad attachment requires a conservative-escalation explanation and counters
- narrow attachment requires enough dependency detail to prove that related
  dead ends will not escape suppression through renaming, isomorphism, or
  cosmetic embedding changes
- future experiment planning must query graph-resident failures before
  generating candidate payloads

Derived frontier law:

- `DerivedFrontierState` is signal-shaped derived state and may be cached or
  recomputed later, but it must be reproducible from graph-resident failures,
  pattern signatures, hypotheses, Query support/readiness, and checker-budget
  evidence
- a stale derived frontier must block automatic experiment execution until it is
  recomputed or explicitly downgraded to advisory preview

Research graph invariant law:

- `HadwigerResearchInvariantCatalog` is the Milestone 1 representation of the
  relational invariant layer Hadwiger intends to register through Query after
  graph-memory semantics stabilize
- `ResearchGraphInvariantRule` must name the graph-memory state it governs, the
  relational execution point it expects, the Query registration or denial
  surface it maps to, and the evidence needed to treat the rule as admitted
- `ResearchGraphInvariantViolation` must record the illegal graph-memory state,
  offending relation or artifact identities, blocking rule, Query posture, and
  digest
- `ResearchGraphInvariantDenial` must be emitted when a graph, gadget,
  suppression, branch-promotion, or experiment-admission operation would violate
  a named research-graph invariant
- research-graph invariants may block commits, promotions, or executable
  experiment admission; they may not admit theorem claims or mathematical
  conjectures
- catalog registration must remain a separate step from catalog drafting, so a
  plausible but unstable rule cannot accidentally calcify the wrong graph
  schema

The discovery loop must expose counters for:

- candidates considered
- candidates pruned by equivalence
- experiments suppressed by dead-end signatures
- retired hypotheses reused as suppression basis
- motif observations extracted
- hypotheses proposed
- hypotheses falsified
- counterexample obligations generated
- experiments planned
- experiments executed
- experiments skipped by support/readiness posture
- repeated-shape suppression hits
- frontier width
- checker budget consumed

Discovery ranking must be explainable from retained artifacts. A score may use
novelty, survival across rejected attempts, checker cost, dependency breadth,
motif recurrence, and expected falsification value, but it must not use an
opaque "AI confidence" number as authority.

Required discovery laws:

- Negative evidence is first-class. Rejections and stale aspects must be mined
  for counterexample obligations and repair constraints.
- Survival matters. A motif that survives across unrelated failures should be
  promoted as more interesting than a motif that appears only in one lucky
  candidate.
- Falsification beats accumulation. The planner must prefer experiments that
  can disprove a hypothesis over experiments that merely add one more example.
- Equivalence is mandatory. Pattern reuse, near-duplicate suppression, and
  motif novelty require explicit canonical signatures and invalidation bases.
- Search cost is visible. Any generated experiment batch must carry budget and
  breadth counters before execution.
- AI is a hypothesis source and explanation assistant, not a discovery
  authority.

## Required Query-Facing Operations

Milestone 1 must specify and later implement these operations through
Query-facing declaration entry handles.

### Declare Candidate Graph

Input:

- graph identity metadata
- graph version payload
- optional embedding proposal
- optional intended route
- optional AI advisory source

Output:

- Query declaration/progression artifact
- canonical Hadwiger graph artifacts
- `AbstractGraphStructure` aspect candidate
- optional `GeometricEmbedding` aspect candidate

### Validate Unit-Distance Graph

Input:

- graph version identity
- embedding identity

Output:

- `UnitDistanceVerification`
- affected aspects
- proof claim candidate for finite unit-distance graph
- ordinary outcome or checked/proof-visible posture

Rules:

- exact or interval-certified geometry may admit unit-distance aspects
- floating-only geometry must defer geometry and theorem promotion
- unsupported coordinate fields must return unsupported posture
- failed exact edges reject unit-distance witness posture without invalidating
  abstract graph colorability unless graph structure changed

### Test K-Colorability

Input:

- graph version identity
- color count

Output:

- `ColorabilityEncoding`
- `SolverRun`
- `ColorabilityVerification`
- affected aspects
- proof claim candidate for abstract graph colorability

Rules:

- solver output alone is not authority
- satisfiable requires verified coloring model
- unsatisfiable requires verified certificate or independently checkable proof
- malformed encoding rejects the colorability result and downstream claims
- geometry is not required for abstract colorability

### Verify Whole-Plane Coloring Construction

Input:

- construction identity
- color count
- construction payload or retained construction source reference
- coverage contract
- boundary/exclusion contract

Output:

- whole-plane coloring construction artifact
- construction checker run
- construction verification artifact
- affected aspects
- proof claim candidate for upper-bound evidence

Rules:

- sampled renderings, screenshots, diagrams, or AI explanations are advisory
- a construction admits upper-bound evidence only when the checker verifies
  coverage, color assignment validity, and unit-distance exclusion according to
  the construction contract
- unsupported construction languages or incomplete coverage contracts return
  unsupported or missing posture rather than theorem authority
- whole-plane upper-bound evidence is independent from finite lower-bound
  graph obstruction evidence, but proof claims may depend on both when claiming
  equality

### Admit Lower-Bound Witness

Input:

- graph version identity
- embedding identity
- color count

Output:

- `ProofClaim`
- authority chain
- blocked-claim explanation if not admitted

Rules:

- for any `color_count = k`, `chi(plane) >= k + 1` requires admitted graph
  structure, admitted unit-distance verification, and admitted
  non-k-colorability
- for `color_count = 5`, this means `chi(plane) >= 6` must remain blocked until
  the crate can produce or retain an admitted finite unit-distance graph with
  admitted non-5-colorability; treating this as a missing smoke fixture would
  be unsound
- `chi(plane) = 7` requires an admitted lower-bound proof claim at 7 plus an
  admitted 7-color upper-bound source; weaker lower bounds must not be combined
  with the upper-bound theorem to claim exact equality
- proof claims may never be admitted above the weakest required dependency

### Explain Rejection

Input:

- rejected artifact identity or rejected aspect identity

Output:

- invalidated aspects
- stale proof claims
- surviving artifacts
- surviving aspects
- repeat obligations
- reusable negative evidence
- recommended repair branches
- conservative escalation explanation

### Explain Partial Admission

Input:

- candidate graph identity

Output:

- admitted aspects
- rejected aspects
- deferred aspects
- stale aspects
- theorem claims currently blocked
- next obligations
- reusable value
- advisory history

This operation is mandatory. The system must be able to report:

> Graph G is admitted as an abstract non-5-colorable graph, rejected as an
> exact unit-distance witness, and retained as a promising gadget source.

### Mine Research Patterns

Input:

- evidence corpus identity
- candidate graph or grouped neighborhood scope
- aspect filters
- observation method
- budget

Output:

- `MotifObservation` artifacts
- `PatternSignature` artifacts
- equivalence classes
- novelty report
- negative-evidence report

Rules:

- mining must consume retained admitted, rejected, deferred, stale, and
  advisory evidence
- motif observations must never imply proof authority
- equivalent pattern signatures must converge to one canonical identity
- negative evidence must remain queryable as a discovery source

### Propose Invariant Hypotheses

Input:

- motif observations
- surviving aspect reports
- rejected aspect reports
- optional AI advisory source

Output:

- `InvariantHypothesis`
- `CounterexampleObligation`
- explanation of why the hypothesis is falsifiable
- blocked posture if required checkers or Query lanes are unsupported

Rules:

- example support can only produce `SupportedByExamples`
- AI-authored hypotheses start at `Proposed` with advisory provenance
- a hypothesis without a counterexample obligation is not operationalized
- promotion to `InvariantCandidate` requires an explicit checker plan

### Plan Next Experiments

Input:

- discovery frontier
- hypothesis or pattern target
- search budget
- checker support posture
- novelty/equivalence policy

Output:

- `ExperimentPlan`
- `ExperimentBatch`
- `ExperimentSuppressionProof` for unsupported, duplicate, retired, or
  dead-end work
- graph-resident failure hits used for suppression
- expected information-gain explanation

Rules:

- plans must route later candidate work through Query-facing declaration
  handles
- plans must include stopping conditions and budget counters
- duplicate or near-duplicate generation must be suppressed through explicit
  equivalence contracts
- retired hypotheses and dead-end signatures must be checked before a plan is
  emitted
- graph-resident failures and suppression relations must be checked before
  candidate payload generation
- suppressed work must return an `ExperimentSuppressionProof`, not a string
  diagnostic
- plans that cannot falsify or refine a hypothesis must be ranked below plans
  that can

### Update Discovery Frontier

Input:

- experiment results
- rejection and partial-admission explanations
- graph-resident failures
- newly mined motifs
- hypothesis updates

Output:

- `DiscoveryScorecard`
- updated `DiscoveryFrontier`
- updated `DerivedFrontierState`
- retired hypotheses
- dead-end signatures
- graph-resident failures
- experiment suppression proofs
- new counterexample obligations
- next experiment priorities

Rules:

- failed attempts must update the frontier rather than disappearing
- ranking changes must be explainable from retained artifacts and counters
- no frontier score may promote an advisory artifact to theorem authority
- dead ends must suppress recurrence until new evidence satisfies their
  reactivation condition
- graph-resident failures must be inserted or updated before derived frontier
  recomputation

### Attach Failure To Research Graph

Input:

- failed checker, Query, contribution, or proof-claim posture
- affected graph, graph version, or grouped neighborhood
- dependency closure report
- structural fingerprint report
- conservative escalation basis, when narrow scope is unsafe

Output:

- `GraphResidentFailure`
- `FailureScope`
- `FailureBasisFingerprint`
- `SuppressionRelation`
- `ReactivationCondition`
- updated graph-memory digest

Rules:

- only checked or Query-owned failures can create graph-resident failure truth
- AI advisory may suggest a failure scope but cannot attach authoritative
  graph-resident failure truth
- the operation must choose the narrowest honest failure scope or explicitly
  emit conservative escalation
- failure attachment must be reachable through Query-facing declaration,
  contribution, ordinary outcome, or recovery posture
- future planning must consume the resulting suppression relations before
  candidate generation

### Recompute Derived Discovery Frontier

Input:

- graph-memory digest
- Query support/readiness digest
- checker budget state
- current hypothesis graph
- current equivalence contracts

Output:

- `DerivedFrontierState`
- stale frontier entries
- suppressed frontier entries
- next experiment priorities
- recomputation counters

Rules:

- derived frontier state may not become proof authority
- derived frontier state must be reproducible from graph-resident failures and
  Query evidence
- stale derived frontier state blocks automatic experiment execution

### Draft Research Graph Invariant Catalog

Input:

- graph-memory schema digest
- failure-scope model digest
- suppression relation model digest
- branch-promotion rule digest
- experiment-admission rule digest
- Query invariant registration support posture

Output:

- `HadwigerResearchInvariantCatalog`
- `ResearchGraphInvariantRule` artifacts
- `ResearchGraphInvariantRegistrationPlan`
- unsupported-registration posture, when Query or lower runtime support is
  missing

Rules:

- catalog drafting is Query-facing Hadwiger work, not direct relational runtime
  mutation
- every rule must identify whether it is intended for commit blocking,
  publication blocking, audit-only certification, or future runtime
  registration
- rules may govern graph-memory legality, suppression legality, branch
  promotion legality, and executable experiment admission
- rules may not encode theorem claims, unverified mathematical conjectures, or
  AI confidence
- runtime registration remains blocked until the governed graph-memory state is
  stable and the rule has checked Hadwiger authority

### Deny Invariant-Violating Research Graph Operation

Input:

- proposed graph-memory, suppression, branch-promotion, gadget, composition, or
  experiment-admission operation
- active research invariant catalog or named invariant rule
- Query contribution posture

Output:

- `ResearchGraphInvariantViolation`
- `ResearchGraphInvariantDenial`
- canonical Query invariant-denial artifact where the lower-runtime target
  family is supported
- recovery or reactivation explanation

Rules:

- invariant denials must be specific to a named rule and offending graph-memory
  relation or artifact identity
- capability gaps and invariant denials must remain separate outcomes
- an invariant denial may block operation admission, commit, promotion, or
  automatic execution
- an invariant denial may not become evidence for a Hadwiger theorem claim
- denial materialization should use
  `WORTHQueryInvariantCapabilityContributionAuthoring::graph_invariant_denial(...)`
  and
  `materialize_graph_composition_domain_invariant_denial(...)` when the
  operation maps to Query's supported runtime-facing target family

## Invalidation Law

When an aspect is rejected, the crate must compute impact through dependency
closure before any downstream claim is updated.

Granular invalidation is allowed only when dependency closure is complete and
trusted. If the failed evidence depends on shared parameters or incomplete
dependency tracking, the system must escalate to conservative invalidation.

Example:

- edge `e42` fails exact unit-distance verification
- `e42` depends on coordinate parameter `alpha`
- `alpha` also supports 500 other edge proofs
- local-only invalidation is unsafe
- every aspect depending on `alpha` becomes stale or rejected according to its
  dependency contract

The invalidation result must expose counters for:

- dependency edges traversed
- directly rejected aspects
- stale aspects
- preserved aspects
- blocked claims
- repair obligations generated
- conservative escalation groups
- reusable artifacts retained

## AI Advisory Law

AI may:

- propose candidate graphs
- propose embeddings
- propose graph mutations
- propose gadgets
- propose motif interpretations
- propose invariant hypotheses
- propose falsification experiments
- rank experiment plans using retained evidence
- summarize rejection impact
- suggest repair branches
- explain why a motif appears promising
- draft human-readable notes over artifact IDs and aspect IDs

AI may not:

- admit a proof claim
- assert exact unit-distance validity
- assert non-k-colorability
- promote sampling to theorem authority
- erase failed attempts
- bypass Query declaration/progression posture
- promote advisory text into proof authority

Every AI artifact must include:

- model identity
- prompt digest
- source artifacts
- source aspects
- advisory posture
- hypothesis or experiment target where applicable
- counterexample obligation references where applicable
- promotion path through checked artifacts

AI advisory artifacts should be represented through Query contribution and
explanation/aftermath surfaces where possible.

AI-discovered hypotheses must be reproducible without the AI transcript: the
retained artifact graph must include the motif observations, source aspects,
counterexample obligations, and experiment plans that justify why the
hypothesis is worth testing.

## Real Checker And Runtime Adapter Law

Milestone 1 must integrate real checker engines for the mathematical facts it
claims to verify. There is no mock checker admission path. A checker artifact
that carries mathematical posture must come from the real SAT, geometry,
construction, canonicalization, reduction, or contract-checking implementation
named by the artifact.

Synthetic records may appear only at non-mathematical boundaries, such as typed
Query stop construction, malformed input tests, or unsupported capability
declarations. They must not be named as checker adapters, must not produce
checker authority artifacts, and must not satisfy any aspect dependency.

Required real checker subsystems:

- SAT solving for k-colorability
- SAT model verification for satisfiable coloring results
- SAT unsat certificate or independently checkable proof verification for
  non-k-colorability
- exact geometry checking for unit-distance embedding claims
- whole-plane coloring construction verification for upper-bound claims
- graph canonicalization/isomorphism checking for recurrence suppression,
  duplicate experiment suppression, and graph-resident failure matching
- graph reduction and gadget/contract checking where a proof claim depends on
  reduced or composed artifacts

Each adapter output must include:

- adapter identity
- adapter version
- input artifact digests
- result posture
- certificate or model reference when applicable
- affected aspects
- digest

Real checker subsystems must enter through Query-shaped Hadwiger operations.
They may reuse `worth-math` exact arithmetic and certified predicate surfaces
and `worth-geom` exact geometry facade surfaces when those surfaces satisfy the
Hadwiger authority contract. If the existing Worth crates do not expose the
needed Hadwiger-specific surface, this crate must define a local geometry or
solver subsystem rather than weakening the checker requirement.

External-process and long-running forms should route through
`worth-runtime-bridge` style boundary artifacts rather than changing Hadwiger
domain semantics. The public Hadwiger entry remains Query.

## Phases

### Phase 1: Query Domain Entry And Declaration Families

Define the Hadwiger Query domain marker and declaration families for
pre-runtime research artifacts.

Must ship:

- `HadwigerResearchDomainEntry`
- `HadwigerResearchOperatingContext`
- declaration family markers for candidate graph, embedding, colorability,
  lower-bound witness, advisory note, rejection explanation, and partial
  admission explanation
- `WORTHQueryDomainEntryMarker` implementation for
  `HadwigerResearchDomainEntry`
- `WORTHQueryDomainOperatingContext<HadwigerResearchDomainEntry>`
  implementation for `HadwigerResearchOperatingContext`
- `WORTHQueryDeclarationInput<HadwigerResearchDomainEntry>` implementations
  for the first declaration request types
- `WORTHQueryDeclarationFamilyMarker<HadwigerResearchDomainEntry>`
  implementations for the first declaration family markers
- tests or examples proving the ordinary path starts from
  `WORTHQueryApplicationFacade::domain(...).with_operating_context(...).validate().admit()`
- Query-facing helper API that lowers to `declare_checked(...)`,
  `orchestrate_declaration_entry_outcome(...)`, or
  `orchestrate_declaration_with_contributions_outcome(...)`
- support/readiness examples that call
  `declaration_entry_crossing_inventory::<I>()`,
  `declaration_entry_readiness::<I>()`, and any relevant grouped or
  lower-authority support report before stronger execution is attempted
- compile-fail or facade tests preventing raw theorem admission bypass

### Phase 2: Canonical Artifact And Digest Model

Define the finite graph lower-bound artifact model.

Must ship:

- graph, embedding, encoding, solver, checker, reduction, gadget, proof claim,
  and AI advisory artifacts
- stable digest rules
- parent/source references
- authority owner fields
- retained Query declaration digest/reference fields where an artifact comes
  from `WORTHQueryCanonicalDeclarationArtifact`
- retained Query foundational evidence, route plan, receipt, and envelope
  digest/reference fields where an artifact crosses those Query seams
- bridge-compatible causal evidence fields for real checker artifacts:
  truth-view basis digest, route/evaluation identity, diagnostics digest,
  replay digest, and authority-crossing posture
- no public constructors for admitted proof claims

### Phase 3: Aspect Authority And Dependency Closure

Build the aspect record model and dependency graph.

Must ship:

- aspect kinds and postures
- typed dependency edges
- promotion rule descriptors
- recompute policies
- dependency closure reports
- conservative invalidation escalation posture
- mapping from Hadwiger aspect kinds to Query declaration aspect contracts where
  an aspect participates in declaration entry, binding, contribution
  composition, or recovery
- explicit use of `WORTHQueryBindingStale`,
  `WORTHQueryBindingRebindRequired`, `WORTHQueryBindingMissingRequiredAspect`,
  `WORTHQueryBindingAspectConflict`, or equivalent local posture only when the
  stop is outside Query-owned binding
- grouped-aspect participation rules for gadget, reduction, or composition
  neighborhoods whose group-level posture changes the claim meaning

### Phase 4: Real Checker Admission Pipeline

Build query-facing operations over real checker outputs.

Must ship:

- real unit-distance validation pipeline
- real k-colorability validation pipeline
- real solver result verification pipeline
- real whole-plane coloring construction verification pipeline
- malformed encoding rejection
- floating geometry deferment
- unsupported checker gap posture
- public ordinary/checkable operation paths that return or project through
  `WORTHQueryOrdinaryOutcome<T>` where the stop is Query-owned
- checker result artifacts that retain the Query declaration/progression
  identity that authorized the request
- real checker boundary outputs that can be wrapped by Query foundational
  evidence, route plan, receipt, and envelope surfaces without changing
  Hadwiger math semantics
- checker outputs that are future-compatible with Runtime Bridge
  `route(...)`, `evaluate_current(...)`, `evaluate(...)`, `speculate(...)`, and
  diagnostics/replay bundle obligations without exposing bridge as the
  Hadwiger public entry point
- typed non-authority Query stops for replay, unsupported, malformed, and
  failure posture tests, without checker artifact construction
- explicit refusal to treat signal compatibility or prepared continuation as
  checker execution

### Phase 5: Proof Claim Admission

Build lower-bound witness and upper-bound construction admission over admitted
aspects.

Must ship:

- weakest-dependency posture calculation
- generic sealed lower-bound claim admission rule `not-k-colorable =>
  chi(plane) >= k + 1`, with tests proving the `k = 5` frontier lane remains
  blocked unless real admitted non-5-colorability and real admitted
  unit-distance evidence both exist
- whole-plane upper-bound construction admission rule
- `chi(plane) = 7` claim rule requiring admitted lower-bound evidence strong
  enough for 7 and
  admitted whole-plane 7-coloring upper-bound construction evidence, or an
  explicitly retained admitted background theorem artifact when the construction
  itself is outside the checked corpus
- blocked claim explanation
- authority chain output that retains Query declaration references, checker
  artifacts, admitted aspect records, projection-consumption receipts where
  materialized facts were consumed, and background theorem references where
  they are used
- proof-claim admission API that is private to the crate or sealed behind a
  proving function; public callers may inspect proof claims but may not mint
  admitted proof claims directly
- proof-claim blocked state that records whether the blocker came from
  Hadwiger aspect authority, Query declaration progression, Query route/receipt
  posture, contribution denial, or future lower-authority compatibility
- blocked proof claims must project Query-owned stops into Query recovery shape
  rather than inventing a Hadwiger-only blocked-status lifecycle
- admitted background theorem artifacts may participate in the `chi(plane)=7`
  rule only through a sealed retention/admission function that records source,
  theorem statement, authority owner, provenance digest, and why the theorem is
  acceptable as retained background authority
- checked whole-plane construction evidence remains the preferred upper-bound
  lane when available; the background theorem lane is for explicitly retained
  external authority, not for AI prose or informal citation text

### Phase 6: Rejection And Partial Admission Explanations

Build the explanation surfaces that make failed research useful.

Must ship:

- `explain_rejection`
- `explain_partial_admission`
- surviving aspect/artifact reports
- repair obligations
- reusable negative evidence
- conservative escalation explanation
- projection into `WORTHQueryRecoveryBrief` when the rejection was reached
  through declaration entry, contribution composition, binding, or continuation
  posture that Query already owns
- grouped recovery projection through `recover_from_grouped_orchestration_checked(...)`
  or `recover_from_grouped_orchestration_proof(...)` when the stop preserves
  member-local grouped context
- blocked proof claims, failed checker results, and partial authority chains
  remain retained next-step inputs for recovery, repair planning, and discovery;
  explanations may summarize them but must not replace the retained records
- explanation surfaces must distinguish Query-owned stops, Hadwiger
  aspect-authority blockers, checker rejections, projection-consumption
  failures, grouped-neighborhood stops, and future lower-runtime compatibility
  stops

### Phase 7: Discovery Loop And Invariant Hypotheses

Build the operational research loop that turns retained evidence into motifs,
invariant candidates, and next experiments.

Must ship:

- `ResearchEvidenceCorpus`
- `MotifObservation`
- `PatternSignature`
- `InvariantHypothesis`
- `InvariantCandidate`
- `CounterexampleObligation`
- `DeadEndSignature`
- `GraphResidentFailure`
- `FailureScope`
- `FailureBasisFingerprint`
- `SuppressionRelation`
- `ReactivationCondition`
- `RetiredHypothesisRecord`
- `ExperimentSuppressionProof`
- `ExperimentPlan`
- `ExperimentBatch`
- `ExperimentResult`
- `DiscoveryScorecard`
- `DiscoveryFrontier`
- `DerivedFrontierState`
- `mine_research_patterns`
- `propose_invariant_hypotheses`
- `plan_next_experiments`
- `update_discovery_frontier`
- `attach_failure_to_research_graph`
- `recompute_derived_discovery_frontier`
- canonical equivalence contracts for pattern signatures and near-duplicate
  experiment suppression
- Query binding/resolver consumption of retained evidence before planning a
  next experiment from a prior artifact, blocked claim, failure, hypothesis, or
  scorecard
- projection-consumption receipts for any materialized Query fact used as a
  discovery feature, motif observation, failure fingerprint, or experiment
  eligibility input
- graph-resident failure attachment for checked failures with structural
  recurrence potential
- failure-scope selection that proves narrow suppression or emits conservative
  escalation
- derived frontier recomputation from graph memory, Query support/readiness,
  hypothesis posture, and checker-budget evidence
- research-graph invariant candidate inputs derived from stabilized
  graph-memory, failure-scope, suppression, branch-promotion, and
  experiment-admission records
- dead-end suppression that prevents retired or falsified paths from becoming
  new experiment plans without qualifying new evidence
- reactivation conditions for retired hypotheses and dead-end signatures
- discovery counters for candidate breadth, suppression hits, hypothesis
  falsification, skipped unsupported work, and checker budget
- proof that rejected, stale, and deferred artifacts remain available to the
  discovery loop as negative or partial evidence
- Query invariant registration remains blocked until an invariant candidate is
  admitted by the proper Hadwiger/checker authority

### Phase 8: Research Graph Invariant Catalog Draft

Define the lower-authority legality layer that can later be registered through
Query into relational invariant authority.

Must ship:

- `HadwigerResearchInvariantCatalog`
- `ResearchGraphInvariantRule`
- `ResearchGraphInvariantViolation`
- `ResearchGraphInvariantDenial`
- `ResearchGraphInvariantRegistrationPlan`
- Query-facing catalog drafting operation
- Query-facing invariant-denial operation for supported runtime-facing graph
  targets
- explicit separation between graph-legality invariants, mathematical
  conjecture hypotheses, and theorem claims
- rule families for failure residency, suppression relation legality,
  hypothesis lifecycle legality, branch-promotion legality, and executable
  experiment-admission legality
- compatibility fields for Query invariant registration surfaces:
  `WORTHQueryRuntime::builder().invariant_catalog(...)`,
  `WORTHQueryRuntime::builder().custom_invariant(...)`,
  `WORTHQueryRuntime::builder().register_invariant(...)`, and
  `WORTHQueryRuntime::builder().invariant_registration_artifact(...)`
- compatibility fields for Query invariant-denial surfaces:
  `WORTHQueryInvariantCapabilityContributionAuthoring::graph_invariant_denial(...)`
  and `materialize_graph_composition_domain_invariant_denial(...)`
- certification cases proving each rule blocks the illegal graph-memory
  shape it claims to govern
- proof that catalog drafting does not register always-on lower-runtime
  authority until graph-memory semantics are stable and the rule has checked
  Hadwiger authority

### Phase 9: Agent Advisory And Experiment Proposal Intake

Make Hadwiger an optimal closed-loop research instrument for external agents.
Phase 9 does not embed an AI runtime, call model APIs, choose models, or run
prompt orchestration inside the crate. Instead, it gives agents a typed,
Query-shaped way to attach creative observations, hypotheses, repair ideas, and
experiment proposals to retained Hadwiger evidence. The runtime remains the
discipline layer: proposals are canonicalized, checked against suppression and
legality rules, routed through Query contribution surfaces where appropriate,
and kept non-authoritative until real checker/proof/invariant lanes admit them.

Must ship:

- `AgentAdvisoryArtifact` or a revised `AIAdvisoryArtifact` shape whose public
  vocabulary makes clear that the source is external advisory input, not an
  in-crate AI generator
- prompt/source digest, agent identity, tool identity, transcript digest, and
  cited retained-evidence references
- typed intake requests for motif observations, invariant hypothesis
  suggestions, experiment proposals, repair suggestions, and admission
  caution/violation advisories
- `AgentExplorationBatch` and checked admission of a batch into retained
  advisory/proposal artifacts
- advisory-to-checked-artifact promotion path descriptors that name the future
  checker/proof/Query lane required for promotion but do not perform promotion
- `WORTHQueryContributionIntent::explanation(...)` or
  `WORTHQueryContributionIntent::support(...)` usage for advisory notes that
  annotate declaration-scoped research artifacts
- `WORTHQueryContributionIntent::admission(...)` only for advisory/violation
  posture about declaration admission, never for theorem authority
- `WORTHQueryContributionIntent::aftermath(...)` is not currently exported;
  aftermath-specific use must go through the standalone aftermath contribution
  authoring/materialization APIs named in the domain-capabilities facade, or
  the spec must be revised when a composed aftermath intent exists
- grouped advisory/support/explanation contributions must use grouped
  contribution surfaces when one advisory applies to a whole neighborhood and
  separate member-local advisory where it applies only to one member
- agent-generated motif, invariant, repair, and experiment suggestions must
  materialize as advisory/proposal artifacts that point at retained discovery
  records, not as free-form search commands
- every experiment proposal must still pass Phase 7 dead-end suppression and
  Phase 8 research-graph legality before becoming an `ExperimentPlan`
- tests proving advisory/proposal artifacts cannot admit theorem, checker,
  proof-claim, or Query invariant authority

### Phase 10: Certification Harness

Close the first milestone with hostile scenarios and canonical bundles.

Must ship certification for:

- bad edge rejection with partial survival
- floating geometry deferment
- exact geometry unit-distance admission and exact geometry rejection
- bad SAT encoding rejection
- real SAT satisfiable model verification
- real SAT unsatisfiable certificate or independently checkable proof
  verification
- unchecked SAT output rejection
- whole-plane coloring construction admission for a verified construction
- whole-plane coloring construction rejection for coverage or unit-distance
  exclusion failure
- gadget composition contract failure
- coloring boundary failure as a real unsupported/missing checker posture, not
  as postponed future-route handwaving
- granular invalidation unsafe
- admitted lower-bound candidate
- AI advisory boundary
- explain partial admission
- route/receipt/envelope parity for a candidate graph declaration
- grouped neighborhood parity for a gadget or composition-port bundle
- support/readiness denial before attempting an unsupported stronger lane
- recovery parity for ordinary, checked, proof-visible, and grouped stops
- bridge-shaped causal bundle parity for real checker lanes, including
  truth-view basis, route identity, evaluation identity, diagnostics digest,
  replay digest, and explicit promotion/discard posture where a preview was
  involved
- discovery loop that mines a rejected graph for reusable motifs and produces a
  falsifiable next experiment
- invariant hypothesis lifecycle from proposed to falsified without losing the
  negative evidence
- invariant hypothesis lifecycle from proposed to supported-by-examples without
  accidental admission
- duplicate experiment suppression through an explicit equivalence contract
- dead-end recurrence suppression through `DeadEndSignature`
- retired hypothesis recurrence suppression through `RetiredHypothesisRecord`
- suppressed experiment return path through `ExperimentSuppressionProof`
- skipped discovery work when required Query/checker support is not admitted
- graph-resident failure suppresses an isomorphic renamed dead end before
  candidate payload generation
- failure attached too broadly triggers over-suppression certification failure
- failure attached too narrowly triggers repeated-dead-end certification failure
- stale `DerivedFrontierState` blocks automatic experiment execution until
  recomputed or explicitly downgraded to advisory preview
- research-graph invariant catalog draft rejects malformed failure residency,
  suppression, branch-promotion, hypothesis lifecycle, and executable
  experiment-admission states
- invariant denial materializes through Query-owned denial posture without
  collapsing into a capability gap or theorem claim
- runtime invariant registration remains blocked for unstable or merely
  advisory rules

The certification harness should mirror Query's public-surface pattern:

- inventory of public Hadwiger helper paths and their generic Query paths
- golden transcripts for ordinary, checked, proof, grouped, and recovery lanes
- hostile cases showing helper APIs cannot bypass Query entry or theorem
  authority

## Acceptance Evidence

The milestone is complete only when the crate emits a certification bundle with:

- candidate graph digest
- graph version digest
- embedding digest
- unit-distance verification digest
- colorability encoding digest
- solver run digest
- colorability verification digest
- whole-plane coloring construction digest
- whole-plane coloring construction verification digest
- aspect dependency graph digest
- proof claim digest
- invalidation report digest
- partial admission explanation digest
- advisory artifact digest where applicable
- evidence corpus digest
- motif observation digest
- pattern signature digest
- invariant hypothesis digest
- dead-end signature digest
- graph-resident failure digest
- failure basis fingerprint digest
- suppression relation digest
- research graph invariant catalog digest
- research graph invariant rule digest
- research graph invariant violation digest
- research graph invariant denial digest
- research graph invariant registration plan digest
- retired hypothesis digest
- experiment suppression proof digest
- experiment plan digest
- discovery frontier digest
- derived frontier state digest
- authority chain digest
- blocked claim digest
- counter snapshot

Required assertion classes:

- equivalent Query declaration/helper paths converge to the same canonical
  Hadwiger artifacts
- intentionally different graph, embedding, or checker evidence changes the
  relevant digests
- illegal theorem admission fails through typed or compile-time boundaries
- floating geometry cannot admit theorem claims
- unchecked solver output cannot admit colorability claims
- real SAT model and unsat-certificate verification are distinguished from
  unchecked solver output
- exact geometry checker artifacts, not floating distances, admit
  unit-distance aspects
- whole-plane coloring construction verification admits upper-bound evidence
  only after coverage, coloring, and unit-distance exclusion contracts are
  checked
- AI advisory cannot admit proof claims
- rejection preserves independent admitted aspects
- unsafe local invalidation escalates conservatively
- rejected and stale artifacts still participate in motif mining
- hypotheses without counterexample obligations cannot enter experiment
  planning
- AI-proposed invariants remain advisory until checked and admitted
- supported-by-examples does not equal admitted invariant
- experiment planning suppresses near-duplicates through canonical equivalence
- experiment planning suppresses known dead ends through proof-bearing records
- graph-resident failures suppress structurally recurring dead ends before
  candidate generation
- research-graph invariants block illegal graph-memory, suppression,
  branch-promotion, hypothesis lifecycle, and experiment-admission states
- invariant denials remain distinct from capability gaps and proof-claim
  blockers
- drafted research-graph invariant catalogs do not become registered
  lower-runtime authority without checked Hadwiger admission
- derived frontier state is reproducible from graph memory and Query evidence
- retired hypotheses cannot be replanned unless their reactivation condition is
  satisfied by new evidence
- discovery ranking is explainable from retained evidence and counters

## Must Preserve

- Query remains the public entry point.
- Hadwiger math semantics remain in this crate.
- External checkers own checked mathematical authority.
- WORTH Runtime Bridge owns later controlled external engine routing.
- WORTH Signal owns later derived invalidation/recompute execution.
- Relational-shaped graph memory and Signal-shaped derived frontier state are
  consumed through Query-shaped Hadwiger surfaces.
- Relational invariant authority remains below graph truth and is reached
  through Query-owned registration or denial surfaces.
- Research-graph legality invariants do not admit Hadwiger mathematical claims.
- AI remains advisory.
- Failed attempts remain structured evidence.
- Failed attempts remain discovery input, not only diagnostics.
- Invariant discovery remains hypothesis-driven and falsification-oriented.
- Proof claims remain dependency-posture bounded.

## Out Of Scope

- UI
- chatbot surface
- durable store-backed replay
- long-running async solver lifecycle beyond Query-shaped operation results and
  retained bridge-compatible causal evidence
- external checker orchestration that bypasses Query declaration/progression,
  route, receipt, envelope, and ordinary outcome posture
- arbitrary whole-plane construction languages without a checked construction
  contract
- always-on relational runtime invariant registration before the research graph
  schema and checked rule authority are stable
- automated candidate generation at scale without the discovery frontier,
  equivalence, and falsification machinery defined here

These remain later milestones once the aspect-aware proof artifact pipeline and
real checker authority lanes are certified.

## Self-Check

- Does this milestone solve a real structural problem? Yes: false theorem
  authority, loss of partial evidence, and unstructured search that cannot
  learn from failures.
- Is the adversarial constraint precise and load-bearing? Yes: every phase
  exists to prevent authority leakage or unsound invalidation.
- Does the milestone preserve crate authority boundaries? Yes: Query owns
  entry/progression, Hadwiger owns domain meaning, checkers own mathematical
  authority, Bridge/Signal own later runtime mechanics.
- Does the milestone define proof obligations, not just implementation tasks?
  Yes: certification requires hostile lanes, parity, typed rejection, discovery
  frontier updates, falsifiable hypotheses, and canonical bundles.
- Could a competent engineer map this spec into honest types, modules, and
  tests? Yes: phases name the public surface, internal topology, artifacts,
  discovery loop, operations, and certification outputs.
- Does the milestone belong here? Yes: it is the first crate-local milestone
  because it makes Query-first artifact authority, real checker authority, and
  discovery memory real before search automation, UI, or unchecked exploration
  loops.
