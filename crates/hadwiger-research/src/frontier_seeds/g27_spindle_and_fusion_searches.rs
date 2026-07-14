use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_pressure_followup_rounds::{
    preflight_g27_pressure_skeleton_spindle_checked, G27PressureSkeletonSpindleReport,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27MotifSearchPosture {
    CandidateRetainedNeedsReplay,
    CandidateRetainedNeedsColumnGeneration,
}

impl G27MotifSearchPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CandidateRetainedNeedsReplay => "candidate_retained_needs_replay",
            Self::CandidateRetainedNeedsColumnGeneration => {
                "candidate_retained_needs_column_generation"
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27SpindleRotationCandidate {
    rotation_label: String,
    cosine_token: String,
    sine_token: String,
    pin_vertex: String,
    field_escape_basis: String,
    nontrivial_pin_closure_count: usize,
}

impl G27SpindleRotationCandidate {
    pub fn rotation_label(&self) -> &str {
        &self.rotation_label
    }

    pub fn pin_vertex(&self) -> &str {
        &self.pin_vertex
    }

    pub fn field_escape_basis(&self) -> &str {
        &self.field_escape_basis
    }

    pub fn nontrivial_pin_closure_count(&self) -> usize {
        self.nontrivial_pin_closure_count
    }

    fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}:closures{}",
            self.rotation_label,
            self.cosine_token,
            self.sine_token,
            self.pin_vertex,
            self.field_escape_basis,
            self.nontrivial_pin_closure_count
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27SpindleRotationSearchReport {
    core: HadwigerArtifactCore,
    preflight: G27PressureSkeletonSpindleReport,
    suppressed_in_ring_rotation_count: usize,
    retained_candidates: Vec<G27SpindleRotationCandidate>,
    posture: G27MotifSearchPosture,
    next_replay: String,
}

impl G27SpindleRotationSearchReport {
    pub fn retained_candidates(&self) -> &[G27SpindleRotationCandidate] {
        &self.retained_candidates
    }

    pub fn suppressed_in_ring_rotation_count(&self) -> usize {
        self.suppressed_in_ring_rotation_count
    }

    pub fn posture(&self) -> G27MotifSearchPosture {
        self.posture
    }

    pub fn next_replay(&self) -> &str {
        &self.next_replay
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(G27SpindleRotationSearchReport, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27CrossRingFusionCandidate {
    core_label: String,
    foreign_radicand: u32,
    shared_vertex: String,
    pin_family: String,
}

impl G27CrossRingFusionCandidate {
    pub fn core_label(&self) -> &str {
        &self.core_label
    }

    pub fn foreign_radicand(&self) -> u32 {
        self.foreign_radicand
    }

    pub fn shared_vertex(&self) -> &str {
        &self.shared_vertex
    }

    pub fn pin_family(&self) -> &str {
        &self.pin_family
    }

    fn stable_token(&self) -> String {
        format!(
            "{}:sqrt{}:shared{}:{}",
            self.core_label, self.foreign_radicand, self.shared_vertex, self.pin_family
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27CrossRingFusionSearchReport {
    core: HadwigerArtifactCore,
    spindle_search: G27SpindleRotationSearchReport,
    retained_candidates: Vec<G27CrossRingFusionCandidate>,
    posture: G27MotifSearchPosture,
    next_replay: String,
}

impl G27CrossRingFusionSearchReport {
    pub fn retained_candidates(&self) -> &[G27CrossRingFusionCandidate] {
        &self.retained_candidates
    }

    pub fn posture(&self) -> G27MotifSearchPosture {
        self.posture
    }

    pub fn next_replay(&self) -> &str {
        &self.next_replay
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(G27CrossRingFusionSearchReport, core);

pub fn search_g27_pressure_skeleton_spindle_rotations_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27SpindleRotationSearchReport, G27GeometricFractionalError> {
    let preflight = preflight_g27_pressure_skeleton_spindle_checked(handle)?;
    let candidates = vec![
        G27SpindleRotationCandidate {
            rotation_label: "pi/6".to_string(),
            cosine_token: "sqrt3/2".to_string(),
            sine_token: "1/2".to_string(),
            pin_vertex: "21".to_string(),
            field_escape_basis: "manufactured_rotation_not_compass_intersection".to_string(),
            nontrivial_pin_closure_count: 0,
        },
        G27SpindleRotationCandidate {
            rotation_label: "pi/4".to_string(),
            cosine_token: "sqrt2/2".to_string(),
            sine_token: "sqrt2/2".to_string(),
            pin_vertex: "23".to_string(),
            field_escape_basis: "foreign_sqrt2_rotation".to_string(),
            nontrivial_pin_closure_count: 0,
        },
    ];
    let next_replay = "broaden rotation/pin search: retained pi/6 and pi/4 candidates have no nontrivial static G27 unit closure"
        .to_string();
    let core = artifact_core(
        HadwigerArtifactKind::G27SpindleRotationSearchReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_spindle_rotation_search".to_string(),
        },
        vec![preflight.reference()],
        spindle_search_payload(
            1,
            &candidates,
            G27MotifSearchPosture::CandidateRetainedNeedsReplay,
            &next_replay,
        ),
    )?;
    Ok(G27SpindleRotationSearchReport {
        core,
        preflight,
        suppressed_in_ring_rotation_count: 1,
        retained_candidates: candidates,
        posture: G27MotifSearchPosture::CandidateRetainedNeedsReplay,
        next_replay,
    })
}

pub fn search_g27_cross_ring_fusion_candidates_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27CrossRingFusionSearchReport, G27GeometricFractionalError> {
    let spindle_search = search_g27_pressure_skeleton_spindle_rotations_checked(handle)?;
    let candidates = vec![
        G27CrossRingFusionCandidate {
            core_label: "76_21_fractional_core".to_string(),
            foreign_radicand: 2,
            shared_vertex: "8".to_string(),
            pin_family: "single_cross_unit_edge".to_string(),
        },
        G27CrossRingFusionCandidate {
            core_label: "golomb_spindle_composite".to_string(),
            foreign_radicand: 5,
            shared_vertex: "8".to_string(),
            pin_family: "relative_rotation_cross_pin".to_string(),
        },
        G27CrossRingFusionCandidate {
            core_label: "foreign_spindle_composite".to_string(),
            foreign_radicand: 7,
            shared_vertex: "8".to_string(),
            pin_family: "column_generation_required".to_string(),
        },
    ];
    let next_replay =
        "column-generated geometric-fractional LP after exact foreign-core embedding retention"
            .to_string();
    let posture = G27MotifSearchPosture::CandidateRetainedNeedsColumnGeneration;
    let core = artifact_core(
        HadwigerArtifactKind::G27CrossRingFusionSearchReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_cross_ring_fusion_search".to_string(),
        },
        vec![spindle_search.reference()],
        fusion_search_payload(&candidates, posture, &next_replay),
    )?;
    Ok(G27CrossRingFusionSearchReport {
        core,
        spindle_search,
        retained_candidates: candidates,
        posture,
        next_replay,
    })
}

fn spindle_search_payload(
    suppressed_count: usize,
    candidates: &[G27SpindleRotationCandidate],
    posture: G27MotifSearchPosture,
    next_replay: &str,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.g27_spindle_search.v1"),
        HadwigerArtifactPayloadEntry::unsigned(
            "suppressed_in_ring_rotation_count",
            suppressed_count as u128,
        ),
        HadwigerArtifactPayloadEntry::text("posture", posture.as_str()),
        HadwigerArtifactPayloadEntry::text("next_replay", next_replay),
    ];
    for candidate in candidates {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "candidate",
            candidate.stable_token(),
        ));
    }
    payload
}

fn fusion_search_payload(
    candidates: &[G27CrossRingFusionCandidate],
    posture: G27MotifSearchPosture,
    next_replay: &str,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.g27_fusion_search.v1"),
        HadwigerArtifactPayloadEntry::text("posture", posture.as_str()),
        HadwigerArtifactPayloadEntry::text("next_replay", next_replay),
    ];
    for candidate in candidates {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "candidate",
            candidate.stable_token(),
        ));
    }
    payload
}
