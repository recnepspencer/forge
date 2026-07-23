use std::convert::Infallible;

use worth_proof::ProofOutcome;

use super::{PhysicalSignalProfileIdentity, PhysicalWorkDeclarationDenial, PhysicalWorkIdentity};

pub type PhysicalWorkSubmissionOutcome = ProofOutcome<
    PhysicalWorkSubmissionReceipt,
    PhysicalWorkSubmissionDenial,
    PhysicalWorkSubmissionDeferred,
    PhysicalWorkSubmissionStale,
    Infallible,
    PhysicalWorkSubmissionFailure,
>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalWorkSubmissionReceipt {
    pub(super) identity: PhysicalWorkIdentity,
    pub(super) signal_profile: PhysicalSignalProfileIdentity,
}

impl PhysicalWorkSubmissionReceipt {
    pub const fn identity(self) -> PhysicalWorkIdentity {
        self.identity
    }

    pub const fn signal_profile(self) -> PhysicalSignalProfileIdentity {
        self.signal_profile
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkSubmissionDenial {
    SemanticPostureMismatch,
    SecurityScopeWitnessMismatch,
    SemanticContractNotInstalled,
    Declaration(PhysicalWorkDeclarationDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkCapacityDimension {
    Commands,
    ScopeMembersPerWork,
    TotalScopeMembers,
    SemanticBytesPerWork,
    TotalSemanticBytes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalWorkSubmissionDeferred {
    pub(super) dimension: PhysicalWorkCapacityDimension,
    pub(super) capacity: usize,
}

impl PhysicalWorkSubmissionDeferred {
    pub const fn dimension(self) -> PhysicalWorkCapacityDimension {
        self.dimension
    }

    pub const fn capacity(self) -> usize {
        self.capacity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkSubmissionStale {
    OwnerReleased,
    LifecycleGenerationAdvanced,
    AdmissionStopped,
    SignalOwnerUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkSubmissionFailure {
    OperationIdentityExhausted,
}
