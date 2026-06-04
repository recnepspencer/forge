mod accessors;
mod canonical_basis;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BridgeSubscriptionCertificationCounterSnapshot {
    bundle_assembly_plan_count: usize,
    bundle_cost_profile_count: usize,
    certification_bundle_count: usize,
    source_artifact_index_entry_count: usize,
    source_artifact_index_scan_count: usize,
    global_history_scan_count: usize,
    global_subscription_scan_count: usize,
    dense_rebuild_count: usize,
    over_budget_rejection_count: usize,
    scratch_allocation_count: usize,
    scratch_reuse_count: usize,
    comparison_plan_count: usize,
    bundle_comparison_count: usize,
    bundle_comparison_mismatch_count: usize,
    failure_localization_count: usize,
    offline_audit_bundle_index_count: usize,
    offline_audit_plan_count: usize,
    offline_audit_report_count: usize,
    offline_audit_bundle_count: usize,
    offline_audit_comparison_report_count: usize,
    host_log_dependency_count: usize,
    live_state_dependency_count: usize,
    reference_workload_lane_count: usize,
    reference_workload_report_count: usize,
    reference_workload_coverage_report_count: usize,
    cost_posture_report_count: usize,
    schema_parity_report_count: usize,
    multi_failure_precedence_report_count: usize,
    ordering_hostility_report_count: usize,
    stale_checkpoint_report_count: usize,
    bundle_insufficiency_report_count: usize,
    historical_basis_report_count: usize,
    strategy_lowering_report_count: usize,
    fanout_report_count: usize,
    denied_continuation_report_count: usize,
}

impl BridgeSubscriptionCertificationCounterSnapshot {
    pub(crate) fn combine(snapshots: impl IntoIterator<Item = Self>) -> Self {
        let mut combined = Self::default();
        for snapshot in snapshots {
            combined.bundle_assembly_plan_count += snapshot.bundle_assembly_plan_count;
            combined.bundle_cost_profile_count += snapshot.bundle_cost_profile_count;
            combined.certification_bundle_count += snapshot.certification_bundle_count;
            combined.source_artifact_index_entry_count +=
                snapshot.source_artifact_index_entry_count;
            combined.source_artifact_index_scan_count += snapshot.source_artifact_index_scan_count;
            combined.global_history_scan_count += snapshot.global_history_scan_count;
            combined.global_subscription_scan_count += snapshot.global_subscription_scan_count;
            combined.dense_rebuild_count += snapshot.dense_rebuild_count;
            combined.over_budget_rejection_count += snapshot.over_budget_rejection_count;
            combined.scratch_allocation_count += snapshot.scratch_allocation_count;
            combined.scratch_reuse_count += snapshot.scratch_reuse_count;
            combined.comparison_plan_count += snapshot.comparison_plan_count;
            combined.bundle_comparison_count += snapshot.bundle_comparison_count;
            combined.bundle_comparison_mismatch_count += snapshot.bundle_comparison_mismatch_count;
            combined.failure_localization_count += snapshot.failure_localization_count;
            combined.offline_audit_bundle_index_count += snapshot.offline_audit_bundle_index_count;
            combined.offline_audit_plan_count += snapshot.offline_audit_plan_count;
            combined.offline_audit_report_count += snapshot.offline_audit_report_count;
            combined.offline_audit_bundle_count += snapshot.offline_audit_bundle_count;
            combined.offline_audit_comparison_report_count +=
                snapshot.offline_audit_comparison_report_count;
            combined.host_log_dependency_count += snapshot.host_log_dependency_count;
            combined.live_state_dependency_count += snapshot.live_state_dependency_count;
            combined.reference_workload_lane_count += snapshot.reference_workload_lane_count;
            combined.reference_workload_report_count += snapshot.reference_workload_report_count;
            combined.reference_workload_coverage_report_count +=
                snapshot.reference_workload_coverage_report_count;
            combined.cost_posture_report_count += snapshot.cost_posture_report_count;
            combined.schema_parity_report_count += snapshot.schema_parity_report_count;
            combined.multi_failure_precedence_report_count +=
                snapshot.multi_failure_precedence_report_count;
            combined.ordering_hostility_report_count += snapshot.ordering_hostility_report_count;
            combined.stale_checkpoint_report_count += snapshot.stale_checkpoint_report_count;
            combined.bundle_insufficiency_report_count +=
                snapshot.bundle_insufficiency_report_count;
            combined.historical_basis_report_count += snapshot.historical_basis_report_count;
            combined.strategy_lowering_report_count += snapshot.strategy_lowering_report_count;
            combined.fanout_report_count += snapshot.fanout_report_count;
            combined.denied_continuation_report_count += snapshot.denied_continuation_report_count;
        }
        combined
    }

    pub(crate) fn from_source_artifact_index(entry_count: usize, scan_count: usize) -> Self {
        Self {
            source_artifact_index_entry_count: entry_count,
            source_artifact_index_scan_count: scan_count,
            ..Self::default()
        }
    }

    pub(crate) fn from_cost_profile(dense_rebuild: bool) -> Self {
        Self {
            bundle_cost_profile_count: 1,
            dense_rebuild_count: usize::from(dense_rebuild),
            ..Self::default()
        }
    }

    pub(crate) fn from_cost_profile_rejection(over_budget_rejection: bool) -> Self {
        Self {
            over_budget_rejection_count: usize::from(over_budget_rejection),
            ..Self::default()
        }
    }

    pub(crate) fn from_assembly_plan() -> Self {
        Self {
            bundle_assembly_plan_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn from_scratch(allocation_count: usize) -> Self {
        Self {
            scratch_allocation_count: allocation_count,
            scratch_reuse_count: usize::from(allocation_count > 0),
            ..Self::default()
        }
    }

    pub(crate) fn from_scratch_reuse() -> Self {
        Self {
            scratch_reuse_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn from_bundle() -> Self {
        Self {
            certification_bundle_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn from_comparison_plan() -> Self {
        Self {
            comparison_plan_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn from_bundle_comparison(mismatch_count: usize, localized_failure: bool) -> Self {
        Self {
            bundle_comparison_count: 1,
            bundle_comparison_mismatch_count: mismatch_count,
            failure_localization_count: usize::from(localized_failure),
            ..Self::default()
        }
    }

    pub(crate) fn from_offline_audit_bundle_index(bundle_count: usize) -> Self {
        Self {
            offline_audit_bundle_index_count: 1,
            offline_audit_bundle_count: bundle_count,
            ..Self::default()
        }
    }

    pub(crate) fn from_offline_audit_plan(comparison_report_count: usize) -> Self {
        Self {
            offline_audit_plan_count: 1,
            offline_audit_comparison_report_count: comparison_report_count,
            ..Self::default()
        }
    }

    pub(crate) fn from_offline_audit_rejection(
        host_log_dependency: bool,
        live_state_dependency: bool,
    ) -> Self {
        Self {
            host_log_dependency_count: usize::from(host_log_dependency),
            live_state_dependency_count: usize::from(live_state_dependency),
            ..Self::default()
        }
    }

    pub(crate) fn from_offline_audit_report() -> Self {
        Self {
            offline_audit_report_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn from_reference_workload(lane_count: usize) -> Self {
        Self {
            reference_workload_lane_count: lane_count,
            reference_workload_report_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn from_reference_workload_coverage_report() -> Self {
        Self {
            reference_workload_coverage_report_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn from_cost_posture_report() -> Self {
        Self {
            cost_posture_report_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn from_schema_parity_report() -> Self {
        Self {
            schema_parity_report_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn from_multi_failure_precedence_report() -> Self {
        Self {
            multi_failure_precedence_report_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn from_ordering_hostility_report() -> Self {
        Self {
            ordering_hostility_report_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn from_stale_checkpoint_report() -> Self {
        Self {
            stale_checkpoint_report_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn from_bundle_insufficiency_report() -> Self {
        Self {
            bundle_insufficiency_report_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn from_historical_basis_report() -> Self {
        Self {
            historical_basis_report_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn from_strategy_lowering_report() -> Self {
        Self {
            strategy_lowering_report_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn from_fanout_report() -> Self {
        Self {
            fanout_report_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn from_denied_continuation_report() -> Self {
        Self {
            denied_continuation_report_count: 1,
            ..Self::default()
        }
    }
}
