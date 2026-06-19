use crate::workload_platform::planar_boolean_events::PlanarBooleanSourceIntervalSense;
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanFragmentContinuationEndpointRole, PlanarBooleanFragmentContinuationRow,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanClosedWalkCandidateContinuation {
    continuation_identity: String,
    neighborhood_identity: String,
    split_vertex_identity: String,
    fragment_identity: String,
    source_face_identity: String,
    source_loop_carrier_identity: String,
    fragment_endpoint_role: PlanarBooleanFragmentContinuationEndpointRole,
    source_sense: PlanarBooleanSourceIntervalSense,
    local_frame_identity: String,
    precision_basis_identity: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanClosedWalkCandidate {
    closed_walk_candidate_identity: String,
    source_loop_identity: String,
    source_face_identities: Vec<String>,
    source_loop_carrier_identities: Vec<String>,
    source_senses: Vec<PlanarBooleanSourceIntervalSense>,
    fragment_identities: Vec<String>,
    split_vertex_identities: Vec<String>,
    continuations: Vec<PlanarBooleanClosedWalkCandidateContinuation>,
    local_frame_identities: Vec<String>,
    precision_basis_identities: Vec<String>,
}

impl PlanarBooleanClosedWalkCandidateContinuation {
    pub(crate) fn from_continuation_row(row: &PlanarBooleanFragmentContinuationRow) -> Self {
        Self {
            continuation_identity: row.continuation_identity().to_string(),
            neighborhood_identity: row.neighborhood_identity().to_string(),
            split_vertex_identity: row.split_vertex_identity().to_string(),
            fragment_identity: row.fragment_identity().to_string(),
            source_face_identity: row.source_face_identity().to_string(),
            source_loop_carrier_identity: row.source_loop_carrier_identity().to_string(),
            fragment_endpoint_role: row.fragment_endpoint_role(),
            source_sense: row.source_sense(),
            local_frame_identity: row.local_frame_identity().to_string(),
            precision_basis_identity: row.precision_basis_identity().to_string(),
        }
    }

    pub fn continuation_identity(&self) -> &str {
        &self.continuation_identity
    }

    pub fn neighborhood_identity(&self) -> &str {
        &self.neighborhood_identity
    }

    pub fn split_vertex_identity(&self) -> &str {
        &self.split_vertex_identity
    }

    pub fn fragment_identity(&self) -> &str {
        &self.fragment_identity
    }

    pub fn source_face_identity(&self) -> &str {
        &self.source_face_identity
    }

    pub fn source_loop_carrier_identity(&self) -> &str {
        &self.source_loop_carrier_identity
    }

    pub fn fragment_endpoint_role(&self) -> PlanarBooleanFragmentContinuationEndpointRole {
        self.fragment_endpoint_role
    }

    pub fn source_sense(&self) -> PlanarBooleanSourceIntervalSense {
        self.source_sense
    }

    pub fn local_frame_identity(&self) -> &str {
        &self.local_frame_identity
    }

    pub fn precision_basis_identity(&self) -> &str {
        &self.precision_basis_identity
    }
}

impl PlanarBooleanClosedWalkCandidate {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        closed_walk_candidate_identity: String,
        source_loop_identity: String,
        source_face_identities: Vec<String>,
        source_loop_carrier_identities: Vec<String>,
        source_senses: Vec<PlanarBooleanSourceIntervalSense>,
        fragment_identities: Vec<String>,
        split_vertex_identities: Vec<String>,
        continuations: Vec<PlanarBooleanClosedWalkCandidateContinuation>,
        local_frame_identities: Vec<String>,
        precision_basis_identities: Vec<String>,
    ) -> Self {
        Self {
            closed_walk_candidate_identity,
            source_loop_identity,
            source_face_identities,
            source_loop_carrier_identities,
            source_senses,
            fragment_identities,
            split_vertex_identities,
            continuations,
            local_frame_identities,
            precision_basis_identities,
        }
    }

    pub fn closed_walk_candidate_identity(&self) -> &str {
        &self.closed_walk_candidate_identity
    }

    pub fn source_loop_identity(&self) -> &str {
        &self.source_loop_identity
    }

    pub fn source_face_identities(&self) -> &[String] {
        &self.source_face_identities
    }

    pub fn source_loop_carrier_identities(&self) -> &[String] {
        &self.source_loop_carrier_identities
    }

    pub fn source_senses(&self) -> &[PlanarBooleanSourceIntervalSense] {
        &self.source_senses
    }

    pub fn fragment_identities(&self) -> &[String] {
        &self.fragment_identities
    }

    pub fn split_vertex_identities(&self) -> &[String] {
        &self.split_vertex_identities
    }

    pub fn continuations(&self) -> &[PlanarBooleanClosedWalkCandidateContinuation] {
        &self.continuations
    }

    pub fn local_frame_identities(&self) -> &[String] {
        &self.local_frame_identities
    }

    pub fn precision_basis_identities(&self) -> &[String] {
        &self.precision_basis_identities
    }
}
