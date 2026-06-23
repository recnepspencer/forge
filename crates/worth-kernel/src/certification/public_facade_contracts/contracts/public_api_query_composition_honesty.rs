#[cfg(test)]
mod tests {
    use worth_kernel::query_adoption::current_kernel_composition_honesty_report;

    #[test]
    fn kernel_composition_honesty_is_lower_crate_evidence_backed() {
        let report = current_kernel_composition_honesty_report()
            .expect("kernel composition honesty report must be Query backed");

        assert_eq!(report.kernel_composition_source_count(), 3);
        assert_eq!(report.lower_crate_receipt_family_count(), 2);
        assert_eq!(report.kernel_workload_receipt_family_count(), 8);
        assert_eq!(report.spatial_workload_support_pin_row_count(), 7);
        assert_eq!(report.representative_workload_evidence_row_count(), 8);
        assert_eq!(report.representative_spatial_receipt_identity_count(), 7);
        assert!(!report.evidence_report_identity().is_empty());
        assert!(!report.digest_participation_identity().is_empty());
        assert_ne!(
            report.evidence_report_identity(),
            report.digest_participation_identity()
        );
    }
}
