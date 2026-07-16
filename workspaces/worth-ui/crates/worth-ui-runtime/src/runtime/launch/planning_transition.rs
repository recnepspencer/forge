use std::borrow::Borrow;

use crate::graph::UiGraphSnapshot;
use crate::obligations::selection::UiSelectedObligationSet;
use crate::runtime::execution::WorthUiExecutionLaneInput;
use crate::runtime::execution_plan_input::WorthUiExecutionPlanInputPreparer;
use crate::runtime::handle_allocation::WorthUiRuntimeHandleAllocator;
use crate::runtime::planning::{
    collect_planning_measurement_basis, construct_planning_lane_input,
    plan_allocation_for_pending_activation, WorthUiPlanningLaneAdmissionDenial,
    WorthUiPlanningLaneInput,
};
use crate::runtime::{
    UiAllocationCandidate, UiAllocationReceipt, WorthUiPendingActivation,
    WorthUiPlanLoweringDenial, WorthUiRuntimeHandleAllocation,
    WorthUiRuntimeHandleAllocationDenial,
};

use super::runtime_instance::WorthUiRuntime;

pub(crate) struct UiAllocationCatalogMintAuthority(());

#[derive(Debug)]
pub(crate) struct UiAllocationPlanningCatalogInput<'pending> {
    inputs: Box<[WorthUiPlanningLaneInput<&'pending WorthUiPendingActivation>]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum UiAllocationPlanningCatalogAdmissionDenial {
    Neighborhood(crate::graph::UiAllocationNeighborhoodDenial),
    PlanningLane,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum UiAllocationCatalogPreparationDenial {
    PlanningAdmission(UiAllocationPlanningCatalogAdmissionDenial),
    CatalogPlanning(
        crate::runtime::invalidation_narrowing::UiAllocationActivationCatalogDenial,
    ),
    ReceiptCommit(crate::runtime::UiAllocationReceiptCommitOutcome),
}

impl UiAllocationCatalogMintAuthority {
    fn new() -> Self {
        Self(())
    }
}

impl WorthUiRuntime {
    pub(crate) fn prepare_allocation_catalog_activation(
        &self,
        pending: &WorthUiPendingActivation,
        admitted: crate::graph::UiAdmittedAllocationCatalogBasisSet,
    ) -> Result<
        crate::runtime::UiCommittedAllocationActivationAttempt,
        UiAllocationCatalogPreparationDenial,
    > {
        let input = self
            .admit_allocation_planning_catalog(pending, admitted)
            .map_err(UiAllocationCatalogPreparationDenial::PlanningAdmission)?;
        let catalog = self
            .plan_allocation_catalog(input)
            .map_err(UiAllocationCatalogPreparationDenial::CatalogPlanning)?;
        self.seal_allocation_catalog_activation(
            catalog,
            pending.frame_epoch(),
            pending.staged_replacement().reconciliation_plan(),
        )
        .map_err(UiAllocationCatalogPreparationDenial::ReceiptCommit)
    }

    pub(crate) fn admit_allocation_planning_catalog<'pending>(
        &self,
        pending: &'pending WorthUiPendingActivation,
        admitted: crate::graph::UiAdmittedAllocationCatalogBasisSet,
    ) -> Result<
        UiAllocationPlanningCatalogInput<'pending>,
        UiAllocationPlanningCatalogAdmissionDenial,
    > {
        let mut inputs = Vec::with_capacity(admitted.entries.len());
        for (basis, selected) in admitted.entries.into_vec() {
            let preliminary = basis
                .admit_allocation_neighborhood(&admitted.snapshot, &selected)
                .map_err(UiAllocationPlanningCatalogAdmissionDenial::Neighborhood)?;
            let basis = collect_planning_measurement_basis(
                &basis,
                &preliminary,
                pending
                    .staged_replacement()
                    .reconciliation_plan()
                    .durable_resize_inputs(),
            );
            let neighborhood = basis
                .admit_allocation_neighborhood(&admitted.snapshot, &selected)
                .map_err(UiAllocationPlanningCatalogAdmissionDenial::Neighborhood)?;
            inputs.push(
                construct_planning_lane_input(pending, basis, neighborhood)
                    .map_err(|_| UiAllocationPlanningCatalogAdmissionDenial::PlanningLane)?,
            );
        }
        Ok(UiAllocationPlanningCatalogInput {
            inputs: inputs.into_boxed_slice(),
        })
    }

    pub(crate) fn plan_allocation_catalog(
        &self,
        input: UiAllocationPlanningCatalogInput<'_>,
    ) -> Result<
        crate::runtime::invalidation_narrowing::UiAllocationActivationCatalog,
        crate::runtime::invalidation_narrowing::UiAllocationActivationCatalogDenial,
    > {
        let candidates = input
            .inputs
            .into_vec()
            .into_iter()
            .map(|input| self.plan_allocation(input))
            .collect::<Vec<_>>();
        crate::runtime::invalidation_narrowing::UiAllocationActivationCatalog::from_planning(
            candidates,
            UiAllocationCatalogMintAuthority::new(),
        )
    }

    pub(crate) fn seal_allocation_catalog_activation(
        &self,
        catalog: crate::runtime::invalidation_narrowing::UiAllocationActivationCatalog,
        frame_epoch: crate::runtime::UiAllocationFrameEpoch,
        reconciliation: &crate::runtime::WorthUiDurableStateReconciliationPlan,
    ) -> Result<
        crate::runtime::UiCommittedAllocationActivationAttempt,
        crate::runtime::UiAllocationReceiptCommitOutcome,
    > {
        self.allocation_receipt_ledger
            .seal_activation_catalog(catalog, frame_epoch, reconciliation)
    }

    pub(crate) fn prepare_execution_plan_input<P>(
        &self,
        pending_activation: P,
    ) -> Result<crate::runtime::WorthUiExecutionPlanInput, WorthUiPlanLoweringDenial>
    where
        P: Borrow<WorthUiPendingActivation>,
    {
        WorthUiExecutionPlanInputPreparer::prepare(
            pending_activation.borrow(),
            self.active.frame_epoch(),
            &[],
        )
    }

    pub fn allocate_runtime_handles_from_lane_input(
        &self,
        input: WorthUiExecutionLaneInput<'_>,
    ) -> Result<WorthUiRuntimeHandleAllocation, WorthUiRuntimeHandleAllocationDenial> {
        WorthUiRuntimeHandleAllocator::allocate(input.allocation_receipt())
    }

    pub fn allocate_runtime_handles(
        &self,
        allocation_receipt: &UiAllocationReceipt,
    ) -> Result<WorthUiRuntimeHandleAllocation, WorthUiRuntimeHandleAllocationDenial> {
        self.allocate_runtime_handles_from_lane_input(WorthUiExecutionLaneInput::new(
            allocation_receipt,
        ))
    }

    pub fn plan_allocation<P>(&self, input: WorthUiPlanningLaneInput<P>) -> UiAllocationCandidate
    where
        P: Borrow<WorthUiPendingActivation>,
    {
        let impact = input
            .pending_activation()
            .staged_replacement()
            .impact()
            .clone();
        let narrowing = input
            .pending_activation()
            .staged_replacement()
            .narrowing()
            .clone();
        let mut candidate = plan_allocation_for_pending_activation(
            self,
            input.pending_activation(),
            input.measurement_basis(),
            input.allocation_neighborhood(),
        );
        if candidate.is_admitted() {
            candidate.seal_replan_admission(impact, narrowing);
        }
        candidate
    }

    pub fn admit_planning_lane_input<P>(
        &self,
        pending_activation: P,
        graph_snapshot: &UiGraphSnapshot,
        measurement_basis: crate::evidence::UiMeasurementBasis,
        selected_obligations: &UiSelectedObligationSet,
    ) -> Result<WorthUiPlanningLaneInput<P>, WorthUiPlanningLaneAdmissionDenial>
    where
        P: Borrow<WorthUiPendingActivation>,
    {
        let preliminary_neighborhood = selected_obligations
            .admit_allocation_neighborhood(graph_snapshot, &measurement_basis)?;
        let measurement_basis = collect_planning_measurement_basis(
            &measurement_basis,
            &preliminary_neighborhood,
            pending_activation
                .borrow()
                .staged_replacement()
                .reconciliation_plan()
                .durable_resize_inputs(),
        );
        let allocation_neighborhood = selected_obligations
            .admit_allocation_neighborhood(graph_snapshot, &measurement_basis)?;
        construct_planning_lane_input(
            pending_activation,
            measurement_basis,
            allocation_neighborhood,
        )
        .map_err(WorthUiPlanningLaneAdmissionDenial::from)
    }

    pub fn project_allocation_preview(
        &self,
        candidate: UiAllocationCandidate,
    ) -> crate::runtime::UiAllocationPreviewCandidate {
        crate::runtime::project_allocation_preview(candidate)
    }
}
