#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiSourceIngressDenial {
    reason: WorthUiSourceIngressDenialReason,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiSourceIngressDenialReason {
    EmptyProvider,
    MixedCandidateMaterial,
    MultipleArtifactInputs,
    PartialWriteWithoutStableSnapshot,
    MissingOrderingReceipt,
    OrderingReceiptDrift,
    UnsupportedHookOutput,
    NoCandidateMaterial,
    SourcePackageRejected,
    SourceParseRejected,
    AuthoringEntryRejected,
    ArtifactResolutionRejected,
    StructuralLegalityRejected,
    BindingSemanticsRejected,
    IdentitySeedingRejected,
    CanonicalAssemblyRejected,
}

impl WorthUiSourceIngressDenial {
    pub(crate) fn new(reason: WorthUiSourceIngressDenialReason) -> Self {
        Self { reason }
    }

    pub fn reason(&self) -> WorthUiSourceIngressDenialReason {
        self.reason
    }
}
