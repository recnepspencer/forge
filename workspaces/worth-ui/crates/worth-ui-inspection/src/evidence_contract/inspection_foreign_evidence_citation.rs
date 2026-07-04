use super::{
    UiInspectionForeignEvidenceRef, UiInspectionQueryForeignEvidenceArtifactKind,
    UiInspectionQueryForeignEvidenceKind,
    UiInspectionQueryForeignEvidenceRef,
};
use worth_ui_query_binding::WorthUiQueryPrerequisiteEvidence;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiInspectionQueryForeignEvidenceCitation {
    foreign_ref: UiInspectionQueryForeignEvidenceRef,
    prerequisite_evidence: Option<WorthUiQueryPrerequisiteEvidence>,
}

impl UiInspectionQueryForeignEvidenceCitation {
    pub fn new(
        foreign_ref: UiInspectionQueryForeignEvidenceRef,
        prerequisite_evidence: Option<WorthUiQueryPrerequisiteEvidence>,
    ) -> Self {
        Self {
            foreign_ref,
            prerequisite_evidence,
        }
    }

    pub fn foreign_ref(&self) -> UiInspectionQueryForeignEvidenceRef {
        self.foreign_ref
    }

    pub fn kind(&self) -> UiInspectionQueryForeignEvidenceKind {
        self.foreign_ref.kind()
    }

    pub fn artifact_kind(&self) -> UiInspectionQueryForeignEvidenceArtifactKind {
        self.foreign_ref.artifact_kind()
    }

    pub fn artifact_identity_digest(&self) -> u64 {
        self.foreign_ref.artifact_identity_digest()
    }

    pub fn obligation_handle_digest(&self) -> u64 {
        self.foreign_ref.obligation_handle_digest()
    }

    pub fn graph_node_digest(&self) -> u64 {
        self.foreign_ref.graph_node_digest()
    }

    pub fn touch_identity_digest(&self) -> Option<u64> {
        self.foreign_ref.touch_identity_digest()
    }

    pub fn prerequisite_evidence(&self) -> Option<&WorthUiQueryPrerequisiteEvidence> {
        self.prerequisite_evidence.as_ref()
    }

    pub fn is_available(&self) -> bool {
        self.prerequisite_evidence.is_some()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiInspectionForeignEvidenceCitation {
    Query(UiInspectionQueryForeignEvidenceCitation),
}

impl UiInspectionForeignEvidenceCitation {
    pub fn foreign_ref(&self) -> UiInspectionForeignEvidenceRef {
        match self {
            Self::Query(citation) => UiInspectionForeignEvidenceRef::Query(citation.foreign_ref()),
        }
    }
}
