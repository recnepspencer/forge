use crate::application::{
    WorthQueryDeclarationFutureProjection, WorthQueryDeclarationInput,
    WorthQueryDeclarationSignalCompatibility, WorthQueryDeclarationSignalExecutionFamily,
    WorthQueryDomainEntryMarker,
};
use crate::basis_lifecycle::BasisFamily;
use crate::continuation_pipeline::WorthQueryPreparedContinuation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQuerySignalCompatibilityOrchestrationClass {
    Compatible,
    Prepared,
}

pub enum WorthQuerySignalCompatibilityOrchestration<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    Compatible(WorthQueryDeclarationSignalCompatibility<D, I>),
    Prepared(WorthQueryPreparedContinuation<D, I>),
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQuerySignalCompatibilityOrchestration<D, I>
{
    pub fn class(&self) -> WorthQuerySignalCompatibilityOrchestrationClass {
        match self {
            Self::Compatible(_) => WorthQuerySignalCompatibilityOrchestrationClass::Compatible,
            Self::Prepared(_) => WorthQuerySignalCompatibilityOrchestrationClass::Prepared,
        }
    }

    pub fn signal_execution_family(&self) -> Option<WorthQueryDeclarationSignalExecutionFamily> {
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

    pub fn future_projection(&self) -> &WorthQueryDeclarationFutureProjection {
        match self {
            Self::Compatible(compatibility) => compatibility.future_projection(),
            Self::Prepared(prepared) => prepared.future_projection(),
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

    pub fn receipt_digest(&self) -> &worth_foundational::facade::CanonicalDerivedDigest {
        match self {
            Self::Compatible(compatibility) => compatibility.receipt_digest(),
            Self::Prepared(prepared) => prepared.receipt_digest(),
        }
    }

    pub fn envelope_digest(&self) -> &worth_foundational::facade::CanonicalDerivedDigest {
        match self {
            Self::Compatible(compatibility) => compatibility.envelope_digest(),
            Self::Prepared(prepared) => prepared.envelope_digest(),
        }
    }
}
