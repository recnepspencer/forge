#[cfg(test)]
mod tests {
    use forge_query::facade::consumer_kit::{
        hard_prohibition_compile_fail_fixtures, hard_prohibition_registry, ForgeQueryProhibitedSeam,
    };
    use worth_kernel::query_adoption::{
        WorthQuerySyntheticProofDisposition, WorthQuerySyntheticProofDispositionReport,
    };

    #[test]
    fn synthetic_proof_disposition_report_covers_every_inventoried_shortcut() {
        let report = WorthQuerySyntheticProofDispositionReport::current()
            .expect("synthetic proof disposition report");

        assert_eq!(report.rows().len(), 13);
        assert_eq!(
            report
                .rows_for(WorthQuerySyntheticProofDisposition::ReplacedByProductionSurface)
                .count(),
            5
        );
        assert_eq!(
            report
                .rows_for(WorthQuerySyntheticProofDisposition::DeniedByBoundary)
                .count(),
            5
        );
        assert_eq!(
            report
                .rows_for(WorthQuerySyntheticProofDisposition::ExplicitResidue)
                .count(),
            3
        );
        assert!(report
            .rows()
            .iter()
            .all(|row| !row.proof_surface().is_empty()));
        assert!(report
            .rows_for(WorthQuerySyntheticProofDisposition::ExplicitResidue)
            .all(|row| row.proof_surface().ends_with("/query_adoption/residue.rs")));
    }

    #[test]
    fn production_synthetic_paths_are_replaced_not_silently_residue() {
        let report = WorthQuerySyntheticProofDispositionReport::current()
            .expect("synthetic proof disposition report");

        for source_set in [
            "crates/worth-kernel/src/workload_composition",
            "crates/worth-kernel/src/workload_composition/workload_catalog",
            "crates/worth-spatial/src/workload_platform",
            "crates/worth-spatial/src/witness_resolution",
            "crates/worth-topo/src/workload_platform",
        ] {
            let row = report
                .require_source_set(source_set)
                .unwrap_or_else(|| panic!("missing synthetic disposition for {source_set}"));
            assert_eq!(
                row.disposition(),
                WorthQuerySyntheticProofDisposition::ReplacedByProductionSurface
            );
        }
    }

    #[test]
    fn query_hard_prohibition_compile_fail_manifest_covers_workspace_shortcuts() {
        let registry = hard_prohibition_registry();
        let fixtures = hard_prohibition_compile_fail_fixtures();

        for seam in [
            ForgeQueryProhibitedSeam::WorkspaceDirectWrite,
            ForgeQueryProhibitedSeam::WorkspaceDirectBatch,
            ForgeQueryProhibitedSeam::WorkspaceExistingTruthUpdate,
            ForgeQueryProhibitedSeam::WorkspaceExistingTruthDelete,
        ] {
            assert!(
                registry.contains_seam(seam),
                "Query registry must own hard prohibition seam {}",
                seam.key()
            );
            assert!(
                fixtures.iter().any(|fixture| fixture.seam() == seam),
                "Query compile-fail manifest must prove seam {}",
                seam.key()
            );
        }
    }
}
