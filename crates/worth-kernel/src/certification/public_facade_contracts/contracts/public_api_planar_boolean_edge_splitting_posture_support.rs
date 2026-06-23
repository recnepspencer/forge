use std::collections::BTreeMap;

use worth_spatial::facade::planar_boolean_edge_splitting::{
    AdmittedPointSplitCandidate, PlanarBooleanAdmittedPointSplitCandidateSet,
    PlanarBooleanPointSplitPosture, PlanarBooleanPointSplitPostureSet,
    PlanarBooleanSplitPointEndpointPosture,
};
use worth_spatial::facade::planar_boolean_events::PlanarBooleanPointEventKind;

pub(crate) fn assert_point_split_postures_match_admitted_events(
    admitted: &PlanarBooleanAdmittedPointSplitCandidateSet,
    postures: &PlanarBooleanPointSplitPostureSet,
) {
    assert_eq!(
        postures.point_candidate_set_identity(),
        admitted.point_candidate_set_identity()
    );
    assert_eq!(
        postures.counters().admitted_point_candidates(),
        admitted.admitted_candidates().len()
    );
    assert_eq!(
        postures.counters().postured_point_candidates(),
        admitted.admitted_candidates().len()
    );
    let expected_by_event = expected_point_split_postures_by_event(admitted);
    let expected_counters = expected_point_split_posture_counters(admitted, &expected_by_event);
    assert_posture_counters_match(postures, expected_counters);
    assert!(
        expected_counters.t_junction_promotions > 0,
        "metaboss posture proof must exercise T-junction promotion"
    );
    assert!(
        expected_counters.shared_endpoint_noops > 0,
        "metaboss posture proof must exercise shared endpoint no-ops"
    );
    for postured_candidate in postures.postured_candidates() {
        let admitted_candidate = postured_candidate.admitted_candidate();
        let expected = expected_by_event[admitted_candidate.candidate().point_event_identity()];
        assert_eq!(postured_candidate.posture(), expected);
        assert_eq!(
            postured_candidate.posture().produces_split_vertex(),
            expected.produces_split_vertex()
        );
        if expected == PlanarBooleanPointSplitPosture::SharedEndpoint {
            assert_shared_endpoint_posture_preserves_exact_endpoint(admitted_candidate);
        }
    }
}

fn expected_point_split_postures_by_event(
    admitted: &PlanarBooleanAdmittedPointSplitCandidateSet,
) -> BTreeMap<String, PlanarBooleanPointSplitPosture> {
    grouped_admitted_candidates_by_event(admitted)
        .into_iter()
        .map(|(point_event_identity, candidates)| {
            (
                point_event_identity,
                expected_point_split_posture_for_event_group(&candidates),
            )
        })
        .collect()
}

fn grouped_admitted_candidates_by_event(
    admitted: &PlanarBooleanAdmittedPointSplitCandidateSet,
) -> BTreeMap<String, Vec<&AdmittedPointSplitCandidate>> {
    let mut grouped = BTreeMap::<String, Vec<_>>::new();
    for admitted_candidate in admitted.admitted_candidates() {
        grouped
            .entry(
                admitted_candidate
                    .candidate()
                    .point_event_identity()
                    .to_string(),
            )
            .or_default()
            .push(admitted_candidate);
    }
    grouped
}

fn expected_point_split_posture_for_event_group(
    candidates: &[&AdmittedPointSplitCandidate],
) -> PlanarBooleanPointSplitPosture {
    let point_event_kind = candidates[0].candidate().point_event_kind();
    let has_endpoint = candidates.iter().any(|candidate| {
        candidate.endpoint_posture() != PlanarBooleanSplitPointEndpointPosture::Interior
    });
    let has_interior = candidates.iter().any(|candidate| {
        candidate.endpoint_posture() == PlanarBooleanSplitPointEndpointPosture::Interior
    });
    match point_event_kind {
        PlanarBooleanPointEventKind::SharedEndpoint => {
            assert!(candidates.len() >= 2);
            PlanarBooleanPointSplitPosture::SharedEndpoint
        }
        PlanarBooleanPointEventKind::OperandAEndpointOnOperandBInterior
        | PlanarBooleanPointEventKind::OperandBEndpointOnOperandAInterior => {
            assert!(has_endpoint);
            assert!(has_interior);
            PlanarBooleanPointSplitPosture::TJunctionPromotion
        }
        PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing if has_interior => {
            PlanarBooleanPointSplitPosture::InteriorSplit
        }
        PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing => {
            PlanarBooleanPointSplitPosture::EndpointNoOp
        }
    }
}

fn expected_point_split_posture_counters(
    admitted: &PlanarBooleanAdmittedPointSplitCandidateSet,
    expected_by_event: &BTreeMap<String, PlanarBooleanPointSplitPosture>,
) -> ExpectedPointSplitPostureCounters {
    let mut expected_counters = ExpectedPointSplitPostureCounters::default();
    for admitted_candidate in admitted.admitted_candidates() {
        expected_counters
            .record(expected_by_event[admitted_candidate.candidate().point_event_identity()]);
    }
    expected_counters
}

fn assert_posture_counters_match(
    postures: &PlanarBooleanPointSplitPostureSet,
    expected_counters: ExpectedPointSplitPostureCounters,
) {
    assert_eq!(
        postures.counters().interior_splits(),
        expected_counters.interior_splits
    );
    assert_eq!(
        postures.counters().t_junction_promotions(),
        expected_counters.t_junction_promotions
    );
    assert_eq!(
        postures.counters().shared_endpoint_noops(),
        expected_counters.shared_endpoint_noops
    );
    assert_eq!(
        postures.counters().endpoint_noops(),
        expected_counters.endpoint_noops
    );
}

fn assert_shared_endpoint_posture_preserves_exact_endpoint(
    admitted_candidate: &AdmittedPointSplitCandidate,
) {
    let candidate = admitted_candidate.candidate();
    let exact_endpoint = admitted_candidate
        .exact_endpoint_source_identity()
        .expect("shared endpoint posture must preserve exact source endpoint identity");
    let exact_projection = admitted_candidate
        .exact_projected_endpoint_fact_identity()
        .expect("shared endpoint posture must preserve exact projection fact identity");
    assert!(
        candidate
            .shared_endpoint_source_identities()
            .iter()
            .any(|identity| identity == exact_endpoint),
        "shared endpoint source provenance {:?} should contain admitted exact endpoint {exact_endpoint}",
        candidate.shared_endpoint_source_identities()
    );
    assert!(!exact_projection.is_empty());
    assert!(!candidate
        .shared_endpoint_projection_fact_digests()
        .is_empty());
}

#[derive(Clone, Copy, Default)]
struct ExpectedPointSplitPostureCounters {
    interior_splits: usize,
    t_junction_promotions: usize,
    shared_endpoint_noops: usize,
    endpoint_noops: usize,
}

impl ExpectedPointSplitPostureCounters {
    fn record(&mut self, posture: PlanarBooleanPointSplitPosture) {
        match posture {
            PlanarBooleanPointSplitPosture::InteriorSplit => self.interior_splits += 1,
            PlanarBooleanPointSplitPosture::TJunctionPromotion => self.t_junction_promotions += 1,
            PlanarBooleanPointSplitPosture::SharedEndpoint => self.shared_endpoint_noops += 1,
            PlanarBooleanPointSplitPosture::EndpointNoOp => self.endpoint_noops += 1,
        }
    }
}
