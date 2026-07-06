use crate::runtime::activation_staging::WorthUiActivationStager;
use crate::runtime::active::{
    WorthUiActiveArtifact, WorthUiActiveExecutionPlan, WorthUiActiveRuntimeObservation,
    WorthUiActiveRuntimeState,
};
use crate::runtime::admission::WorthUiActiveReplacementBasis;
use crate::runtime::allocation_planning::{
    WorthUiAllocationPlanner, WorthUiAllocationPlanningAdmission,
    WorthUiRetainedAllocationPlanningEvidenceRegistry,
};
use crate::runtime::equivalence::WorthUiRuntimeArtifactComparator;
use crate::runtime::execution_plan_input::WorthUiExecutionPlanInputPreparer;
use crate::runtime::handle_allocation::WorthUiRuntimeHandleAllocator;
use crate::runtime::impact::WorthUiReplacementImpactClassifier;
use crate::runtime::lifecycle::WorthUiRuntimeShutdownReceipt;
use crate::runtime::matching::WorthUiIdentityMatchGraphBuilder;
use crate::runtime::narrowing::WorthUiRuntimeImpactNarrower;
use crate::runtime::preservation::{WorthUiLastValidObservation, WorthUiLastValidRuntimeState};
use crate::runtime::query_binding::WorthUiQueryBindingComparisonPlanner;
use crate::runtime::query_live_rebind::WorthUiQueryLiveRebindPlanner;
use crate::runtime::reconciliation::WorthUiDurableStateReconciliationPlanner;
use crate::runtime::replacement::WorthUiNodeReplacementClassifier;
use crate::runtime::source_ingress::WorthUiSourceWatcher;
use crate::runtime::state_inventory::WorthUiDurableStateInventoryBuilder;
#[cfg(test)]
use crate::runtime::WorthUiComponentLoweringHook;
use crate::runtime::{
    WorthUiActivationStagingDenial, WorthUiAdmittedReplacementCandidate, WorthUiAllocationPlanning,
    WorthUiAmbiguousReplacementDenial, WorthUiExecutionPlanInput, WorthUiIdentityMatchDenial,
    WorthUiIdentityMatchReport, WorthUiNodeReplacementPlan, WorthUiPendingActivation,
    WorthUiPendingExecutionPlanLoweringInput, WorthUiPlanLoweringDenial,
    WorthUiReplacementImpactClassification, WorthUiReplacementImpactDenial,
    WorthUiRuntimeArtifactComparison, WorthUiRuntimeArtifactComparisonDenial,
    WorthUiRuntimeDiagnosticPolicy, WorthUiRuntimeFrameEpoch, WorthUiRuntimeHandleAllocation,
    WorthUiRuntimeHandleAllocationDenial, WorthUiRuntimeImpactNarrowing,
    WorthUiRuntimeImpactNarrowingDenial, WorthUiRuntimeLifecycle,
};
use crate::runtime::{
    WorthUiDurableStateInventory, WorthUiDurableStateReconciliationDenial,
    WorthUiDurableStateReconciliationPlan, WorthUiQueryBindingComparison,
    WorthUiQueryBindingComparisonDenial, WorthUiQueryLiveRebindPlan,
    WorthUiQueryLiveRebindPlanDenial,
};
use crate::source::{WorthUiArtifact, WorthUiArtifactDigestor, WorthUiArtifactEquivalenceBasis};
use crate::{capability::CapabilitySnapshotDigest, source::WorthUiArtifactDigest};
use std::borrow::Borrow;
use std::rc::Rc;

/// Runtime host that owns active Worth UI runtime truth.
#[derive(Debug)]
pub struct WorthUiRuntimeHost {
    active: WorthUiActiveRuntimeState,
    last_valid: WorthUiLastValidRuntimeState,
    pub(crate) retained_allocation_planning_evidence:
        Rc<WorthUiRetainedAllocationPlanningEvidenceRegistry>,
}

/// Launch request for creating an active runtime host from canonical artifact truth.
#[derive(Debug)]
pub struct WorthUiRuntimeLaunch {
    artifact: WorthUiArtifact,
    frame_epoch: WorthUiRuntimeFrameEpoch,
    diagnostic_policy: WorthUiRuntimeDiagnosticPolicy,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiRuntimeLaunchDenial {
    StalePendingActivation {
        pending_epoch: WorthUiRuntimeFrameEpoch,
        active_epoch: WorthUiRuntimeFrameEpoch,
    },
}

impl WorthUiRuntimeHost {
    pub(crate) fn launch(
        launch: WorthUiRuntimeLaunch,
        snapshot_digest: CapabilitySnapshotDigest,
        retained_allocation_planning_evidence: Rc<WorthUiRetainedAllocationPlanningEvidenceRegistry>,
    ) -> Result<Self, WorthUiRuntimeLaunchDenial> {
        let WorthUiRuntimeLaunch {
            artifact,
            frame_epoch,
            diagnostic_policy,
        } = launch;
        let (active_artifact, artifact_digest) = seal_launch_artifact(artifact);
        let active_plan = derive_launch_execution_plan(artifact_digest, snapshot_digest);
        let active = build_active_runtime_state(
            active_artifact,
            active_plan,
            snapshot_digest,
            frame_epoch,
            diagnostic_policy,
        );
        let last_valid = WorthUiLastValidRuntimeState::record_from_active(&active);

        Ok(Self {
            active,
            last_valid,
            retained_allocation_planning_evidence,
        })
    }

    pub fn lifecycle(&self) -> WorthUiRuntimeLifecycle {
        self.active.lifecycle()
    }

    pub fn frame_epoch(&self) -> WorthUiRuntimeFrameEpoch {
        self.active.frame_epoch()
    }

    pub fn inspect_active(&self) -> WorthUiActiveRuntimeObservation {
        self.active.observation()
    }

    pub fn replacement_admission_basis(&self) -> WorthUiActiveReplacementBasis {
        WorthUiActiveReplacementBasis::from_observation(self.inspect_active())
    }

    pub fn compare_admitted_replacement(
        &self,
        admitted: &WorthUiAdmittedReplacementCandidate,
    ) -> Result<WorthUiRuntimeArtifactComparison, WorthUiRuntimeArtifactComparisonDenial> {
        WorthUiRuntimeArtifactComparator::for_active_artifact(self.active.active_artifact())
            .compare_admitted(admitted)
    }

    pub fn classify_replacement_impact(
        &self,
        comparison: &WorthUiRuntimeArtifactComparison,
        admitted: &WorthUiAdmittedReplacementCandidate,
    ) -> Result<WorthUiReplacementImpactClassification, WorthUiReplacementImpactDenial> {
        WorthUiReplacementImpactClassifier::classify(comparison, admitted)
    }

    pub fn narrow_replacement_impact(
        &self,
        classification: &WorthUiReplacementImpactClassification,
        admitted: &WorthUiAdmittedReplacementCandidate,
    ) -> Result<WorthUiRuntimeImpactNarrowing, WorthUiRuntimeImpactNarrowingDenial> {
        WorthUiRuntimeImpactNarrower::narrow(classification, admitted)
    }

    pub fn build_identity_match_graph(
        &self,
        narrowing: &WorthUiRuntimeImpactNarrowing,
        admitted: &WorthUiAdmittedReplacementCandidate,
    ) -> Result<WorthUiIdentityMatchReport, WorthUiIdentityMatchDenial> {
        WorthUiIdentityMatchGraphBuilder::build(self.active.active_artifact(), narrowing, admitted)
    }

    pub fn classify_node_replacements(
        &self,
        impact: &WorthUiReplacementImpactClassification,
        narrowing: &WorthUiRuntimeImpactNarrowing,
        identity_report: &WorthUiIdentityMatchReport,
    ) -> Result<WorthUiNodeReplacementPlan, WorthUiAmbiguousReplacementDenial> {
        WorthUiNodeReplacementClassifier::classify(impact, narrowing, identity_report)
    }

    pub fn durable_state_inventory(&self) -> WorthUiDurableStateInventoryBuilder {
        WorthUiDurableStateInventoryBuilder::new()
    }

    pub fn source_ingress(
        &self,
        provider: crate::runtime::WorthUiSourceProvider,
    ) -> WorthUiSourceWatcher {
        WorthUiSourceWatcher::new(provider)
    }

    pub fn reconcile_durable_state(
        &self,
        node_plan: &WorthUiNodeReplacementPlan,
        inventory: &WorthUiDurableStateInventory,
    ) -> Result<WorthUiDurableStateReconciliationPlan, WorthUiDurableStateReconciliationDenial>
    {
        WorthUiDurableStateReconciliationPlanner::reconcile(node_plan, inventory)
    }

    pub fn compare_query_bindings(
        &self,
        node_plan: &WorthUiNodeReplacementPlan,
        narrowing: &WorthUiRuntimeImpactNarrowing,
        admitted: &WorthUiAdmittedReplacementCandidate,
    ) -> Result<WorthUiQueryBindingComparison, WorthUiQueryBindingComparisonDenial> {
        WorthUiQueryBindingComparisonPlanner::compare(
            self.active.active_artifact().artifact(),
            node_plan,
            narrowing,
            admitted,
        )
    }

    pub fn plan_query_live_rebinds(
        &self,
        comparison: &WorthUiQueryBindingComparison,
        node_plan: &WorthUiNodeReplacementPlan,
        narrowing: &WorthUiRuntimeImpactNarrowing,
        admitted: &WorthUiAdmittedReplacementCandidate,
    ) -> Result<WorthUiQueryLiveRebindPlan, WorthUiQueryLiveRebindPlanDenial> {
        WorthUiQueryLiveRebindPlanner::plan(comparison, node_plan, narrowing, admitted)
    }

    pub fn prepare_pending_execution_plan_lowering_input(
        &self,
        node_plan: &WorthUiNodeReplacementPlan,
        reconciliation_plan: &WorthUiDurableStateReconciliationPlan,
        query_rebind_plan: &WorthUiQueryLiveRebindPlan,
    ) -> WorthUiPendingExecutionPlanLoweringInput {
        WorthUiPendingExecutionPlanLoweringInput::from_staged_plans(
            node_plan,
            reconciliation_plan,
            query_rebind_plan,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn stage_replacement_activation(
        &self,
        admitted: WorthUiAdmittedReplacementCandidate,
        impact: &WorthUiReplacementImpactClassification,
        narrowing: &WorthUiRuntimeImpactNarrowing,
        node_plan: &WorthUiNodeReplacementPlan,
        reconciliation_plan: Option<&WorthUiDurableStateReconciliationPlan>,
        query_rebind_plan: Option<&WorthUiQueryLiveRebindPlan>,
        pending_execution_plan_lowering_input: Option<&WorthUiPendingExecutionPlanLoweringInput>,
    ) -> Result<WorthUiPendingActivation, WorthUiActivationStagingDenial> {
        let active_before = self.inspect_active();
        let active_after = self.inspect_active();
        WorthUiActivationStager::stage(
            active_before,
            active_after,
            admitted,
            impact,
            narrowing,
            node_plan,
            reconciliation_plan,
            query_rebind_plan,
            pending_execution_plan_lowering_input,
        )
    }

    pub(crate) fn prepare_execution_plan_input<P>(
        &self,
        pending_activation: P,
    ) -> Result<WorthUiExecutionPlanInput, WorthUiPlanLoweringDenial>
    where
        P: Borrow<WorthUiPendingActivation>,
    {
        WorthUiExecutionPlanInputPreparer::prepare(
            pending_activation.borrow(),
            self.active.frame_epoch(),
            &[],
        )
    }

    pub fn allocate_runtime_handles(
        &self,
        allocation_planning: &WorthUiAllocationPlanning,
    ) -> Result<WorthUiRuntimeHandleAllocation, WorthUiRuntimeHandleAllocationDenial> {
        WorthUiRuntimeHandleAllocator::allocate(allocation_planning)
    }

    pub fn plan_allocation<P>(
        &self,
        pending_activation: P,
        measurement_basis: &crate::evidence::UiMeasurementBasis,
        allocation_neighborhood: &crate::evidence::UiAllocationNeighborhood,
    ) -> WorthUiAllocationPlanning
    where
        P: Borrow<WorthUiPendingActivation>,
    {
        let pending_activation = pending_activation.borrow();
        let measurement_basis = measurement_basis_with_runtime_durable_resize_support(
            measurement_basis,
            allocation_neighborhood,
            pending_activation
                .staged_replacement()
                .reconciliation_plan()
                .durable_resize_inputs(),
        );
        let constraint_set =
            match measurement_basis.admit_allocation_constraint_set(allocation_neighborhood) {
                Ok(constraint_set) => constraint_set,
                Err(constraint_set_denial) => {
                    let mut counters = crate::runtime::WorthUiAllocationPlanningCounters::default();
                    counters.record_planning_attempt();
                    counters.record_measurement_basis_read();
                    let basis = crate::runtime::WorthUiAllocationPlanningBasis::new(
                        measurement_basis.clone(),
                        allocation_neighborhood.clone(),
                        None,
                    );
                    let denial_posture = Some(crate::runtime::WorthUiAllocationPlanningDenial::new(
                        crate::runtime::WorthUiAllocationPlanningDenialReason::ConstraintSetDenied,
                        None,
                        Some(constraint_set_denial),
                        None,
                        None,
                        counters,
                    ));
                    return crate::runtime::WorthUiAllocationPlanning::new(
                        basis,
                        None,
                        denial_posture,
                        counters,
                    );
                }
            };
        let admission = WorthUiAllocationPlanningAdmission::from_pending_activation(
            pending_activation,
            &measurement_basis,
            allocation_neighborhood,
            &constraint_set,
        );
        match self.prepare_execution_plan_input(pending_activation) {
            Ok(lowered_input) => {
                WorthUiAllocationPlanner::plan_from_lowered_input(admission, lowered_input)
            }
            Err(plan_lowering_denial) => WorthUiAllocationPlanner::deny_from_plan_lowering(
                &measurement_basis,
                allocation_neighborhood,
                plan_lowering_denial,
            ),
        }
    }

    #[cfg(test)]
    pub(crate) fn plan_allocation_for_lowered_input_for_test(
        &self,
        plan_input: WorthUiExecutionPlanInput,
        measurement_basis: &crate::evidence::UiMeasurementBasis,
        allocation_neighborhood: &crate::evidence::UiAllocationNeighborhood,
    ) -> WorthUiAllocationPlanning {
        let measurement_basis = measurement_basis_with_runtime_durable_resize_support(
            measurement_basis,
            allocation_neighborhood,
            &[],
        );
        WorthUiAllocationPlanner::plan_from_lowered_input(
            WorthUiAllocationPlanningAdmission::from_lowered_input_for_test(
                &plan_input,
                &measurement_basis,
                allocation_neighborhood,
                &measurement_basis
                    .admit_allocation_constraint_set(allocation_neighborhood)
                    .expect("constraint set should admit in lowered-input test path"),
            ),
            plan_input,
        )
    }

    #[cfg(test)]
    pub(crate) fn plan_allocation_for_pending_and_lowered_input_for_test<P>(
        &self,
        pending_activation: P,
        plan_input: WorthUiExecutionPlanInput,
        measurement_basis: &crate::evidence::UiMeasurementBasis,
        allocation_neighborhood: &crate::evidence::UiAllocationNeighborhood,
    ) -> WorthUiAllocationPlanning
    where
        P: Borrow<WorthUiPendingActivation>,
    {
        let measurement_basis = measurement_basis_with_runtime_durable_resize_support(
            measurement_basis,
            allocation_neighborhood,
            pending_activation
                .borrow()
                .staged_replacement()
                .reconciliation_plan()
                .durable_resize_inputs(),
        );
        WorthUiAllocationPlanner::plan_from_lowered_input(
            WorthUiAllocationPlanningAdmission::from_pending_activation(
                pending_activation.borrow(),
                &measurement_basis,
                allocation_neighborhood,
                &measurement_basis
                    .admit_allocation_constraint_set(allocation_neighborhood)
                    .expect("constraint set should admit in pending-lowered-input test path"),
            ),
            plan_input,
        )
    }

    #[cfg(test)]
    pub(crate) fn prepare_execution_plan_input_with_component_hooks_for_test<P>(
        &self,
        pending_activation: P,
        component_hooks: &[WorthUiComponentLoweringHook],
    ) -> Result<WorthUiExecutionPlanInput, WorthUiPlanLoweringDenial>
    where
        P: Borrow<WorthUiPendingActivation>,
    {
        WorthUiExecutionPlanInputPreparer::prepare(
            pending_activation.borrow(),
            self.active.frame_epoch(),
            component_hooks,
        )
    }

    #[cfg(test)]
    pub(crate) fn compare_admitted_replacement_with_basis_for_test(
        &self,
        admitted: &WorthUiAdmittedReplacementCandidate,
        runtime_basis: crate::runtime::WorthUiRuntimeEquivalenceBasis,
    ) -> Result<WorthUiRuntimeArtifactComparison, WorthUiRuntimeArtifactComparisonDenial> {
        WorthUiRuntimeArtifactComparator::for_active_artifact(self.active.active_artifact())
            .with_runtime_basis_for_test(runtime_basis)
            .compare_admitted(admitted)
    }

    pub fn last_valid(&self) -> WorthUiLastValidObservation {
        self.last_valid.observation()
    }

    pub fn shutdown(self) -> WorthUiRuntimeShutdownReceipt {
        WorthUiRuntimeShutdownReceipt::new(self.active.frame_epoch())
    }

    pub(crate) fn active_state_for_swap_mut(&mut self) -> &mut WorthUiActiveRuntimeState {
        &mut self.active
    }

    pub(crate) fn record_last_valid_from_active_for_swap(&mut self) {
        self.last_valid = WorthUiLastValidRuntimeState::record_from_active(&self.active);
    }

    #[allow(dead_code)]
    pub(crate) fn reject_if_pending_activation_is_stale(
        &self,
        pending_activation: WorthUiPendingActivation,
    ) -> Result<(), WorthUiRuntimeLaunchDenial> {
        let active_epoch = self.active.frame_epoch();
        let pending_epoch = pending_activation.frame_epoch();
        if pending_epoch == active_epoch {
            Ok(())
        } else {
            Err(WorthUiRuntimeLaunchDenial::StalePendingActivation {
                pending_epoch,
                active_epoch,
            })
        }
    }

    #[cfg(test)]
    pub(crate) fn advance_frame_epoch_for_test(&mut self) {
        self.active
            .advance_frame_epoch_for_test(self.active.frame_epoch().next());
    }
}

fn measurement_basis_with_runtime_durable_resize_support(
    measurement_basis: &crate::evidence::UiMeasurementBasis,
    allocation_neighborhood: &crate::evidence::UiAllocationNeighborhood,
    durable_resize_inputs: &[crate::runtime::WorthUiAdmittedDurableResizeInput],
) -> crate::evidence::UiMeasurementBasis {
    let axis_scope = match allocation_neighborhood
        .layout_operator_planning_contract()
        .operator_kind()
    {
        crate::declaration::UiDeclarationPlanningOperatorKind::Split => {
            crate::evidence::UiConstraintAxisScope::Primary
        }
        crate::declaration::UiDeclarationPlanningOperatorKind::Mosaic => {
            crate::evidence::UiConstraintAxisScope::Both
        }
        _ => return measurement_basis.clone(),
    };
    let root_provenance_digest = allocation_neighborhood
        .members()
        .iter()
        .find(|member| {
            matches!(
                member.role(),
                crate::evidence::UiAllocationNeighborhoodMemberRole::Root
            )
        })
        .expect("allocation neighborhood must preserve a root member")
        .authored_provenance_digest();
    let Some(durable_resize_input) = durable_resize_inputs.iter().find(|input| {
        input.is_admitted() && input.authored_provenance_digest() == Some(root_provenance_digest)
    }) else {
        return measurement_basis.clone();
    };
    let Some(runtime_resize_support) =
        crate::evidence::MeasurementEvidenceInput::runtime_durable_resize_support(
            durable_resize_input,
            allocation_neighborhood.root_graph_node_identity(),
            axis_scope,
            allocation_neighborhood
                .layout_operator_planning_contract()
                .mosaic_sizing_contract_id(),
        )
    else {
        return measurement_basis.clone();
    };
    let mut evidence_inputs = measurement_basis.evidence_inputs().to_vec();
    evidence_inputs.push(runtime_resize_support);
    crate::evidence::admit_measurement_basis(
        measurement_basis.declaration_identity().clone(),
        measurement_basis.graph_node_identity(),
        measurement_basis.world_profile().clone(),
        measurement_basis.declaration_support_authority_generation(),
        measurement_basis.declared_measurement_policy(),
        &evidence_inputs,
    )
}

fn seal_launch_artifact(
    artifact: WorthUiArtifact,
) -> (WorthUiActiveArtifact, WorthUiArtifactDigest) {
    let artifact_digest =
        WorthUiArtifactDigestor::digest(&artifact, WorthUiArtifactEquivalenceBasis::semantic());
    (
        WorthUiActiveArtifact::new(artifact, artifact_digest),
        artifact_digest,
    )
}

fn derive_launch_execution_plan(
    artifact_digest: WorthUiArtifactDigest,
    snapshot_digest: CapabilitySnapshotDigest,
) -> WorthUiActiveExecutionPlan {
    WorthUiActiveExecutionPlan::from_launch_authority(artifact_digest, snapshot_digest)
}

fn build_active_runtime_state(
    active_artifact: WorthUiActiveArtifact,
    active_plan: WorthUiActiveExecutionPlan,
    snapshot_digest: CapabilitySnapshotDigest,
    frame_epoch: WorthUiRuntimeFrameEpoch,
    diagnostic_policy: WorthUiRuntimeDiagnosticPolicy,
) -> WorthUiActiveRuntimeState {
    WorthUiActiveRuntimeState::new(
        active_artifact,
        active_plan,
        snapshot_digest,
        frame_epoch,
        diagnostic_policy,
    )
}

impl WorthUiRuntimeLaunch {
    #[allow(dead_code)]
    pub(crate) fn from_canonical_artifact(artifact: WorthUiArtifact) -> Self {
        Self {
            artifact,
            frame_epoch: WorthUiRuntimeFrameEpoch::initial(),
            diagnostic_policy: WorthUiRuntimeDiagnosticPolicy::minimal(),
        }
    }

    pub fn with_diagnostics(mut self, diagnostic_policy: WorthUiRuntimeDiagnosticPolicy) -> Self {
        self.diagnostic_policy = diagnostic_policy;
        self
    }
}
