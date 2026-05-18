use super::*;

#[test]
fn runtime_floor_certification_reports_representative_seeded_and_doc_example_outputs() {
    let bundle = certify_intent_admission_runtime_floor();

    for name in [
        "raw_intent_digest",
        "intent_eligibility_digest",
        "admission_decision_digest",
        "admitted_intent_plan_digest",
        "admitted_execution_handoff_digest",
        "advisory_decision_digest",
        "violation_decision_digest",
        "decision_trace_digest",
        "decision_trace_envelope_digest",
        "policy_decision_digest",
        "capability_decision_digest",
        "invariant_decision_digest",
        "basis_decision_digest",
        "projection_decision_digest",
        "routing_posture_digest",
        "execution_provenance_chain_digest",
        "failure_digest",
        "basis_observation_fixture_digest",
        "projection_consumption_fixture_digest",
    ] {
        assert_eq!(
            bundle.output_digest(name),
            bundle.representative_output_report().digest_for(name)
        );
    }
    assert_eq!(bundle.doc_example_report().rows().len(), 18);
    assert_eq!(bundle.seeded_report().rows().len(), 4);
    assert!(bundle
        .doc_example_report()
        .rows()
        .iter()
        .any(|row| row.label() == "basis_common_path"));
    assert!(bundle
        .doc_example_report()
        .rows()
        .iter()
        .any(|row| row.label() == "projection_common_path"));
    assert!(bundle
        .doc_example_report()
        .rows()
        .iter()
        .any(|row| row.label() == "read_common_path"));
    assert!(bundle
        .doc_example_report()
        .rows()
        .iter()
        .any(|row| row.label() == "read_basis_common_path"));
    assert!(!bundle
        .doc_example_report()
        .crate_doc_example_digest()
        .is_empty());
    assert!(!bundle.seeded_report().seeded_sequence_digest().is_empty());
    assert!(!bundle.seeded_report().seed_replay_digest().is_empty());
    assert!(!bundle
        .seeded_report()
        .seed_generator_class_digest()
        .is_empty());
}
