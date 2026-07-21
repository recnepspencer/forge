use crate::runtime::replacement::file_rust_replacement_parity::WorthUiFileRustReplacementPipelineReportParts;
use crate::runtime::tests::allocation_planning_test_support::admitted_planning_admission;
use crate::runtime::WorthUiRuntime;
use crate::runtime::{
    WorthUiAdmittedReplacementCandidate, WorthUiCandidateAdmission, WorthUiDurableStateFamily,
    WorthUiExecutionLaneSupport, WorthUiFileRustReplacementParityCounters,
    WorthUiFileRustReplacementParityDenial, WorthUiFileRustReplacementParityDenialReason,
    WorthUiFileRustReplacementPipelineReport, WorthUiReplacementCandidate,
    WorthUiRuntimeArtifactComparison,
};

impl WorthUiRuntime {
    pub fn activate_replacement_for_file_rust_parity_report(
        &mut self,
        candidate: WorthUiReplacementCandidate,
    ) -> Result<WorthUiFileRustReplacementPipelineReport, WorthUiFileRustReplacementParityDenial>
    {
        let mut counters = WorthUiFileRustReplacementParityCounters::default();
        let authoring_lane = candidate.authoring_lane();
        counters.record_candidate(authoring_lane);

        counters.record_candidate_admission();
        let admitted =
            WorthUiCandidateAdmission::for_active_basis(self.replacement_admission_basis())
                .admit(candidate)
                .map_err(|_| {
                    denial(
                        WorthUiFileRustReplacementParityDenialReason::CandidateAdmissionDenied,
                        counters,
                    )
                })?;

        counters.record_artifact_comparison();
        let comparison = self.compare_admitted_replacement(&admitted).map_err(|_| {
            denial(
                WorthUiFileRustReplacementParityDenialReason::ArtifactComparisonDenied,
                counters,
            )
        })?;

        self.activate_admitted_replacement_for_file_rust_parity_report(
            admitted, comparison, counters,
        )
    }

    pub(crate) fn activate_admitted_replacement_for_file_rust_parity_report(
        &mut self,
        admitted: WorthUiAdmittedReplacementCandidate,
        comparison: WorthUiRuntimeArtifactComparison,
        mut counters: WorthUiFileRustReplacementParityCounters,
    ) -> Result<WorthUiFileRustReplacementPipelineReport, WorthUiFileRustReplacementParityDenial>
    {
        let authoring_lane = admitted.candidate().authoring_lane();
        let candidate_basis = admitted.candidate().basis();
        let provenance_handle = admitted.candidate().provenance_handle();

        counters.record_impact_classification();
        let impact = self
            .classify_replacement_impact(&comparison, &admitted)
            .map_err(|_| {
                denial(
                    WorthUiFileRustReplacementParityDenialReason::ImpactClassificationDenied,
                    counters,
                )
            })?;

        counters.record_impact_narrowing();
        let narrowing = self
            .narrow_replacement_impact(&impact, &admitted)
            .map_err(|_| {
                denial(
                    WorthUiFileRustReplacementParityDenialReason::ImpactNarrowingDenied,
                    counters,
                )
            })?;

        counters.record_identity_matching();
        let identity = self
            .build_identity_match_graph(&narrowing, &admitted)
            .map_err(|_| {
                denial(
                    WorthUiFileRustReplacementParityDenialReason::IdentityMatchingDenied,
                    counters,
                )
            })?;

        counters.record_node_replacement();
        let node_plan = self
            .classify_node_replacements(&impact, &narrowing, &identity)
            .map_err(|_| {
                denial(
                    WorthUiFileRustReplacementParityDenialReason::NodeReplacementDenied,
                    counters,
                )
            })?;

        let inventory = platform_inventory(self)
            .build_for_replacement(&node_plan)
            .map_err(|_| {
                denial(
                    WorthUiFileRustReplacementParityDenialReason::StateInventoryDenied,
                    counters,
                )
            })?;

        counters.record_durable_state_reconciliation();
        let reconciliation = self
            .reconcile_durable_state(&node_plan, &inventory)
            .map_err(|_| {
                denial(
                    WorthUiFileRustReplacementParityDenialReason::DurableStateReconciliationDenied,
                    counters,
                )
            })?;

        counters.record_query_binding_comparison();
        let query_comparison = self
            .compare_query_bindings(&node_plan, &narrowing, &admitted)
            .map_err(|_| {
                denial(
                    WorthUiFileRustReplacementParityDenialReason::QueryBindingComparisonDenied,
                    counters,
                )
            })?;

        counters.record_query_live_rebind();
        let query_rebind = self
            .plan_query_live_rebinds(&query_comparison, &node_plan, &narrowing, &admitted)
            .map_err(|_| {
                denial(
                    WorthUiFileRustReplacementParityDenialReason::QueryLiveRebindDenied,
                    counters,
                )
            })?;

        counters.record_activation_stage();
        let pending = self
            .stage_replacement_activation(
                admitted,
                &impact,
                &narrowing,
                &node_plan,
                crate::runtime::WorthUiActivationStagingPlans::new(
                    Some(&reconciliation),
                    Some(&query_rebind),
                ),
            )
            .map_err(|_| {
                denial(
                    WorthUiFileRustReplacementParityDenialReason::ActivationStagingDenied,
                    counters,
                )
            })?;

        counters.record_plan_lowering();
        let (measurement_basis, graph_snapshot, selected_obligations) =
            admitted_planning_admission("file-rust-parity.activation", "operator:stack");
        let admitted_catalog = graph_snapshot
            .admit_allocation_catalog_basis_set(vec![(
                measurement_basis.clone(),
                selected_obligations.clone(),
            )])
            .map_err(|_| {
                denial(
                    WorthUiFileRustReplacementParityDenialReason::PlanLoweringDenied,
                    counters,
                )
            })?;
        counters.record_handle_allocation();
        let boundary = self.safe_frame_boundary();
        counters.record_lane_admission();
        counters.record_topology_assembly();
        let mut candidate_plan_digest = 0;
        let mut lane_support_digest = 0;
        let mut plan_node_count = 0;
        counters.record_ready_activation();
        let swap_receipt = self
            .activate_admitted_allocation_catalog_with_boundary_source(
                pending,
                admitted_catalog,
                |runtime, _allocation_receipt, candidate_plan, lowering_facts| {
                    let lane_admission = runtime
                        .admit_execution_lanes(
                            lowering_facts,
                            &WorthUiExecutionLaneSupport::platform_default(),
                        )
                        .map_err(|_| crate::runtime::WorthUiAllocationCatalogActivationDenial::CertificationBoundary("lane admission"))?;
                    candidate_plan_digest =
                        runtime.digest_execution_plan(candidate_plan).raw();
                    lane_support_digest = lane_admission.support_digest();
                    plan_node_count = candidate_plan.region_count();
                    Ok((boundary, None))
                },
            )
            .map_err(|_| {
                denial(
                    WorthUiFileRustReplacementParityDenialReason::ReadyActivationDenied,
                    counters,
                )
            })?;
        counters.record_plan_swap();
        Ok(WorthUiFileRustReplacementPipelineReport::new(
            WorthUiFileRustReplacementPipelineReportParts {
                authoring_lane,
                candidate_basis,
                provenance_handle,
                active_artifact_digest: comparison.active_artifact_digest(),
                candidate_artifact_digest: comparison.candidate_artifact_digest(),
                artifact_comparison_outcome: comparison.outcome(),
                candidate_plan_digest,
                lane_support_digest,
                plan_node_count,
                swap_receipt,
                counters,
            },
        ))
    }
}

fn platform_inventory(
    runtime: &WorthUiRuntime,
) -> crate::runtime::WorthUiDurableStateInventoryBuilder {
    runtime
        .durable_state_inventory()
        .register_platform_family(WorthUiDurableStateFamily::focus_chain())
        .register_platform_family(WorthUiDurableStateFamily::scroll_anchor())
        .register_platform_family(WorthUiDurableStateFamily::selection_range())
        .register_platform_family(WorthUiDurableStateFamily::text_edit_buffer())
        .register_platform_family(WorthUiDurableStateFamily::splitter_position())
        .register_platform_family(WorthUiDurableStateFamily::tab_state())
        .register_platform_family(WorthUiDurableStateFamily::panel_visibility())
}

fn denial(
    reason: WorthUiFileRustReplacementParityDenialReason,
    counters: WorthUiFileRustReplacementParityCounters,
) -> WorthUiFileRustReplacementParityDenial {
    WorthUiFileRustReplacementParityDenial::new(reason, counters)
}
