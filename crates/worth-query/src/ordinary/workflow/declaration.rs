use crate::evidence_identity::{
    WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};
use crate::ordinary::mutation::WorthQueryMutationDeclaration;
use crate::session_label::WorthQuerySessionLabel;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryWorkflowFamily {
    PreviewPromotion,
    DeferredWriteback,
}

impl WorthQueryWorkflowFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PreviewPromotion => "preview-promotion",
            Self::DeferredWriteback => "deferred-writeback",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryWorkflowDeclarationIdentity {
    identity: WorthQueryEvidenceIdentity,
}

impl WorthQueryWorkflowDeclarationIdentity {
    pub fn as_str(&self) -> &str {
        self.identity.as_str()
    }

    pub fn evidence_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.identity
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryWorkflowDeclaration {
    identity: WorthQueryWorkflowDeclarationIdentity,
    family: WorthQueryWorkflowFamily,
    label: WorthQuerySessionLabel,
    mutation: WorthQueryMutationDeclaration,
}

impl WorthQueryWorkflowDeclaration {
    pub fn identity(&self) -> &WorthQueryWorkflowDeclarationIdentity {
        &self.identity
    }

    pub fn family(&self) -> WorthQueryWorkflowFamily {
        self.family
    }

    pub fn session_label(&self) -> &WorthQuerySessionLabel {
        &self.label
    }

    pub fn deferred_writeback(mut self) -> Self {
        self.family = WorthQueryWorkflowFamily::DeferredWriteback;
        self.identity = workflow_identity(self.family, &self.label, &self.mutation);
        self
    }

    pub(crate) fn mutation(&self) -> &WorthQueryMutationDeclaration {
        &self.mutation
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        WorthQueryWorkflowDeclarationIdentity,
        WorthQueryWorkflowFamily,
        WorthQuerySessionLabel,
        WorthQueryMutationDeclaration,
    ) {
        (self.identity, self.family, self.label, self.mutation)
    }
}

pub fn declare(
    label: WorthQuerySessionLabel,
    mutation: WorthQueryMutationDeclaration,
) -> WorthQueryWorkflowDeclaration {
    let family = WorthQueryWorkflowFamily::PreviewPromotion;
    WorthQueryWorkflowDeclaration {
        identity: workflow_identity(family, &label, &mutation),
        family,
        label,
        mutation,
    }
}

fn workflow_identity(
    family: WorthQueryWorkflowFamily,
    label: &WorthQuerySessionLabel,
    mutation: &WorthQueryMutationDeclaration,
) -> WorthQueryWorkflowDeclarationIdentity {
    WorthQueryWorkflowDeclarationIdentity {
        identity: WorthQueryEvidenceIdentity::compose(
            WorthQueryEvidenceScope::WorkflowMutationLowering,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("role"),
            "ordinary-workflow-declaration",
        )
        .field_shape(WorthQueryEvidenceTag::new("family"), family.as_str())
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("session_label"),
            label.identity_digest(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("mutation"),
            mutation.identity().evidence_identity(),
        )
        .seal(),
    }
}
