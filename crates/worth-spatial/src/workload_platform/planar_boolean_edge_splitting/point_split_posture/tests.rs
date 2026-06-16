use super::PlanarBooleanPointSplitPostureDenialKind;
use crate::workload_platform::planar_boolean_edge_splitting::point_split_candidates::{
    PlanarBooleanPointSplitCandidate, PlanarBooleanPointSplitCandidateInput,
};
use crate::workload_platform::planar_boolean_edge_splitting::proof_chain_support::{
    candidate_input_from, point_candidate, point_candidate_set,
};
use crate::workload_platform::planar_boolean_events::PlanarBooleanPointEventKind;

#[test]
fn mixed_point_event_kind_group_denies_before_posture_selection() {
    let denial = point_candidate_set(vec![
        point_candidate(
            "interior",
            "event:mixed",
            "carrier:a",
            0.5,
            PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
        ),
        point_candidate(
            "t-endpoint",
            "event:mixed",
            "carrier:b",
            0.0,
            PlanarBooleanPointEventKind::OperandAEndpointOnOperandBInterior,
        ),
    ])
    .admit_parameter_domain()
    .expect("parameters should admit")
    .classify_point_split_postures()
    .expect_err("mixed point-event groups must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanPointSplitPostureDenialKind::MixedPointEventKind
    );
    assert_eq!(denial.evidence_identity(), "event:mixed");
}

#[test]
fn endpoint_on_interior_without_interior_participant_denies() {
    let denial = point_candidate_set(vec![point_candidate(
        "endpoint",
        "event:t-missing-interior",
        "carrier:a",
        0.0,
        PlanarBooleanPointEventKind::OperandAEndpointOnOperandBInterior,
    )])
    .admit_parameter_domain()
    .expect("endpoint parameter should admit")
    .classify_point_split_postures()
    .expect_err("T-junction promotion requires an interior participant");

    assert_eq!(
        denial.kind(),
        PlanarBooleanPointSplitPostureDenialKind::TJunctionMissingInteriorParticipant
    );
}

#[test]
fn endpoint_on_interior_without_endpoint_participant_denies() {
    let denial = point_candidate_set(vec![point_candidate(
        "interior",
        "event:t-missing-endpoint",
        "carrier:b",
        0.5,
        PlanarBooleanPointEventKind::OperandBEndpointOnOperandAInterior,
    )])
    .admit_parameter_domain()
    .expect("interior parameter should admit")
    .classify_point_split_postures()
    .expect_err("T-junction promotion requires an endpoint participant");

    assert_eq!(
        denial.kind(),
        PlanarBooleanPointSplitPostureDenialKind::TJunctionMissingEndpointParticipant
    );
}

#[test]
fn shared_endpoint_missing_projection_provenance_denies() {
    let denial = point_candidate_set(vec![
        shared_endpoint_without_projection_facts(),
        valid_shared_endpoint_candidate("peer", "carrier:b", 1.0),
    ])
    .admit_parameter_domain()
    .expect("shared endpoint parameter should admit")
    .classify_point_split_postures()
    .expect_err("shared endpoint projection provenance must be present");

    assert_eq!(
        denial.kind(),
        PlanarBooleanPointSplitPostureDenialKind::SharedEndpointMissingProvenance
    );
}

#[test]
fn shared_endpoint_mismatched_provenance_cardinality_denies() {
    let denial = point_candidate_set(vec![
        shared_endpoint_with_one_projection_fact(),
        valid_shared_endpoint_candidate("peer", "carrier:b", 1.0),
    ])
    .admit_parameter_domain()
    .expect("shared endpoint parameter should admit")
    .classify_point_split_postures()
    .expect_err("shared endpoint source/projection cardinality mismatch must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanPointSplitPostureDenialKind::SharedEndpointProvenanceMismatch
    );
}

#[test]
fn shared_endpoint_interior_participant_denies() {
    let denial = point_candidate_set(vec![
        shared_endpoint_with_interior_parameter(),
        valid_shared_endpoint_candidate("peer", "carrier:b", 1.0),
    ])
    .admit_parameter_domain()
    .expect("interior parameter should admit")
    .classify_point_split_postures()
    .expect_err("shared endpoint posture must not hide an interior split");

    assert_eq!(
        denial.kind(),
        PlanarBooleanPointSplitPostureDenialKind::SharedEndpointInteriorParticipant
    );
}

#[test]
fn shared_endpoint_single_participant_denies() {
    let denial = point_candidate_set(vec![valid_shared_endpoint_candidate(
        "single",
        "carrier:a",
        0.0,
    )])
    .admit_parameter_domain()
    .expect("single shared endpoint parameter should admit")
    .classify_point_split_postures()
    .expect_err("shared endpoint posture requires at least two participants");

    assert_eq!(
        denial.kind(),
        PlanarBooleanPointSplitPostureDenialKind::SharedEndpointMissingParticipant
    );
}

#[test]
fn shared_endpoint_exact_endpoint_must_match_shared_provenance() {
    let denial = point_candidate_set(vec![
        shared_endpoint_with_foreign_endpoint_provenance(),
        valid_shared_endpoint_candidate("peer", "carrier:b", 1.0),
    ])
    .admit_parameter_domain()
    .expect("shared endpoint parameters should admit")
    .classify_point_split_postures()
    .expect_err("shared provenance must match admitted exact endpoint facts");

    assert_eq!(
        denial.kind(),
        PlanarBooleanPointSplitPostureDenialKind::SharedEndpointExactEndpointMismatch
    );
}

fn valid_shared_endpoint_candidate(
    label: &str,
    carrier_identity: &str,
    parameter: f64,
) -> PlanarBooleanPointSplitCandidate {
    let candidate = point_candidate(
        label,
        "event:shared-test",
        carrier_identity,
        parameter,
        PlanarBooleanPointEventKind::SharedEndpoint,
    );
    PlanarBooleanPointSplitCandidate::new(PlanarBooleanPointSplitCandidateInput {
        shared_endpoint_source_identities: vec![
            "endpoint:start".to_string(),
            "endpoint:end".to_string(),
        ],
        shared_endpoint_projection_fact_digests: vec![
            "projection:start".to_string(),
            "projection:end".to_string(),
        ],
        ..candidate_input_from(candidate)
    })
}

fn shared_endpoint_without_projection_facts() -> PlanarBooleanPointSplitCandidate {
    let candidate = point_candidate(
        "shared-no-projection",
        "event:shared-test",
        "carrier:a",
        0.0,
        PlanarBooleanPointEventKind::SharedEndpoint,
    );
    PlanarBooleanPointSplitCandidate::new(PlanarBooleanPointSplitCandidateInput {
        shared_endpoint_source_identities: vec!["endpoint:a".to_string(), "endpoint:b".to_string()],
        shared_endpoint_projection_fact_digests: Vec::new(),
        ..candidate_input_from(candidate)
    })
}

fn shared_endpoint_with_one_projection_fact() -> PlanarBooleanPointSplitCandidate {
    let candidate = point_candidate(
        "shared-one-projection",
        "event:shared-test",
        "carrier:a",
        0.0,
        PlanarBooleanPointEventKind::SharedEndpoint,
    );
    PlanarBooleanPointSplitCandidate::new(PlanarBooleanPointSplitCandidateInput {
        shared_endpoint_source_identities: vec!["endpoint:a".to_string(), "endpoint:b".to_string()],
        shared_endpoint_projection_fact_digests: vec!["projection:a".to_string()],
        ..candidate_input_from(candidate)
    })
}

fn shared_endpoint_with_interior_parameter() -> PlanarBooleanPointSplitCandidate {
    let candidate = point_candidate(
        "shared-interior",
        "event:shared-test",
        "carrier:a",
        0.5,
        PlanarBooleanPointEventKind::SharedEndpoint,
    );
    PlanarBooleanPointSplitCandidate::new(PlanarBooleanPointSplitCandidateInput {
        shared_endpoint_source_identities: vec!["endpoint:a".to_string(), "endpoint:b".to_string()],
        shared_endpoint_projection_fact_digests: vec![
            "projection:a".to_string(),
            "projection:b".to_string(),
        ],
        ..candidate_input_from(candidate)
    })
}

fn shared_endpoint_with_foreign_endpoint_provenance() -> PlanarBooleanPointSplitCandidate {
    let candidate = point_candidate(
        "shared-foreign-provenance",
        "event:shared-test",
        "carrier:a",
        0.0,
        PlanarBooleanPointEventKind::SharedEndpoint,
    );
    PlanarBooleanPointSplitCandidate::new(PlanarBooleanPointSplitCandidateInput {
        shared_endpoint_source_identities: vec!["endpoint:foreign-a".to_string()],
        shared_endpoint_projection_fact_digests: vec!["projection:foreign-a".to_string()],
        ..candidate_input_from(candidate)
    })
}
