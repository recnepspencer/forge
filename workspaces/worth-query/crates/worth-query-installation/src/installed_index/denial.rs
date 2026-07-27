#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryInstalledPackageIndexDenialKind {
    ConflictingPackage,
    ConflictingAdmissionProfile,
    ConflictingDefinition,
    DomainNotInstalled,
    ForeignRuntime,
    StaleGeneration,
    PackageIdentityChanged,
    AdmissionIdentityChanged,
    AuthorityMismatch,
    OperationNotInstalled,
    OperationSemanticsChanged,
    ConflictingArtifactContract,
    ArtifactContractNotInstalled,
    ArtifactContractSemanticsChanged,
    ConflictingApplicationSchema,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryInstalledPackageIndexDenial {
    kind: WorthQueryInstalledPackageIndexDenialKind,
    subject: String,
}

impl WorthQueryInstalledPackageIndexDenial {
    pub(crate) fn new(
        kind: WorthQueryInstalledPackageIndexDenialKind,
        subject: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            subject: subject.into(),
        }
    }

    pub fn kind(&self) -> WorthQueryInstalledPackageIndexDenialKind {
        self.kind
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }
}
