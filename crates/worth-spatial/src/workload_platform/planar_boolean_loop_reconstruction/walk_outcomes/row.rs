use crate::workload_platform::planar_boolean_events::PlanarBooleanSourceIntervalSense;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanWalkOutcomeKind {
    Closed,
    Open,
    Residual,
    Unsupported,
    SelfColliding,
    Denied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanWalkOutcomeCause {
    ClosedTwoSlotCoverage,
    OpenInsufficientSlots,
    ResidualCoverageMismatch,
    UnsupportedBranchMultiplicity,
    UnsupportedOrientationCoverage,
    SelfCollisionSingleFragment,
    DeniedProofMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanWalkOutcomeRow {
    walk_outcome_identity: String,
    closed_walk_candidate_identity: String,
    source_loop_identity: String,
    source_face_identities: Vec<String>,
    source_loop_carrier_identities: Vec<String>,
    source_senses: Vec<PlanarBooleanSourceIntervalSense>,
    fragment_identities: Vec<String>,
    split_vertex_identities: Vec<String>,
    neighborhood_identities: Vec<String>,
    continuation_identities: Vec<String>,
    local_frame_identities: Vec<String>,
    precision_basis_identities: Vec<String>,
    kind: PlanarBooleanWalkOutcomeKind,
    cause: PlanarBooleanWalkOutcomeCause,
    human_reason: String,
}

impl PlanarBooleanWalkOutcomeRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        walk_outcome_identity: String,
        closed_walk_candidate_identity: String,
        source_loop_identity: String,
        source_face_identities: Vec<String>,
        source_loop_carrier_identities: Vec<String>,
        source_senses: Vec<PlanarBooleanSourceIntervalSense>,
        fragment_identities: Vec<String>,
        split_vertex_identities: Vec<String>,
        neighborhood_identities: Vec<String>,
        continuation_identities: Vec<String>,
        local_frame_identities: Vec<String>,
        precision_basis_identities: Vec<String>,
        kind: PlanarBooleanWalkOutcomeKind,
        cause: PlanarBooleanWalkOutcomeCause,
        human_reason: String,
    ) -> Self {
        Self {
            walk_outcome_identity,
            closed_walk_candidate_identity,
            source_loop_identity,
            source_face_identities,
            source_loop_carrier_identities,
            source_senses,
            fragment_identities,
            split_vertex_identities,
            neighborhood_identities,
            continuation_identities,
            local_frame_identities,
            precision_basis_identities,
            kind,
            cause,
            human_reason,
        }
    }

    pub fn walk_outcome_identity(&self) -> &str {
        &self.walk_outcome_identity
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

    pub fn neighborhood_identities(&self) -> &[String] {
        &self.neighborhood_identities
    }

    pub fn continuation_identities(&self) -> &[String] {
        &self.continuation_identities
    }

    pub fn local_frame_identities(&self) -> &[String] {
        &self.local_frame_identities
    }

    pub fn precision_basis_identities(&self) -> &[String] {
        &self.precision_basis_identities
    }

    pub fn kind(&self) -> PlanarBooleanWalkOutcomeKind {
        self.kind
    }

    pub fn cause(&self) -> PlanarBooleanWalkOutcomeCause {
        self.cause
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
