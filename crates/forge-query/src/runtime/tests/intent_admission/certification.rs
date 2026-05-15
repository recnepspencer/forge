use super::*;
use crate::basis_lifecycle::certify_basis_lifecycle;
use crate::identity::hash_parts;
use crate::projection_consumption::certify_projection_consumption_closeout_core;

#[test]
fn runtime_floor_certification_bundle_assembles_phase_six_foundation_artifacts() {
    let bundle = certify_intent_admission_runtime_floor();

    assert_eq!(
        bundle.family_inventory().rows(),
        forge_query_intent_admission_family_inventory().rows()
    );
    assert_eq!(
        bundle.coverage_inventory().rows(),
        forge_query_intent_admission_coverage_inventory().rows()
    );
    assert_eq!(
        bundle.support_matrix().rows(),
        forge_query_intent_admission_support_matrix().rows()
    );
    assert_eq!(
        bundle.public_boundary_audit().compile_fail_targets(),
        forge_query_intent_admission_compile_fail_targets()
    );
    assert_eq!(
        bundle.public_boundary_audit().golden_transcripts(),
        forge_query_intent_admission_golden_transcripts()
    );
    assert_eq!(
        bundle.output_digest("intent_golden_transcript_digest"),
        Some(bundle.public_boundary_audit().golden_transcript_digest())
    );
    assert_eq!(
        bundle.output_digest("compile_fail_boundary_digest"),
        Some(
            bundle
                .public_boundary_audit()
                .compile_fail_boundary_digest()
        )
    );
    assert_eq!(
        bundle.output_digest("decision_phase_progression_digest"),
        Some(
            bundle
                .proof_shape_audit()
                .decision_phase_progression_digest()
        )
    );
    assert_eq!(
        bundle.output_digest("decision_proof_shape_digest"),
        Some(bundle.proof_shape_audit().decision_proof_shape_digest())
    );
    assert_eq!(
        bundle.output_digest("intent_topology_audit_digest"),
        Some(bundle.topology_audit().topology_digest())
    );
    assert_eq!(
        bundle.output_digest("representative_family_coverage_digest"),
        Some(
            bundle
                .representative_family_report()
                .representative_family_coverage_digest()
        )
    );
    assert_eq!(
        bundle.output_digest("query_digest"),
        bundle
            .representative_output_report()
            .digest_for("query_digest")
    );
    assert_eq!(
        bundle.output_digest("failure_digest"),
        bundle
            .representative_output_report()
            .digest_for("failure_digest")
    );
    assert_eq!(
        bundle.output_digest("crate_doc_example_digest"),
        Some(bundle.doc_example_report().crate_doc_example_digest())
    );
    assert_eq!(
        bundle.output_digest("decision_oracle_digest"),
        Some(bundle.oracle_report().oracle_digest())
    );
    assert_eq!(
        bundle.output_digest("legacy_delegation_parity_digest"),
        Some(
            bundle
                .legacy_parity_report()
                .legacy_delegation_parity_digest()
        )
    );
    assert_eq!(
        bundle.output_digest("decision_support_traceability_digest"),
        Some(
            bundle
                .support_traceability_report()
                .decision_support_traceability_digest()
        )
    );
    assert_eq!(
        bundle.output_digest("counter_snapshot"),
        Some(bundle.counter_snapshot().digest())
    );
    assert_eq!(
        bundle.output_digest("seeded_sequence_digest"),
        Some(bundle.seeded_report().seeded_sequence_digest())
    );
    assert_eq!(
        bundle.output_digest("seed_replay_digest"),
        Some(bundle.seeded_report().seed_replay_digest())
    );
    assert_eq!(
        bundle.output_digest("seed_generator_class_digest"),
        Some(bundle.seeded_report().seed_generator_class_digest())
    );
    assert!(!bundle.certification_bundle_digest().is_empty());
}

#[test]
fn runtime_floor_certification_proof_shape_audit_freezes_shared_phase_progressions() {
    let bundle = certify_intent_admission_runtime_floor();
    let audit = bundle.proof_shape_audit();

    assert_eq!(
        audit.admitted_phase_progression(),
        &[
            ForgeQueryIntentDecisionTraceStage::RawIntent,
            ForgeQueryIntentDecisionTraceStage::Eligibility,
            ForgeQueryIntentDecisionTraceStage::AdmittedDecision,
            ForgeQueryIntentDecisionTraceStage::ExecutionHandoff,
            ForgeQueryIntentDecisionTraceStage::ExecutionOutcome,
        ]
    );
    assert_eq!(
        audit.advisory_phase_progression(),
        &[
            ForgeQueryIntentDecisionTraceStage::RawIntent,
            ForgeQueryIntentDecisionTraceStage::Eligibility,
            ForgeQueryIntentDecisionTraceStage::AdvisoryStop,
        ]
    );
    assert_eq!(
        audit.violation_phase_progression(),
        &[
            ForgeQueryIntentDecisionTraceStage::RawIntent,
            ForgeQueryIntentDecisionTraceStage::Eligibility,
            ForgeQueryIntentDecisionTraceStage::ViolationStop,
        ]
    );
}

#[test]
fn runtime_floor_certification_topology_and_representative_family_artifacts_match_phase_six_scope()
{
    let bundle = certify_intent_admission_runtime_floor();
    let basis = certify_basis_lifecycle();
    let projection = certify_projection_consumption_closeout_core();
    let rows = bundle.representative_family_report().rows();
    let basis_row = rows
        .iter()
        .find(|row| row.lane() == ForgeQueryIntentAdmissionRepresentativeFamilyLane::BasisParity)
        .expect("basis representative lane should exist");
    let projection_row = rows
        .iter()
        .find(|row| {
            row.lane() == ForgeQueryIntentAdmissionRepresentativeFamilyLane::ProjectionAdvisory
        })
        .expect("projection representative lane should exist");
    let inspection_row = rows
        .iter()
        .find(|row| {
            row.lane()
                == ForgeQueryIntentAdmissionRepresentativeFamilyLane::InspectionAdvisoryRedaction
        })
        .expect("inspection representative lane should exist");
    let routing_row = rows
        .iter()
        .find(|row| {
            row.lane() == ForgeQueryIntentAdmissionRepresentativeFamilyLane::RoutingFutureNeighbor
        })
        .expect("routing representative lane should exist");

    assert_eq!(bundle.topology_audit().rows().len(), 8);
    assert_eq!(
        bundle
            .topology_audit()
            .rows()
            .iter()
            .map(|row| row.domain().as_str())
            .collect::<Vec<_>>(),
        vec![
            "intent_admission/families",
            "intent_admission/eligibility",
            "intent_admission/decisions",
            "intent_admission/handoffs",
            "intent_admission/trace",
            "intent_admission/dx",
            "intent_admission/support",
            "intent_admission/certification",
        ]
    );
    assert_eq!(rows.len(), 4);
    assert_eq!(
        basis_row.evidence_digest(),
        hash_parts(&[
            basis.certification_bundle_digest().to_string(),
            basis
                .output_digest("query_digest")
                .expect("basis query digest should exist")
                .to_string(),
            basis
                .output_digest("basis_proof_shape_digest")
                .expect("basis proof-shape digest should exist")
                .to_string(),
            basis
                .output_digest("basis_phase_progression_digest")
                .expect("basis phase-progression digest should exist")
                .to_string(),
        ])
    );
    assert_eq!(
        projection_row.evidence_digest(),
        hash_parts(&[
            projection.certification_bundle_digest().to_string(),
            projection
                .output_digest("query_digest")
                .expect("projection query digest should exist")
                .to_string(),
            projection
                .output_digest("failure_digest")
                .expect("projection failure digest should exist")
                .to_string(),
            projection
                .output_digest("projection_phase_progression_digest")
                .expect("projection phase-progression digest should exist")
                .to_string(),
        ])
    );
    assert_ne!(
        inspection_row.evidence_digest(),
        routing_row.evidence_digest(),
        "inspection advisory-redaction must not collapse into routing future-neighbor evidence"
    );
    assert!(inspection_row
        .posture_detail()
        .contains("preserving one causal identity"));
    assert!(routing_row.posture_detail().contains("typed deferred lane"));
    assert!(routing_row
        .posture_detail()
        .contains("typed unsupported lane"));
    assert_eq!(
        bundle.output_digest("intent_family_digest"),
        Some(
            hash_parts(
                &forge_query_intent_admission_family_inventory()
                    .rows()
                    .iter()
                    .map(|row| row.family().as_str().to_string())
                    .collect::<Vec<_>>()
            )
            .as_str()
        )
    );
}

#[test]
fn runtime_floor_certification_reports_oracle_parity_and_slope_evidence() {
    let bundle = certify_intent_admission_runtime_floor();
    let comparison_rows = bundle.oracle_report().comparison_rows();
    let support_rows = bundle.support_traceability_report().rows();
    let deferred_support_row = support_rows
        .iter()
        .find(|row| row.lane() == "deferred")
        .expect("deferred support traceability row should exist");
    let unsupported_support_row = support_rows
        .iter()
        .find(|row| row.lane() == "unsupported")
        .expect("unsupported support traceability row should exist");

    assert_eq!(bundle.oracle_report().manifest_rows().len(), 5);
    assert_eq!(comparison_rows.len(), 5);
    assert!(comparison_rows
        .iter()
        .all(|row| !row.row_digest().is_empty()));
    for row in comparison_rows {
        assert_eq!(
            row.expected_digest(),
            row.actual_digest(),
            "oracle lane {:?} must converge\nexpected:{}\nactual:{}",
            row.lane(),
            row.expected_detail(),
            row.actual_detail()
        );
    }
    assert_eq!(bundle.legacy_parity_report().rows().len(), 2);
    assert_eq!(support_rows.len(), 5);
    assert_eq!(
        deferred_support_row.family(),
        ForgeQueryIntentAdmissionFamily::ReadExecutionIntent.as_str()
    );
    assert_eq!(
        unsupported_support_row.family(),
        ForgeQueryIntentAdmissionFamily::InspectionMaterializationIntent.as_str()
    );
    assert!(
        deferred_support_row
            .support_detail()
            .starts_with("support:deferred:"),
        "deferred support lane must certify against the executable support matrix"
    );
    assert!(
        unsupported_support_row
            .support_detail()
            .starts_with("unsupported:"),
        "unsupported support lane must certify against coverage posture rather than a fake matrix row"
    );
    assert_ne!(
        deferred_support_row.row_digest(),
        unsupported_support_row.row_digest(),
        "deferred and unsupported support lanes must stay mechanically distinct"
    );
    assert_eq!(
        bundle.counter_snapshot().intent_family_lookup_width(),
        forge_query_intent_admission_family_inventory().rows().len()
    );
    assert_eq!(
        bundle.counter_snapshot().covered_entrypoint_lookup_width(),
        forge_query_intent_admission_coverage_inventory()
            .rows()
            .len()
    );
    assert!(!bundle
        .output_digest("admission_classification_slope_digest")
        .expect("slope output should exist")
        .is_empty());
    assert!(!bundle
        .output_digest("decision_trace_assembly_slope_digest")
        .expect("slope output should exist")
        .is_empty());
    assert!(!bundle
        .output_digest("legacy_delegation_parity_slope_digest")
        .expect("slope output should exist")
        .is_empty());
}

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
    ] {
        assert_eq!(
            bundle.output_digest(name),
            bundle.representative_output_report().digest_for(name)
        );
    }
    assert_eq!(bundle.doc_example_report().rows().len(), 6);
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
