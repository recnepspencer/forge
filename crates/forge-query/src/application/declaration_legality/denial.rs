use forge_foundational::facade::{
    FoundationalBoundaryRoleClaimDenial, FoundationalBoundarySurfaceDispositionDenial,
};

use crate::application::{
    ForgeQueryAsyncFailurePosture, ForgeQueryAsyncLoadingPosture, ForgeQueryAsyncSourceFamily,
    ForgeQueryCanonicalDeclarationArtifact, ForgeQueryDeclarationCapabilityStatus,
    ForgeQueryDeclarationFamilySupportReport, ForgeQueryDeclarationInput,
    ForgeQueryDomainEntryMarker,
};

use super::contract::ForgeQueryDeclarationLegalityContract;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryTemporalLegalityDenialKind {
    RuntimeFacadeDeferred,
    RuntimeFacadeUnsupported,
    HistoricalTruthBasisUnsupported,
    PreviewTruthBasisUnsupported,
    HistoricalSignalBasisUnsupported,
    PreviewSignalBasisUnsupported,
}

impl ForgeQueryTemporalLegalityDenialKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeFacadeDeferred => "runtime_facade_deferred",
            Self::RuntimeFacadeUnsupported => "runtime_facade_unsupported",
            Self::HistoricalTruthBasisUnsupported => "historical_truth_basis_unsupported",
            Self::PreviewTruthBasisUnsupported => "preview_truth_basis_unsupported",
            Self::HistoricalSignalBasisUnsupported => "historical_signal_basis_unsupported",
            Self::PreviewSignalBasisUnsupported => "preview_signal_basis_unsupported",
        }
    }

    pub fn reason(self) -> &'static str {
        match self {
            Self::RuntimeFacadeDeferred => {
                "temporal declaration legality remains deferred until the temporal runtime facade is admitted"
            }
            Self::RuntimeFacadeUnsupported => {
                "temporal declaration legality is unavailable because the temporal runtime facade is unsupported for this operating world"
            }
            Self::HistoricalTruthBasisUnsupported => {
                "temporal declarations cannot currently bind bridge truth context to historical truth"
            }
            Self::PreviewTruthBasisUnsupported => {
                "temporal declarations cannot currently bind bridge truth context to preview truth"
            }
            Self::HistoricalSignalBasisUnsupported => {
                "temporal declarations cannot currently bind signal compatibility to historical basis families"
            }
            Self::PreviewSignalBasisUnsupported => {
                "temporal declarations cannot currently bind signal compatibility to preview-derived basis families"
            }
        }
    }

    pub fn is_deferred(self) -> bool {
        matches!(self, Self::RuntimeFacadeDeferred)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryAsyncLegalityDenialKind {
    RuntimeFacadeDeferred,
    RuntimeFacadeUnsupported,
    UnsupportedSourceFamily(ForgeQueryAsyncSourceFamily),
    UnsupportedLoadingPosture(ForgeQueryAsyncLoadingPosture),
    UnsupportedFailurePosture(ForgeQueryAsyncFailurePosture),
    CompletionLifecycleUnsupported,
    HistoricalTruthBasisUnsupported,
    PreviewTruthBasisUnsupported,
    HistoricalSignalBasisUnsupported,
    PreviewSignalBasisUnsupported,
}

impl ForgeQueryAsyncLegalityDenialKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeFacadeDeferred => "runtime_facade_deferred",
            Self::RuntimeFacadeUnsupported => "runtime_facade_unsupported",
            Self::UnsupportedSourceFamily(_) => "unsupported_source_family",
            Self::UnsupportedLoadingPosture(_) => "unsupported_loading_posture",
            Self::UnsupportedFailurePosture(_) => "unsupported_failure_posture",
            Self::CompletionLifecycleUnsupported => "completion_lifecycle_unsupported",
            Self::HistoricalTruthBasisUnsupported => "historical_truth_basis_unsupported",
            Self::PreviewTruthBasisUnsupported => "preview_truth_basis_unsupported",
            Self::HistoricalSignalBasisUnsupported => "historical_signal_basis_unsupported",
            Self::PreviewSignalBasisUnsupported => "preview_signal_basis_unsupported",
        }
    }

    pub fn reason(self) -> &'static str {
        match self {
            Self::RuntimeFacadeDeferred => {
                "async declaration legality remains deferred until the async-resource runtime facade is admitted"
            }
            Self::RuntimeFacadeUnsupported => {
                "async declaration legality is unavailable because the async-resource runtime facade is unsupported for this operating world"
            }
            Self::UnsupportedSourceFamily(ForgeQueryAsyncSourceFamily::BridgeResource) => {
                "bridge-resource async declarations are not currently admitted for legality projection"
            }
            Self::UnsupportedSourceFamily(ForgeQueryAsyncSourceFamily::ExternalResource) => {
                "external-resource async declarations are not currently admitted for legality projection"
            }
            Self::UnsupportedSourceFamily(ForgeQueryAsyncSourceFamily::HostResource) => {
                "host-resource async declarations are not currently admitted for legality projection"
            }
            Self::UnsupportedLoadingPosture(ForgeQueryAsyncLoadingPosture::Blocking) => {
                "blocking async loading posture is not currently admitted for legality projection"
            }
            Self::UnsupportedLoadingPosture(ForgeQueryAsyncLoadingPosture::BackgroundRefresh) => {
                "background-refresh async loading posture is not currently admitted for legality projection"
            }
            Self::UnsupportedFailurePosture(ForgeQueryAsyncFailurePosture::FailClosed) => {
                "fail-closed async failure posture is not currently admitted for legality projection"
            }
            Self::UnsupportedFailurePosture(ForgeQueryAsyncFailurePosture::RetainStaleValue) => {
                "retain-stale-value async failure posture is not currently admitted for legality projection"
            }
            Self::CompletionLifecycleUnsupported => {
                "completion-request async declarations are not currently admitted for legality projection"
            }
            Self::HistoricalTruthBasisUnsupported => {
                "async declarations cannot currently bind bridge truth context to historical truth"
            }
            Self::PreviewTruthBasisUnsupported => {
                "async declarations cannot currently bind bridge truth context to preview truth"
            }
            Self::HistoricalSignalBasisUnsupported => {
                "async declarations cannot currently bind signal compatibility to historical basis families"
            }
            Self::PreviewSignalBasisUnsupported => {
                "async declarations cannot currently bind signal compatibility to preview-derived basis families"
            }
        }
    }

    pub fn is_deferred(self) -> bool {
        matches!(self, Self::RuntimeFacadeDeferred)
    }
}

#[derive(Debug)]
pub enum ForgeQueryDeclarationLegalityDenial<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    WrongAdmittedWorld {
        declaration: ForgeQueryCanonicalDeclarationArtifact<D, I>,
        expected_handle_identity_digest: String,
        actual_handle_identity_digest: String,
        operating_context_identity_digest: String,
        support_report: ForgeQueryDeclarationFamilySupportReport<D, I::Family>,
        legality_contract: ForgeQueryDeclarationLegalityContract,
    },
    IllegalRoleClaim {
        declaration: ForgeQueryCanonicalDeclarationArtifact<D, I>,
        denial: FoundationalBoundaryRoleClaimDenial,
        operating_context_identity_digest: String,
        support_report: ForgeQueryDeclarationFamilySupportReport<D, I::Family>,
        legality_contract: ForgeQueryDeclarationLegalityContract,
    },
    IllegalSurfaceDisposition {
        declaration: ForgeQueryCanonicalDeclarationArtifact<D, I>,
        denial: FoundationalBoundarySurfaceDispositionDenial,
        operating_context_identity_digest: String,
        support_report: ForgeQueryDeclarationFamilySupportReport<D, I::Family>,
        legality_contract: ForgeQueryDeclarationLegalityContract,
    },
    DeferredByLegalityBoundary {
        declaration: ForgeQueryCanonicalDeclarationArtifact<D, I>,
        operating_context_identity_digest: String,
        support_report: ForgeQueryDeclarationFamilySupportReport<D, I::Family>,
        legality_contract: ForgeQueryDeclarationLegalityContract,
    },
    UnsupportedLegalityClass {
        declaration: ForgeQueryCanonicalDeclarationArtifact<D, I>,
        operating_context_identity_digest: String,
        support_report: ForgeQueryDeclarationFamilySupportReport<D, I::Family>,
        legality_contract: ForgeQueryDeclarationLegalityContract,
    },
    TemporalProjectionUnsupported {
        declaration: ForgeQueryCanonicalDeclarationArtifact<D, I>,
        kind: ForgeQueryTemporalLegalityDenialKind,
        operating_context_identity_digest: String,
        support_report: ForgeQueryDeclarationFamilySupportReport<D, I::Family>,
        legality_contract: ForgeQueryDeclarationLegalityContract,
    },
    AsyncProjectionUnsupported {
        declaration: ForgeQueryCanonicalDeclarationArtifact<D, I>,
        kind: ForgeQueryAsyncLegalityDenialKind,
        operating_context_identity_digest: String,
        support_report: ForgeQueryDeclarationFamilySupportReport<D, I::Family>,
        legality_contract: ForgeQueryDeclarationLegalityContract,
    },
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationLegalityDenial<D, I>
{
    pub fn canonical_declaration(&self) -> &ForgeQueryCanonicalDeclarationArtifact<D, I> {
        match self {
            Self::WrongAdmittedWorld { declaration, .. }
            | Self::IllegalRoleClaim { declaration, .. }
            | Self::IllegalSurfaceDisposition { declaration, .. }
            | Self::DeferredByLegalityBoundary { declaration, .. }
            | Self::UnsupportedLegalityClass { declaration, .. }
            | Self::TemporalProjectionUnsupported { declaration, .. }
            | Self::AsyncProjectionUnsupported { declaration, .. } => declaration,
        }
    }

    pub fn support_report(&self) -> &ForgeQueryDeclarationFamilySupportReport<D, I::Family> {
        match self {
            Self::WrongAdmittedWorld { support_report, .. }
            | Self::IllegalRoleClaim { support_report, .. }
            | Self::IllegalSurfaceDisposition { support_report, .. }
            | Self::DeferredByLegalityBoundary { support_report, .. }
            | Self::UnsupportedLegalityClass { support_report, .. }
            | Self::TemporalProjectionUnsupported { support_report, .. }
            | Self::AsyncProjectionUnsupported { support_report, .. } => support_report,
        }
    }

    pub fn legality_contract(&self) -> ForgeQueryDeclarationLegalityContract {
        match self {
            Self::WrongAdmittedWorld {
                legality_contract, ..
            }
            | Self::IllegalRoleClaim {
                legality_contract, ..
            }
            | Self::IllegalSurfaceDisposition {
                legality_contract, ..
            }
            | Self::DeferredByLegalityBoundary {
                legality_contract, ..
            }
            | Self::UnsupportedLegalityClass {
                legality_contract, ..
            }
            | Self::TemporalProjectionUnsupported {
                legality_contract, ..
            }
            | Self::AsyncProjectionUnsupported {
                legality_contract, ..
            } => *legality_contract,
        }
    }

    pub fn capability_status(&self) -> ForgeQueryDeclarationCapabilityStatus {
        self.support_report().declare_status()
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.canonical_declaration().declaration_family_key()
    }

    pub fn handle_identity_digest(&self) -> &str {
        self.canonical_declaration().handle_identity_digest()
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        match self {
            Self::WrongAdmittedWorld {
                operating_context_identity_digest,
                ..
            }
            | Self::IllegalRoleClaim {
                operating_context_identity_digest,
                ..
            }
            | Self::IllegalSurfaceDisposition {
                operating_context_identity_digest,
                ..
            }
            | Self::DeferredByLegalityBoundary {
                operating_context_identity_digest,
                ..
            }
            | Self::UnsupportedLegalityClass {
                operating_context_identity_digest,
                ..
            }
            | Self::TemporalProjectionUnsupported {
                operating_context_identity_digest,
                ..
            }
            | Self::AsyncProjectionUnsupported {
                operating_context_identity_digest,
                ..
            } => operating_context_identity_digest,
        }
    }

    pub fn declaration_digest(&self) -> String {
        format!("{:?}", self.canonical_declaration().declaration_digest())
    }

    pub fn temporal_denial_kind(&self) -> Option<ForgeQueryTemporalLegalityDenialKind> {
        match self {
            Self::TemporalProjectionUnsupported { kind, .. } => Some(*kind),
            _ => None,
        }
    }

    pub fn async_denial_kind(&self) -> Option<ForgeQueryAsyncLegalityDenialKind> {
        match self {
            Self::AsyncProjectionUnsupported { kind, .. } => Some(*kind),
            _ => None,
        }
    }
}
