use crate::workload_platform::planar_boolean_events::PlanarBooleanSourceIntervalSense;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopCandidate {
    loop_candidate_identity: String,
    walk_outcome_identity: String,
    source_loop_identity: String,
    source_face_identity: String,
    local_frame_identity: String,
    precision_basis_identity: String,
    source_senses: Vec<PlanarBooleanSourceIntervalSense>,
    fragment_identities: Vec<String>,
    split_vertex_identities: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanDeniedLoopCandidateKind {
    LineageContradiction,
    InsufficientCardinality,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanDeniedLoopCandidate {
    denied_loop_candidate_identity: String,
    walk_outcome_identity: String,
    source_loop_identity: String,
    kind: PlanarBooleanDeniedLoopCandidateKind,
    fragment_identities: Vec<String>,
    split_vertex_identities: Vec<String>,
    human_reason: String,
}

impl PlanarBooleanLoopCandidate {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        loop_candidate_identity: String,
        walk_outcome_identity: String,
        source_loop_identity: String,
        source_face_identity: String,
        local_frame_identity: String,
        precision_basis_identity: String,
        source_senses: Vec<PlanarBooleanSourceIntervalSense>,
        fragment_identities: Vec<String>,
        split_vertex_identities: Vec<String>,
    ) -> Self {
        Self {
            loop_candidate_identity,
            walk_outcome_identity,
            source_loop_identity,
            source_face_identity,
            local_frame_identity,
            precision_basis_identity,
            source_senses,
            fragment_identities,
            split_vertex_identities,
        }
    }

    pub fn loop_candidate_identity(&self) -> &str {
        &self.loop_candidate_identity
    }

    pub fn walk_outcome_identity(&self) -> &str {
        &self.walk_outcome_identity
    }

    pub fn source_loop_identity(&self) -> &str {
        &self.source_loop_identity
    }

    pub fn source_face_identity(&self) -> &str {
        &self.source_face_identity
    }

    pub fn local_frame_identity(&self) -> &str {
        &self.local_frame_identity
    }

    pub fn precision_basis_identity(&self) -> &str {
        &self.precision_basis_identity
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
}

impl PlanarBooleanDeniedLoopCandidate {
    pub(crate) fn new(
        denied_loop_candidate_identity: String,
        walk_outcome_identity: String,
        source_loop_identity: String,
        kind: PlanarBooleanDeniedLoopCandidateKind,
        fragment_identities: Vec<String>,
        split_vertex_identities: Vec<String>,
        human_reason: String,
    ) -> Self {
        Self {
            denied_loop_candidate_identity,
            walk_outcome_identity,
            source_loop_identity,
            kind,
            fragment_identities,
            split_vertex_identities,
            human_reason,
        }
    }

    pub fn denied_loop_candidate_identity(&self) -> &str {
        &self.denied_loop_candidate_identity
    }

    pub fn walk_outcome_identity(&self) -> &str {
        &self.walk_outcome_identity
    }

    pub fn source_loop_identity(&self) -> &str {
        &self.source_loop_identity
    }

    pub fn kind(&self) -> PlanarBooleanDeniedLoopCandidateKind {
        self.kind
    }

    pub fn fragment_identities(&self) -> &[String] {
        &self.fragment_identities
    }

    pub fn split_vertex_identities(&self) -> &[String] {
        &self.split_vertex_identities
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
