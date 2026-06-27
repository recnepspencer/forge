#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupForbiddenAuthorityKind {
    RawEvidenceVectorAccess,
    BroadReceiptScan,
    CopiedDigestLookup,
    StageLocalNearbyLookup,
    QueryLookupProductSubstitution,
    PublicEvidenceRowExposure,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupSourceFirewallExceptionKind {
    CertificationOnlyCodec,
    DocumentationReportCodec,
    TestSupportFixture,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupSourceFirewallRowPosture {
    ForbiddenProductionAuthority,
    AllowedNamedException,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupSourceFirewallRow {
    source_path: String,
    matched_surface: String,
    forbidden_authority_kind: EvidenceLookupForbiddenAuthorityKind,
    posture: EvidenceLookupSourceFirewallRowPosture,
    exception_kind: Option<EvidenceLookupSourceFirewallExceptionKind>,
    reason: String,
}

impl EvidenceLookupSourceFirewallRow {
    pub(crate) fn forbidden(
        source_path: impl Into<String>,
        matched_surface: impl Into<String>,
        forbidden_authority_kind: EvidenceLookupForbiddenAuthorityKind,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            source_path: source_path.into(),
            matched_surface: matched_surface.into(),
            forbidden_authority_kind,
            posture: EvidenceLookupSourceFirewallRowPosture::ForbiddenProductionAuthority,
            exception_kind: None,
            reason: reason.into(),
        }
    }

    pub(crate) fn allowed_exception(
        source_path: impl Into<String>,
        matched_surface: impl Into<String>,
        forbidden_authority_kind: EvidenceLookupForbiddenAuthorityKind,
        exception_kind: EvidenceLookupSourceFirewallExceptionKind,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            source_path: source_path.into(),
            matched_surface: matched_surface.into(),
            forbidden_authority_kind,
            posture: EvidenceLookupSourceFirewallRowPosture::AllowedNamedException,
            exception_kind: Some(exception_kind),
            reason: reason.into(),
        }
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub fn matched_surface(&self) -> &str {
        &self.matched_surface
    }

    pub const fn forbidden_authority_kind(&self) -> EvidenceLookupForbiddenAuthorityKind {
        self.forbidden_authority_kind
    }

    pub const fn posture(&self) -> EvidenceLookupSourceFirewallRowPosture {
        self.posture
    }

    pub const fn exception_kind(&self) -> Option<EvidenceLookupSourceFirewallExceptionKind> {
        self.exception_kind
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub const fn claims_lookup_execution_authority(&self) -> bool {
        false
    }
}

impl EvidenceLookupForbiddenAuthorityKind {
    pub(crate) const fn as_digest_label(self) -> &'static str {
        match self {
            Self::RawEvidenceVectorAccess => "raw-evidence-vector-access",
            Self::BroadReceiptScan => "broad-receipt-scan",
            Self::CopiedDigestLookup => "copied-digest-lookup",
            Self::StageLocalNearbyLookup => "stage-local-nearby-lookup",
            Self::QueryLookupProductSubstitution => "query-lookup-product-substitution",
            Self::PublicEvidenceRowExposure => "public-evidence-row-exposure",
        }
    }
}

impl EvidenceLookupSourceFirewallExceptionKind {
    pub(crate) const fn as_digest_label(self) -> &'static str {
        match self {
            Self::CertificationOnlyCodec => "certification-only-codec",
            Self::DocumentationReportCodec => "documentation-report-codec",
            Self::TestSupportFixture => "test-support-fixture",
        }
    }
}
