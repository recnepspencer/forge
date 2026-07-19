use super::{ConsumedProjectionAuthorityCounters, ProjectionAuthorityRequirement};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConsumedProjectionAuthorityDenialKind {
    DeclarationContractMismatch,
    DeclarationSourceMismatch,
    DeclarationBasisMismatch,
    SourceReferenceMismatch,
    ContractFactSetMismatch,
    FactSetReceiptMismatch,
    SourceFamilyMismatch,
    SourceIdentityMismatch,
    SupportPostureMismatch,
    ContractRequestMismatch,
    MissingRequirement(ProjectionAuthorityRequirement),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsumedProjectionAuthorityDenial {
    kind: ConsumedProjectionAuthorityDenialKind,
    counters: ConsumedProjectionAuthorityCounters,
}

impl ConsumedProjectionAuthorityDenial {
    pub fn kind(&self) -> ConsumedProjectionAuthorityDenialKind {
        self.kind
    }

    pub fn counters(&self) -> &ConsumedProjectionAuthorityCounters {
        &self.counters
    }

    pub(super) fn new(
        kind: ConsumedProjectionAuthorityDenialKind,
        relationship_checks: usize,
        requirement_checks: usize,
        source_reference_checks: usize,
    ) -> Self {
        Self {
            kind,
            counters: ConsumedProjectionAuthorityCounters::denied(
                relationship_checks,
                requirement_checks,
                source_reference_checks,
            ),
        }
    }
}
