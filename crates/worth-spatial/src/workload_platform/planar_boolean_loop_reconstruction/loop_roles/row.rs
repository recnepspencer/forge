use crate::workload_platform::planar_boolean_events::PlanarBooleanLoopRole;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PlanarBooleanLoopClassifiedProductKind {
    BornLoop,
    ReconstructedLoop,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanLoopRoleOutcomeKind {
    PreservedSourceRole,
    SingleSourceBornLoopRoleDerivedFromEvidence,
    BornLoopRoleAmbiguous,
    ContradictorySourceRoleEvidence,
    MissingSourceRoleEvidence,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanLoopContainmentEvidencePostureKind {
    PreservedSourceContainmentEvidence,
    SplitSourceContainmentEvidence,
    SingleSourceBornLoopContainmentEvidence,
    MultiSourceBornLoopContainmentEvidence,
    ContradictorySourceContainmentEvidence,
    MissingSourceContainmentEvidence,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopRoleOutcome {
    role_outcome_identity: String,
    loop_identity: String,
    loop_kind: PlanarBooleanLoopClassifiedProductKind,
    island_identities: Vec<String>,
    source_loop_identities: Vec<String>,
    preserved_source_role: Option<PlanarBooleanLoopRole>,
    kind: PlanarBooleanLoopRoleOutcomeKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopContainmentEvidencePosture {
    containment_posture_identity: String,
    loop_identity: String,
    loop_kind: PlanarBooleanLoopClassifiedProductKind,
    island_identities: Vec<String>,
    source_loop_identities: Vec<String>,
    kind: PlanarBooleanLoopContainmentEvidencePostureKind,
}

impl PlanarBooleanLoopRoleOutcome {
    pub(crate) fn new(
        role_outcome_identity: String,
        loop_identity: String,
        loop_kind: PlanarBooleanLoopClassifiedProductKind,
        island_identities: Vec<String>,
        source_loop_identities: Vec<String>,
        preserved_source_role: Option<PlanarBooleanLoopRole>,
        kind: PlanarBooleanLoopRoleOutcomeKind,
    ) -> Self {
        Self {
            role_outcome_identity,
            loop_identity,
            loop_kind,
            island_identities,
            source_loop_identities,
            preserved_source_role,
            kind,
        }
    }

    pub fn role_outcome_identity(&self) -> &str {
        &self.role_outcome_identity
    }

    pub fn loop_identity(&self) -> &str {
        &self.loop_identity
    }

    pub fn loop_kind(&self) -> PlanarBooleanLoopClassifiedProductKind {
        self.loop_kind
    }

    pub fn island_identities(&self) -> &[String] {
        &self.island_identities
    }

    pub fn source_loop_identities(&self) -> &[String] {
        &self.source_loop_identities
    }

    pub fn preserved_source_role(&self) -> Option<PlanarBooleanLoopRole> {
        self.preserved_source_role
    }

    pub fn kind(&self) -> PlanarBooleanLoopRoleOutcomeKind {
        self.kind
    }
}

impl PlanarBooleanLoopContainmentEvidencePosture {
    pub(crate) fn new(
        containment_posture_identity: String,
        loop_identity: String,
        loop_kind: PlanarBooleanLoopClassifiedProductKind,
        island_identities: Vec<String>,
        source_loop_identities: Vec<String>,
        kind: PlanarBooleanLoopContainmentEvidencePostureKind,
    ) -> Self {
        Self {
            containment_posture_identity,
            loop_identity,
            loop_kind,
            island_identities,
            source_loop_identities,
            kind,
        }
    }

    pub fn containment_posture_identity(&self) -> &str {
        &self.containment_posture_identity
    }

    pub fn loop_identity(&self) -> &str {
        &self.loop_identity
    }

    pub fn loop_kind(&self) -> PlanarBooleanLoopClassifiedProductKind {
        self.loop_kind
    }

    pub fn island_identities(&self) -> &[String] {
        &self.island_identities
    }

    pub fn source_loop_identities(&self) -> &[String] {
        &self.source_loop_identities
    }

    pub fn kind(&self) -> PlanarBooleanLoopContainmentEvidencePostureKind {
        self.kind
    }
}
