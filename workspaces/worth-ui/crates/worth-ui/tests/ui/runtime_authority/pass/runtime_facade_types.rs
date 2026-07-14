use worth_ui::facade::{
    WorthUi, WorthUiAccessibilityImpact, WorthUiActiveReplacementBasis,
    WorthUiActiveRuntimeObservation, WorthUiActivationGateCounters, WorthUiActivationGateDenial,
    WorthUiActivationGateDenialReason, WorthUiActivationGateReceipt, WorthUiActivationReadiness,
    WorthUiActivationStagingCounters, WorthUiActivationStagingDenial,
    WorthUiActivationStagingDenialReason, WorthUiActivationStagingReport,
    WorthUiAdmittedReplacementCandidate, WorthUiAmbiguousReplacementDenial,
    WorthUiCandidateAdmission, WorthUiCandidateAdmissionCounters, WorthUiCandidateAdmissionDenial,
    WorthUiCandidateAdmissionReport, WorthUiCandidateArtifactBundle,
    WorthUiCandidateAuthoringLane, WorthUiCandidateDependencyMetadata,
    WorthUiCandidateLoweringBasis, WorthUiCandidateProvenanceHandle, WorthUiChildRangeHandle,
    WorthUiCommandHandle, WorthUiCommandImpact, WorthUiComponentHandle,
    WorthUiComponentLoweringHook, WorthUiComponentLoweringHookFamily,
    WorthUiDurableStateCarryForward, WorthUiDurableStateEligibility, WorthUiDurableStateFamily,
    WorthUiDurableStateFamilyHook, WorthUiDurableStateFamilyId,
    WorthUiDurableStateImpactReceipts, WorthUiDurableStateInventory,
    WorthUiDurableStateInventoryBuilder, WorthUiDurableStateInventoryCounters,
    WorthUiDurableStateInventoryDenial, WorthUiDurableStateReconciliationCounters,
    WorthUiDurableStateReconciliationDenial, WorthUiDurableStateReconciliationOutcome,
    WorthUiDurableStateReconciliationPlan, WorthUiDurableStateReconciliationReceipt,
    WorthUiDurableStateReplacement, WorthUiDurableStateReplacementPolicy,
    WorthUiEguiBoundaryContact, WorthUiEguiBoundaryInput, WorthUiEguiPlanBoundary,
    WorthUiExecutionLane, WorthUiExecutionLaneDescriptor, WorthUiExecutionLaneSupport,
    WorthUiExecutionPlan, WorthUiExecutionPlanDigest, WorthUiExecutionPlanEquivalence,
    WorthUiExecutionPlanEquivalenceBasis, WorthUiExecutionPlanEquivalenceCounters,
    WorthUiExecutionPlanInput, WorthUiExtensionHookAdmission, WorthUiFocusChainReconciliation,
    WorthUiFrameBoundary, WorthUiFrameBoundaryPosture, WorthUiHandlePlanGeneration,
    WorthUiIdentityMatchCounters, WorthUiIdentityMatchDenial, WorthUiIdentityMatchEdge,
    WorthUiIdentityMatchGraph, WorthUiIdentityMatchNode, WorthUiIdentityMatchNodeKind,
    WorthUiIdentityMatchNodeSide, WorthUiIdentityMatchReport, WorthUiIdentitySeedContribution,
    WorthUiLaneAdapterHook, WorthUiLaneAdapterHookKind, WorthUiLaneAdmission,
    WorthUiLaneAdmissionCounters, WorthUiLaneAdmissionDenial, WorthUiLaneAdmissionDenialReason,
    WorthUiLaneCostRegime, WorthUiLaneFailureMode, WorthUiLaneHandle,
    WorthUiLaneImpactClassification, WorthUiLaneSupportDiagnostic, WorthUiLaneSupportRow,
    WorthUiLaneSupportStatus, WorthUiLaneTeachingPosture, WorthUiLastValidObservation,
    WorthUiMovedNodeIdentity, WorthUiNodeLifecycleTransition, WorthUiNodeReplacementClassification,
    WorthUiNodeReplacementCounters, WorthUiNodeReplacementPlan, WorthUiPlanChildRange,
    WorthUiPlanExecutionLane, WorthUiPlanLanePartition, WorthUiPlanLookupIndex,
    WorthUiPlanReuseClassification,
    WorthUiPendingActivation,
    WorthUiPendingExecutionPlanLoweringInput, WorthUiQueryBindingComparison,
    WorthUiQueryBindingComparisonCounters, WorthUiQueryBindingComparisonDenial,
    WorthUiQueryBindingComparisonEntry, WorthUiQueryBindingComparisonOutcome,
    WorthUiQueryBindingDriftDenial, WorthUiQueryBindingDriftDenialKind,
    WorthUiQueryBindingIdentity, WorthUiQueryBindingPosture,
    WorthUiQueryBindingPostureDriftFamily, WorthUiQueryBindingPreservation,
    WorthUiQueryBindingRebind, WorthUiQueryBindingRebindReason,
    WorthUiQueryBindingRetirement, WorthUiQueryBindingRetirementReason,
    WorthUiQueryLaneSupportLinks, WorthUiQueryLiveRebindCounters, WorthUiQueryLiveRebindEntry,
    WorthUiQueryLiveRebindOutcome, WorthUiQueryLiveRebindPlan,
    WorthUiQueryLiveRebindPlanDenial, WorthUiQueryRebindRequiredSurface,
    WorthUiQuerySupportReceipt, WorthUiQuerySupportStatus, WorthUiRendererResourceImpact,
    WorthUiRepeatedTemplateIdentity, WorthUiReplacementCandidate,
    WorthUiReplacementCandidateBasis,
    WorthUiReplacementCandidateDenial, WorthUiReplacementCause,
    WorthUiReplacementImpact, WorthUiReplacementImpactClassification, WorthUiReplacementImpactClassifier,
    WorthUiReplacementImpactCounters, WorthUiReplacementImpactDenial, WorthUiReplacementScope,
    WorthUiRuntimeActivationStatus, WorthUiRuntimeArtifactComparator,
    WorthUiRuntimeArtifactComparison, WorthUiRuntimeArtifactComparisonCounters,
    WorthUiRuntimeArtifactComparisonDenial, WorthUiRuntimeArtifactComparisonOutcome,
    WorthUiRuntimeDiagnosticPolicy, WorthUiRuntimeEquivalenceBasis, WorthUiRuntimeFrameEpoch,
    WorthUiPanelVisibilityReconciliation, WorthUiRuntimeHandle, WorthUiRuntimeHandleAllocation,
    WorthUiRuntimeHandleAllocationBasis, WorthUiRuntimeHandleAllocationCounters,
    WorthUiRuntimeHandleAllocationDenial, WorthUiRuntimeHandleAllocationDenialReason,
    WorthUiRuntime, WorthUiRuntimeHandleAllocationReceipt, WorthUiRuntimeHandleFamilyWidths,
    WorthUiRuntimeLaunch, WorthUiPlanLoweringBasis, WorthUiPlanLoweringContext,
    WorthUiPlanLoweringCounters, WorthUiPlanLoweringDenial, WorthUiPlanLoweringDenialReason,
    WorthUiPlanNode, WorthUiPlanNodeFamily, WorthUiPlanNodeInput, WorthUiPlanNodeInputFamily,
    WorthUiPlanNodeTopologyInput, WorthUiPlanRegionStructure, WorthUiPlanTopology,
    WorthUiPlanTopologyCounters, WorthUiPlanTopologyDenial, WorthUiPlanTopologyDenialReason,
    WorthUiRuntimeLaunchDenial, WorthUiRuntimeLifecycle, WorthUiRuntimeReplacementPosture,
    WorthUiRuntimeShutdownReceipt, WorthUiScrollAnchorReconciliation,
    WorthUiSelectionRangeReconciliation, WorthUiSplitterPositionReconciliation,
    WorthUiStagedReplacement, WorthUiStateOwnerIdentity, WorthUiStateOwnershipClass,
    WorthUiStatePersistencePosture, WorthUiStateSlotHandle, WorthUiTabStateReconciliation,
    WorthUiTextEditStateReconciliation, WorthUiTokenHandle, WorthUiTokenThemeImpact,
    WorthUiTransientInteractionPolicy, WorthUiTransientInteractionState,
    WorthUiUnsupportedHookDenial, WorthUiUnsupportedHookDenialReason,
    WorthUiUnsupportedReplacementImpact, WorthUiViewBindingHandle, WorthUiRenderResourceRef,
};

fn accepts_runtime_types(
    _host: Option<WorthUiRuntime>,
    _pending_activation: Option<WorthUiPendingActivation>,
    _pending_plan_lowering_input: Option<WorthUiPendingExecutionPlanLoweringInput>,
    _execution_plan_input: Option<WorthUiExecutionPlanInput>,
    _plan_lowering_basis: Option<WorthUiPlanLoweringBasis>,
    _plan_lowering_context: Option<WorthUiPlanLoweringContext>,
    _plan_lowering_counters: Option<WorthUiPlanLoweringCounters>,
    _plan_lowering_denial: Option<WorthUiPlanLoweringDenial>,
    _plan_node_input: Option<WorthUiPlanNodeInput>,
    _component_hook: Option<WorthUiComponentLoweringHook>,
    _component_hook_family: Option<WorthUiComponentLoweringHookFamily>,
    _runtime_handle: Option<WorthUiRuntimeHandle>,
    _component_handle: Option<WorthUiComponentHandle>,
    _command_handle: Option<WorthUiCommandHandle>,
    _token_handle: Option<WorthUiTokenHandle>,
    _child_range_handle: Option<WorthUiChildRangeHandle>,
    _view_binding_handle: Option<WorthUiViewBindingHandle>,
    _lane_handle: Option<WorthUiLaneHandle>,
    _state_slot_handle: Option<WorthUiStateSlotHandle>,
    _handle_generation: Option<WorthUiHandlePlanGeneration>,
    _handle_allocation: Option<WorthUiRuntimeHandleAllocation>,
    _handle_allocation_basis: Option<WorthUiRuntimeHandleAllocationBasis>,
    _handle_allocation_counters: Option<WorthUiRuntimeHandleAllocationCounters>,
    _handle_allocation_denial: Option<WorthUiRuntimeHandleAllocationDenial>,
    _handle_allocation_receipt: Option<WorthUiRuntimeHandleAllocationReceipt>,
    _handle_family_widths: Option<WorthUiRuntimeHandleFamilyWidths>,
    _staged_replacement: Option<WorthUiStagedReplacement>,
    _activation_readiness: Option<WorthUiActivationReadiness>,
    _activation_staging_report: Option<WorthUiActivationStagingReport>,
    _activation_staging_counters: Option<WorthUiActivationStagingCounters>,
    _activation_staging_denial: Option<WorthUiActivationStagingDenial>,
    _activation_gate_counters: Option<WorthUiActivationGateCounters>,
    _activation_gate_denial: Option<WorthUiActivationGateDenial>,
    _activation_gate_denial_reason: Option<WorthUiActivationGateDenialReason>,
    _activation_gate_receipt: Option<WorthUiActivationGateReceipt>,
    _frame_boundary: Option<WorthUiFrameBoundary>,
    _frame_boundary_posture: Option<WorthUiFrameBoundaryPosture>,
    _launch: Option<WorthUiRuntimeLaunch>,
    _launch_denial: Option<WorthUiRuntimeLaunchDenial>,
    _active: Option<WorthUiActiveRuntimeObservation>,
    _last_valid: Option<WorthUiLastValidObservation>,
    _shutdown: Option<WorthUiRuntimeShutdownReceipt>,
    _candidate: Option<WorthUiReplacementCandidate>,
    _candidate_basis: Option<WorthUiReplacementCandidateBasis>,
    _candidate_lowering_basis: Option<WorthUiCandidateLoweringBasis>,
    _candidate_bundle: Option<WorthUiCandidateArtifactBundle>,
    _candidate_metadata: Option<WorthUiCandidateDependencyMetadata>,
    _candidate_cause: Option<WorthUiReplacementCause>,
    _candidate_provenance: Option<WorthUiCandidateProvenanceHandle>,
    _active_replacement_basis: Option<WorthUiActiveReplacementBasis>,
    _admission: Option<WorthUiCandidateAdmission>,
    _admission_counters: Option<WorthUiCandidateAdmissionCounters>,
    _admission_report: Option<WorthUiCandidateAdmissionReport>,
    _admission_denial: Option<WorthUiCandidateAdmissionDenial>,
    _admitted: Option<WorthUiAdmittedReplacementCandidate>,
    _query_receipt: Option<WorthUiQuerySupportReceipt>,
    _runtime_comparator: Option<WorthUiRuntimeArtifactComparator<'static>>,
    _runtime_comparison: Option<WorthUiRuntimeArtifactComparison>,
    _runtime_comparison_counters: Option<WorthUiRuntimeArtifactComparisonCounters>,
    _runtime_comparison_denial: Option<WorthUiRuntimeArtifactComparisonDenial>,
    _runtime_equivalence_basis: Option<WorthUiRuntimeEquivalenceBasis>,
    _replacement_impact: Option<WorthUiReplacementImpact>,
    _replacement_scope: Option<WorthUiReplacementScope>,
    _lane_impact: Option<WorthUiLaneImpactClassification>,
    _unsupported_impact: Option<WorthUiUnsupportedReplacementImpact>,
    _impact_classification: Option<WorthUiReplacementImpactClassification>,
    _impact_classifier: Option<WorthUiReplacementImpactClassifier>,
    _impact_counters: Option<WorthUiReplacementImpactCounters>,
    _impact_denial: Option<WorthUiReplacementImpactDenial>,
    _durable_state_receipts: Option<WorthUiDurableStateImpactReceipts>,
    _command_impact: Option<WorthUiCommandImpact>,
    _theme_impact: Option<WorthUiTokenThemeImpact>,
    _accessibility_impact: Option<WorthUiAccessibilityImpact>,
    _renderer_impact: Option<WorthUiRendererResourceImpact>,
    _identity_graph: Option<WorthUiIdentityMatchGraph>,
    _identity_node: Option<WorthUiIdentityMatchNode>,
    _identity_edge: Option<WorthUiIdentityMatchEdge>,
    _identity_report: Option<WorthUiIdentityMatchReport>,
    _identity_denial: Option<WorthUiIdentityMatchDenial>,
    _identity_counters: Option<WorthUiIdentityMatchCounters>,
    _seed_contribution: Option<WorthUiIdentitySeedContribution>,
    _moved_identity: Option<WorthUiMovedNodeIdentity>,
    _repeated_identity: Option<WorthUiRepeatedTemplateIdentity>,
    _node_plan: Option<WorthUiNodeReplacementPlan>,
    _node_classification: Option<WorthUiNodeReplacementClassification>,
    _node_transition: Option<WorthUiNodeLifecycleTransition>,
    _node_counters: Option<WorthUiNodeReplacementCounters>,
    _node_denial: Option<WorthUiAmbiguousReplacementDenial>,
    _query_binding_comparison: Option<WorthUiQueryBindingComparison>,
    _query_binding_comparison_entry: Option<WorthUiQueryBindingComparisonEntry>,
    _query_binding_comparison_counters: Option<WorthUiQueryBindingComparisonCounters>,
    _query_binding_comparison_denial: Option<WorthUiQueryBindingComparisonDenial>,
    _query_binding_identity: Option<WorthUiQueryBindingIdentity>,
    _query_binding_posture: Option<WorthUiQueryBindingPosture>,
    _state_family_id: Option<WorthUiDurableStateFamilyId>,
    _state_family: Option<WorthUiDurableStateFamily>,
    _state_family_hook: Option<WorthUiDurableStateFamilyHook>,
    _state_inventory: Option<WorthUiDurableStateInventory>,
    _state_inventory_builder: Option<WorthUiDurableStateInventoryBuilder>,
    _state_inventory_counters: Option<WorthUiDurableStateInventoryCounters>,
    _state_inventory_denial: Option<WorthUiDurableStateInventoryDenial>,
    _state_replacement_policy: Option<WorthUiDurableStateReplacementPolicy>,
    _state_owner_identity: Option<WorthUiStateOwnerIdentity>,
    _state_ownership_class: Option<WorthUiStateOwnershipClass>,
    _state_persistence_posture: Option<WorthUiStatePersistencePosture>,
    _state_eligibility: Option<WorthUiDurableStateEligibility>,
    _transient_state: Option<WorthUiTransientInteractionState>,
    _transient_policy: Option<WorthUiTransientInteractionPolicy>,
) {
}

fn accepts_lane_admission_types(
    _lane: Option<WorthUiExecutionLane>,
    _descriptor: Option<WorthUiExecutionLaneDescriptor>,
    _support: Option<WorthUiExecutionLaneSupport>,
    _admission: Option<WorthUiLaneAdmission>,
    _admission_counters: Option<WorthUiLaneAdmissionCounters>,
    _admission_denial: Option<WorthUiLaneAdmissionDenial>,
    _support_row: Option<WorthUiLaneSupportRow>,
    _support_diagnostic: Option<WorthUiLaneSupportDiagnostic>,
    _query_links: Option<WorthUiQueryLaneSupportLinks>,
    _hook: Option<WorthUiLaneAdapterHook>,
    _hook_admission: Option<WorthUiExtensionHookAdmission>,
    _hook_denial: Option<WorthUiUnsupportedHookDenial>,
) {
}

fn accepts_plan_topology_types(
    _execution_plan: Option<WorthUiExecutionPlan>,
    _plan_topology: Option<WorthUiPlanTopology>,
    _plan_node: Option<WorthUiPlanNode>,
    _plan_node_family: Option<WorthUiPlanNodeFamily>,
    _plan_node_topology_input: Option<WorthUiPlanNodeTopologyInput>,
    _plan_region_structure: Option<WorthUiPlanRegionStructure>,
    _plan_child_range: Option<WorthUiPlanChildRange>,
    _plan_lane_partition: Option<WorthUiPlanLanePartition>,
    _plan_lookup_index: Option<WorthUiPlanLookupIndex>,
    _plan_digest: Option<WorthUiExecutionPlanDigest>,
    _plan_equivalence: Option<WorthUiExecutionPlanEquivalence>,
    _plan_equivalence_basis: Option<WorthUiExecutionPlanEquivalenceBasis>,
    _plan_equivalence_counters: Option<WorthUiExecutionPlanEquivalenceCounters>,
    _plan_topology_counters: Option<WorthUiPlanTopologyCounters>,
    _plan_topology_denial: Option<WorthUiPlanTopologyDenial>,
    _egui_plan_boundary: Option<WorthUiEguiPlanBoundary>,
    _egui_boundary_contact: Option<WorthUiEguiBoundaryContact>,
    _render_resource_ref: Option<WorthUiRenderResourceRef>,
) {
}

fn accepts_reconciliation_types(
    _plan: Option<WorthUiDurableStateReconciliationPlan>,
    _receipt: Option<WorthUiDurableStateReconciliationReceipt>,
    _counters: Option<WorthUiDurableStateReconciliationCounters>,
    _denial: Option<WorthUiDurableStateReconciliationDenial>,
    _outcome: Option<WorthUiDurableStateReconciliationOutcome>,
    _carry: Option<WorthUiDurableStateCarryForward>,
    _replacement: Option<WorthUiDurableStateReplacement>,
    _focus: Option<WorthUiFocusChainReconciliation>,
    _scroll: Option<WorthUiScrollAnchorReconciliation>,
    _selection: Option<WorthUiSelectionRangeReconciliation>,
    _text: Option<WorthUiTextEditStateReconciliation>,
    _splitter: Option<WorthUiSplitterPositionReconciliation>,
    _tab: Option<WorthUiTabStateReconciliation>,
    _panel: Option<WorthUiPanelVisibilityReconciliation>,
) {
}

fn accepts_query_live_rebind_types(
    _plan: Option<WorthUiQueryLiveRebindPlan>,
    _entry: Option<WorthUiQueryLiveRebindEntry>,
    _outcome: Option<WorthUiQueryLiveRebindOutcome>,
    _counters: Option<WorthUiQueryLiveRebindCounters>,
    _plan_denial: Option<WorthUiQueryLiveRebindPlanDenial>,
    _preservation: Option<WorthUiQueryBindingPreservation>,
    _rebind: Option<WorthUiQueryBindingRebind>,
    _retirement: Option<WorthUiQueryBindingRetirement>,
    _drift_denial: Option<WorthUiQueryBindingDriftDenial>,
) {
}

fn main() {
    let app = WorthUi::app().freeze();
    let _ = app.capabilities().digest();
    let _ = WorthUiRuntimeFrameEpoch::initial();
    let _ = WorthUiRuntimeDiagnosticPolicy::minimal();
    let _ = WorthUiRuntimeActivationStatus::Active;
    let _ = WorthUiRuntimeLifecycle::Active;
    let _ = WorthUiActivationStagingDenialReason::MissingQueryLiveRebindPlan;
    let _ = WorthUiPlanLoweringDenialReason::StalePendingActivation;
    let _ = WorthUiPlanNodeInputFamily::QueryViewBinding;
    let _ = WorthUiEguiBoundaryInput::QueryBinding;
    let _ = WorthUiPlanExecutionLane::QueryView;
    let _ = WorthUiPlanReuseClassification::Reusable;
    let _ = WorthUiPlanReuseClassification::RebuildRequired;
    let _ = WorthUiPlanTopologyDenialReason::MissingEguiBoundaryDeclaration;
    let _ = WorthUiPlanTopologyDenialReason::MissingRegionStructure;
    let _ = WorthUiEguiBoundaryContact::FrameTiming;
    let _ = WorthUiRuntimeHandleAllocationDenialReason::DuplicatePlanLocalHandleClaim;
    let _ = WorthUiCandidateAuthoringLane::file_authored();
    let _ = WorthUiCandidateAuthoringLane::rust_authored();
    let _ = WorthUiReplacementCandidateDenial::MissingDependencyMetadata;
    let _ = WorthUiRuntimeReplacementPosture::Supported;
    let _ = WorthUiQuerySupportStatus::Supported;
    let _ = WorthUiRuntimeArtifactComparisonOutcome::EquivalentNoOp;
    let _ = WorthUiRuntimeEquivalenceBasis::semantic_artifact_meaning();
    let _ = WorthUiReplacementImpact::NoOp;
    let _ = WorthUiCommandImpact::Unchanged;
    let _ = WorthUiTokenThemeImpact::Unchanged;
    let _ = WorthUiAccessibilityImpact::Unchanged;
    let _ = WorthUiRendererResourceImpact::Unchanged;
    let _ = WorthUiLaneImpactClassification::Unaffected;
    let _ = WorthUiIdentityMatchNodeSide::Active;
    let _ = WorthUiIdentityMatchNodeKind::Component;
    let _ = WorthUiNodeLifecycleTransition::Preserve;
    let _ = WorthUiQueryBindingComparisonOutcome::PreserveMeaning;
    let _ = WorthUiQueryBindingPostureDriftFamily::AsyncResultState;
    let _ = WorthUiQueryBindingDriftDenialKind::UiLocalDenialPresentationWouldReplaceQueryRecovery;
    let _ = WorthUiQueryBindingRebindReason::QueryOwnedPostureDrift;
    let _ = WorthUiQueryBindingRetirementReason::CandidateRemovedQueryBinding;
    let _ = WorthUiQueryRebindRequiredSurface::ContinuationPipeline;
    let _ = WorthUiDurableStateFamilyId::FocusChain;
    let _ = WorthUiDurableStateFamily::focus_chain();
    let _ = WorthUiDurableStateReplacementPolicy::ReconcileOnLaneChange;
    let _ = WorthUiStateOwnerIdentity::platform_shell();
    let _ = WorthUiStateOwnershipClass::PlatformShell;
    let _ = WorthUiStatePersistencePosture::RuntimeOnly;
    let _ = WorthUiDurableStateEligibility::Eligible;
    let _ = WorthUiTransientInteractionState::Hover;
    let _ = WorthUiTransientInteractionPolicy::Drop;
    let _ = WorthUiDurableStateReconciliationOutcome::CarryForward;
    let _ = WorthUiFocusChainReconciliation::preserve_by_durable_identity();
    let _ = WorthUiScrollAnchorReconciliation::stable_anchor_identity();
    let _ = WorthUiSelectionRangeReconciliation::backing_collection_identity();
    let _ = WorthUiTextEditStateReconciliation::drop_on_incompatible_shape();
    let _ = WorthUiSplitterPositionReconciliation::stable_identity();
    let _ = WorthUiTabStateReconciliation::stable_identity();
    let _ = WorthUiPanelVisibilityReconciliation::stable_identity();
    let _ = WorthUiExecutionLane::QueryBound;
    let _ = WorthUiLaneCostRegime::QueryRuntimeBacked;
    let _ = WorthUiLaneFailureMode::QuerySupportDenial;
    let _ = WorthUiLaneSupportStatus::Supported;
    let _ = WorthUiLaneTeachingPosture::SupportGateOnly;
    let _ = WorthUiLaneAdmissionDenialReason::UnsupportedLaneReference;
    let _ = WorthUiLaneAdapterHookKind::CanvasSpatialDrawAndHitTest;
    let _ = WorthUiUnsupportedHookDenialReason::ActivePlanTruthOverride;

    accepts_runtime_types(
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None,
        None, None, None, None, None, None, None,
        None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None,
    );
    accepts_plan_topology_types(
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None,
    );
    accepts_reconciliation_types(
        None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    );
    accepts_query_live_rebind_types(None, None, None, None, None, None, None, None, None);
    accepts_lane_admission_types(
        None, None, None, None, None, None, None, None, None, None, None, None,
    );
}
