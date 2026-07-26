#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum WorthQueryDomainEvidenceContract {
    #[default]
    NotRequired,
    InstalledArtifact(crate::domain_computation::WorthQueryArtifactContractReference),
}

impl WorthQueryDomainEvidenceContract {
    pub const fn not_required() -> Self {
        Self::NotRequired
    }

    pub fn installed_artifact(
        reference: crate::domain_computation::WorthQueryArtifactContractReference,
    ) -> Self {
        Self::InstalledArtifact(reference)
    }

    pub fn artifact_reference(
        &self,
    ) -> Option<&crate::domain_computation::WorthQueryArtifactContractReference> {
        match self {
            Self::NotRequired => None,
            Self::InstalledArtifact(reference) => Some(reference),
        }
    }

    pub(crate) fn canonical_token(&self) -> String {
        match self {
            Self::NotRequired => "not-required".into(),
            Self::InstalledArtifact(reference) => format!(
                "installed-artifact:{}:{}:{}",
                reference.family().as_str(),
                reference.schema_version().get(),
                reference.protocol_version().get()
            ),
        }
    }
}
