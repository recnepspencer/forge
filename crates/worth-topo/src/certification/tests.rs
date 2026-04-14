#[cfg(test)]
mod certification_tests {
    use forge_relational::facade::runtime::RelationalRuntimeApi;
    use worth_schema::facade::{seed_minimal_topology, worth_bootstrap_schema_registry};
    use worth_schema::facade::{WorthShellInterpretationClass, WorthWireInterpretationClass};

    use crate::certification::certify_milestone_one_read_view;

    #[test]
    fn seeded_bootstrap_earns_milestone_one_certification_report() {
        let mut runtime = RelationalRuntimeApi::builder()
            .schema_registry(
                worth_bootstrap_schema_registry().expect("worth bootstrap schema registry"),
            )
            .build();

        let seeded = seed_minimal_topology(&mut runtime, "cert-harness")
            .expect("seed worth topology");
        let read_view = runtime
            .read_truth()
            .read_snapshot(&seeded.snapshot)
            .expect("worth snapshot read");

        let report =
            certify_milestone_one_read_view(&read_view, seeded.read_basis.clone())
                .expect("milestone one certification should succeed");

        assert!(report.named_truth_validated);
        assert!(report.topology_validated);
        assert_eq!(report.topology_truth_digest.algorithm, "fnv1a64");
        assert!(report.topology_truth_digest.row_count > 0);
        assert_eq!(report.naming_truth_digest.algorithm, "fnv1a64");
        assert!(report.naming_attachment_report.fully_named);
        assert_eq!(
            report.branch_local_topology_report.mutation_origin,
            worth_schema::facade::WorthMutationOrigin::Seed
        );
        assert!(!report.branch_local_topology_report.branch_local);
        assert_eq!(report.milestone_1_replay_parity_report.parity_status, "direct-origin");
        assert_eq!(report.read_artifact.snapshot, seeded.snapshot);
        assert_eq!(report.read_artifact.interpretations.wires.len(), 1);
        assert_eq!(report.read_artifact.interpretations.shells.len(), 1);
        assert_eq!(
            report.read_artifact.interpretations.wires[0].class,
            WorthWireInterpretationClass::OpenChain
        );
        assert_eq!(
            report.read_artifact.interpretations.shells[0].class,
            WorthShellInterpretationClass::OpenSheet
        );
        assert_eq!(
            report.certified_interpretation.interpretations,
            report.read_artifact.interpretations
        );
        assert!(
            report
                .primitive_family_coverage_matrix
                .entries
                .iter()
                .any(|entry| entry.family == "WireOpen(n)" && entry.observed)
        );
    }
}
