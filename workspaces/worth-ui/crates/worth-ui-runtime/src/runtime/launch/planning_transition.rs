use std::borrow::Borrow;

use crate::graph::UiGraphSnapshot;
use crate::obligations::selection::UiSelectedObligationSet;
use crate::runtime::execution::handle_allocation::WorthUiRuntimeHandleAllocator;
use crate::runtime::planning::{
    collect_planning_measurement_basis, construct_planning_lane_input,
    plan_allocation_for_pending_activation, WorthUiPlanningLaneAdmissionDenial,
    WorthUiPlanningLaneInput,
};
#[cfg(test)]
use crate::runtime::WorthUiRuntimeHandleAllocationDenial;
use crate::runtime::{
    UiAllocationCandidate, WorthUiPendingActivation, WorthUiRuntimeHandleAllocation,
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
    PlanningLane(WorthUiPlanningLaneAdmissionDenial),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum UiAllocationCatalogPreparationDenial {
    PlanningAdmission(UiAllocationPlanningCatalogAdmissionDenial),
    CatalogPlanning(crate::runtime::invalidation_narrowing::UiAllocationActivationCatalogDenial),
    ReceiptCommit(Box<crate::runtime::UiAllocationReceiptCommitOutcome>),
}

impl UiAllocationCatalogMintAuthority {
    fn new() -> Self {
        Self(())
    }
}

impl WorthUiRuntime {
    #[cfg(any(test, feature = "certification-support"))]
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
        .map_err(|denial| UiAllocationCatalogPreparationDenial::ReceiptCommit(Box::new(denial)))
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn admit_allocation_planning_catalog<'pending>(
        &self,
        pending: &'pending WorthUiPendingActivation,
        admitted: crate::graph::UiAdmittedAllocationCatalogBasisSet,
    ) -> Result<
        UiAllocationPlanningCatalogInput<'pending>,
        UiAllocationPlanningCatalogAdmissionDenial,
    > {
        self.admit_allocation_planning_entries(
            pending,
            admitted.snapshot,
            admitted.entries.into_vec(),
        )
    }

    fn admit_allocation_planning_entries<'pending>(
        &self,
        pending: &'pending WorthUiPendingActivation,
        snapshot: crate::graph::UiGraphSnapshot,
        entries: Vec<(
            crate::evidence::UiMeasurementBasis,
            crate::obligations::selection::UiSelectedObligationSet,
        )>,
    ) -> Result<
        UiAllocationPlanningCatalogInput<'pending>,
        UiAllocationPlanningCatalogAdmissionDenial,
    > {
        admit_pending_activation_planning_freshness(
            self.active.frame_epoch(),
            pending.frame_epoch(),
        )
        .map_err(UiAllocationPlanningCatalogAdmissionDenial::PlanningLane)?;
        let mut inputs = Vec::with_capacity(entries.len());
        for (basis, selected) in entries {
            let preliminary = basis
                .admit_allocation_neighborhood(&snapshot, &selected)
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
                .admit_allocation_neighborhood(&snapshot, &selected)
                .map_err(UiAllocationPlanningCatalogAdmissionDenial::Neighborhood)?;
            inputs.push(
                construct_planning_lane_input(pending, basis, neighborhood)
                    .map_err(WorthUiPlanningLaneAdmissionDenial::from)
                    .map_err(UiAllocationPlanningCatalogAdmissionDenial::PlanningLane)?,
            );
        }
        Ok(UiAllocationPlanningCatalogInput {
            inputs: inputs.into_boxed_slice(),
        })
    }

    pub(crate) fn prepare_allocation_catalog_delta_activation(
        &self,
        pending: &WorthUiPendingActivation,
        closure: crate::runtime::allocation_catalog_successor::UiAllocationCatalogDeltaClosure,
    ) -> Result<
        (
            crate::runtime::UiCommittedAllocationActivationAttempt,
            crate::runtime::UiAllocationCatalogSuccessorReceipt,
        ),
        UiAllocationCatalogPreparationDenial,
    > {
        let crate::runtime::allocation_catalog_successor::UiAllocationCatalogDeltaClosure {
            delta,
            affected_predecessor_scopes,
            counters,
            receipt,
        } = closure;
        let input = self
            .admit_allocation_planning_entries(pending, delta.snapshot, delta.changed.into_vec())
            .map_err(UiAllocationCatalogPreparationDenial::PlanningAdmission)?;
        let removal_only = input.inputs.is_empty();
        let catalog = if removal_only {
            crate::runtime::invalidation_narrowing::UiAllocationActivationCatalog::empty_successor(
                UiAllocationCatalogMintAuthority::new(),
            )
        } else {
            self.plan_allocation_catalog(input)
                .map_err(UiAllocationCatalogPreparationDenial::CatalogPlanning)?
        };
        let mut attempt = if removal_only {
            self.allocation_receipt_ledger
                .seal_removal_only_catalog_activation(
                    catalog,
                    pending.frame_epoch(),
                    pending.staged_replacement().reconciliation_plan(),
                    &affected_predecessor_scopes,
                )
        } else {
            self.seal_allocation_catalog_activation(
                catalog,
                pending.frame_epoch(),
                pending.staged_replacement().reconciliation_plan(),
            )
        }
        .map_err(|denial| UiAllocationCatalogPreparationDenial::ReceiptCommit(Box::new(denial)))?;
        attempt.apply_catalog_successor_delta(&affected_predecessor_scopes);
        debug_assert_eq!(receipt.counters(), counters);
        Ok((attempt, receipt))
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

    #[cfg(test)]
    pub(crate) fn allocate_runtime_handles(
        &self,
        authority: &crate::runtime::planning::WorthUiExecutionPlanLoweringFacts,
    ) -> Result<WorthUiRuntimeHandleAllocation, WorthUiRuntimeHandleAllocationDenial> {
        WorthUiRuntimeHandleAllocator::allocate(authority, self.active.handle_arena_identity())
    }

    pub(crate) fn authorize_regional_successor_handles(
        &self,
        authority: &crate::runtime::planning::WorthUiExecutionPlanLoweringFacts,
    ) -> WorthUiRuntimeHandleAllocation {
        WorthUiRuntimeHandleAllocator::authorize_regional_successor(
            authority,
            self.active.handle_arena_identity(),
        )
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
        admit_pending_activation_planning_freshness(
            self.active.frame_epoch(),
            pending_activation.borrow().frame_epoch(),
        )?;
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

fn admit_pending_activation_planning_freshness(
    active_frame_epoch: crate::runtime::WorthUiRuntimeFrameEpoch,
    pending_frame_epoch: crate::runtime::WorthUiRuntimeFrameEpoch,
) -> Result<(), WorthUiPlanningLaneAdmissionDenial> {
    if active_frame_epoch != pending_frame_epoch {
        return Err(WorthUiPlanningLaneAdmissionDenial::StalePendingActivation {
            active_frame_epoch,
            pending_frame_epoch,
        });
    }
    Ok(())
}
