#[cfg(test)]
mod tests {
    use worth_kernel::query_adoption::{
        current_worth_phase_eight_performance_counter_report, WorthPhaseEightDiagnosticPolicy,
    };

    #[test]
    fn phase_eight_counter_report_is_public_and_exact() {
        let report = current_worth_phase_eight_performance_counter_report(
            WorthPhaseEightDiagnosticPolicy::Minimal,
        )
        .expect("phase eight counter report");

        assert_eq!(report.support_requirement_count(), 9);
        assert_eq!(report.support_observed_row_count(), 3);
        assert_eq!(report.support_matched_required_count(), 9);
        assert_eq!(report.support_snapshot_row_count(), 66);
        assert_eq!(report.support_blocking_finding_count(), 0);
        assert_eq!(report.boundary_audit_source_count(), 6);
        assert_eq!(report.synthetic_denial_localization_row_count(), 5);
        assert_eq!(report.topology_read_touched_scope_count(), 4);
        assert_eq!(report.topology_mutation_lane_touched_scope_count(), 14);
        assert_eq!(report.spatial_witness_resolution_request_count(), 8);
        assert_eq!(report.spatial_witness_denial_count(), 4);
        assert_eq!(report.spatial_witness_catalog_lookup_count(), 2);
        assert_eq!(report.kernel_receipt_breadth_count(), 8);
        assert_eq!(report.kernel_lower_crate_receipt_family_count(), 2);
    }
}
