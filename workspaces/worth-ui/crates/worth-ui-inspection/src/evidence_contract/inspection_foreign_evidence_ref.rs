#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum UiInspectionQueryForeignEvidenceArtifactKind {
    PrerequisiteEvidence,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum UiInspectionQueryForeignEvidenceKind {
    ProjectionConsumption,
    Inspection,
    CausalExplanation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UiInspectionQueryForeignEvidenceRef {
    kind: UiInspectionQueryForeignEvidenceKind,
    artifact_kind: UiInspectionQueryForeignEvidenceArtifactKind,
    artifact_identity_digest: u64,
    obligation_handle_digest: u64,
    graph_node_digest: u64,
    touch_identity_digest: Option<u64>,
}

impl UiInspectionQueryForeignEvidenceRef {
    pub const fn new(
        kind: UiInspectionQueryForeignEvidenceKind,
        artifact_kind: UiInspectionQueryForeignEvidenceArtifactKind,
        artifact_identity_digest: u64,
        obligation_handle_digest: u64,
        graph_node_digest: u64,
        touch_identity_digest: Option<u64>,
    ) -> Self {
        Self {
            kind,
            artifact_kind,
            artifact_identity_digest,
            obligation_handle_digest,
            graph_node_digest,
            touch_identity_digest,
        }
    }

    pub const fn kind(self) -> UiInspectionQueryForeignEvidenceKind {
        self.kind
    }

    pub const fn artifact_kind(self) -> UiInspectionQueryForeignEvidenceArtifactKind {
        self.artifact_kind
    }

    pub const fn artifact_identity_digest(self) -> u64 {
        self.artifact_identity_digest
    }

    pub const fn obligation_handle_digest(self) -> u64 {
        self.obligation_handle_digest
    }

    pub const fn graph_node_digest(self) -> u64 {
        self.graph_node_digest
    }

    pub const fn touch_identity_digest(self) -> Option<u64> {
        self.touch_identity_digest
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum UiInspectionForeignEvidenceRef {
    Query(UiInspectionQueryForeignEvidenceRef),
}
