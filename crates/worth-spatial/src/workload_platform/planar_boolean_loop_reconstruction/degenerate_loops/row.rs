use crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanLoopClassifiedProductKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanDegenerateLoopOutcomeKind {
    AdmittedForIdentityMinting,
    DeniedTinyCardinality,
    DeniedSelfTouching,
    DeniedZeroArea,
    PolicyRequiredGeometryEvidence,
    PolicyRequiredRoleEvidence,
    PolicyRequiredContainmentEvidence,
}

impl PlanarBooleanDegenerateLoopOutcomeKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdmittedForIdentityMinting => "admitted-for-identity-minting",
            Self::DeniedTinyCardinality => "denied-tiny-cardinality",
            Self::DeniedSelfTouching => "denied-self-touching",
            Self::DeniedZeroArea => "denied-zero-area",
            Self::PolicyRequiredGeometryEvidence => "policy-required-geometry-evidence",
            Self::PolicyRequiredRoleEvidence => "policy-required-role-evidence",
            Self::PolicyRequiredContainmentEvidence => "policy-required-containment-evidence",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanDegenerateLoopOutcome {
    degenerate_loop_outcome_identity: String,
    loop_identity: String,
    loop_kind: PlanarBooleanLoopClassifiedProductKind,
    source_loop_identities: Vec<String>,
    local_frame_identity: String,
    precision_basis_identity: String,
    fragment_identities: Vec<String>,
    split_vertex_identities: Vec<String>,
    role_outcome_identity: Option<String>,
    containment_posture_identity: Option<String>,
    kind: PlanarBooleanDegenerateLoopOutcomeKind,
    human_reason: String,
}

impl PlanarBooleanDegenerateLoopOutcome {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        degenerate_loop_outcome_identity: String,
        loop_identity: String,
        loop_kind: PlanarBooleanLoopClassifiedProductKind,
        source_loop_identities: Vec<String>,
        local_frame_identity: String,
        precision_basis_identity: String,
        fragment_identities: Vec<String>,
        split_vertex_identities: Vec<String>,
        role_outcome_identity: Option<String>,
        containment_posture_identity: Option<String>,
        kind: PlanarBooleanDegenerateLoopOutcomeKind,
        human_reason: String,
    ) -> Self {
        Self {
            degenerate_loop_outcome_identity,
            loop_identity,
            loop_kind,
            source_loop_identities,
            local_frame_identity,
            precision_basis_identity,
            fragment_identities,
            split_vertex_identities,
            role_outcome_identity,
            containment_posture_identity,
            kind,
            human_reason,
        }
    }

    pub fn degenerate_loop_outcome_identity(&self) -> &str {
        &self.degenerate_loop_outcome_identity
    }

    pub fn loop_identity(&self) -> &str {
        &self.loop_identity
    }

    pub fn loop_kind(&self) -> PlanarBooleanLoopClassifiedProductKind {
        self.loop_kind
    }

    pub fn source_loop_identities(&self) -> &[String] {
        &self.source_loop_identities
    }

    pub fn local_frame_identity(&self) -> &str {
        &self.local_frame_identity
    }

    pub fn precision_basis_identity(&self) -> &str {
        &self.precision_basis_identity
    }

    pub fn fragment_identities(&self) -> &[String] {
        &self.fragment_identities
    }

    pub fn split_vertex_identities(&self) -> &[String] {
        &self.split_vertex_identities
    }

    pub fn role_outcome_identity(&self) -> Option<&str> {
        self.role_outcome_identity.as_deref()
    }

    pub fn containment_posture_identity(&self) -> Option<&str> {
        self.containment_posture_identity.as_deref()
    }

    pub fn kind(&self) -> PlanarBooleanDegenerateLoopOutcomeKind {
        self.kind
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
