use crate::{
    AuthenticityRequiredDecodeCounters, ChecksumAlgorithmMismatchDenial,
    PreDecodeAdmissionCounters, ProtectedPhysicalByteView,
};
use worth_store_physical_format::{PhysicalGenerationOwner, PhysicalHeaderKind};
use worth_store_security::StoreAuthenticityCheckDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreDecodePhysicalDenialKind {
    EntryWitnessMismatch,
    UnsupportedChecksumAlgorithm,
    TruncatedPhysicalPage,
    TruncatedPhysicalFrame,
    ChecksumMismatch,
    AuthenticityRequiredPhysicalDenial,
    AuthenticityResultPhysicalIdentityMismatch,
    StaleGeneration,
    PhysicalHeaderDenied,
    PoisonedPhysicalInput,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreDecodePhysicalDenial {
    kind: PreDecodePhysicalDenialKind,
    observed_kind: Option<PhysicalHeaderKind>,
    locality: Option<PhysicalGenerationOwner>,
    expected_checksum: Option<u64>,
    actual_checksum: Option<u64>,
    protected_byte_count: u64,
    counters: PreDecodeAdmissionCounters,
    checksum_denial: Option<ChecksumAlgorithmMismatchDenial>,
    authenticity_denial: Option<StoreAuthenticityCheckDenial>,
    authenticity_required_counters: Option<AuthenticityRequiredDecodeCounters>,
}

impl PreDecodePhysicalDenial {
    pub(crate) fn new(
        kind: PreDecodePhysicalDenialKind,
        view: ProtectedPhysicalByteView<'_>,
    ) -> Self {
        let protected_byte_count = view.len_bytes() as u64;
        Self {
            kind,
            observed_kind: None,
            locality: None,
            expected_checksum: None,
            actual_checksum: None,
            protected_byte_count,
            counters: PreDecodeAdmissionCounters::denied_before_decode(protected_byte_count),
            checksum_denial: None,
            authenticity_denial: None,
            authenticity_required_counters: None,
        }
    }

    pub(crate) fn after_checksum(
        kind: PreDecodePhysicalDenialKind,
        view: ProtectedPhysicalByteView<'_>,
    ) -> Self {
        let protected_byte_count = view.len_bytes() as u64;
        Self {
            counters: PreDecodeAdmissionCounters::denied_after_checksum(protected_byte_count),
            ..Self::new(kind, view)
        }
    }

    pub(crate) const fn with_observed_kind(mut self, kind: PhysicalHeaderKind) -> Self {
        self.observed_kind = Some(kind);
        self
    }

    pub(crate) const fn with_locality(mut self, locality: PhysicalGenerationOwner) -> Self {
        self.locality = Some(locality);
        self
    }

    pub(crate) const fn with_checksum_values(mut self, expected: u64, actual: u64) -> Self {
        self.expected_checksum = Some(expected);
        self.actual_checksum = Some(actual);
        self
    }

    pub(crate) const fn with_checksum_denial(
        mut self,
        denial: ChecksumAlgorithmMismatchDenial,
    ) -> Self {
        self.checksum_denial = Some(denial);
        self
    }

    pub(crate) const fn with_authenticity_denial(
        mut self,
        denial: StoreAuthenticityCheckDenial,
    ) -> Self {
        self.authenticity_required_counters = Some(AuthenticityRequiredDecodeCounters::denied(
            self.counters,
            denial.counters(),
            denial.is_checksum_valid_authenticity_failed(),
            denial.is_checksum_valid_authenticity_unavailable(),
            denial.is_checksum_valid_authenticity_unsupported(),
        ));
        self.authenticity_denial = Some(denial);
        self
    }

    pub const fn kind(&self) -> PreDecodePhysicalDenialKind {
        self.kind
    }

    pub const fn observed_kind(&self) -> Option<PhysicalHeaderKind> {
        self.observed_kind
    }

    pub const fn locality(&self) -> Option<PhysicalGenerationOwner> {
        self.locality
    }

    pub const fn expected_checksum(&self) -> Option<u64> {
        self.expected_checksum
    }

    pub const fn actual_checksum(&self) -> Option<u64> {
        self.actual_checksum
    }

    pub const fn protected_byte_count(&self) -> u64 {
        self.protected_byte_count
    }

    pub const fn counters(&self) -> PreDecodeAdmissionCounters {
        self.counters
    }

    pub const fn checksum_denial(&self) -> Option<ChecksumAlgorithmMismatchDenial> {
        self.checksum_denial
    }

    pub const fn authenticity_denial(&self) -> Option<StoreAuthenticityCheckDenial> {
        self.authenticity_denial
    }

    pub const fn authenticity_required_counters(
        &self,
    ) -> Option<AuthenticityRequiredDecodeCounters> {
        self.authenticity_required_counters
    }

    pub fn handoff_evidence(&self) -> crate::PhysicalDamageHandoffEvidence {
        crate::classify_physical_damage_for_handoff(self)
    }
}

#[cfg(any(test, feature = "test-support"))]
pub fn test_pre_decode_denial_for_kind(
    kind: PreDecodePhysicalDenialKind,
) -> PreDecodePhysicalDenial {
    PreDecodePhysicalDenial {
        kind,
        observed_kind: None,
        locality: None,
        expected_checksum: None,
        actual_checksum: None,
        protected_byte_count: 0,
        counters: PreDecodeAdmissionCounters::denied_before_decode(0),
        checksum_denial: None,
        authenticity_denial: None,
        authenticity_required_counters: None,
    }
}
