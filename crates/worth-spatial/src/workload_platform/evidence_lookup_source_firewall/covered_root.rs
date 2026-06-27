#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupSourceFirewallCoveredRootKind {
    PublicFacadeVocabulary,
    LegacyLedgerSurface,
    LegacyStageIndexSurface,
    RawEvidenceRowSurface,
    SpatialTouchAdmissionLane,
    DocumentationReportCodec,
    CertificationCodec,
    QueryAdoptionSurface,
    KernelResidueSurface,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupSourceFirewallCoveredRoot {
    source_path: String,
    kind: EvidenceLookupSourceFirewallCoveredRootKind,
}

impl EvidenceLookupSourceFirewallCoveredRoot {
    pub(crate) fn new(
        source_path: impl Into<String>,
        kind: EvidenceLookupSourceFirewallCoveredRootKind,
    ) -> Self {
        Self {
            source_path: source_path.into(),
            kind,
        }
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub const fn kind(&self) -> EvidenceLookupSourceFirewallCoveredRootKind {
        self.kind
    }
}
