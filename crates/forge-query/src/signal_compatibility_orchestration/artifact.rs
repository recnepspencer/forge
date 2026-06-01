use crate::application::{
    ForgeQueryDeclarationInput, ForgeQueryDeclarationSignalCompatibility,
    ForgeQueryDeclarationSignalExecutionFamily, ForgeQueryDomainEntryMarker,
};
use crate::basis_lifecycle::BasisFamily;
use crate::continuation_pipeline::ForgeQueryPreparedContinuation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQuerySignalCompatibilityOrchestrationClass {
    Compatible,
    Prepared,
}

pub enum ForgeQuerySignalCompatibilityOrchestration<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Compatible(ForgeQueryDeclarationSignalCompatibility<D, I>),
    Prepared(ForgeQueryPreparedContinuation<D, I>),
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQuerySignalCompatibilityOrchestration<D, I>
{
    pub fn class(&self) -> ForgeQuerySignalCompatibilityOrchestrationClass {
        match self {
            Self::Compatible(_) => ForgeQuerySignalCompatibilityOrchestrationClass::Compatible,
            Self::Prepared(_) => ForgeQuerySignalCompatibilityOrchestrationClass::Prepared,
        }
    }

    pub fn signal_execution_family(&self) -> Option<ForgeQueryDeclarationSignalExecutionFamily> {
        match self {
            Self::Compatible(compatibility) => Some(compatibility.execution_family()),
            Self::Prepared(prepared) => prepared.signal_execution_family(),
        }
    }

    pub fn basis_families(&self) -> &[BasisFamily] {
        match self {
            Self::Compatible(compatibility) => compatibility.basis_families(),
            Self::Prepared(prepared) => prepared.required_basis_families(),
        }
    }

    pub fn handle_identity_digest(&self) -> &str {
        match self {
            Self::Compatible(compatibility) => compatibility.handle_identity_digest(),
            Self::Prepared(prepared) => prepared.handle_identity_digest(),
        }
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        match self {
            Self::Compatible(compatibility) => compatibility.operating_context_identity_digest(),
            Self::Prepared(prepared) => prepared.operating_context_identity_digest(),
        }
    }

    pub fn declaration_digest(&self) -> &str {
        match self {
            Self::Compatible(compatibility) => compatibility.declaration_digest(),
            Self::Prepared(prepared) => prepared.declaration_digest(),
        }
    }

    pub fn progression_digest(&self) -> Option<&str> {
        match self {
            Self::Compatible(compatibility) => compatibility.progression_digest(),
            Self::Prepared(prepared) => prepared.progression_digest(),
        }
    }

    pub fn route_plan_digest(&self) -> Option<&str> {
        match self {
            Self::Compatible(compatibility) => compatibility.route_plan_digest(),
            Self::Prepared(prepared) => prepared.route_plan_digest(),
        }
    }

    pub fn receipt_digest(&self) -> &forge_foundational::facade::CanonicalDerivedDigest {
        match self {
            Self::Compatible(compatibility) => compatibility.receipt_digest(),
            Self::Prepared(prepared) => prepared.receipt_digest(),
        }
    }

    pub fn envelope_digest(&self) -> &forge_foundational::facade::CanonicalDerivedDigest {
        match self {
            Self::Compatible(compatibility) => compatibility.envelope_digest(),
            Self::Prepared(prepared) => prepared.envelope_digest(),
        }
    }
}
