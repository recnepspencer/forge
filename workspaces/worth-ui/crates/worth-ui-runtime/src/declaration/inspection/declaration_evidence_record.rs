use crate::declaration::{UiDeclarationArtifact, UiDeclarationArtifactDigest};
use crate::evidence::{
    evidence_authority_binding, evidence_handle, evidence_identity, evidence_ref,
    UiEvidenceAuthorityGeneration, UiEvidenceAuthorityKind, UiEvidenceFamily, UiEvidenceIdentity,
    UiEvidenceMaterializationPosture, UiEvidenceRef, UiEvidenceRetentionPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiDeclarationEvidenceRecord {
    identity: UiEvidenceIdentity,
    artifact_digest: UiDeclarationArtifactDigest,
}

impl UiDeclarationEvidenceRecord {
    pub(crate) fn for_artifact(artifact: &UiDeclarationArtifact) -> Self {
        Self {
            identity: evidence_identity(
                UiEvidenceFamily::Declaration,
                artifact.identity_digest().raw(),
            ),
            artifact_digest: artifact.artifact_digest(),
        }
    }

    pub(crate) fn bind_ref(
        &self,
        authority_generation: UiEvidenceAuthorityGeneration,
    ) -> UiEvidenceRef {
        let authority_binding = evidence_authority_binding(
            UiEvidenceAuthorityKind::DeclarationArtifact,
            self.artifact_digest.raw(),
            authority_generation,
            None,
        );
        let handle = evidence_handle(
            UiEvidenceFamily::Declaration,
            self.identity,
            self.artifact_digest.raw(),
        );

        evidence_ref(
            UiEvidenceFamily::Declaration,
            self.identity,
            authority_binding,
            UiEvidenceMaterializationPosture::RefsOnly,
            UiEvidenceRetentionPosture::CurrentGenerationOnly,
            handle,
        )
    }
}
