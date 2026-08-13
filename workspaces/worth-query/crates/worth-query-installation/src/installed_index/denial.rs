#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryInstalledPackageIndexDenialKind {
    AuthorityEntropyUnavailable,
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
    ConflictingConditionalApplicationOperation,
    ConditionalApplicationOperationNotInstalled,
    ConditionalApplicationOperationMeaningChanged,
    CanonicalEntryBudgetExceeded,
    CanonicalEncodedByteBudgetExceeded,
    CanonicalDigestSlotRejected,
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
