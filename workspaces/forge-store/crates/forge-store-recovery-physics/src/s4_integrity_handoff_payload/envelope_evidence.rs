use crate::{S4IntegrityHandoffDenial, S4IntegrityHandoffDenialKind};
use forge_store_physical_integrity::{
    ChecksumAlgorithmDeclaration, ChecksumAlgorithmId, ChecksumCoverageBasis,
    ChecksumScopeDeclaration, IntegrityCheckedFrame, IntegrityCheckedPage,
    PreDecodeAdmissionCounters,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedInspectionEnvelopeEvidence {
    resident_byte_limit: u64,
    protected_read_limit: u64,
    streaming_window_limit: u64,
    counters: PreDecodeAdmissionCounters,
    checksum_basis: S4ChecksumAlgorithmScopeBasis,
}

impl BoundedInspectionEnvelopeEvidence {
    pub fn from_checked_page(
        checked: &IntegrityCheckedPage<'_>,
        resident_byte_limit: u64,
        protected_read_limit: u64,
        streaming_window_limit: u64,
    ) -> Result<Self, S4IntegrityHandoffDenial> {
        Self::from_checked_counters(
            checked.counters(),
            checked.gate_evidence().coverage_basis(),
            resident_byte_limit,
            protected_read_limit,
            streaming_window_limit,
        )
    }

    pub fn from_checked_frame(
        checked: &IntegrityCheckedFrame<'_>,
        resident_byte_limit: u64,
        protected_read_limit: u64,
        streaming_window_limit: u64,
    ) -> Result<Self, S4IntegrityHandoffDenial> {
        Self::from_checked_counters(
            checked.counters(),
            checked.gate_evidence().coverage_basis(),
            resident_byte_limit,
            protected_read_limit,
            streaming_window_limit,
        )
    }

    pub const fn resident_byte_limit(&self) -> u64 {
        self.resident_byte_limit
    }

    pub const fn protected_read_limit(&self) -> u64 {
        self.protected_read_limit
    }

    pub const fn streaming_window_limit(&self) -> u64 {
        self.streaming_window_limit
    }

    pub const fn counters(&self) -> PreDecodeAdmissionCounters {
        self.counters
    }

    pub const fn checksum_basis(&self) -> &S4ChecksumAlgorithmScopeBasis {
        &self.checksum_basis
    }

    fn from_checked_counters(
        counters: PreDecodeAdmissionCounters,
        basis: &ChecksumCoverageBasis,
        resident_byte_limit: u64,
        protected_read_limit: u64,
        streaming_window_limit: u64,
    ) -> Result<Self, S4IntegrityHandoffDenial> {
        let checked_bytes = counters.checked_byte_count();
        if checked_bytes > resident_byte_limit
            || checked_bytes > protected_read_limit
            || checked_bytes > streaming_window_limit
        {
            return Err(S4IntegrityHandoffDenial::new(
                S4IntegrityHandoffDenialKind::InspectionEnvelopeExceeded,
            ));
        }
        Ok(Self {
            resident_byte_limit,
            protected_read_limit,
            streaming_window_limit,
            counters,
            checksum_basis: S4ChecksumAlgorithmScopeBasis::from_coverage_basis(basis)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S4ChecksumAlgorithmScopeBasis {
    algorithm: ChecksumAlgorithmId,
    scope: ChecksumScopeDeclaration,
}

impl S4ChecksumAlgorithmScopeBasis {
    pub fn from_checksum_declaration(declaration: &ChecksumAlgorithmDeclaration) -> Self {
        Self::from_coverage_basis(declaration.coverage_basis())
            .expect("S.3 checksum declaration already admitted its coverage scope")
    }

    pub const fn algorithm(&self) -> ChecksumAlgorithmId {
        self.algorithm
    }

    pub const fn scope(&self) -> &ChecksumScopeDeclaration {
        &self.scope
    }

    fn from_coverage_basis(
        basis: &ChecksumCoverageBasis,
    ) -> Result<Self, S4IntegrityHandoffDenial> {
        let scope = ChecksumScopeDeclaration::for_physical_format(
            basis.physical_format_identity(),
            basis.coverage_map().clone(),
        )
        .map_err(|_| {
            S4IntegrityHandoffDenial::new(S4IntegrityHandoffDenialKind::ChecksumBasisMismatch)
        })?;
        Ok(Self {
            algorithm: basis.algorithm_id(),
            scope,
        })
    }
}
