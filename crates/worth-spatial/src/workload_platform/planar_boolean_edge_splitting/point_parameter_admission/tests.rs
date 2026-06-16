use crate::workload_platform::planar_boolean_edge_splitting::point_split_candidates::{
    PlanarBooleanPointSplitCandidate, PlanarBooleanPointSplitCandidateCounters,
    PlanarBooleanPointSplitCandidateInput, PlanarBooleanPointSplitCandidateSet,
};
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanPointEventCoordinateFact, PlanarBooleanPointEventKind,
};

use super::{PlanarBooleanSplitPointAdmissionDenialKind, PlanarBooleanSplitPointEndpointPosture};

#[test]
fn split_point_parameter_domain_accepts_interior_and_exact_endpoint_points() {
    let admitted = multi_candidate_set(&[0.0, 0.5, 1.0])
        .admit_parameter_domain()
        .expect("interior and exact endpoints should admit");

    assert_eq!(admitted.counters().inspected_point_candidates(), 3);
    assert_eq!(admitted.counters().admitted_point_candidates(), 3);
    assert_eq!(admitted.counters().endpoint_candidates(), 2);
    assert_eq!(admitted.counters().interior_candidates(), 1);
    assert_eq!(admitted.counters().rejected_out_of_domain_points(), 0);
}

#[test]
fn split_point_parameter_domain_rejects_out_of_range_or_nan_parameters() {
    let nan = candidate_set(f64::NAN)
        .admit_parameter_domain()
        .expect_err("NaN point parameter must deny");
    assert_eq!(
        nan.kind(),
        PlanarBooleanSplitPointAdmissionDenialKind::NonFiniteParameter
    );
    assert_eq!(nan.rejected_non_finite_points(), 1);
    assert_eq!(nan.rejected_out_of_domain_points(), 0);

    let positive_infinity = candidate_set(f64::INFINITY)
        .admit_parameter_domain()
        .expect_err("infinite point parameter must deny");
    assert_eq!(
        positive_infinity.kind(),
        PlanarBooleanSplitPointAdmissionDenialKind::NonFiniteParameter
    );
    assert_eq!(positive_infinity.rejected_non_finite_points(), 1);

    let negative_out_of_domain = candidate_set(-0.0001)
        .admit_parameter_domain()
        .expect_err("negative point parameter must deny");
    assert_eq!(
        negative_out_of_domain.kind(),
        PlanarBooleanSplitPointAdmissionDenialKind::OutOfDomainParameter
    );
    assert_eq!(negative_out_of_domain.rejected_out_of_domain_points(), 1);

    let positive_out_of_domain = candidate_set(1.0001)
        .admit_parameter_domain()
        .expect_err("greater-than-one point parameter must deny");
    assert_eq!(
        positive_out_of_domain.kind(),
        PlanarBooleanSplitPointAdmissionDenialKind::OutOfDomainParameter
    );
    assert_eq!(positive_out_of_domain.rejected_out_of_domain_points(), 1);
}

#[test]
fn split_point_parameter_domain_denies_mixed_candidate_sets_instead_of_filtering_bad_rows() {
    let candidates = multi_candidate_set(&[0.25, 1.0 + f64::EPSILON, 0.75]);
    let poisoned_candidate_identity = candidates.candidates()[1].candidate_identity().to_string();

    let denial = candidates
        .admit_parameter_domain()
        .expect_err("one poisoned point parameter must deny the whole admission product");

    assert_eq!(
        denial.kind(),
        PlanarBooleanSplitPointAdmissionDenialKind::OutOfDomainParameter
    );
    assert_eq!(denial.evidence_identity(), poisoned_candidate_identity);
    assert_eq!(denial.rejected_out_of_domain_points(), 1);
}

#[test]
fn split_point_parameter_domain_preserves_endpoint_identity_when_exact() {
    let admitted = multi_candidate_set(&[0.0, 0.5, 1.0])
        .admit_parameter_domain()
        .expect("exact endpoints should admit");

    let start = admitted_candidate_with_posture(
        &admitted,
        PlanarBooleanSplitPointEndpointPosture::StartEndpoint,
    );
    assert_eq!(
        start.exact_endpoint_source_identity(),
        Some("start source endpoint")
    );
    assert_eq!(
        start.exact_projected_endpoint_fact_identity(),
        Some("start projected endpoint")
    );

    let interior = admitted_candidate_with_posture(
        &admitted,
        PlanarBooleanSplitPointEndpointPosture::Interior,
    );
    assert_eq!(interior.exact_endpoint_source_identity(), None);
    assert_eq!(interior.exact_projected_endpoint_fact_identity(), None);

    let end = admitted_candidate_with_posture(
        &admitted,
        PlanarBooleanSplitPointEndpointPosture::EndEndpoint,
    );
    assert_eq!(
        end.exact_endpoint_source_identity(),
        Some("end source endpoint")
    );
    assert_eq!(
        end.exact_projected_endpoint_fact_identity(),
        Some("end projected endpoint")
    );
}

#[test]
fn split_point_parameter_domain_keeps_near_endpoint_points_interior() {
    let admitted = multi_candidate_set(&[f64::MIN_POSITIVE, 1.0 - f64::EPSILON])
        .admit_parameter_domain()
        .expect("near endpoints inside the domain should admit as interior points");

    assert_eq!(admitted.counters().endpoint_candidates(), 0);
    assert_eq!(admitted.counters().interior_candidates(), 2);
    assert!(admitted.admitted_candidates().iter().all(|candidate| {
        candidate.endpoint_posture() == PlanarBooleanSplitPointEndpointPosture::Interior
            && candidate.exact_endpoint_source_identity().is_none()
            && candidate.exact_projected_endpoint_fact_identity().is_none()
    }));
}

#[test]
fn split_point_parameter_domain_classifies_negative_zero_as_exact_start_endpoint() {
    let admitted = candidate_set(-0.0)
        .admit_parameter_domain()
        .expect("signed zero is the canonical exact start endpoint parameter");
    let admitted_candidate = admitted
        .admitted_candidates()
        .first()
        .expect("signed-zero candidate should admit");

    assert_eq!(
        admitted_candidate.endpoint_posture(),
        PlanarBooleanSplitPointEndpointPosture::StartEndpoint
    );
    assert_eq!(
        admitted_candidate.exact_endpoint_source_identity(),
        Some("start source endpoint")
    );
    assert_eq!(
        admitted_candidate.exact_projected_endpoint_fact_identity(),
        Some("start projected endpoint")
    );
    assert_eq!(admitted.counters().endpoint_candidates(), 1);
    assert_eq!(admitted.counters().interior_candidates(), 0);
}

#[test]
fn split_point_parameter_domain_rejects_exact_endpoint_without_endpoint_identity() {
    let missing_start = candidate_set_with_endpoint_identity(0.0, "", "start projected endpoint")
        .admit_parameter_domain()
        .expect_err("exact start endpoint without source identity must deny");
    assert_eq!(
        missing_start.kind(),
        PlanarBooleanSplitPointAdmissionDenialKind::MissingExactEndpointIdentity
    );
    assert_eq!(missing_start.rejected_missing_endpoint_identity_points(), 1);

    let missing_end = candidate_set_with_endpoint_identity(1.0, "end source endpoint", "")
        .admit_parameter_domain()
        .expect_err("exact end endpoint without projected endpoint fact must deny");
    assert_eq!(
        missing_end.kind(),
        PlanarBooleanSplitPointAdmissionDenialKind::MissingExactEndpointIdentity
    );
    assert_eq!(missing_end.rejected_missing_endpoint_identity_points(), 1);
}

#[test]
fn split_point_parameter_domain_does_not_clamp_out_of_domain_points() {
    let candidates = candidate_set(-f64::MIN_POSITIVE);
    let expected_evidence_identity = candidates.candidates()[0].candidate_identity().to_string();
    let denial = candidates
        .admit_parameter_domain()
        .expect_err("negative near-zero parameter must deny instead of clamping to endpoint");

    assert_eq!(
        denial.kind(),
        PlanarBooleanSplitPointAdmissionDenialKind::OutOfDomainParameter
    );
    assert_eq!(denial.evidence_identity(), expected_evidence_identity);
}

#[test]
fn split_point_parameter_domain_preserves_candidate_set_identity_and_order() {
    let candidates = multi_candidate_set(&[1.0, 0.5, 0.0]);
    let original_order = candidates
        .candidates()
        .iter()
        .map(|candidate| candidate.candidate_identity().to_string())
        .collect::<Vec<_>>();

    let admitted = candidates
        .admit_parameter_domain()
        .expect("valid point candidates should admit in existing canonical order");
    let admitted_order = admitted
        .admitted_candidates()
        .iter()
        .map(|candidate| candidate.candidate().candidate_identity().to_string())
        .collect::<Vec<_>>();

    assert_eq!(
        admitted.point_candidate_set_identity(),
        "test point candidate set"
    );
    assert_eq!(admitted_order, original_order);
}

fn admitted_candidate_with_posture(
    admitted: &super::PlanarBooleanAdmittedPointSplitCandidateSet,
    posture: PlanarBooleanSplitPointEndpointPosture,
) -> &super::AdmittedPointSplitCandidate {
    admitted
        .admitted_candidates()
        .iter()
        .find(|candidate| candidate.endpoint_posture() == posture)
        .expect("admitted candidate with requested posture should be present")
}

fn candidate_set(parameter: f64) -> PlanarBooleanPointSplitCandidateSet {
    multi_candidate_set(&[parameter])
}

fn candidate_set_with_endpoint_identity(
    parameter: f64,
    source_endpoint_identity: &str,
    projected_endpoint_fact_identity: &str,
) -> PlanarBooleanPointSplitCandidateSet {
    PlanarBooleanPointSplitCandidateSet::new(
        "test point candidate set".to_string(),
        "test participation index".to_string(),
        vec![candidate(
            parameter,
            source_endpoint_identity,
            projected_endpoint_fact_identity,
            source_endpoint_identity,
            projected_endpoint_fact_identity,
        )],
        PlanarBooleanPointSplitCandidateCounters::default(),
    )
}

fn multi_candidate_set(parameters: &[f64]) -> PlanarBooleanPointSplitCandidateSet {
    PlanarBooleanPointSplitCandidateSet::new(
        "test point candidate set".to_string(),
        "test participation index".to_string(),
        parameters
            .iter()
            .map(|parameter| {
                candidate(
                    *parameter,
                    "start source endpoint",
                    "start projected endpoint",
                    "end source endpoint",
                    "end projected endpoint",
                )
            })
            .collect(),
        PlanarBooleanPointSplitCandidateCounters::default(),
    )
}

fn candidate(
    parameter: f64,
    start_source_endpoint_identity: &str,
    start_projected_endpoint_fact_identity: &str,
    end_source_endpoint_identity: &str,
    end_projected_endpoint_fact_identity: &str,
) -> PlanarBooleanPointSplitCandidate {
    PlanarBooleanPointSplitCandidate::new(PlanarBooleanPointSplitCandidateInput {
        candidate_identity: format!("candidate:{parameter:?}"),
        point_event_identity: "point event".to_string(),
        point_event_kind: PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
        carrier_identity: "carrier".to_string(),
        source_edge_identity: "source edge".to_string(),
        segment_identity: "segment".to_string(),
        coordinate_fact: PlanarBooleanPointEventCoordinateFact::new(
            [0.0, 0.0],
            "local frame",
            "precision basis",
        ),
        parameter_fact_identity: "parameter fact".to_string(),
        parameter,
        participation_row_identity: "participation row".to_string(),
        segment_pair_identities: vec!["segment pair".to_string()],
        participating_carrier_identities: vec!["carrier".to_string()],
        event_endpoint_source_identities: vec![
            "start source endpoint".to_string(),
            "end source endpoint".to_string(),
        ],
        event_endpoint_projection_fact_digests: vec![
            "start projected endpoint".to_string(),
            "end projected endpoint".to_string(),
        ],
        predicate_receipt_identities: vec!["predicate receipt".to_string()],
        event_group_identities: vec!["event group".to_string()],
        shared_endpoint_source_identities: Vec::new(),
        shared_endpoint_projection_fact_digests: Vec::new(),
        start_source_endpoint_identity: start_source_endpoint_identity.to_string(),
        start_projected_endpoint_fact_identity: start_projected_endpoint_fact_identity.to_string(),
        end_source_endpoint_identity: end_source_endpoint_identity.to_string(),
        end_projected_endpoint_fact_identity: end_projected_endpoint_fact_identity.to_string(),
    })
}
