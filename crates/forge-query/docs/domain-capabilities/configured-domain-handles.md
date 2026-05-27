# Configured Domain Handles

## What This Feature Is

Configured domain handles are the next step after platform entry. They bind a
downstream domain marker to a typed operating context and turn that pair into a
Query-owned configured handle.

The important boundary is:

- your downstream crate owns domain identity and operating-context values
- Query owns the configured-handle lifecycle around them

That lifecycle gives you draft, validated, admitted, and checked forms without
falling back to raw IDs, ambient builder state, or host-local policy glue.

## Why You Use It

- freeze the stable operating regime your domain is working inside
- make capability and config-section requirements explicit before declaration
  work begins
- get one canonical handle identity digest for the configured domain context
- fail early when the operating regime is deferred, unsupported, or invalid for
  the current Query build

## Stable Entry Points

- `ForgeQueryDomainOperatingContext`
- `ForgeQueryDomainEntryRoot::with_operating_context(...)`
- `ForgeQueryDomainEntryProofRoot::with_operating_context(...)`
- `ForgeQueryDomainEntryChecked::with_operating_context(...)`
- `ForgeQueryConfiguredDomainHandleDraft`
- `ForgeQueryValidatedConfiguredDomainHandle`
- `ForgeQueryAdmittedConfiguredDomainHandle`
- `ForgeQueryConfiguredDomainHandleChecked`

## API Reference

Operating-context contract:

- `required_capability_families() -> &'static [ForgeQueryCapabilityFamily]`
- `required_config_sections() -> &'static [ForgeQueryConfigSectionFamily]`
- `context_identity_digest() -> String`

Configured-handle entry points:

- `with_operating_context(context) -> ForgeQueryConfiguredDomainHandleDraft<D, C>`
- `validate() -> Result<ForgeQueryValidatedConfiguredDomainHandle<D, C>, ForgeQueryConfiguredDomainHandleInvalidContext<D, C>>`
- `admit() -> Result<ForgeQueryAdmittedConfiguredDomainHandle<D, C>, ForgeQueryConfiguredDomainHandleAdmissionError<D, C>>`

Validated and admitted handle inspection:

- `domain_key() -> &'static str`
- `display_name() -> &'static str`
- `operating_context() -> &C`
- `support_snapshot() -> &ForgeQueryDomainEntrySupportSnapshot`
- `required_capability_families() -> &[ForgeQueryCapabilityFamily]`
- `required_config_sections() -> &[ForgeQueryConfigSectionFamily]`
- `operating_context_identity_digest() -> &str`
- `handle_identity_digest() -> &str`

Admitted-handle declaration evidence entry points:

- `describe_foundational(subject) -> Result<ForgeQueryDeclarationFoundationalEvidence<D, I>, ForgeQueryDeclarationFoundationalEvidenceDenial<D, I>>`
- `describe_foundational_checked(subject) -> ForgeQueryDeclarationFoundationalEvidenceChecked<D, I>`
- `describe_foundational_with_profile(subject, profile) -> Result<ForgeQueryDeclarationFoundationalEvidence<D, I>, ForgeQueryDeclarationFoundationalEvidenceDenial<D, I>>`

Admitted-handle route-planning entry points:

- `plan_routes(subject) -> Result<ForgeQueryDeclarationRoutePlan<D, I>, ForgeQueryDeclarationRoutePlanTerminalError<D, I>>`
- `plan_routes_checked(subject) -> ForgeQueryDeclarationRoutePlanChecked<D, I>`
- `plan_routes_from_progressed(progressed) -> Result<ForgeQueryDeclarationRoutePlan<D, I>, ForgeQueryDeclarationRoutePlanTerminalError<D, I>>`
- `plan_routes_from_progressed_with_intent(progressed, intent) -> Result<ForgeQueryDeclarationRoutePlan<D, I>, ForgeQueryDeclarationRoutePlanTerminalError<D, I>>`
- `declare_review_progress_describe_and_plan(input) -> Result<ForgeQueryDeclarationRoutePlan<D, I>, ForgeQueryDeclarationEntryRoutePlanError<D, I>>`

Admitted-handle receipt entry points:

- `receipt_routes(subject) -> Result<ForgeQueryDeclarationReceipt<D, I>, ForgeQueryDeclarationReceiptTerminalError<D, I>>`
- `receipt_routes_checked(subject) -> ForgeQueryDeclarationReceiptChecked<D, I>`
- `receipt_routes_from_progressed(progressed) -> Result<ForgeQueryDeclarationReceipt<D, I>, ForgeQueryDeclarationReceiptTerminalError<D, I>>`
- `receipt_routes_from_progressed_with_intent(progressed, intent) -> Result<ForgeQueryDeclarationReceipt<D, I>, ForgeQueryDeclarationReceiptTerminalError<D, I>>`
- `declare_review_progress_describe_plan_and_receipt(input) -> Result<ForgeQueryDeclarationReceipt<D, I>, ForgeQueryDeclarationEntryReceiptError<D, I>>`

Admitted-handle envelope entry points:

- `envelope_routes(subject) -> Result<ForgeQueryDeclarationEnvelope<D, I>, ForgeQueryDeclarationEnvelopeTerminalError<D, I>>`
- `envelope_routes_checked(subject) -> ForgeQueryDeclarationEnvelopeChecked<D, I>`
- `envelope_routes_from_progressed(progressed) -> Result<ForgeQueryDeclarationEnvelope<D, I>, ForgeQueryDeclarationEnvelopeTerminalError<D, I>>`
- `envelope_routes_from_progressed_with_intent(progressed, intent) -> Result<ForgeQueryDeclarationEnvelope<D, I>, ForgeQueryDeclarationEnvelopeTerminalError<D, I>>`
- `declare_review_progress_describe_plan_receipt_and_envelope(input) -> Result<ForgeQueryDeclarationEnvelope<D, I>, ForgeQueryDeclarationEntryEnvelopeError<D, I>>`

Admitted-handle relational-routing entry points:

- `route_relational_truth(subject) -> Result<ForgeQueryDeclarationRelationalRouting<D, I>, ForgeQueryDeclarationRelationalRoutingTerminalError<D, I>>`
- `route_relational_truth_checked(subject) -> ForgeQueryDeclarationRelationalRoutingChecked<D, I>`
- `route_relational_truth_from_progressed(progressed) -> Result<ForgeQueryDeclarationRelationalRouting<D, I>, ForgeQueryDeclarationRelationalRoutingTerminalError<D, I>>`
- `route_relational_truth_from_progressed_with_intent(progressed, intent) -> Result<ForgeQueryDeclarationRelationalRouting<D, I>, ForgeQueryDeclarationRelationalRoutingTerminalError<D, I>>`
- `declare_review_progress_describe_plan_receipt_envelope_and_route_relational_truth(input) -> Result<ForgeQueryDeclarationRelationalRouting<D, I>, ForgeQueryDeclarationEntryRelationalRoutingError<D, I>>`
- `relational_truth_support::<I>() -> ForgeQueryDeclarationRelationalRoutingSupportReport<D, I>`

Admitted-handle bridge-routing entry points:

- `route_bridge_continuation(subject) -> Result<ForgeQueryDeclarationBridgeRouting<D, I>, ForgeQueryDeclarationBridgeRoutingTerminalError<D, I>>`
- `route_bridge_continuation_checked(subject) -> ForgeQueryDeclarationBridgeRoutingChecked<D, I>`
- `route_bridge_continuation_from_progressed(progressed) -> Result<ForgeQueryDeclarationBridgeRouting<D, I>, ForgeQueryDeclarationBridgeRoutingTerminalError<D, I>>`
- `route_bridge_continuation_from_progressed_with_intent(progressed, intent) -> Result<ForgeQueryDeclarationBridgeRouting<D, I>, ForgeQueryDeclarationBridgeRoutingTerminalError<D, I>>`
- `declare_review_progress_describe_plan_receipt_envelope_and_route_bridge_continuation(input) -> Result<ForgeQueryDeclarationBridgeRouting<D, I>, ForgeQueryDeclarationEntryBridgeRoutingError<D, I>>`
- `bridge_continuation_support::<I>() -> ForgeQueryDeclarationBridgeRoutingSupportReport<D, I>`

Admitted-handle signal-compatibility entry points:

- `signal_compatibility(subject) -> Result<ForgeQueryDeclarationSignalCompatibility<D, I>, ForgeQueryDeclarationSignalCompatibilityTerminalError<D, I>>`
- `signal_compatibility_checked(subject) -> ForgeQueryDeclarationSignalCompatibilityChecked<D, I>`
- `signal_compatibility_from_progressed(progressed) -> Result<ForgeQueryDeclarationSignalCompatibility<D, I>, ForgeQueryDeclarationSignalCompatibilityTerminalError<D, I>>`
- `signal_compatibility_from_progressed_with_intent(progressed, intent) -> Result<ForgeQueryDeclarationSignalCompatibility<D, I>, ForgeQueryDeclarationSignalCompatibilityTerminalError<D, I>>`
- `declare_review_progress_describe_plan_receipt_envelope_and_check_signal_compatibility(input) -> Result<ForgeQueryDeclarationSignalCompatibility<D, I>, ForgeQueryDeclarationEntrySignalCompatibilityError<D, I>>`
- `signal_compatibility_support::<I>() -> ForgeQueryDeclarationSignalCompatibilitySupportReport<D, I>`

Admitted-handle continuation entry points:

- `prepare_continuation_from_target(request) -> ForgeQueryPreparedContinuationOutcome<D, I>`
- `prepare_continuation_from_target_outcome(request) -> ForgeQueryOrdinaryOutcome<ForgeQueryPreparedContinuation<D, I>>`
- `prepare_continuation_from_target_checked(request) -> ForgeQueryPreparedContinuationChecked<D, I>`
- `prepare_continuation_from_target_proof(request) -> ForgeQueryPreparedContinuationTranscript<D, I>`
- `prepare_continuation_from_context(request) -> ForgeQueryPreparedContinuationOutcome<D, I>`
- `prepare_continuation_from_context_outcome(request) -> ForgeQueryOrdinaryOutcome<ForgeQueryPreparedContinuation<D, I>>`
- `prepare_continuation_from_context_checked(request) -> ForgeQueryPreparedContinuationChecked<D, I>`
- `prepare_continuation_from_context_proof(request) -> ForgeQueryPreparedContinuationTranscript<D, I>`
- `execute_prepared_continuation(prepared) -> ForgeQueryContinuationExecutionOutcome<D, I>`
- `execute_prepared_continuation_outcome(prepared) -> ForgeQueryOrdinaryOutcome<ForgeQueryContinuationExecution<D, I>>`
- `execute_prepared_continuation_checked(prepared) -> ForgeQueryContinuationExecutionChecked<D, I>`
- `execute_prepared_continuation_proof(prepared) -> ForgeQueryContinuationExecutionTranscript<D, I>`

Admitted-handle seam-ledger entry points:

- `declaration_entry_crossing_inventory::<I>() -> ForgeQueryDeclarationEntryCrossingInventory<D, I>`
- `declaration_entry_readiness::<I>() -> ForgeQueryDeclarationEntryReadinessReport<D, I>`
- `inspect_declaration_entry(subject) -> Result<ForgeQueryDeclarationEntryInspection<D, I>, ForgeQueryDeclarationEntryInspectionError<D, I>>`

Admitted-handle orchestration entry points:

- `orchestrate_declaration_entry(input) -> Result<ForgeQueryDeclarationEnvelope<D, I>, ForgeQueryDeclarationEntryOrchestrationTerminalError<D, I>>`
- `orchestrate_declaration_entry_outcome(input) -> ForgeQueryOrdinaryOutcome<ForgeQueryDeclarationEnvelope<D, I>>`
- `orchestrate_declaration_entry_checked(input) -> ForgeQueryDeclarationEntryOrchestrationOutcome<D, I>`
- `orchestrate_declaration_entry_proof(input) -> ForgeQueryDeclarationEntryOrchestrationTranscript<D, I>`
- `orchestrate_routes_from_progressed(progressed) -> Result<ForgeQueryDeclarationRoutePlan<D, I>, ForgeQueryDeclarationRoutePlanTerminalError<D, I>>`
- `orchestrate_routes_from_progressed_with_intent(progressed, intent) -> Result<ForgeQueryDeclarationRoutePlan<D, I>, ForgeQueryDeclarationRoutePlanTerminalError<D, I>>`
- `orchestrate_receipt_from_progressed(progressed) -> Result<ForgeQueryDeclarationReceipt<D, I>, ForgeQueryDeclarationReceiptTerminalError<D, I>>`
- `orchestrate_receipt_from_progressed_with_intent(progressed, intent) -> Result<ForgeQueryDeclarationReceipt<D, I>, ForgeQueryDeclarationReceiptTerminalError<D, I>>`
- `orchestrate_envelope_from_progressed(progressed) -> Result<ForgeQueryDeclarationEnvelope<D, I>, ForgeQueryDeclarationEnvelopeTerminalError<D, I>>`
- `orchestrate_envelope_from_progressed_with_intent(progressed, intent) -> Result<ForgeQueryDeclarationEnvelope<D, I>, ForgeQueryDeclarationEnvelopeTerminalError<D, I>>`

Admitted-handle typed binding entry points:

- `bind_declaration_from_context(request) -> ForgeQueryBindingOutcome<ForgeQueryCanonicalDeclarationArtifact<D, I>>`
- `bind_declaration_from_context_outcome(request) -> ForgeQueryOrdinaryOutcome<ForgeQueryCanonicalDeclarationArtifact<D, I>>`
- `bind_declaration_from_context_checked(request) -> ForgeQueryBindingChecked<ForgeQueryCanonicalDeclarationArtifact<D, I>>`
- `bind_declaration_from_context_proof(request) -> ForgeQueryBindingTranscript<ForgeQueryCanonicalDeclarationArtifact<D, I>>`
- `bind_route_request_from_context(request) -> ForgeQueryBindingOutcome<ForgeQueryDeclarationRoutePlanInput<D, I>>`
- `bind_route_request_from_context_outcome(request) -> ForgeQueryOrdinaryOutcome<ForgeQueryDeclarationRoutePlanInput<D, I>>`
- `bind_route_request_from_context_checked(request) -> ForgeQueryBindingChecked<ForgeQueryDeclarationRoutePlanInput<D, I>>`
- `bind_route_request_from_context_proof(request) -> ForgeQueryBindingTranscript<ForgeQueryDeclarationRoutePlanInput<D, I>>`
- `bind_receipt_request_from_context(request) -> ForgeQueryBindingOutcome<ForgeQueryDeclarationReceiptInput<D, I>>`
- `bind_receipt_request_from_context_outcome(request) -> ForgeQueryOrdinaryOutcome<ForgeQueryDeclarationReceiptInput<D, I>>`
- `bind_receipt_request_from_context_checked(request) -> ForgeQueryBindingChecked<ForgeQueryDeclarationReceiptInput<D, I>>`
- `bind_receipt_request_from_context_proof(request) -> ForgeQueryBindingTranscript<ForgeQueryDeclarationReceiptInput<D, I>>`
- `bind_envelope_request_from_context(request) -> ForgeQueryBindingOutcome<ForgeQueryDeclarationEnvelopeInput<D, I>>`
- `bind_envelope_request_from_context_outcome(request) -> ForgeQueryOrdinaryOutcome<ForgeQueryDeclarationEnvelopeInput<D, I>>`
- `bind_envelope_request_from_context_checked(request) -> ForgeQueryBindingChecked<ForgeQueryDeclarationEnvelopeInput<D, I>>`
- `bind_envelope_request_from_context_proof(request) -> ForgeQueryBindingTranscript<ForgeQueryDeclarationEnvelopeInput<D, I>>`
- `bind_continuation_request_from_context(request) -> ForgeQueryBindingOutcome<ForgeQueryContinuationBindingInput<D, I>>`
- `bind_continuation_request_from_context_outcome(request) -> ForgeQueryOrdinaryOutcome<ForgeQueryContinuationBindingInput<D, I>>`
- `bind_continuation_request_from_context_checked(request) -> ForgeQueryBindingChecked<ForgeQueryContinuationBindingInput<D, I>>`
- `bind_continuation_request_from_context_proof(request) -> ForgeQueryBindingTranscript<ForgeQueryContinuationBindingInput<D, I>>`
- `bind_route_from_target(request) -> ForgeQueryBindingOutcome<ForgeQueryDeclarationRoutePlanInput<D, I>>`
- `bind_route_from_target_outcome(request) -> ForgeQueryOrdinaryOutcome<ForgeQueryDeclarationRoutePlanInput<D, I>>`
- `bind_route_from_target_checked(request) -> ForgeQueryBindingChecked<ForgeQueryDeclarationRoutePlanInput<D, I>>`
- `bind_route_from_target_proof(request) -> ForgeQueryBindingTranscript<ForgeQueryDeclarationRoutePlanInput<D, I>>`
- `bind_receipt_from_target(request) -> ForgeQueryBindingOutcome<ForgeQueryDeclarationReceiptInput<D, I>>`
- `bind_receipt_from_target_outcome(request) -> ForgeQueryOrdinaryOutcome<ForgeQueryDeclarationReceiptInput<D, I>>`
- `bind_receipt_from_target_checked(request) -> ForgeQueryBindingChecked<ForgeQueryDeclarationReceiptInput<D, I>>`
- `bind_receipt_from_target_proof(request) -> ForgeQueryBindingTranscript<ForgeQueryDeclarationReceiptInput<D, I>>`
- `bind_envelope_from_target(request) -> ForgeQueryBindingOutcome<ForgeQueryDeclarationEnvelopeInput<D, I>>`
- `bind_envelope_from_target_outcome(request) -> ForgeQueryOrdinaryOutcome<ForgeQueryDeclarationEnvelopeInput<D, I>>`
- `bind_envelope_from_target_checked(request) -> ForgeQueryBindingChecked<ForgeQueryDeclarationEnvelopeInput<D, I>>`
- `bind_envelope_from_target_proof(request) -> ForgeQueryBindingTranscript<ForgeQueryDeclarationEnvelopeInput<D, I>>`
- `bind_continuation_from_target(request) -> ForgeQueryBindingOutcome<ForgeQueryContinuationBindingInput<D, I>>`
- `bind_continuation_from_target_outcome(request) -> ForgeQueryOrdinaryOutcome<ForgeQueryContinuationBindingInput<D, I>>`
- `bind_continuation_from_target_checked(request) -> ForgeQueryBindingChecked<ForgeQueryContinuationBindingInput<D, I>>`
- `bind_continuation_from_target_proof(request) -> ForgeQueryBindingTranscript<ForgeQueryContinuationBindingInput<D, I>>`

Handle-independent orchestration grammar inventory:

- `ForgeQueryDeclarationEntryOrchestrationVerbInventory::current()`
- `ForgeQueryDeclarationEntryOrchestrationVerbInventory::verbs()`
- `ForgeQueryDeclarationEntryOrchestrationVerb::{public_name, family, exposure_level, ceiling, canonical_base_name}`

Orchestration artifact inspection:

- `ForgeQueryDeclarationEntryOrchestrationInput::{declaration_family_key, handle_identity_digest, operating_context_identity_digest, exposure_level, artifact_policy}`
- `ForgeQueryDeclarationEntryOrchestrationPlan::{declaration_family_key, handle_identity_digest, operating_context_identity_digest, exposure_level, artifact_policy, ceiling_stage, automation_boundary, automation_steps, explicit_caller_handoff_steps, step_plan, orchestration_identity_digest}`
- `ForgeQueryDeclarationEntryOrchestrationPlan::{materialization_policy, materialization_tier, cost_posture, materialization_gate, foundational_evidence_profile, descriptive_materialization_cost}`
- `ForgeQueryDeclarationEntryOrchestrationOutcome::{stop_stage, declaration_family_key, retained_digest, outcome_identity_digest, is_automation_refusal, is_expensive_work_refusal}`
- `ForgeQueryDeclarationEntryOrchestrationTranscript::{plan, outcome, step_records, automation_boundary, materialization_policy, cost_posture, orchestration_digest}`
- `ForgeQueryDeclarationEntryOrchestrationStepRecord::{stage, automation_step, disposition, materialization_tier, retained_digest, reason, is_reached, is_stop, is_terminal}`
- `ForgeQueryDeclarationEntryOrchestrationRefusal::{refusal_class, automation_refusal_class, stop_stage, reason, retained_digest, orchestration_identity_digest, automation_boundary}`
- `ForgeQueryDeclarationEntryOrchestrationAutomationBoundary::{EnvelopeCeiling}`
- `ForgeQueryDeclarationEntryOrchestrationAutomationStep::{AdmittedHandle, CanonicalDeclaration, Legality, Progression, FoundationalEvidence, RoutePlan, Receipt, Envelope}`
- `ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass::{ExplicitIntentRequired, ExpensiveAutomationForbidden, AuthorityTransitionRequired, PreparedButNotExecuted, UnsupportedAutomation, StrongerProofRequired}`
- `ForgeQueryDeclarationEntryOrchestrationMaterializationTier::{OperationalLean, SupportReady, FullDescriptive}`
- `ForgeQueryDeclarationEntryOrchestrationCostPosture::{OrdinaryDefault, ExplicitlyLean, ExplicitlyRich, PreparedButNotExecuted, ExpensiveByDefault}`
- `ForgeQueryDeclarationEntryOrchestrationMaterializationGate::{AdmittedByDefault, ExplicitRequestRequired, ForbiddenOnOrdinaryLane, PreparedOnly, UnsupportedForCurrentArtifactSet}`

Checked admission outcomes:

- `ForgeQueryConfiguredDomainHandleChecked::Admitted(ForgeQueryAdmittedConfiguredDomainHandle<D, C>)`
- `ForgeQueryConfiguredDomainHandleChecked::Deferred(ForgeQueryConfiguredDomainHandleDeferred<D, C>)`
- `ForgeQueryConfiguredDomainHandleChecked::Unsupported(ForgeQueryConfiguredDomainHandleUnsupported<D, C>)`
- `ForgeQueryConfiguredDomainHandleChecked::InvalidContext(ForgeQueryConfiguredDomainHandleInvalidContext<D, C>)`

Checked denial inspection:

- `blocking_capability_families() -> &[ForgeQueryCapabilityFamily]`
- `blocking_config_sections() -> &[ForgeQueryConfigSectionFamily]`
- `reason() -> &str`

## Core Mental Model

A configured domain handle is not a declaration and not a runtime binding.
It is the stable admitted world that later declaration work is allowed to
depend on.

That same admitted world is also the ownership boundary for declaration-entry
inventory, readiness, inspection, and declaration-entry orchestration.
Seam-ledger projections and orchestration both reject retained artifacts from
the wrong admitted handle or operating world.

The new typed binding pipeline follows the same rule. Binding does not replace
handle admission and it does not introduce ambient dependency injection.
Instead, it lets the admitted handle verify:

- current context candidates
- retained target identity
- admitted-world alignment
- aspect-fit and specificity posture

before it prepares the next explicit Query input.

Phase 26 adds the ordinary outcome layer on top of both binding and
declaration-entry orchestration. The admitted handle now exposes
`..._outcome(...)` entry points when you want one compact public result type
without dropping the checked topology underneath.

Phase 27 adds the continuation pipeline on top of binding and retained bridge
or signal truth. The admitted handle now owns one explicit prepared/executed
continuation lane instead of leaving callers to manually rebuild continuation
requests, basis posture, workspace posture, and execution handoff glue.

When later declaration-entry artifacts expose shared binding targets, those
targets are still scoped by this admitted world. The binding seam does not
replace handle admission; it carries retained artifact identity forward after
the admitted world already exists.

That means it should carry stable regime facts such as:

- policy or access class
- invariant regime
- assumption or tolerance regime
- collaborator or tenant-like operating class when it changes the admitted
  operating world

It should not carry:

- declaration-specific meaning
- per-operation trigger dependencies
- exact preview, historical, or runtime basis binding
- callback-shaped permission or invariant logic

Query validates and admits the handle structurally. It does not pretend to own
your downstream domain semantics.

## How It Works

1. define a downstream marker type with `ForgeQueryDomainEntryMarker`
2. define a downstream operating-context type with
   `ForgeQueryDomainOperatingContext`
3. enter Query through `domain(...)`, `domain_checked(...)`, or
   `domain_proof_root(...)`
4. bind the operating context with `with_operating_context(...)`
5. validate the configured handle
6. admit it against the current support matrix and validated Query config

Validation checks structural honesty:

- capability-family canonicalization
- config-section canonicalization
- capability-to-section coverage
- stable configured-handle identity

Admission checks current support posture:

- deferred capability families
- unsupported capability families
- disabled required config sections

## Small Example

```rust
use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomainEntry;

impl ForgeQueryDomainEntryMarker for GeometryDomainEntry {
    fn domain_key(&self) -> &'static str {
        "example.geometry"
    }

    fn display_name(&self) -> &'static str {
        "GeometryDomainEntry"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
        ]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryOperatingContext;

impl ForgeQueryDomainOperatingContext<GeometryDomainEntry> for GeometryOperatingContext {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::PreviewSession]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::RuntimeBridge,
        ]
    }

    fn context_identity_digest(&self) -> String {
        "access:collaborative|invariant:conservative|assumption:tight".to_string()
    }
}

let query = ForgeQueryApplicationFacade::runtime_backed_default();
let handle = query
    .domain(GeometryDomainEntry)
    .with_operating_context(GeometryOperatingContext)
    .validate()?
    .admit()?;
```

## Real Example

```rust
use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryConfiguredDomainHandleChecked, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AccessClass {
    CollaborativeEditor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum InvariantRegime {
    Conservative,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AssumptionRegime {
    TightTolerance,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomainEntry;

impl ForgeQueryDomainEntryMarker for GeometryDomainEntry {
    fn domain_key(&self) -> &'static str {
        "worth.geometry"
    }

    fn display_name(&self) -> &'static str {
        "GeometryDomainEntry"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
            ForgeQueryCapabilityFamily::IdentityEvolution,
        ]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryOperatingContext {
    access_class: AccessClass,
    invariant_regime: InvariantRegime,
    assumption_regime: AssumptionRegime,
}

impl GeometryOperatingContext {
    fn collaborative() -> Self {
        Self {
            access_class: AccessClass::CollaborativeEditor,
            invariant_regime: InvariantRegime::Conservative,
            assumption_regime: AssumptionRegime::TightTolerance,
        }
    }
}

impl ForgeQueryDomainOperatingContext<GeometryDomainEntry> for GeometryOperatingContext {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::PreviewSession,
            ForgeQueryCapabilityFamily::HistoricalEvaluation,
        ]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::RuntimeBridge,
            ForgeQueryConfigSectionFamily::Relational,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!(
            "access:{:?}|invariant:{:?}|assumption:{:?}",
            self.access_class, self.invariant_regime, self.assumption_regime
        )
    }
}

let query = ForgeQueryApplicationFacade::runtime_backed_default();

match query
    .domain_checked(GeometryDomainEntry)
    .with_operating_context(GeometryOperatingContext::collaborative())
{
    ForgeQueryConfiguredDomainHandleChecked::Admitted(handle) => {
        let _ = handle.operating_context_identity_digest();
        let _ = handle.handle_identity_digest();
        let _ = handle.required_capability_families();
    }
    ForgeQueryConfiguredDomainHandleChecked::Deferred(denial) => {
        let _ = denial.blocking_capability_families();
    }
    ForgeQueryConfiguredDomainHandleChecked::Unsupported(denial) => {
        let _ = denial.blocking_capability_families();
    }
    ForgeQueryConfiguredDomainHandleChecked::InvalidContext(denial) => {
        let _ = denial.blocking_config_sections();
    }
}
```

## Stable Operating Context Vs Dynamic Eligibility

This is the most important boundary to keep straight.

Stable operating context belongs in the configured handle:

- the general policy or access regime
- the general invariant regime
- the general assumption or tolerance regime
- other stable admitted-world posture

Dynamic eligibility belongs later:

- whether a specific operation may trigger now
- whether current truth satisfies a specific precondition
- whether a preview or historical basis makes one declaration legal
- whether a runtime dependency is available at this exact moment

If a value changes the stable admitted world, it belongs here.
If it changes the legality of one specific operation later, it does not.

That retained admitted-world identity is also what later legality,
progression, and foundational evidence surfaces consume. Those later features
should not call back into the operating-context object and rediscover world
identity on their own.

Once the handle is admitted, it is also the stable front door for the current
declaration-entry orchestration ceiling. You do not need to manually chain
declaration review, legality, progression, foundational description, route
planning, receipt, and envelope calls unless you specifically want one of those
intermediate artifacts directly.

That front door now comes with one locked orchestration artifact model:

- one Query-owned orchestration input
- one Query-owned orchestration plan
- one Query-owned orchestration outcome
- one proof-visible orchestration transcript

Ordinary, checked, and proof-visible entry points are visibility levels over
that same canonical lowering path. They are not competing helper pipelines.

Use `orchestrate_declaration_entry(...)` when the handle should own the current
declaration-entry lowering path through the envelope ceiling. Use the earlier
handle entry points when you specifically want to stop at legality,
progression, foundational evidence, route planning, receipt, or envelope
materialization yourself.

That orchestration trio also locks one sequencing boundary:

- Query starts after your tool or session has already assembled declaration
  intent
- Query automates only the declaration-entry sequence through the envelope
  ceiling
- Query does not treat handle admission as blanket authorization for later
  lower-authority transitions
- `Refused` means automation stopped intentionally; it does not erase stale,
  rebind-required, denied, deferred, or failed posture

It also locks one materialization boundary:

- visibility level and materialization policy are separate axes
- the default orchestration lane uses lean foundational evidence publication
  plus support-ready receipt and envelope publication
- checked and proof-visible lanes may inspect richer policy metadata without
  silently changing declaration-entry truth
- prepared publication still does not imply later execution

That trio remains the generic declaration-input front door. Phase 24 adds
public route/receipt/envelope orchestration from progressed declarations, but
those product-target methods still compile onto the same retained plan,
materialization policy, and stop-boundary law rather than introducing a second
pipeline.

If you need to inspect the locked public grammar itself rather than invoke one
of those methods, use `ForgeQueryDeclarationEntryOrchestrationVerbInventory`
instead of probing the handle surface by convention.

## Inspection And Debugging

The most useful inspection points are:

- `operating_context_identity_digest()`
- `handle_identity_digest()`
- `required_capability_families()`
- `required_config_sections()`
- `support_snapshot()`
- checked-lane denial posture

When a configured handle is denied:

- `Deferred` means the current build exposes the family but keeps it as debt
- `Unsupported` means the family is not available here
- `InvalidContext` means the operating context was structurally or
  configuration-wise incompatible with the current build

## Anti-Patterns

- passing raw collaborator IDs or tenant IDs as Query authority
- using bool shortcuts like `can_edit` or `preview`
- hiding access or invariant logic behind callbacks
- smuggling declaration-specific operation details into operating-context
  identity
- treating the configured handle as if it already proved dynamic eligibility

## Current Limits

Configured domain handles stop at stable admitted context.
They do not yet provide:

- declaration canonicalization
- declaration legality proof by themselves
- declaration progression proof by themselves
- foundational declaration evidence by themselves
- declaration route planning by themselves
- declaration boundary receipts by themselves
- declaration boundary envelopes by themselves
- declaration relational truth routing by themselves
- declaration bridge continuation routing by themselves
- declaration signal compatibility by themselves
- continuation preparation or explicit continuation execution by themselves
- declaration-entry orchestration beyond the retained envelope ceiling
- dynamic operation eligibility
- preview, historical, or runtime basis binding
- lower-authority routing
- lower-authority continuation routing

## Related Docs

- [Canonical Domain Declarations](./canonical-domain-declarations.md)
- [Typed Binding Pipeline](./typed-binding-pipeline.md)
- [Ordinary Outcomes](./ordinary-outcomes.md)
- [Continuation Pipeline](./continuation-pipeline.md)
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
- [Platform Entry](./platform-entry.md)
- [Domain Capabilities Index](./README.md)
- [Support Matrix And Admission](../foundations/support-matrix-and-admission.md)
