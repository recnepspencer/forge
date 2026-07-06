use crate::runtime::file_rust_replacement_parity::WorthUiFileRustReplacementPipelineReportParts;
use crate::runtime::host::WorthUiRuntimeHost;
use crate::runtime::runtime_test_modules::allocation_planning_test_support::{
    admitted_allocation_neighborhood, admitted_measurement_basis,
};
use crate::runtime::{
    WorthUiAdmittedReplacementCandidate, WorthUiCandidateAdmission, WorthUiDurableStateFamily,
    WorthUiExecutionLaneSupport, WorthUiFileRustReplacementParityCounters,
    WorthUiFileRustReplacementParityDenial, WorthUiFileRustReplacementParityDenialReason,
    WorthUiFileRustReplacementPipelineReport, WorthUiReplacementCandidate,
    WorthUiRuntimeArtifactComparison,
};

impl WorthUiRuntimeHost {
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

        let lowering_input = self.prepare_pending_execution_plan_lowering_input(
            &node_plan,
            &reconciliation,
            &query_rebind,
        );

        counters.record_activation_stage();
        let pending = self
            .stage_replacement_activation(
                admitted,
                &impact,
                &narrowing,
                &node_plan,
                Some(&reconciliation),
                Some(&query_rebind),
                Some(&lowering_input),
            )
            .map_err(|_| {
                denial(
                    WorthUiFileRustReplacementParityDenialReason::ActivationStagingDenied,
                    counters,
                )
            })?;

        counters.record_plan_lowering();
        let measurement_basis = admitted_measurement_basis("file-rust-parity.activation");
        let allocation_neighborhood =
            admitted_allocation_neighborhood("file-rust-parity.activation");
        let allocation_planning =
            self.plan_allocation(&pending, &measurement_basis, &allocation_neighborhood);
        let plan_input = allocation_planning.lowered_input().ok_or_else(|| {
            denial(
                WorthUiFileRustReplacementParityDenialReason::PlanLoweringDenied,
                counters,
            )
        })?;

        counters.record_handle_allocation();
        let handles = self
            .allocate_runtime_handles(&allocation_planning)
            .map_err(|_| {
                denial(
                    WorthUiFileRustReplacementParityDenialReason::HandleAllocationDenied,
                    counters,
                )
            })?;

        counters.record_lane_admission();
        let lane_admission = self
            .admit_execution_lanes(
                &allocation_planning,
                &WorthUiExecutionLaneSupport::platform_default(),
            )
            .map_err(|_| {
                denial(
                    WorthUiFileRustReplacementParityDenialReason::LaneAdmissionDenied,
                    counters,
                )
            })?;

        counters.record_topology_assembly();
        let candidate_plan = self
            .assemble_execution_plan_topology_with_lane_admission(
                &allocation_planning,
                &handles,
                &lane_admission,
            )
            .map_err(|_| {
                denial(
                    WorthUiFileRustReplacementParityDenialReason::TopologyAssemblyDenied,
                    counters,
                )
            })?;
        let candidate_plan_digest = self.digest_execution_plan(&candidate_plan).raw();
        let plan_node_count = candidate_plan.topology().traversal_order().len();

        counters.record_ready_activation();
        let ready = self
            .prepare_ready_activation(pending, &plan_input, &handles, &candidate_plan, None)
            .map_err(|_| {
                denial(
                    WorthUiFileRustReplacementParityDenialReason::ReadyActivationDenied,
                    counters,
                )
            })?;
        let boundary = self.safe_frame_boundary();

        counters.record_plan_swap();
        let swap_receipt = self
            .swap_ready_activation_at_frame_boundary(ready, candidate_plan, boundary)
            .map_err(|_| {
                denial(
                    WorthUiFileRustReplacementParityDenialReason::PlanSwapDenied,
                    counters,
                )
            })?;
        let swap_counters = swap_receipt.counters();
        counters.record_swap_forbidden_work(
            swap_counters.source_reparse_count(),
            swap_counters.registry_rebuild_count(),
        );

        Ok(WorthUiFileRustReplacementPipelineReport::new(
            WorthUiFileRustReplacementPipelineReportParts {
                authoring_lane,
                candidate_basis,
                provenance_handle,
                active_artifact_digest: comparison.active_artifact_digest(),
                candidate_artifact_digest: comparison.candidate_artifact_digest(),
                artifact_comparison_outcome: comparison.outcome(),
                candidate_plan_digest,
                lane_support_digest: lane_admission.support_digest(),
                plan_node_count,
                swap_receipt,
                counters,
            },
        ))
    }
}

fn platform_inventory(
    runtime: &WorthUiRuntimeHost,
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
