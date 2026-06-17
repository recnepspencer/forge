use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBridgeMutationArtifactIdentity {
    identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryBridgeMutationArtifactIdentity {
    pub(crate) fn imported(role: &'static str, artifact: impl Into<String>) -> Self {
        let artifact = artifact.into();
        let identity =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceSourceDigest)
                .field_shape(ForgeQueryEvidenceTag::new("role"), role)
                .field_value(
                    ForgeQueryEvidenceTag::new("imported_artifact"),
                    artifact.as_str(),
                )
                .seal();
        Self { identity }
    }

    pub fn evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.identity
    }
}
