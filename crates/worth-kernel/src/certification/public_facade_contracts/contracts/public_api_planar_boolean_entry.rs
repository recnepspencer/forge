#[cfg(test)]
#[path = "public_api_planar_boolean_entry/tests/support.rs"]
mod support;

#[cfg(test)]
#[path = "public_api_planar_boolean_entry/tests/outcomes.rs"]
mod outcomes;

#[cfg(test)]
#[path = "public_api_planar_boolean_entry/tests/phase5_evidence.rs"]
mod phase5_evidence;

#[cfg(test)]
#[path = "public_api_planar_boolean_entry/tests/anti_theatre_fences.rs"]
mod anti_theatre_fences;

#[cfg(test)]
mod tests {
    use std::thread;

    use worth_kernel::workload_composition::{
        PlanarBooleanDeclaration, PlanarBooleanEntryBasis, PlanarBooleanEntryError,
        PlanarBooleanExecutionLane, PlanarBooleanFamily, PlanarBooleanOperandPairIdentity,
        PlanarBooleanOperation, PlanarBooleanSupportPosture,
    };

    use super::support::{
        assert_planar_boolean_query_digest, certified_boolean_readiness_workload_receipt,
    };

    #[test]
    fn planar_boolean_declaration_family_has_explicit_support_rows() {
        run_with_large_stack(|| {
            let readiness = certified_boolean_readiness_workload_receipt("phase1-brep-support");
            let basis = PlanarBooleanEntryBasis::bind(
                readiness.clone(),
                "phase 2 planar boolean basis through Forge Query",
            )
            .expect("basis should certify");
            let declaration = PlanarBooleanDeclaration::new(
                PlanarBooleanFamily::PlanarRegions,
                PlanarBooleanOperation::Union,
                PlanarBooleanOperandPairIdentity::new("mb-planar-pair")
                    .expect("operand identity should certify"),
                PlanarBooleanExecutionLane::BRepNow,
            )
            .from_basis(basis.clone())
            .declared_by_query("phase 1 planar boolean declaration through Forge Query")
            .bind()
            .expect("basis-backed declaration should bind");
            let support = PlanarBooleanDeclaration::new(
                PlanarBooleanFamily::PlanarRegions,
                PlanarBooleanOperation::Union,
                PlanarBooleanOperandPairIdentity::new("mb-planar-pair")
                    .expect("operand identity should certify"),
                PlanarBooleanExecutionLane::BRepNow,
            )
            .from_basis(basis)
            .declared_by_query("phase 1 planar boolean declaration through Forge Query")
            .inspect_support()
            .expect("support row should certify");

            assert_eq!(declaration.family(), PlanarBooleanFamily::PlanarRegions);
            assert_eq!(declaration.operation(), PlanarBooleanOperation::Union);
            assert_eq!(
                declaration.requested_lane(),
                PlanarBooleanExecutionLane::BRepNow
            );
            assert_eq!(
                declaration.readiness_basis_digest(),
                readiness.m7_readiness_receipt().readiness_digest()
            );
            assert_eq!(
                declaration.readiness_workload_digest(),
                readiness.workload_digest()
            );
            assert_eq!(support.posture(), PlanarBooleanSupportPosture::Admitted);
            assert!(support.human_reason().contains("B-rep execution lane"));

            assert_planar_boolean_query_digest(declaration.basis_query_declaration_digest());
            assert_planar_boolean_query_digest(declaration.basis_query_envelope_digest());
            assert_planar_boolean_query_digest(declaration.basis_query_handle_digest());
            assert_planar_boolean_query_digest(declaration.query_declaration_digest());
            assert_planar_boolean_query_digest(declaration.query_envelope_digest());
            assert_planar_boolean_query_digest(declaration.query_handle_digest());
            assert_planar_boolean_query_digest(support.query_support_digest());
            assert_ne!(
                declaration.query_declaration_digest(),
                support.query_support_digest()
            );
        });
    }

    #[test]
    fn planar_boolean_declaration_rejects_blank_query_identity_and_missing_basis() {
        let declaration = PlanarBooleanDeclaration::new(
            PlanarBooleanFamily::PlanarRegions,
            PlanarBooleanOperation::Union,
            PlanarBooleanOperandPairIdentity::new("mb-planar-pair")
                .expect("operand identity should certify"),
            PlanarBooleanExecutionLane::BRepNow,
        );

        let missing_basis = declaration
            .clone()
            .declared_by_query("phase-1 declaration without basis")
            .bind()
            .expect_err("entry basis is required before declaration can bind");
        assert_eq!(missing_basis, PlanarBooleanEntryError::MissingEntryBasis);

        let blank_query = declaration
            .declared_by_query("   ")
            .bind()
            .expect_err("blank Query identity must be rejected");
        assert_eq!(
            blank_query,
            PlanarBooleanEntryError::MissingQueryDeclaration
        );

        let blank_operand = PlanarBooleanOperandPairIdentity::new("   ")
            .expect_err("blank operand identity must fail before Query binding");
        assert_eq!(
            blank_operand,
            PlanarBooleanEntryError::InvalidOperandPairIdentity
        );
    }

    #[test]
    fn ember_lane_is_visible_but_not_admitted_on_the_7_0_boundary() {
        run_with_large_stack(|| {
            let readiness = certified_boolean_readiness_workload_receipt("phase1-ember-support");
            let basis = PlanarBooleanEntryBasis::bind(
                readiness,
                "phase 2 ember visibility basis through Forge Query",
            )
            .expect("basis should certify");
            let brep_support = PlanarBooleanDeclaration::new(
                PlanarBooleanFamily::PlanarRegions,
                PlanarBooleanOperation::Intersect,
                PlanarBooleanOperandPairIdentity::new("mb-planar-pair-brep")
                    .expect("operand identity should certify"),
                PlanarBooleanExecutionLane::BRepNow,
            )
            .from_basis(basis.clone())
            .declared_by_query("phase 1 brep support row")
            .inspect_support()
            .expect("B-rep support should certify");
            let ember_declaration = PlanarBooleanDeclaration::new(
                PlanarBooleanFamily::PlanarRegions,
                PlanarBooleanOperation::Intersect,
                PlanarBooleanOperandPairIdentity::new("mb-planar-pair-ember")
                    .expect("operand identity should certify"),
                PlanarBooleanExecutionLane::EmberFuture,
            )
            .from_basis(basis)
            .declared_by_query("phase 1 ember visibility row");
            let ember_support = ember_declaration
                .inspect_support()
                .expect("EMBER visibility row should still bind through Query");
            let ember_receipt = ember_declaration
                .bind()
                .expect("EMBER declaration should still bind through Query");

            assert_eq!(
                ember_support.posture(),
                PlanarBooleanSupportPosture::VisibleNotAdmitted
            );
            assert!(!ember_support.posture().is_admitted());
            assert_eq!(
                ember_receipt.requested_lane(),
                PlanarBooleanExecutionLane::EmberFuture
            );
            assert!(ember_support.human_reason().contains("not admitted"));
            assert_planar_boolean_query_digest(ember_receipt.query_declaration_digest());
            assert_planar_boolean_query_digest(ember_support.query_support_digest());
            assert_ne!(
                brep_support.query_support_digest(),
                ember_support.query_support_digest()
            );
        });
    }

    #[test]
    fn planar_boolean_declaration_query_identity_tracks_basis_identity() {
        run_with_large_stack(|| {
            let first_basis = PlanarBooleanEntryBasis::bind(
                certified_boolean_readiness_workload_receipt("phase2-declaration-first"),
                "phase 2 declaration basis identity",
            )
            .expect("first basis should certify");
            let second_basis = PlanarBooleanEntryBasis::bind(
                certified_boolean_readiness_workload_receipt("phase2-declaration-second"),
                "phase 2 declaration basis identity",
            )
            .expect("second basis should certify");

            let first = PlanarBooleanDeclaration::new(
                PlanarBooleanFamily::PlanarRegions,
                PlanarBooleanOperation::Subtract,
                PlanarBooleanOperandPairIdentity::new("mb-planar-pair-same")
                    .expect("operand identity should certify"),
                PlanarBooleanExecutionLane::BRepNow,
            )
            .from_basis(first_basis)
            .declared_by_query("phase 2 declaration identity binding")
            .bind()
            .expect("first declaration should bind");
            let second = PlanarBooleanDeclaration::new(
                PlanarBooleanFamily::PlanarRegions,
                PlanarBooleanOperation::Subtract,
                PlanarBooleanOperandPairIdentity::new("mb-planar-pair-same")
                    .expect("operand identity should certify"),
                PlanarBooleanExecutionLane::BRepNow,
            )
            .from_basis(second_basis)
            .declared_by_query("phase 2 declaration identity binding")
            .bind()
            .expect("second declaration should bind");

            assert_ne!(
                first.readiness_basis_digest(),
                second.readiness_basis_digest()
            );
            assert_ne!(
                first.basis_query_declaration_digest(),
                second.basis_query_declaration_digest()
            );
            assert_ne!(
                first.query_declaration_digest(),
                second.query_declaration_digest()
            );
        });
    }

    fn run_with_large_stack(test: impl FnOnce() + Send + 'static) {
        thread::Builder::new()
            .stack_size(16 * 1024 * 1024)
            .spawn(test)
            .expect("spawn large-stack planar boolean test")
            .join()
            .expect("join large-stack planar boolean test");
    }
}
