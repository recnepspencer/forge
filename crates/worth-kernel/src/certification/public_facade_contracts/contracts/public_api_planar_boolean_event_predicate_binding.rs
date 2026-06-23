use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanEventClassifierInput, PlanarBooleanEventPredicateBinding,
    PlanarBooleanEventPredicateBindingDenialKind,
};

#[path = "public_api_planar_boolean_event_predicate_binding_support.rs"]
mod predicate_binding_support;
#[path = "public_api_planar_boolean_common_plane_reduced_operand_pair_support.rs"]
mod reduced_pair_support;

#[test]
fn boolean_event_pipeline_consumes_candidate_index_product_not_local_work_items() {
    reduced_pair_support::run_with_large_stack(|| {
        let subject = predicate_binding_support::binding_subject(
            "phase7.2 predicate binding preserves identities",
        );
        let binding_plan = PlanarBooleanEventPredicateBinding::plan(&subject.pair_worklist)
            .for_reduced_pair(subject.reduced_pair_identity.clone())
            .with_segment_segment_receipts(subject.segment_receipts.clone())
            .with_predicate_consumption_receipt(subject.predicate_consumption.clone())
            .compile()
            .expect("predicate binding plan should compile before certification");

        assert_eq!(
            binding_plan.required_segment_contracts(),
            subject.pair_worklist.candidate_rows().len()
        );
        assert_eq!(
            binding_plan.required_predicate_receipts(),
            subject.pair_worklist.candidate_rows().len() * 4
        );

        let binding = binding_plan
            .certify()
            .expect("predicate binding should certify from real worklist and predicate receipts");

        assert_eq!(
            binding.reduced_pair_identity(),
            subject.reduced_pair_identity
        );
        assert_eq!(
            binding.segment_pair_enumeration_identity(),
            subject.pair_worklist.segment_pair_enumeration_identity()
        );
        assert_eq!(
            binding.counters().required_segment_contracts(),
            subject.pair_worklist.candidate_rows().len()
        );
        assert_eq!(
            binding.counters().bound_segment_pairs(),
            subject.pair_worklist.candidate_rows().len()
        );
        assert_eq!(
            binding.counters().required_predicate_rows(),
            subject.pair_worklist.candidate_rows().len() * 4
        );

        for candidate_row in subject.pair_worklist.candidate_rows() {
            let bound_pair = binding
                .bound_pair(candidate_row.candidate_identity())
                .expect("every candidate row must have a predicate-bound pair");
            assert_eq!(
                bound_pair.left_segment_identity(),
                candidate_row.left().canonical_segment_identity()
            );
            assert_eq!(
                bound_pair.right_segment_identity(),
                candidate_row.right().canonical_segment_identity()
            );
            assert_eq!(
                bound_pair.local_frame_identity(),
                candidate_row.local_frame_identity()
            );
            assert_eq!(
                bound_pair.precision_basis_identity(),
                candidate_row.precision_basis_identity()
            );
            assert_eq!(
                bound_pair.predicate_consumption_fact_digest(),
                subject.predicate_consumption.fact_digest()
            );
            let classifier_input =
                PlanarBooleanEventClassifierInput::from_predicate_bound_pair(bound_pair);
            assert_eq!(
                classifier_input.segment_pair_identity(),
                candidate_row.candidate_identity()
            );
            assert_eq!(
                classifier_input.predicate_binding_identity(),
                binding.predicate_binding_identity()
            );
            assert_eq!(
                classifier_input.predicate_bound_pair_identity(),
                bound_pair.bound_pair_identity()
            );
        }
    });
}

#[test]
fn event_predicate_binding_rejects_mismatched_segment_segment_contracts() {
    reduced_pair_support::run_with_large_stack(|| {
        let mut subject = predicate_binding_support::binding_subject(
            "phase7.2 predicate binding rejects mismatched segment",
        );
        subject.segment_receipts.pop();

        let denial = PlanarBooleanEventPredicateBinding::plan(&subject.pair_worklist)
            .for_reduced_pair(subject.reduced_pair_identity)
            .with_segment_segment_receipts(subject.segment_receipts)
            .with_predicate_consumption_receipt(subject.predicate_consumption)
            .compile()
            .expect("count mismatch is caught when predicate consumption and segment sets bind")
            .certify()
            .expect_err("missing segment-segment receipt must deny before binding");

        assert_eq!(
            denial.kind(),
            PlanarBooleanEventPredicateBindingDenialKind::PredicateConsumptionSegmentSetMismatch
        );
    });
}

#[test]
fn event_predicate_binding_rejects_certified_segment_contract_with_wrong_frame() {
    reduced_pair_support::run_with_large_stack(|| {
        let subject =
            predicate_binding_support::binding_subject_with_segment_contract_frame_mismatch(
                "phase7.2 predicate binding rejects certified wrong frame",
            );

        let denial = PlanarBooleanEventPredicateBinding::plan(&subject.pair_worklist)
            .for_reduced_pair(subject.reduced_pair_identity)
            .with_segment_segment_receipts(subject.segment_receipts)
            .with_predicate_consumption_receipt(subject.predicate_consumption)
            .compile()
            .expect("wrong frame is caught after certified predicate consumption binds")
            .certify()
            .expect_err("certified segment contract with wrong frame must deny");

        assert_eq!(
            denial.kind(),
            PlanarBooleanEventPredicateBindingDenialKind::SegmentContractLocalFrameMismatch
        );
    });
}

#[test]
fn event_predicate_binding_rejects_certified_segment_contract_with_wrong_precision() {
    reduced_pair_support::run_with_large_stack(|| {
        let subject =
            predicate_binding_support::binding_subject_with_segment_contract_precision_mismatch(
                "phase7.2 predicate binding rejects certified wrong precision",
            );

        let denial = PlanarBooleanEventPredicateBinding::plan(&subject.pair_worklist)
            .for_reduced_pair(subject.reduced_pair_identity)
            .with_segment_segment_receipts(subject.segment_receipts)
            .with_predicate_consumption_receipt(subject.predicate_consumption)
            .compile()
            .expect("wrong precision is caught after certified predicate consumption binds")
            .certify()
            .expect_err("certified segment contract with wrong precision must deny");

        assert_eq!(
            denial.kind(),
            PlanarBooleanEventPredicateBindingDenialKind::SegmentContractPrecisionBasisMismatch
        );
    });
}

#[test]
fn event_predicate_binding_rejects_empty_reduced_pair_identity() {
    reduced_pair_support::run_with_large_stack(|| {
        let subject = predicate_binding_support::binding_subject(
            "phase7.2 predicate binding rejects empty reduced pair",
        );

        let denial = PlanarBooleanEventPredicateBinding::plan(&subject.pair_worklist)
            .for_reduced_pair("")
            .with_segment_segment_receipts(subject.segment_receipts)
            .with_predicate_consumption_receipt(subject.predicate_consumption)
            .compile()
            .expect_err("predicate binding cannot erase reduced-pair identity");

        assert_eq!(
            denial.kind(),
            PlanarBooleanEventPredicateBindingDenialKind::MissingReducedPairIdentity
        );
    });
}
