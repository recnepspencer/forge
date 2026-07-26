#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiSourceIngressDenial {
    reason: WorthUiSourceIngressDenialReason,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiSourceIngressDenialReason {
    EmptyProvider,
    MixedCandidateMaterial,
    MultipleRustAuthoredInputs,
    PartialWriteWithoutStableSnapshot,
    MissingOrderingReceipt,
    OrderingReceiptDrift,
    UnsupportedHookOutput,
    NoCandidateMaterial,
    ArtifactResolutionRejected,
    StructuralLegalityRejected,
    SourceBackedDeclarationProjectionRejected,
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
