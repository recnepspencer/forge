use crate::evidence_identity::{
    WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};
use crate::ordinary::mutation::WorthQueryMutationDeclaration;
use crate::ordinary::workflow;
use crate::session_label::WorthQuerySessionLabel;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryReadOnlyPreviewDeclaration {
    pub(crate) identity: WorthQueryEvidenceIdentity,
    pub(crate) label: WorthQuerySessionLabel,
}

impl WorthQueryReadOnlyPreviewDeclaration {
    pub fn identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.identity
    }

    pub fn session_label(&self) -> &WorthQuerySessionLabel {
        &self.label
    }

    pub fn with_mutation(
        self,
        mutation: WorthQueryMutationDeclaration,
    ) -> WorthQueryPromotionEligiblePreviewDeclaration {
        WorthQueryPromotionEligiblePreviewDeclaration {
            workflow: workflow::declare(self.label, mutation),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryPromotionEligiblePreviewDeclaration {
    pub(crate) workflow: workflow::WorthQueryWorkflowDeclaration,
}

impl WorthQueryPromotionEligiblePreviewDeclaration {
    pub fn identity(&self) -> &workflow::WorthQueryWorkflowDeclarationIdentity {
        self.workflow.identity()
    }

    pub fn session_label(&self) -> &WorthQuerySessionLabel {
        self.workflow.session_label()
    }
}

pub fn declare(label: WorthQuerySessionLabel) -> WorthQueryReadOnlyPreviewDeclaration {
    let identity =
        WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::PreviewExecutionEvidence)
            .field_shape(
                WorthQueryEvidenceTag::new("role"),
                "ordinary-read-only-preview-declaration",
            )
            .field_evidence_identity(
                WorthQueryEvidenceTag::new("session_label"),
                label.identity_digest(),
            )
            .seal();
    WorthQueryReadOnlyPreviewDeclaration { identity, label }
}
