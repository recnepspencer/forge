use worth_store_physical_format::{OfflineVerifierDenial, OfflineVerifierDenialKind};

use super::FixtureMutationBoundary;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyntheticFixtureAuthorityDenied {
    PhysicalReopenDenied(OfflineVerifierDenialKind),
    InvalidRootReference(u64),
    SyntheticInMemoryStore,
    FixtureLabelCannotSatisfyAuthority,
    HandFilledStructCannotSatisfyAuthority,
    PrivateStorageMutationCannotSatisfyAuthority,
    CopiedFixtureReceiptCannotSatisfyAuthority,
    UndeclaredMutationBoundary(FixtureMutationBoundary),
}

impl From<OfflineVerifierDenial> for SyntheticFixtureAuthorityDenied {
    fn from(denial: OfflineVerifierDenial) -> Self {
        Self::PhysicalReopenDenied(denial.kind())
    }
}
