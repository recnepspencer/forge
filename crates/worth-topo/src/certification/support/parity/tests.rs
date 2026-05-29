#[cfg(test)]
mod parity_tests {
    use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

    use crate::facade::{
        build_derived_equivalence_contract, compare_derived_equivalence_contracts,
        digest_derived_validation_report, digest_interpreted_topology_view,
        digest_materialized_topology_view, interpret_topology_view, validate_interpreted_topology,
        MilestoneOneCertificationHarness, TopologyMaterializer,
    };
    use crate::test_support::primitive_corpus::validated_topology::verified_primitive;
    use crate::validation::reference_integrity::milestone_one_runtime_builder;

    #[test]
    fn derived_equivalence_contract_is_deterministic_for_same_snapshot() {
        let mut runtime = milestone_one_runtime_builder()
            .expect(" milestone one runtime builder")
            .build();
        let verified = verified_primitive(
            &mut runtime,
            "phase-six-parity",
            &MilestoneOnePrimitiveCase::SheetPatch { face_count: 3 },
        )
        .expect("verified primitive");
        let read_view = runtime
            .read_truth()
            .read_snapshot(&verified.persisted_truth().snapshot)
            .expect("snapshot read");
        let materialized =
            TopologyMaterializer::materialize_from_truth(&read_view).expect("materialized");
        let interpreted = interpret_topology_view(&materialized);
        let validation =
            validate_interpreted_topology(&materialized, &interpreted).expect("validation");

        let first = build_derived_equivalence_contract(
            &verified.read_basis(),
            &materialized,
            &interpreted,
            &validation,
        );
        let second = build_derived_equivalence_contract(
            &verified.read_basis(),
            &materialized,
            &interpreted,
            &validation,
        );

        assert_eq!(first, second);
        assert_eq!(
            first.materialized_topology_digest,
            digest_materialized_topology_view(&materialized)
        );
        assert_eq!(
            first.interpreted_topology_digest,
            digest_interpreted_topology_view(&interpreted)
        );
        assert_eq!(
            first.derived_validation_digest,
            digest_derived_validation_report(&validation)
        );
    }

    #[test]
    fn replay_basis_preserves_equivalent_derived_meaning_while_recording_derivation_shift() {
        let mut runtime = milestone_one_runtime_builder()
            .expect(" milestone one runtime builder")
            .build();
        let verified = verified_primitive(
            &mut runtime,
            "phase-six-replay",
            &MilestoneOnePrimitiveCase::WireBranch { branch_count: 4 },
        )
        .expect("verified primitive");
        let read_view = runtime
            .read_truth()
            .read_snapshot(&verified.persisted_truth().snapshot)
            .expect("snapshot read");
        let materialized =
            TopologyMaterializer::materialize_from_truth(&read_view).expect("materialized");
        let interpreted = interpret_topology_view(&materialized);
        let validation =
            validate_interpreted_topology(&materialized, &interpreted).expect("validation");

        let mainline = build_derived_equivalence_contract(
            &verified.read_basis(),
            &materialized,
            &interpreted,
            &validation,
        );
        let replay_basis = verified.read_basis().replay_of();
        let replay = build_derived_equivalence_contract(
            &replay_basis,
            &materialized,
            &interpreted,
            &validation,
        );
        let comparison = compare_derived_equivalence_contracts(&mainline, &replay);

        assert_ne!(mainline.derivation_origin, replay.derivation_origin);
        assert!(comparison.authority_identity_match);
        assert!(comparison.branch_identity_match);
        assert!(comparison.equivalent_derived_meaning);
    }

    #[test]
    fn certification_harness_embeds_phase_six_equivalence_contracts() {
        let mut runtime = milestone_one_runtime_builder()
            .expect(" milestone one runtime builder")
            .build();
        let verified = verified_primitive(
            &mut runtime,
            "phase-six-certification",
            &MilestoneOnePrimitiveCase::SolidShell { face_count: 4 },
        )
        .expect("verified primitive");

        let report =
            MilestoneOneCertificationHarness::certify_verified_commit(&mut runtime, &verified)
                .expect("certification");

        assert!(
            report
                .milestone_1_replay_parity_report
                .equivalence_contract
                .materialized_topology_digest
                .row_count
                > 0
        );
        assert!(report
            .milestone_1_replay_parity_report
            .replay_equivalence_contract
            .is_some());
    }
}




