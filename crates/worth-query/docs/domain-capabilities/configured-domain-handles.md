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

Configured handles also own the current operating-world side of continuation
execution readmission. Most domains can keep the default behavior, but this is
also the place where a domain-specific operating context can report that the
current runtime basis or lower-authority evidence has drifted since a prepared
continuation was created.

## Why You Use It

- freeze the stable operating regime your domain is working inside
- make capability and config-section requirements explicit before declaration
  work begins
- get one canonical handle identity digest for the configured domain context
- fail early when the operating regime is deferred, unsupported, or invalid for
  the current Query build

## Stable Entry Points

- `WorthQueryDomainOperatingContext`
- `WorthQueryContinuationExecutionReadmissionObservation`
- `WorthQueryDomainEntryRoot::with_operating_context(...)`
- `WorthQueryDomainEntryProofRoot::with_operating_context(...)`
- `WorthQueryDomainEntryChecked::with_operating_context(...)`
- `WorthQueryConfiguredDomainHandleDraft`
- `WorthQueryValidatedConfiguredDomainHandle`
- `WorthQueryAdmittedConfiguredDomainHandle`
- `WorthQueryConfiguredDomainHandleChecked`

## API Reference

Operating-context contract:

- `required_capability_families() -> &'static [WorthQueryCapabilityFamily]`
- `required_config_sections() -> &'static [WorthQueryConfigSectionFamily]`
- `required_operating_requirements() -> &'static [WorthQueryDomainOperatingRequirement]`
- `context_identity_digest() -> String`
- `continuation_execution_readmission_observation(retained, support_snapshot) -> WorthQueryContinuationExecutionReadmissionObservation`

Configured-handle entry points:

- `with_operating_context(context) -> WorthQueryConfiguredDomainHandleDraft<D, C>`
- `validate() -> Result<WorthQueryValidatedConfiguredDomainHandle<D, C>, WorthQueryConfiguredDomainHandleInvalidContext<D, C>>`
- `admit() -> Result<WorthQueryAdmittedConfiguredDomainHandle<D, C>, WorthQueryConfiguredDomainHandleAdmissionError<D, C>>`

Validated and admitted handle inspection:

- `domain_key() -> &'static str`
- `display_name() -> &'static str`
- `operating_context() -> &C`
- `support_snapshot() -> &WorthQueryDomainEntrySupportSnapshot`
- `required_capability_families() -> &[WorthQueryCapabilityFamily]`
- `required_config_sections() -> &[WorthQueryConfigSectionFamily]`
- `required_operating_requirements() -> &[WorthQueryDomainOperatingRequirement]`
- `operating_context_identity_digest() -> &str`
- `handle_identity_digest() -> &str`
- `retained_world_basis() -> WorthQueryAdmittedWorldBasis`

Admitted-handle declaration evidence entry points:

- `describe_foundational(subject) -> Result<WorthQueryDeclarationFoundationalEvidence<D, I>, WorthQueryDeclarationFoundationalEvidenceDenial<D, I>>`
- `describe_foundational_checked(subject) -> WorthQueryDeclarationFoundationalEvidenceChecked<D, I>`
- `describe_foundational_with_profile(subject, profile) -> Result<WorthQueryDeclarationFoundationalEvidence<D, I>, WorthQueryDeclarationFoundationalEvidenceDenial<D, I>>`

Admitted-handle route-planning entry points:

- `plan_routes(subject) -> Result<WorthQueryDeclarationRoutePlan<D, I>, WorthQueryDeclarationRoutePlanTerminalError<D, I>>`
- `plan_routes_checked(subject) -> WorthQueryDeclarationRoutePlanChecked<D, I>`
- `plan_routes_from_progressed(progressed) -> Result<WorthQueryDeclarationRoutePlan<D, I>, WorthQueryDeclarationRoutePlanTerminalError<D, I>>`
- `plan_routes_from_progressed_with_intent(progressed, intent) -> Result<WorthQueryDeclarationRoutePlan<D, I>, WorthQueryDeclarationRoutePlanTerminalError<D, I>>`
- `declare_review_progress_describe_and_plan(input) -> Result<WorthQueryDeclarationRoutePlan<D, I>, WorthQueryDeclarationEntryRoutePlanError<D, I>>`

Admitted-handle receipt entry points:

- `receipt_routes(subject) -> Result<WorthQueryDeclarationReceipt<D, I>, WorthQueryDeclarationReceiptTerminalError<D, I>>`
- `receipt_routes_checked(subject) -> WorthQueryDeclarationReceiptChecked<D, I>`
- `receipt_routes_from_progressed(progressed) -> Result<WorthQueryDeclarationReceipt<D, I>, WorthQueryDeclarationReceiptTerminalError<D, I>>`
- `receipt_routes_from_progressed_with_intent(progressed, intent) -> Result<WorthQueryDeclarationReceipt<D, I>, WorthQueryDeclarationReceiptTerminalError<D, I>>`
- `declare_review_progress_describe_plan_and_receipt(input) -> Result<WorthQueryDeclarationReceipt<D, I>, WorthQueryDeclarationEntryReceiptError<D, I>>`

Admitted-handle envelope entry points:

- `envelope_routes(subject) -> Result<WorthQueryDeclarationEnvelope<D, I>, WorthQueryDeclarationEnvelopeTerminalError<D, I>>`
- `envelope_routes_checked(subject) -> WorthQueryDeclarationEnvelopeChecked<D, I>`
- `envelope_routes_from_progressed(progressed) -> Result<WorthQueryDeclarationEnvelope<D, I>, WorthQueryDeclarationEnvelopeTerminalError<D, I>>`
- `envelope_routes_from_progressed_with_intent(progressed, intent) -> Result<WorthQueryDeclarationEnvelope<D, I>, WorthQueryDeclarationEnvelopeTerminalError<D, I>>`
- `declare_review_progress_describe_plan_receipt_and_envelope(input) -> Result<WorthQueryDeclarationEnvelope<D, I>, WorthQueryDeclarationEntryEnvelopeError<D, I>>`

Admitted-handle relational-routing entry points:

- `route_relational_truth(subject) -> Result<WorthQueryDeclarationRelationalRouting<D, I>, WorthQueryDeclarationRelationalRoutingTerminalError<D, I>>`
- `route_relational_truth_checked(subject) -> WorthQueryDeclarationRelationalRoutingChecked<D, I>`
- `route_relational_truth_from_progressed(progressed) -> Result<WorthQueryDeclarationRelationalRouting<D, I>, WorthQueryDeclarationRelationalRoutingTerminalError<D, I>>`
- `route_relational_truth_from_progressed_with_intent(progressed, intent) -> Result<WorthQueryDeclarationRelationalRouting<D, I>, WorthQueryDeclarationRelationalRoutingTerminalError<D, I>>`
- `declare_review_progress_describe_plan_receipt_envelope_and_route_relational_truth(input) -> Result<WorthQueryDeclarationRelationalRouting<D, I>, WorthQueryDeclarationEntryRelationalRoutingError<D, I>>`
- `relational_truth_support::<I>() -> WorthQueryDeclarationRelationalRoutingSupportReport<D, I>`

Admitted-handle bridge-routing entry points:

- `route_bridge_continuation(subject) -> Result<WorthQueryDeclarationBridgeRouting<D, I>, WorthQueryDeclarationBridgeRoutingTerminalError<D, I>>`
- `route_bridge_continuation_checked(subject) -> WorthQueryDeclarationBridgeRoutingChecked<D, I>`
- `route_bridge_continuation_from_progressed(progressed) -> Result<WorthQueryDeclarationBridgeRouting<D, I>, WorthQueryDeclarationBridgeRoutingTerminalError<D, I>>`
- `route_bridge_continuation_from_progressed_with_intent(progressed, intent) -> Result<WorthQueryDeclarationBridgeRouting<D, I>, WorthQueryDeclarationBridgeRoutingTerminalError<D, I>>`
- `declare_review_progress_describe_plan_receipt_envelope_and_route_bridge_continuation(input) -> Result<WorthQueryDeclarationBridgeRouting<D, I>, WorthQueryDeclarationEntryBridgeRoutingError<D, I>>`
- `bridge_continuation_support::<I>() -> WorthQueryDeclarationBridgeRoutingSupportReport<D, I>`

Admitted-handle signal-compatibility entry points:

- `signal_compatibility(subject) -> Result<WorthQueryDeclarationSignalCompatibility<D, I>, WorthQueryDeclarationSignalCompatibilityTerminalError<D, I>>`
- `signal_compatibility_checked(subject) -> WorthQueryDeclarationSignalCompatibilityChecked<D, I>`
- `signal_compatibility_from_progressed(progressed) -> Result<WorthQueryDeclarationSignalCompatibility<D, I>, WorthQueryDeclarationSignalCompatibilityTerminalError<D, I>>`
- `signal_compatibility_from_progressed_with_intent(progressed, intent) -> Result<WorthQueryDeclarationSignalCompatibility<D, I>, WorthQueryDeclarationSignalCompatibilityTerminalError<D, I>>`
- `declare_review_progress_describe_plan_receipt_envelope_and_check_signal_compatibility(input) -> Result<WorthQueryDeclarationSignalCompatibility<D, I>, WorthQueryDeclarationEntrySignalCompatibilityError<D, I>>`
- `signal_compatibility_support::<I>() -> WorthQueryDeclarationSignalCompatibilitySupportReport<D, I>`

Admitted-handle signal-compatibility orchestration entry points:

- `orchestrate_signal_compatibility(input) -> WorthQuerySignalCompatibilityOrchestrationOutcome<D, I>`
- `orchestrate_signal_compatibility_outcome(input) -> WorthQueryOrdinaryOutcome<WorthQuerySignalCompatibilityOrchestration<D, I>>`
- `orchestrate_signal_compatibility_checked(input) -> WorthQuerySignalCompatibilityOrchestrationChecked<D, I>`
- `orchestrate_signal_compatibility_proof(input) -> WorthQuerySignalCompatibilityOrchestrationTranscript<D, I>`

Admitted-handle contribution-composed orchestration entry points:

- `orchestrate_declaration_with_contributions(input) -> Result<WorthQueryContributionComposedOrchestration<D, I>, WorthQueryContributionComposedOrchestrationOutcome<D, I>>`
- `orchestrate_declaration_with_contributions_outcome(input) -> WorthQueryOrdinaryOutcome<WorthQueryContributionComposedOrchestration<D, I>>`
- `orchestrate_declaration_with_contributions_checked(input) -> WorthQueryContributionComposedOrchestrationChecked<D, I>`
- `orchestrate_declaration_with_contributions_proof(input) -> WorthQueryContributionComposedOrchestrationTranscript<D, I>`

Admitted-handle continuation entry points:

- `prepare_continuation_from_target(request) -> WorthQueryPreparedContinuationOutcome<D, I>`
- `prepare_continuation_from_target_outcome(request) -> WorthQueryOrdinaryOutcome<WorthQueryPreparedContinuation<D, I>>`
- `prepare_continuation_from_target_checked(request) -> WorthQueryPreparedContinuationChecked<D, I>`
- `prepare_continuation_from_target_proof(request) -> WorthQueryPreparedContinuationTranscript<D, I>`
- `prepare_continuation_from_context(request) -> WorthQueryPreparedContinuationOutcome<D, I>`
- `prepare_continuation_from_context_outcome(request) -> WorthQueryOrdinaryOutcome<WorthQueryPreparedContinuation<D, I>>`
- `prepare_continuation_from_context_checked(request) -> WorthQueryPreparedContinuationChecked<D, I>`
- `prepare_continuation_from_context_proof(request) -> WorthQueryPreparedContinuationTranscript<D, I>`
- `execute_prepared_continuation(prepared) -> WorthQueryContinuationExecutionOutcome<D, I>`
- `execute_prepared_continuation_outcome(prepared) -> WorthQueryOrdinaryOutcome<WorthQueryContinuationExecution<D, I>>`
- `execute_prepared_continuation_checked(prepared) -> WorthQueryContinuationExecutionChecked<D, I>`
- `execute_prepared_continuation_proof(prepared) -> WorthQueryContinuationExecutionTranscript<D, I>`

Admitted-handle recovery entry points:

- `recover_from_outcome(outcome) -> Option<WorthQueryRecoveryBrief>`
- `recover_from_declaration_entry_checked(checked) -> Option<WorthQueryRecoveryBrief>`
- `recover_from_declaration_entry_proof(proof) -> Option<WorthQueryRecoveryBrief>`
- `recover_from_declaration_route_plan_checked(checked) -> Option<WorthQueryRecoveryBrief>`
- `recover_from_declaration_receipt_checked(checked) -> Option<WorthQueryRecoveryBrief>`
- `recover_from_prepared_continuation_checked(checked) -> Option<WorthQueryRecoveryBrief>`
- `recover_from_prepared_continuation_proof(proof) -> Option<WorthQueryRecoveryBrief>`
- `recover_from_continuation_execution_checked(checked) -> Option<WorthQueryRecoveryBrief>`
- `recover_from_continuation_execution_proof(proof) -> Option<WorthQueryRecoveryBrief>`
- `recover_from_signal_compatibility_checked(checked) -> Option<WorthQueryRecoveryBrief>`
- `recover_from_signal_compatibility_proof(proof) -> Option<WorthQueryRecoveryBrief>`
- `recover_from_contribution_composed_checked(checked) -> Option<WorthQueryRecoveryBrief>`
- `recover_from_contribution_composed_proof(proof) -> Option<WorthQueryRecoveryBrief>`
- `recover_from_grouped_orchestration_checked(checked) -> Option<WorthQueryRecoveryBrief>`
- `recover_from_grouped_orchestration_proof(proof) -> Option<WorthQueryRecoveryBrief>`

Admitted-handle family-helper entry points:

- `family_helpers() -> WorthQueryFamilyHelpers<'_, D, C>`
- `geometry_helpers() -> WorthQueryGeometryFamilyHelpers<'_, D, C>`
- `progress_active_face_selection(input) -> Result<WorthQueryAdmittedDeclarationProgression<D, I>, WorthQueryDeclarationEntryProgressionError<D, I>>`
- `prepare_preview_for_active_face_selection(progressed) -> WorthQuerySignalCompatibilityOrchestrationOutcome<D, I>`
- `prepare_preview_for_active_face_selection_outcome(progressed) -> WorthQueryOrdinaryOutcome<WorthQuerySignalCompatibilityOrchestration<D, I>>`
- `prepare_preview_for_active_face_selection_checked(progressed) -> WorthQuerySignalCompatibilityOrchestrationChecked<D, I>`
- `prepare_preview_for_active_face_selection_proof(progressed) -> WorthQuerySignalCompatibilityOrchestrationTranscript<D, I>`
- `prepare_runtime_route_for_active_face_selection(progressed) -> WorthQuerySignalCompatibilityOrchestrationOutcome<D, I>`
- `prepare_runtime_route_for_active_face_selection_outcome(progressed) -> WorthQueryOrdinaryOutcome<WorthQuerySignalCompatibilityOrchestration<D, I>>`
- `prepare_runtime_route_for_active_face_selection_checked(progressed) -> WorthQuerySignalCompatibilityOrchestrationChecked<D, I>`
- `prepare_runtime_route_for_active_face_selection_proof(progressed) -> WorthQuerySignalCompatibilityOrchestrationTranscript<D, I>`
- `prepare_current_truth_view_for_active_face_selection(progressed) -> WorthQuerySignalCompatibilityOrchestrationOutcome<D, I>`
- `prepare_current_truth_view_for_active_face_selection_outcome(progressed) -> WorthQueryOrdinaryOutcome<WorthQuerySignalCompatibilityOrchestration<D, I>>`
- `prepare_current_truth_view_for_active_face_selection_checked(progressed) -> WorthQuerySignalCompatibilityOrchestrationChecked<D, I>`
- `prepare_current_truth_view_for_active_face_selection_proof(progressed) -> WorthQuerySignalCompatibilityOrchestrationTranscript<D, I>`
- `prepare_historical_truth_view_for_active_face_selection(progressed) -> WorthQuerySignalCompatibilityOrchestrationOutcome<D, I>`
- `prepare_historical_truth_view_for_active_face_selection_outcome(progressed) -> WorthQueryOrdinaryOutcome<WorthQuerySignalCompatibilityOrchestration<D, I>>`
- `prepare_historical_truth_view_for_active_face_selection_checked(progressed) -> WorthQuerySignalCompatibilityOrchestrationChecked<D, I>`
- `prepare_historical_truth_view_for_active_face_selection_proof(progressed) -> WorthQuerySignalCompatibilityOrchestrationTranscript<D, I>`
- `orchestrate_material_attachment_for_active_face_selection(input) -> Result<WorthQueryContributionComposedOrchestration<D, I>, WorthQueryContributionComposedOrchestrationOutcome<D, I>>`
- `orchestrate_material_attachment_for_active_face_selection_outcome(input) -> WorthQueryOrdinaryOutcome<WorthQueryContributionComposedOrchestration<D, I>>`
- `orchestrate_material_attachment_for_active_face_selection_checked(input) -> WorthQueryContributionComposedOrchestrationChecked<D, I>`
- `orchestrate_material_attachment_for_active_face_selection_proof(input) -> WorthQueryContributionComposedOrchestrationTranscript<D, I>`
- `local_neighborhood_for_active_face_selection(input) -> WorthQueryGroupedDeclarationInput<D, I>`
- `declare_local_neighborhood_for_active_face_selection(input) -> Result<WorthQueryGroupedDeclarationArtifact<D, I>, WorthQueryGroupedDeclarationStop>`
- `declare_local_neighborhood_for_active_face_selection_checked(input) -> WorthQueryGroupedDeclarationChecked<D, I>`
- `orchestrate_local_neighborhood_for_active_face_selection(declaration) -> Result<WorthQueryGroupedOrchestration<D, I>, WorthQueryGroupedOrchestrationStop<D, I>>`
- `orchestrate_local_neighborhood_for_active_face_selection_outcome(declaration) -> WorthQueryOrdinaryOutcome<WorthQueryGroupedOrchestration<D, I>>`
- `orchestrate_local_neighborhood_for_active_face_selection_checked(declaration) -> WorthQueryGroupedOrchestrationChecked<D, I>`
- `orchestrate_local_neighborhood_for_active_face_selection_proof(declaration) -> WorthQueryGroupedOrchestrationTranscript<D, I>`

Use [Family Helpers](./family-helpers.md) for the mental model, examples, and
family-gating rules behind these helper verbs. This page keeps the configured
handle inventory; the helper page teaches when to reach for the helper surface
instead of the generic orchestration lanes.

Admitted-handle seam-ledger entry points:

- `declaration_entry_crossing_inventory::<I>() -> WorthQueryDeclarationEntryCrossingInventory<D, I>`
- `declaration_entry_readiness::<I>() -> WorthQueryDeclarationEntryReadinessReport<D, I>`
- `inspect_declaration_entry(subject) -> Result<WorthQueryDeclarationEntryInspection<D, I>, WorthQueryDeclarationEntryInspectionError<D, I>>`

Admitted-handle orchestration entry points:

- `orchestrate_declaration_entry(input) -> Result<WorthQueryDeclarationEnvelope<D, I>, WorthQueryDeclarationEntryOrchestrationTerminalError<D, I>>`
- `orchestrate_declaration_entry_outcome(input) -> WorthQueryOrdinaryOutcome<WorthQueryDeclarationEnvelope<D, I>>`
- `orchestrate_declaration_entry_checked(input) -> WorthQueryDeclarationEntryOrchestrationOutcome<D, I>`
- `orchestrate_declaration_entry_proof(input) -> WorthQueryDeclarationEntryOrchestrationTranscript<D, I>`
- `orchestrate_routes_from_progressed(progressed) -> Result<WorthQueryDeclarationRoutePlan<D, I>, WorthQueryDeclarationRoutePlanTerminalError<D, I>>`
- `orchestrate_routes_from_progressed_with_intent(progressed, intent) -> Result<WorthQueryDeclarationRoutePlan<D, I>, WorthQueryDeclarationRoutePlanTerminalError<D, I>>`
- `orchestrate_receipt_from_progressed(progressed) -> Result<WorthQueryDeclarationReceipt<D, I>, WorthQueryDeclarationReceiptTerminalError<D, I>>`
- `orchestrate_receipt_from_progressed_with_intent(progressed, intent) -> Result<WorthQueryDeclarationReceipt<D, I>, WorthQueryDeclarationReceiptTerminalError<D, I>>`
- `orchestrate_envelope_from_progressed(progressed) -> Result<WorthQueryDeclarationEnvelope<D, I>, WorthQueryDeclarationEnvelopeTerminalError<D, I>>`
- `orchestrate_envelope_from_progressed_with_intent(progressed, intent) -> Result<WorthQueryDeclarationEnvelope<D, I>, WorthQueryDeclarationEnvelopeTerminalError<D, I>>`

Admitted-handle typed binding entry points:

- `bind_declaration_from_context(request) -> WorthQueryBindingOutcome<WorthQueryCanonicalDeclarationArtifact<D, I>>`
- `bind_declaration_from_context_outcome(request) -> WorthQueryOrdinaryOutcome<WorthQueryCanonicalDeclarationArtifact<D, I>>`
- `bind_declaration_from_context_checked(request) -> WorthQueryBindingChecked<WorthQueryCanonicalDeclarationArtifact<D, I>>`
- `bind_declaration_from_context_proof(request) -> WorthQueryBindingTranscript<WorthQueryCanonicalDeclarationArtifact<D, I>>`
- `bind_route_request_from_context(request) -> WorthQueryBindingOutcome<WorthQueryDeclarationRoutePlanInput<D, I>>`
- `bind_route_request_from_context_outcome(request) -> WorthQueryOrdinaryOutcome<WorthQueryDeclarationRoutePlanInput<D, I>>`
- `bind_route_request_from_context_checked(request) -> WorthQueryBindingChecked<WorthQueryDeclarationRoutePlanInput<D, I>>`
- `bind_route_request_from_context_proof(request) -> WorthQueryBindingTranscript<WorthQueryDeclarationRoutePlanInput<D, I>>`
- `bind_receipt_request_from_context(request) -> WorthQueryBindingOutcome<WorthQueryDeclarationReceiptInput<D, I>>`
- `bind_receipt_request_from_context_outcome(request) -> WorthQueryOrdinaryOutcome<WorthQueryDeclarationReceiptInput<D, I>>`
- `bind_receipt_request_from_context_checked(request) -> WorthQueryBindingChecked<WorthQueryDeclarationReceiptInput<D, I>>`
- `bind_receipt_request_from_context_proof(request) -> WorthQueryBindingTranscript<WorthQueryDeclarationReceiptInput<D, I>>`
- `bind_envelope_request_from_context(request) -> WorthQueryBindingOutcome<WorthQueryDeclarationEnvelopeInput<D, I>>`
- `bind_envelope_request_from_context_outcome(request) -> WorthQueryOrdinaryOutcome<WorthQueryDeclarationEnvelopeInput<D, I>>`
- `bind_envelope_request_from_context_checked(request) -> WorthQueryBindingChecked<WorthQueryDeclarationEnvelopeInput<D, I>>`
- `bind_envelope_request_from_context_proof(request) -> WorthQueryBindingTranscript<WorthQueryDeclarationEnvelopeInput<D, I>>`
- `bind_continuation_request_from_context(request) -> WorthQueryBindingOutcome<WorthQueryContinuationBindingInput<D, I>>`
- `bind_continuation_request_from_context_outcome(request) -> WorthQueryOrdinaryOutcome<WorthQueryContinuationBindingInput<D, I>>`
- `bind_continuation_request_from_context_checked(request) -> WorthQueryBindingChecked<WorthQueryContinuationBindingInput<D, I>>`
- `bind_continuation_request_from_context_proof(request) -> WorthQueryBindingTranscript<WorthQueryContinuationBindingInput<D, I>>`
- `bind_route_from_target(request) -> WorthQueryBindingOutcome<WorthQueryDeclarationRoutePlanInput<D, I>>`
- `bind_route_from_target_outcome(request) -> WorthQueryOrdinaryOutcome<WorthQueryDeclarationRoutePlanInput<D, I>>`
- `bind_route_from_target_checked(request) -> WorthQueryBindingChecked<WorthQueryDeclarationRoutePlanInput<D, I>>`
- `bind_route_from_target_proof(request) -> WorthQueryBindingTranscript<WorthQueryDeclarationRoutePlanInput<D, I>>`
- `bind_receipt_from_target(request) -> WorthQueryBindingOutcome<WorthQueryDeclarationReceiptInput<D, I>>`
- `bind_receipt_from_target_outcome(request) -> WorthQueryOrdinaryOutcome<WorthQueryDeclarationReceiptInput<D, I>>`
- `bind_receipt_from_target_checked(request) -> WorthQueryBindingChecked<WorthQueryDeclarationReceiptInput<D, I>>`
- `bind_receipt_from_target_proof(request) -> WorthQueryBindingTranscript<WorthQueryDeclarationReceiptInput<D, I>>`
- `bind_envelope_from_target(request) -> WorthQueryBindingOutcome<WorthQueryDeclarationEnvelopeInput<D, I>>`
- `bind_envelope_from_target_outcome(request) -> WorthQueryOrdinaryOutcome<WorthQueryDeclarationEnvelopeInput<D, I>>`
- `bind_envelope_from_target_checked(request) -> WorthQueryBindingChecked<WorthQueryDeclarationEnvelopeInput<D, I>>`
- `bind_envelope_from_target_proof(request) -> WorthQueryBindingTranscript<WorthQueryDeclarationEnvelopeInput<D, I>>`
- `bind_continuation_from_target(request) -> WorthQueryBindingOutcome<WorthQueryContinuationBindingInput<D, I>>`
- `bind_continuation_from_target_outcome(request) -> WorthQueryOrdinaryOutcome<WorthQueryContinuationBindingInput<D, I>>`
- `bind_continuation_from_target_checked(request) -> WorthQueryBindingChecked<WorthQueryContinuationBindingInput<D, I>>`
- `bind_continuation_from_target_proof(request) -> WorthQueryBindingTranscript<WorthQueryContinuationBindingInput<D, I>>`

Handle-independent orchestration grammar inventory:

- `WorthQueryDeclarationEntryOrchestrationVerbInventory::current()`
- `WorthQueryDeclarationEntryOrchestrationVerbInventory::verbs()`
- `WorthQueryDeclarationEntryOrchestrationVerb::{public_name, family, exposure_level, ceiling, canonical_base_name}`

Orchestration artifact inspection:

- `WorthQueryDeclarationEntryOrchestrationInput::{declaration_family_key, handle_identity_digest, operating_context_identity_digest, exposure_level, artifact_policy}`
- `WorthQueryDeclarationEntryOrchestrationPlan::{declaration_family_key, handle_identity_digest, operating_context_identity_digest, exposure_level, artifact_policy, ceiling_stage, automation_boundary, automation_steps, explicit_caller_handoff_steps, step_plan, orchestration_identity_digest}`
- `WorthQueryDeclarationEntryOrchestrationPlan::{materialization_policy, materialization_tier, cost_posture, materialization_gate, foundational_evidence_profile, descriptive_materialization_cost}`
- `WorthQueryDeclarationEntryOrchestrationOutcome::{stop_stage, declaration_family_key, retained_digest, outcome_identity_digest, is_automation_refusal, is_expensive_work_refusal}`
- `WorthQueryDeclarationEntryOrchestrationTranscript::{plan, outcome, step_records, automation_boundary, materialization_policy, cost_posture, orchestration_digest}`
- `WorthQueryDeclarationEntryOrchestrationStepRecord::{stage, automation_step, disposition, materialization_tier, retained_digest, reason, is_reached, is_stop, is_terminal}`
- `WorthQueryDeclarationEntryOrchestrationRefusal::{refusal_class, automation_refusal_class, stop_stage, reason, retained_digest, orchestration_identity_digest, automation_boundary}`
- `WorthQueryDeclarationEntryOrchestrationAutomationBoundary::{EnvelopeCeiling}`
- `WorthQueryDeclarationEntryOrchestrationAutomationStep::{AdmittedHandle, CanonicalDeclaration, Legality, Progression, FoundationalEvidence, RoutePlan, Receipt, Envelope}`
- `WorthQueryDeclarationEntryOrchestrationAutomationRefusalClass::{ExplicitIntentRequired, ExpensiveAutomationForbidden, AuthorityTransitionRequired, PreparedButNotExecuted, UnsupportedAutomation, StrongerProofRequired}`
- `WorthQueryDeclarationEntryOrchestrationMaterializationTier::{OperationalLean, SupportReady, FullDescriptive}`
- `WorthQueryDeclarationEntryOrchestrationCostPosture::{OrdinaryDefault, ExplicitlyLean, ExplicitlyRich, PreparedButNotExecuted, ExpensiveByDefault}`
- `WorthQueryDeclarationEntryOrchestrationMaterializationGate::{AdmittedByDefault, ExplicitRequestRequired, ForbiddenOnOrdinaryLane, PreparedOnly, UnsupportedForCurrentArtifactSet}`

Checked admission outcomes:

- `WorthQueryConfiguredDomainHandleChecked::Admitted(WorthQueryAdmittedConfiguredDomainHandle<D, C>)`
- `WorthQueryConfiguredDomainHandleChecked::Deferred(WorthQueryConfiguredDomainHandleDeferred<D, C>)`
- `WorthQueryConfiguredDomainHandleChecked::Unsupported(WorthQueryConfiguredDomainHandleUnsupported<D, C>)`
- `WorthQueryConfiguredDomainHandleChecked::InvalidContext(WorthQueryConfiguredDomainHandleInvalidContext<D, C>)`

Checked denial inspection:

- `blocking_capability_families() -> &[WorthQueryCapabilityFamily]`
- `blocking_config_sections() -> &[WorthQueryConfigSectionFamily]`
- `blocking_operating_requirements() -> &[WorthQueryDomainOperatingRequirement]`
- `reason() -> &str`

## Core Mental Model

A configured domain handle is not a declaration and not a runtime binding.
It is the stable admitted world that later declaration work is allowed to
depend on.

That same admitted world is also the ownership boundary for declaration-entry
inventory, readiness, inspection, and declaration-entry orchestration.
Seam-ledger projections and orchestration both reject retained artifacts from
the wrong admitted handle or operating world.

Graph touch obligation authority uses the same admitted-world boundary.
Consumers should pass graph touches through Query's operating world descriptor
and registered obligation index instead of deriving local validator tables from
a configured handle. The handle proves the world; it does not make local graph
ceremony authoritative.

The new typed binding pipeline follows the same rule. Binding does not replace
handle admission and it does not introduce ambient dependency injection.
Instead, it lets the admitted handle verify:

- current context candidates
- retained target identity
- admitted-world alignment
- aspect-fit and specificity posture

before it prepares the next explicit Query input.

The admitted handle also exposes the ordinary outcome layer on top of both
binding and declaration-entry orchestration:
`..._outcome(...)` entry points when you want one compact public result type
without dropping the checked topology underneath.

When downstream authority code needs one retained admitted-world witness without
carrying the full admitted handle, call `retained_world_basis()`. It returns
`WorthQueryAdmittedWorldBasis`, a Query-owned artifact with read-only accessors
for:

- `domain_key()`
- `display_name()`
- `operating_context_identity_digest()`
- `handle_identity_digest()`
- `support_snapshot_digest()`
- `basis_lifecycle_support_digest()`

That retained world basis is public to read, compare, and store, but Query owns
construction. Downstream crates can bind to admitted-world truth without
fabricating the witness from raw strings. It also carries the basis-lifecycle
support witness that later preview, historical, replay, temporal, or async
basis-sensitive work should consume instead of reopening raw branch, snapshot,
preview, or source identifiers locally.

The admitted handle also owns one explicit prepared/executed continuation lane
on top of binding and retained bridge or signal truth:
continuation lane instead of leaving callers to manually rebuild continuation
requests, basis posture, workspace posture, and execution handoff glue.

The admitted handle also exposes one signal-facing composition lane on top of
retained signal compatibility and optional continuation preparation:
owns an `orchestrate_signal_compatibility(...)` family when your app wants to
ask "do we stop at signal compatibility, or can Query also prepare the next
continuation step right now?" without flattening compatibility, preparation,
and execution into one result. That orchestration family starts from retained
signal input only. If you begin from progression, the supported path is to
lower through envelope-backed truth first and then enter signal orchestration.

The admitted handle also exposes one declaration-plus-contribution composition
lane on top of the retained declaration-entry path and the standalone contribution-authoring
surfaces. The admitted handle now owns an
`orchestrate_declaration_with_contributions(...)` family when your app wants
one public call that can:

- lower declaration entry through the envelope ceiling
- bind declaration-scoped contribution intents
- preserve declaration-side and contribution-side non-success posture
- optionally materialize contribution summaries

The admitted handle also owns one recovery lane over the ordinary, checked,
and proof-visible stop surfaces:
`recover_from_...(...)` entry points when your app wants one typed answer to
"what stopped, who owns the fix, and what is the next supported repair step?"
without flattening declaration, continuation, signal, or contribution posture
into one generic retry.

The admitted handle also owns `family_helpers()` and one family-helper
projection lane over those same public surfaces:
`geometry_helpers()` when your app already knows the declaration family it is
working with and wants domain-native helper calls that still compile onto the
shared generic Query paths. The continuation-style helper verbs still follow
the same boundary rule: they accept progressed family input ergonomically, then
lower through checked envelope truth before they call the generic
signal-orchestration surface.

When later declaration-entry artifacts expose shared binding targets, those
targets are still scoped by this admitted world. The binding seam does not
replace handle admission; it carries retained artifact identity forward after
the admitted world already exists.

That means it should carry stable regime facts such as:

- policy or access class
- invariant regime
- assumption or tolerance regime
- future runtime posture that changes what this handle is allowed to ask Query
  to do
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

1. define a downstream marker type with `WorthQueryDomainEntryMarker`
2. define a downstream operating-context type with
   `WorthQueryDomainOperatingContext`
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
- support-gated, deferred, or unsupported operating requirements such as
  temporal or async-resource query posture

## Small Example

```rust
use worth_query::facade::foundation::{
    WorthQueryApplicationFacade, WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily,
    WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext,
    WorthQueryDomainOperatingRequirement,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomainEntry;

impl WorthQueryDomainEntryMarker for GeometryDomainEntry {
    fn domain_key(&self) -> &'static str {
        "example.geometry"
    }

    fn display_name(&self) -> &'static str {
        "GeometryDomainEntry"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[
            WorthQueryCapabilityFamily::QueryComposition,
            WorthQueryCapabilityFamily::QueryContext,
        ]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryOperatingContext;

impl WorthQueryDomainOperatingContext<GeometryDomainEntry> for GeometryOperatingContext {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::PreviewSession]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[
            WorthQueryConfigSectionFamily::Query,
            WorthQueryConfigSectionFamily::RuntimeBridge,
        ]
    }

    fn required_operating_requirements(&self) -> &'static [WorthQueryDomainOperatingRequirement] {
        &[]
    }

    fn context_identity_digest(&self) -> String {
        "access:collaborative|invariant:conservative|assumption:tight".to_string()
    }
}

let query = WorthQueryApplicationFacade::runtime_backed_default();
let handle = query
    .domain(GeometryDomainEntry)
    .with_operating_context(GeometryOperatingContext)
    .validate()?
    .admit()?;
```

## Real Example

```rust
use worth_query::facade::foundation::{
    WorthQueryApplicationFacade, WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily,
    WorthQueryConfiguredDomainHandleChecked, WorthQueryDomainEntryMarker,
    WorthQueryDomainOperatingContext, WorthQueryDomainOperatingRequirement,
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

impl WorthQueryDomainEntryMarker for GeometryDomainEntry {
    fn domain_key(&self) -> &'static str {
        "worth.geometry"
    }

    fn display_name(&self) -> &'static str {
        "GeometryDomainEntry"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[
            WorthQueryCapabilityFamily::QueryComposition,
            WorthQueryCapabilityFamily::QueryContext,
            WorthQueryCapabilityFamily::IdentityEvolution,
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

    fn temporal_editor() -> Self {
        Self::collaborative()
    }
}

impl WorthQueryDomainOperatingContext<GeometryDomainEntry> for GeometryOperatingContext {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[
            WorthQueryCapabilityFamily::PreviewSession,
            WorthQueryCapabilityFamily::HistoricalEvaluation,
        ]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[
            WorthQueryConfigSectionFamily::Query,
            WorthQueryConfigSectionFamily::RuntimeBridge,
            WorthQueryConfigSectionFamily::Relational,
        ]
    }

    fn required_operating_requirements(&self) -> &'static [WorthQueryDomainOperatingRequirement] {
        &[WorthQueryDomainOperatingRequirement::TemporalQuery]
    }

    fn context_identity_digest(&self) -> String {
        format!(
            "access:{:?}|invariant:{:?}|assumption:{:?}",
            self.access_class, self.invariant_regime, self.assumption_regime
        )
    }
}

let query = WorthQueryApplicationFacade::runtime_backed_default();

match query
    .domain_checked(GeometryDomainEntry)
    .with_operating_context(GeometryOperatingContext::temporal_editor())
{
    WorthQueryConfiguredDomainHandleChecked::Admitted(handle) => {
        let _ = handle.operating_context_identity_digest();
        let _ = handle.handle_identity_digest();
        let _ = handle.required_capability_families();
        let _ = handle.required_operating_requirements();
    }
    WorthQueryConfiguredDomainHandleChecked::Deferred(denial) => {
        let _ = denial.blocking_capability_families();
        let _ = denial.blocking_operating_requirements();
    }
    WorthQueryConfiguredDomainHandleChecked::Unsupported(denial) => {
        let _ = denial.blocking_capability_families();
        let _ = denial.blocking_operating_requirements();
    }
    WorthQueryConfiguredDomainHandleChecked::InvalidContext(denial) => {
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
- any future runtime family requirement that should deny before declaration
  authoring starts

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

Use `orchestrate_declaration_with_contributions(...)` when that same
declaration-entry run should also carry declaration-scoped contribution
authoring in the same admitted-handle call. It still uses the retained
declaration-entry path underneath, but it keeps contribution denial and
contribution summary materialization typed instead of forcing the app to stitch
declaration entry and contribution authoring together manually.

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

That trio remains the generic declaration-input front door. Query also exposes
public route/receipt/envelope orchestration from progressed declarations, but
those product-target methods still compile onto the same retained plan,
materialization policy, and stop-boundary law rather than introducing a second
pipeline.

If you need to inspect the locked public grammar itself rather than invoke one
of those methods, use `WorthQueryDeclarationEntryOrchestrationVerbInventory`
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

If your context asks for future runtime posture with
`required_operating_requirements()`, inspect
`blocking_operating_requirements()` first. That is the explicit list of
temporal, async-resource, or mixed-cause requirements that the current build
cannot admit yet.

## Anti-Patterns

- passing raw collaborator IDs or tenant IDs as Query authority
- using bool shortcuts like `can_edit` or `preview`
- hiding access or invariant logic behind callbacks
- treating temporal or async posture as ambient session state instead of part
  of the configured handle
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

- [Graph Touch Obligation Authority](../authoring/graph-touch-obligation-authority.md)
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
- [Signal Compatibility Orchestration](./signal-compatibility-orchestration.md)
- [Contribution-Composed Orchestration](./contribution-composed-orchestration.md)
- [Recovery Boundary](./recovery-boundary.md)
- [Family Helpers](./family-helpers.md)
- [Grouped Authoring](./grouped-authoring.md)
- [Declaration Entry Orchestration](./declaration-entry-orchestration.md)
- [Declaration Entry Inspection](./declaration-entry-inspection.md)
- [Declaration Entry Readiness](./declaration-entry-readiness.md)
- [Platform Entry](./platform-entry.md)
- [Domain Capabilities Index](./README.md)
- [Support Matrix And Admission](../foundations/support-matrix-and-admission.md)
