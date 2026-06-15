#[cfg(test)]
#[path = "public_api_planar_boolean_entry/tests/support.rs"]
mod support;

#[cfg(test)]
mod tests {
    use std::thread;

    use worth_kernel::workload_composition::{
        PlanarBooleanEntryBasis, PlanarBooleanEntryBasisError,
    };

    use super::support::{
        assert_planar_boolean_query_digest, certified_boolean_readiness_workload_receipt,
    };

    #[test]
    fn planar_boolean_entry_basis_accepts_only_real_readiness_receipts() {
        run_with_large_stack(|| {
            let readiness = certified_boolean_readiness_workload_receipt("phase2-basis");
            let basis = PlanarBooleanEntryBasis::bind(
                readiness.clone(),
                "phase 2 planar boolean basis through Forge Query",
            )
            .expect("real readiness receipt should bind basis");

            assert_eq!(
                basis.readiness_receipt_identity(),
                readiness.m7_readiness_receipt().readiness_digest()
            );
            assert_eq!(
                basis.readiness_declaration_digest(),
                readiness.m7_readiness_receipt().declaration_digest()
            );
            assert_eq!(
                basis.readiness_envelope_digest(),
                readiness.m7_readiness_receipt().envelope_digest()
            );
            assert_eq!(
                basis.readiness_workload_digest(),
                readiness.workload_digest()
            );
            assert_eq!(
                basis.stage_coverage().coverage_digest(),
                readiness.stage_coverage().coverage_digest()
            );
            assert!(basis.stage_coverage().covers_all_required_stages());
            assert_eq!(basis.blocker_family(), None);
            assert_eq!(basis.denial_identity(), None);
            assert_planar_boolean_query_digest(basis.query_declaration_digest());
            assert_planar_boolean_query_digest(basis.query_envelope_digest());
            assert_planar_boolean_query_digest(basis.query_handle_digest());
        });
    }

    #[test]
    fn boolean_entry_basis_rejects_blank_query_identity() {
        run_with_large_stack(|| {
            let readiness = certified_boolean_readiness_workload_receipt("phase2-blank-query");
            let error = PlanarBooleanEntryBasis::bind(readiness, "   ")
                .expect_err("blank query identity must be rejected");

            assert_eq!(error, PlanarBooleanEntryBasisError::MissingQueryDeclaration);
        });
    }

    #[test]
    fn planar_boolean_entry_basis_preserves_required_stage_identity() {
        run_with_large_stack(|| {
            let readiness = certified_boolean_readiness_workload_receipt("phase2-stage-coverage");
            let basis = PlanarBooleanEntryBasis::bind(
                readiness.clone(),
                "phase 2 required stage identity basis",
            )
            .expect("basis should certify");

            assert_eq!(
                basis.stage_coverage().stages(),
                readiness.stage_coverage().stages()
            );
            assert_eq!(basis.stage_coverage().stages().len(), 10);
        });
    }

    #[test]
    fn planar_boolean_entry_basis_query_identity_tracks_readiness_identity() {
        run_with_large_stack(|| {
            let first = certified_boolean_readiness_workload_receipt("phase2-basis-first");
            let second = certified_boolean_readiness_workload_receipt("phase2-basis-second");
            let first_basis =
                PlanarBooleanEntryBasis::bind(first, "phase 2 stable basis query identity")
                    .expect("first basis should certify");
            let second_basis =
                PlanarBooleanEntryBasis::bind(second, "phase 2 stable basis query identity")
                    .expect("second basis should certify");

            assert_ne!(
                first_basis.readiness_receipt_identity(),
                second_basis.readiness_receipt_identity()
            );
            assert_ne!(
                first_basis.query_declaration_digest(),
                second_basis.query_declaration_digest()
            );
            assert_ne!(
                first_basis.query_envelope_digest(),
                second_basis.query_envelope_digest()
            );
        });
    }

    fn run_with_large_stack(test: impl FnOnce() + Send + 'static) {
        thread::Builder::new()
            .stack_size(16 * 1024 * 1024)
            .spawn(test)
            .expect("spawn large-stack planar boolean basis test")
            .join()
            .expect("join large-stack planar boolean basis test");
    }
}
