#[derive(Debug, PartialEq)]
pub(crate) struct WorthUiMountedAllocationActivationBasis {
    projection: crate::runtime::planning::allocation_planning::WorthUiAllocationPlanningProjection,
    candidate_application_authority:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationLoweringAuthority,
    reconciliation: crate::runtime::WorthUiDurableStateReconciliationPlan,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum WorthUiInitialMountedCatalogPreparationDenial {
    CatalogAlreadyEstablished,
    GraphAuthorityMismatch,
    Neighborhood(crate::graph::UiAllocationNeighborhoodDenial),
    CatalogPlanning(crate::runtime::invalidation_narrowing::UiAllocationActivationCatalogDenial),
    ReceiptCommit(Box<crate::runtime::UiAllocationReceiptCommitOutcome>),
}

impl WorthUiRuntime {
    pub(crate) fn prepare_initial_mounted_catalog_activation(
        &self,
        graph: &crate::graph::UiGraphSnapshot,
        candidate_application_authority: crate::facade::prepared_application_authority::WorthUiPreparedApplicationLoweringAuthority,
        admitted: crate::graph::UiAdmittedAllocationCatalogBasisSet,
    ) -> Result<
        (
            WorthUiMountedAllocationActivationBasis,
            crate::runtime::UiCommittedAllocationActivationAttempt,
        ),
        WorthUiInitialMountedCatalogPreparationDenial,
    > {
        if !self.allocation_receipt_ledger.active_catalog_is_empty() {
            return Err(WorthUiInitialMountedCatalogPreparationDenial::CatalogAlreadyEstablished);
        }
        let frame_epoch = self.active.frame_epoch();
        let artifact_digest = self.active.active_artifact().digest().raw();
        if candidate_application_authority.graph_authority_identity() != graph.authority_identity()
        {
            return Err(WorthUiInitialMountedCatalogPreparationDenial::GraphAuthorityMismatch);
        }
        let projection =
            crate::runtime::planning::allocation_planning::WorthUiAllocationPlanningProjection::seal(
                frame_epoch,
                artifact_digest,
                graph.authority_identity(),
            );
        if admitted.snapshot.authority_identity() != graph.authority_identity() {
            return Err(WorthUiInitialMountedCatalogPreparationDenial::GraphAuthorityMismatch);
        }
        let reconciliation = crate::runtime::replacement::reconciliation::WorthUiInitialMountedReconciliationPlanner::reconcile(
            self.active.active_artifact().artifact(),
            artifact_digest,
        );
        let mut candidates = Vec::with_capacity(admitted.entries.len());
        for (basis, selected) in admitted.entries.into_vec() {
            let preliminary_neighborhood = basis
                .admit_allocation_neighborhood(graph, &selected)
                .map_err(WorthUiInitialMountedCatalogPreparationDenial::Neighborhood)?;
            let basis = crate::runtime::planning::collect_planning_measurement_basis(
                &basis,
                &preliminary_neighborhood,
                reconciliation.durable_resize_inputs(),
            );
            let neighborhood = basis
                .admit_allocation_neighborhood(graph, &selected)
                .map_err(WorthUiInitialMountedCatalogPreparationDenial::Neighborhood)?;
            let mut candidate = crate::runtime::planning::plan_allocation_for_projection(
                &projection,
                &basis,
                &neighborhood,
            );
            candidate.seal_catalog_successor();
            candidates.push(candidate);
        }
        let catalog =
            crate::runtime::invalidation_narrowing::UiAllocationActivationCatalog::from_planning(
                candidates,
                super::UiAllocationCatalogMintAuthority::new(),
            )
            .map_err(WorthUiInitialMountedCatalogPreparationDenial::CatalogPlanning)?;
        let attempt = self
            .allocation_receipt_ledger
            .seal_activation_catalog(catalog, frame_epoch, &reconciliation)
            .map_err(|denial| {
                WorthUiInitialMountedCatalogPreparationDenial::ReceiptCommit(Box::new(denial))
            })?;
        Ok((
            WorthUiMountedAllocationActivationBasis {
                projection,
                candidate_application_authority,
                reconciliation,
            },
            attempt,
        ))
    }
}

impl WorthUiMountedAllocationActivationBasis {
    pub(crate) fn projection(
        &self,
    ) -> &crate::runtime::planning::allocation_planning::WorthUiAllocationPlanningProjection {
        &self.projection
    }

    pub(crate) fn reconciliation(&self) -> &crate::runtime::WorthUiDurableStateReconciliationPlan {
        &self.reconciliation
    }

    pub(crate) fn candidate_application_authority(
        &self,
    ) -> &crate::facade::prepared_application_authority::WorthUiPreparedApplicationLoweringAuthority
    {
        &self.candidate_application_authority
    }
}

use super::runtime_instance::WorthUiRuntime;
