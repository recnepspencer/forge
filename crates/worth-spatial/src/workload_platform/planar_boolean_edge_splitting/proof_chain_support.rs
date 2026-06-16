use crate::workload_platform::planar_boolean_edge_splitting::{
    interval_parameter_admission::{
        AdmittedIntervalSplitCandidate, PlanarBooleanAdmittedIntervalSplitCandidateSet,
        PlanarBooleanSplitIntervalAdmissionCounters,
    },
    interval_split_candidates::{
        PlanarBooleanIntervalSplitCandidate, PlanarBooleanIntervalSplitCandidateInput,
    },
    point_split_candidates::{
        PlanarBooleanPointSplitCandidate, PlanarBooleanPointSplitCandidateCounters,
        PlanarBooleanPointSplitCandidateInput, PlanarBooleanPointSplitCandidateSet,
    },
    raw_edge_split_schedule::PlanarBooleanRawEdgeSplitScheduleSet,
};
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanIntervalEventKind, PlanarBooleanPointEventCoordinateFact,
    PlanarBooleanPointEventKind, PlanarBooleanSourceIntervalSense,
};

pub(crate) fn ordered_digest_for(candidates: Vec<PlanarBooleanPointSplitCandidate>) -> String {
    raw_schedule_for(candidates)
        .canonicalize_split_schedule_order()
        .expect("raw schedule should order")
        .schedules()[0]
        .order_digest()
        .to_string()
}

pub(crate) fn run_point_pipeline(
    candidates: Vec<PlanarBooleanPointSplitCandidate>,
) -> crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanNormalizedEdgeSplitScheduleSet{
    raw_schedule_for(candidates)
        .canonicalize_split_schedule_order()
        .expect("raw schedules should order")
        .collapse_duplicate_split_points()
        .expect("compatible duplicates should normalize")
}

pub(crate) fn raw_schedule_for(
    candidates: Vec<PlanarBooleanPointSplitCandidate>,
) -> crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanRawEdgeSplitScheduleSet {
    let postures = point_candidate_set(candidates)
        .admit_parameter_domain()
        .expect("parameters should admit")
        .classify_point_split_postures()
        .expect("postures should classify");
    PlanarBooleanRawEdgeSplitScheduleSet::assemble_from_admitted_candidates(
        &postures,
        &empty_intervals(),
    )
    .expect("raw schedule should assemble")
}

pub(crate) fn point_candidate_set(
    candidates: Vec<PlanarBooleanPointSplitCandidate>,
) -> PlanarBooleanPointSplitCandidateSet {
    PlanarBooleanPointSplitCandidateSet::new(
        "point candidate set".to_string(),
        "participation index".to_string(),
        candidates,
        PlanarBooleanPointSplitCandidateCounters::default(),
    )
}

pub(crate) fn empty_intervals() -> PlanarBooleanAdmittedIntervalSplitCandidateSet {
    PlanarBooleanAdmittedIntervalSplitCandidateSet::new(
        "empty interval set".to_string(),
        "participation index".to_string(),
        Vec::new(),
        PlanarBooleanSplitIntervalAdmissionCounters::default(),
    )
}

pub(crate) fn single_interval() -> PlanarBooleanAdmittedIntervalSplitCandidateSet {
    interval_with_range([0.25, 0.75])
}

pub(crate) fn interval_with_range(
    admitted_parameter_range: [f64; 2],
) -> PlanarBooleanAdmittedIntervalSplitCandidateSet {
    interval_with_event_identity(
        admitted_parameter_range,
        &format!(
            "event:interval:{}:{}",
            admitted_parameter_range[0], admitted_parameter_range[1]
        ),
    )
}

pub(crate) fn interval_with_event_identity(
    admitted_parameter_range: [f64; 2],
    interval_event_identity: &str,
) -> PlanarBooleanAdmittedIntervalSplitCandidateSet {
    let candidate =
        PlanarBooleanIntervalSplitCandidate::new(PlanarBooleanIntervalSplitCandidateInput {
            candidate_identity: format!(
                "interval candidate:{}:{}",
                admitted_parameter_range[0], admitted_parameter_range[1]
            ),
            interval_event_identity: interval_event_identity.to_string(),
            interval_event_kind: PlanarBooleanIntervalEventKind::PartialOverlap,
            carrier_identity: "carrier:a".to_string(),
            source_edge_identity: "source edge a".to_string(),
            segment_identity: "segment:interval".to_string(),
            source_interval_identity: "source interval".to_string(),
            source_parameter_range: admitted_parameter_range,
            source_sense: PlanarBooleanSourceIntervalSense::Forward,
            normalized_interval_identity: "normalized interval".to_string(),
            normalized_parameter_range: admitted_parameter_range,
            local_frame_identity: "local frame".to_string(),
            precision_basis_identity: "precision basis".to_string(),
            participation_row_identity: "row:interval".to_string(),
            event_group_identities: vec![format!("event-group:{interval_event_identity}")],
        });
    PlanarBooleanAdmittedIntervalSplitCandidateSet::new(
        "single interval set".to_string(),
        "participation index".to_string(),
        vec![AdmittedIntervalSplitCandidate::new(
            candidate,
            admitted_parameter_range,
        )],
        PlanarBooleanSplitIntervalAdmissionCounters::new(1, 1, 0, 0, 0, 0),
    )
}

pub(crate) fn shared_endpoint_candidate(
    label: &str,
    carrier_identity: &str,
    parameter: f64,
) -> PlanarBooleanPointSplitCandidate {
    let candidate = point_candidate(
        label,
        "event:shared",
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

pub(crate) fn point_candidate(
    label: &str,
    point_event_identity: &str,
    carrier_identity: &str,
    parameter: f64,
    point_event_kind: PlanarBooleanPointEventKind,
) -> PlanarBooleanPointSplitCandidate {
    let source_edge = if carrier_identity == "carrier:b" {
        "source edge b"
    } else {
        "source edge a"
    };
    PlanarBooleanPointSplitCandidate::new(PlanarBooleanPointSplitCandidateInput {
        candidate_identity: format!("candidate:{label}"),
        point_event_identity: point_event_identity.to_string(),
        point_event_kind,
        carrier_identity: carrier_identity.to_string(),
        source_edge_identity: source_edge.to_string(),
        segment_identity: format!("segment:{label}"),
        coordinate_fact: PlanarBooleanPointEventCoordinateFact::new(
            [parameter, parameter],
            "local frame",
            "precision basis",
        ),
        parameter_fact_identity: format!("parameter:{label}"),
        parameter,
        participation_row_identity: format!("row:{label}"),
        segment_pair_identities: vec![format!("segment-pair:{point_event_identity}")],
        participating_carrier_identities: vec![carrier_identity.to_string()],
        event_endpoint_source_identities: vec![
            "endpoint:start".to_string(),
            "endpoint:end".to_string(),
        ],
        event_endpoint_projection_fact_digests: vec![
            "projection:start".to_string(),
            "projection:end".to_string(),
        ],
        predicate_receipt_identities: vec![format!("predicate:{point_event_identity}")],
        event_group_identities: vec![format!("event-group:{point_event_identity}")],
        shared_endpoint_source_identities: Vec::new(),
        shared_endpoint_projection_fact_digests: Vec::new(),
        start_source_endpoint_identity: "endpoint:start".to_string(),
        start_projected_endpoint_fact_identity: "projection:start".to_string(),
        end_source_endpoint_identity: "endpoint:end".to_string(),
        end_projected_endpoint_fact_identity: "projection:end".to_string(),
    })
}

pub(crate) fn point_candidate_with_source_edge(
    label: &str,
    carrier_identity: &str,
    source_edge_identity: &str,
    parameter: f64,
) -> PlanarBooleanPointSplitCandidate {
    let candidate = point_candidate(
        label,
        &format!("event:{label}"),
        carrier_identity,
        parameter,
        PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
    );
    PlanarBooleanPointSplitCandidate::new(PlanarBooleanPointSplitCandidateInput {
        source_edge_identity: source_edge_identity.to_string(),
        ..candidate_input_from(candidate)
    })
}

pub(crate) fn point_candidate_with_frame_precision(
    label: &str,
    point_event_identity: &str,
    carrier_identity: &str,
    parameter: f64,
    local_frame_identity: &str,
    precision_basis_identity: &str,
) -> PlanarBooleanPointSplitCandidate {
    let candidate = point_candidate(
        label,
        point_event_identity,
        carrier_identity,
        parameter,
        PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
    );
    PlanarBooleanPointSplitCandidate::new(PlanarBooleanPointSplitCandidateInput {
        coordinate_fact: PlanarBooleanPointEventCoordinateFact::new(
            [parameter, parameter],
            local_frame_identity,
            precision_basis_identity,
        ),
        ..candidate_input_from(candidate)
    })
}

pub(crate) fn candidate_input_from(
    candidate: PlanarBooleanPointSplitCandidate,
) -> PlanarBooleanPointSplitCandidateInput {
    PlanarBooleanPointSplitCandidateInput {
        candidate_identity: candidate.candidate_identity().to_string(),
        point_event_identity: candidate.point_event_identity().to_string(),
        point_event_kind: candidate.point_event_kind(),
        carrier_identity: candidate.carrier_identity().to_string(),
        source_edge_identity: candidate.source_edge_identity().to_string(),
        segment_identity: candidate.segment_identity().to_string(),
        coordinate_fact: candidate.coordinate_fact().clone(),
        parameter_fact_identity: candidate.parameter_fact_identity().to_string(),
        parameter: candidate.parameter(),
        participation_row_identity: candidate.participation_row_identity().to_string(),
        segment_pair_identities: candidate.segment_pair_identities().to_vec(),
        participating_carrier_identities: candidate.participating_carrier_identities().to_vec(),
        event_endpoint_source_identities: candidate.event_endpoint_source_identities().to_vec(),
        event_endpoint_projection_fact_digests: candidate
            .event_endpoint_projection_fact_digests()
            .to_vec(),
        predicate_receipt_identities: candidate.predicate_receipt_identities().to_vec(),
        event_group_identities: candidate.event_group_identities().to_vec(),
        shared_endpoint_source_identities: candidate.shared_endpoint_source_identities().to_vec(),
        shared_endpoint_projection_fact_digests: candidate
            .shared_endpoint_projection_fact_digests()
            .to_vec(),
        start_source_endpoint_identity: candidate.start_source_endpoint_identity().to_string(),
        start_projected_endpoint_fact_identity: candidate
            .start_projected_endpoint_fact_identity()
            .to_string(),
        end_source_endpoint_identity: candidate.end_source_endpoint_identity().to_string(),
        end_projected_endpoint_fact_identity: candidate
            .end_projected_endpoint_fact_identity()
            .to_string(),
    }
}
