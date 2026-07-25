#[derive(Debug)]
pub(crate) enum WorthUiInitialMountedAllocationActivationDenial {
    Preparation(crate::runtime::WorthUiInitialMountedCatalogPreparationDenial),
    Freshness(crate::runtime::UiAllocationFreshnessConsumptionDenial),
    Lowering(crate::runtime::planning::WorthUiExecutionPlanLoweringAuthorityDenial),
    Topology(crate::runtime::WorthUiPlanTopologyDenial),
    PlanBundle(crate::runtime::active::WorthUiExecutionPlanBundleDenial),
    ExecutableEquivalence(crate::runtime::WorthUiExecutablePlanEquivalenceDenial),
    QuerySuccession(worth_ui_query_binding::WorthUiQueryBindingSuccessionDenial),
    Attempt(Box<crate::runtime::UiCommittedAllocationActivationDenial>),
}

impl WorthUiInitialMountedAllocationActivationDenial {
    pub(crate) fn into_public_denial(
        self,
    ) -> crate::runtime::WorthUiAllocationCatalogActivationDenial {
        use crate::runtime::WorthUiAllocationCatalogActivationDenial as PublicDenial;

        match self {
            Self::Preparation(denial) => map_initial_preparation_denial(denial),
            Self::Freshness(denial) => PublicDenial::Freshness(denial),
            Self::Lowering(denial) => map_lowering_denial(denial),
            Self::Topology(denial) => PublicDenial::TopologyAssembly(denial),
            Self::PlanBundle(denial) => map_plan_bundle_denial(denial),
            Self::ExecutableEquivalence(denial) => PublicDenial::ExecutableEquivalence(denial),
            Self::QuerySuccession(denial) => PublicDenial::QuerySuccession(denial),
            Self::Attempt(denial) => PublicDenial::Attempt(denial),
        }
    }
}

fn map_initial_preparation_denial(
    denial: crate::runtime::WorthUiInitialMountedCatalogPreparationDenial,
) -> crate::runtime::WorthUiAllocationCatalogActivationDenial {
    use crate::runtime::WorthUiAllocationCatalogActivationDenial as PublicDenial;
    use crate::runtime::WorthUiInitialMountedCatalogPreparationDenial as InternalDenial;

    match denial {
        InternalDenial::CatalogAlreadyEstablished => PublicDenial::InitialCatalogAlreadyEstablished,
        InternalDenial::GraphAuthorityMismatch => PublicDenial::InitialGraphAuthorityMismatch,
        InternalDenial::Neighborhood(denial) => PublicDenial::InitialNeighborhood(denial),
        InternalDenial::CatalogPlanning(denial) => PublicDenial::InitialCatalogPlanning(denial),
        InternalDenial::ReceiptCommit(denial) => PublicDenial::InitialReceiptCommit(denial),
    }
}

fn map_lowering_denial(
    denial: crate::runtime::planning::WorthUiExecutionPlanLoweringAuthorityDenial,
) -> crate::runtime::WorthUiAllocationCatalogActivationDenial {
    use crate::runtime::planning::WorthUiExecutionPlanLoweringAuthorityDenial as InternalDenial;
    use crate::runtime::WorthUiAllocationCatalogActivationDenial as PublicDenial;

    match denial {
        InternalDenial::CandidateGraphAuthorityMismatch => PublicDenial::CandidateGraphAuthority,
        InternalDenial::CandidateArtifactAuthorityMismatch => {
            PublicDenial::CandidateArtifactAuthority
        }
        InternalDenial::ForeignAllocationProjection => PublicDenial::AllocationProjection,
        InternalDenial::MissingQueryPosture => PublicDenial::MissingQueryPosture,
        InternalDenial::UnexpectedQueryPosture => PublicDenial::UnexpectedQueryPosture,
        InternalDenial::QueryDefinitionNotInstalled => PublicDenial::QueryDefinitionNotInstalled,
        InternalDenial::ForeignQueryInstalledAuthority => {
            PublicDenial::ForeignQueryInstalledAuthority
        }
        InternalDenial::RegionalDelta(denial) => match denial {
            crate::runtime::planning::plan_topology::WorthUiPlanRegionDeltaDenial::DuplicateCandidateRegion => {
                PublicDenial::RegionalDeltaDuplicateCandidateRegion
            }
        },
        InternalDenial::PlanInput(denial) => PublicDenial::PlanInput(denial),
    }
}

fn map_plan_bundle_denial(
    denial: crate::runtime::active::WorthUiExecutionPlanBundleDenial,
) -> crate::runtime::WorthUiAllocationCatalogActivationDenial {
    use crate::runtime::active::WorthUiExecutionPlanBundleDenial as InternalDenial;
    use crate::runtime::WorthUiAllocationCatalogActivationDenial as PublicDenial;

    match denial {
        InternalDenial::ForeignLoweringAuthority => PublicDenial::ExecutionPlanAuthorityMismatch,
        InternalDenial::OrdinaryPlan(denial) => PublicDenial::OrdinaryPlan(denial),
        InternalDenial::VirtualizedPlan(denial) => PublicDenial::VirtualizedPlan(denial),
        InternalDenial::CanvasSpatialPlan(denial) => PublicDenial::CanvasSpatialPlan(denial),
        InternalDenial::RealtimeOverlayPlan(denial) => PublicDenial::RealtimeOverlayPlan(denial),
    }
}

impl crate::runtime::WorthUiRuntime {
    pub(crate) fn activate_initial_mounted_allocation_catalog(
        &mut self,
        active_app: &mut crate::facade::WorthUiApp,
        graph_successor: crate::facade::prepared_application_authority::WorthUiPreparedApplicationGraphSuccessor,
        admitted: crate::graph::UiAdmittedAllocationCatalogBasisSet,
        boundary: crate::runtime::WorthUiFrameBoundary,
    ) -> Result<
        crate::runtime::UiCommittedAllocationReplan,
        WorthUiInitialMountedAllocationActivationDenial,
    > {
        let candidate_application_authority = graph_successor.lowering_authority();
        let (basis, attempt) = self
            .prepare_initial_mounted_catalog_activation(
                graph_successor.graph_snapshot(),
                candidate_application_authority.clone(),
                admitted,
            )
            .map_err(WorthUiInitialMountedAllocationActivationDenial::Preparation)?;
        let committed = attempt.committed_outcome().clone();
        let lowering_input = attempt
            .primary_receipt()
            .lowering_input()
            .map_err(WorthUiInitialMountedAllocationActivationDenial::Freshness)?;
        let active_artifact = self.active.active_artifact();
        let lowering =
            crate::runtime::planning::WorthUiExecutionPlanLoweringAuthority::seal_mounted(
                basis,
                lowering_input,
                active_artifact.artifact(),
                active_artifact.digest(),
                self.active.frame_epoch(),
                self.active.active_plan_ref().digest().as_u64(),
            )
            .map_err(WorthUiInitialMountedAllocationActivationDenial::Lowering)?;
        let handles = self.authorize_regional_successor_handles(lowering.facts());
        let (candidate_plan, lane_admission) = self
            .assemble_execution_plan_topology_with_admission(lowering.facts(), &handles)
            .map_err(WorthUiInitialMountedAllocationActivationDenial::Topology)?;
        let candidate_bundle = crate::runtime::active::WorthUiSealedExecutionPlanBundle::seal(
            lowering.facts(),
            candidate_plan,
            &lane_admission,
            self.host_plan_binding,
        )
        .map_err(WorthUiInitialMountedAllocationActivationDenial::PlanBundle)?;
        if let crate::runtime::WorthUiExecutablePlanDecision::Denied(denial) = self
            .active
            .active_plan_ref()
            .classify_candidate(&candidate_bundle)
        {
            return Err(
                WorthUiInitialMountedAllocationActivationDenial::ExecutableEquivalence(denial),
            );
        }
        let query_changes = self
            .active
            .active_plan_ref()
            .query_succession_changes(&candidate_bundle);
        let candidate_query_binding = candidate_application_authority
            .query_binding_plan()
            .prepare_downstream_state();
        let query_succession = candidate_query_binding
            .prepare_regional_succession(&self.query_binding, query_changes)
            .map_err(WorthUiInitialMountedAllocationActivationDenial::QuerySuccession)?;
        let successor_planning_authority =
            std::rc::Rc::clone(&self.retained_allocation_planning_evidence);
        let (basis, _committed_input, plan_input) = lowering.into_mounted_parts();
        let prepared = attempt
            .activate_mounted(
                self,
                super::committed_allocation_attempt::UiCommittedMountedAllocationActivationInput {
                    basis,
                    plan_input: &plan_input,
                    handle_allocation: &handles,
                    candidate_bundle,
                    query_succession,
                    successor_application_authority: candidate_application_authority,
                    successor_planning_authority,
                    application_publication:
                        crate::runtime::WorthUiPreparedApplicationPublication::mounted_graph(
                            graph_successor,
                        ),
                    boundary,
                },
            )
            .map_err(|denial| {
                WorthUiInitialMountedAllocationActivationDenial::Attempt(Box::new(denial))
            })?;
        let publication = prepared.commit_once(self, Some(active_app));
        let (_plan_swap, query_retirement, _derived_index_counters) = publication.into_parts();
        assert!(
            query_retirement.is_empty(),
            "mount-only allocation establishment cannot retire Query live authority"
        );
        Ok(committed)
    }
}
