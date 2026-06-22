#[cfg(test)]
mod tests {
    use worth_kernel::query_adoption::current_worth_query_native_hardening_closeout_report;

    #[test]
    fn query_native_hardening_closeout_is_public_and_machine_checked() {
        let report = current_worth_query_native_hardening_closeout_report()
            .expect("query-native hardening closeout report");

        assert!(report.gate_closed());
        assert_eq!(report.audited_source_set_count(), 17);
        assert_eq!(report.admitted_source_set_count(), 9);
        assert_eq!(report.denied_source_set_count(), 5);
        assert_eq!(report.explicit_residue_source_set_count(), 3);
        assert_eq!(report.support_snapshot_row_count(), 66);
        assert_eq!(report.kernel_receipt_breadth_count(), 8);
        assert_eq!(report.topology_read_touched_scope_count(), 4);
        assert_eq!(report.spatial_witness_resolution_request_count(), 8);
    }
}
