use forge_foundational::facade::{
    FoundationalBoundaryRoleClaimDenial, FoundationalBoundarySurfaceDispositionDenial,
};

use crate::application::{
    ForgeQueryCanonicalDeclarationArtifact, ForgeQueryDeclarationCapabilityStatus,
    ForgeQueryDeclarationFamilySupportReport, ForgeQueryDeclarationInput,
    ForgeQueryDomainEntryMarker,
};

use super::contract::ForgeQueryDeclarationLegalityContract;

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
            | Self::UnsupportedLegalityClass { declaration, .. } => declaration,
        }
    }

    pub fn support_report(&self) -> &ForgeQueryDeclarationFamilySupportReport<D, I::Family> {
        match self {
            Self::WrongAdmittedWorld { support_report, .. }
            | Self::IllegalRoleClaim { support_report, .. }
            | Self::IllegalSurfaceDisposition { support_report, .. }
            | Self::DeferredByLegalityBoundary { support_report, .. }
            | Self::UnsupportedLegalityClass { support_report, .. } => support_report,
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
            } => operating_context_identity_digest,
        }
    }

    pub fn declaration_digest(&self) -> String {
        format!("{:?}", self.canonical_declaration().declaration_digest())
    }
}
