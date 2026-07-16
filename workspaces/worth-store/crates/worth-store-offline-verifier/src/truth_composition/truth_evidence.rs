use std::path::{Path, PathBuf};
use worth_store_security::{
    StoreAuthenticityResult, StoreAuthenticityResultKind, StoreCustodyPosture,
    StoreSecurityScopeAdmissionReceipt, StoreSecurityScopeAdmissionReceiptId,
    StoreSecurityScopeIdentity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfflineSecurityEvidencePosture {
    Confirmed,
    Unavailable,
    Unsupported,
    WrongScope,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfflineRecoveryAvailability {
    Available,
    Unavailable,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfflineFileTruthEvidence {
    path: PathBuf,
    expected_digest: Option<[u8; 32]>,
    authenticity: OfflineSecurityEvidencePosture,
    custody: OfflineSecurityEvidencePosture,
    recovery_availability: OfflineRecoveryAvailability,
    security_scope: Option<StoreSecurityScopeIdentity>,
    security_scope_receipt: Option<StoreSecurityScopeAdmissionReceiptId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfflineTruthEvidenceAdmissionDenial {
    DigestIdentityMismatch,
    SecurityScopeMismatch,
    CustodyNotAdmitted,
    DuplicatePhysicalSource,
    AllocationFailed,
    OwnedAllocationBudgetExceeded { admitted: u64, limit: u64 },
}

impl OfflineFileTruthEvidence {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            expected_digest: None,
            authenticity: OfflineSecurityEvidencePosture::Unavailable,
            custody: OfflineSecurityEvidencePosture::Unavailable,
            recovery_availability: OfflineRecoveryAvailability::Unknown,
            security_scope: None,
            security_scope_receipt: None,
        }
    }
    pub const fn with_expected_digest(mut self, digest: [u8; 32]) -> Self {
        self.expected_digest = Some(digest);
        self
    }
    pub(crate) const fn with_authenticity(
        mut self,
        posture: OfflineSecurityEvidencePosture,
    ) -> Self {
        self.authenticity = posture;
        self
    }
    pub(crate) const fn with_custody(mut self, posture: OfflineSecurityEvidencePosture) -> Self {
        self.custody = posture;
        self
    }
    #[cfg(test)]
    pub(crate) const fn with_recovery_availability(
        mut self,
        availability: OfflineRecoveryAvailability,
    ) -> Self {
        self.recovery_availability = availability;
        self
    }
    pub fn from_admitted_security_evidence(
        path: impl Into<PathBuf>,
        expected_digest: [u8; 32],
        authenticity: StoreAuthenticityResult<[u8; 32]>,
        security_scope: StoreSecurityScopeAdmissionReceipt,
    ) -> Result<Self, OfflineTruthEvidenceAdmissionDenial> {
        if authenticity.kind() != StoreAuthenticityResultKind::Verified
            || authenticity.physical_identity() != expected_digest
        {
            return Err(OfflineTruthEvidenceAdmissionDenial::DigestIdentityMismatch);
        }
        if authenticity.scope_identity() != security_scope.identity() {
            return Err(OfflineTruthEvidenceAdmissionDenial::SecurityScopeMismatch);
        }
        if !matches!(
            security_scope.identity().custody_posture(),
            StoreCustodyPosture::InternalStoreCustody
                | StoreCustodyPosture::ExportPrepared
                | StoreCustodyPosture::Readmitted
        ) {
            return Err(OfflineTruthEvidenceAdmissionDenial::CustodyNotAdmitted);
        }
        Ok(Self::new(path)
            .with_expected_digest(expected_digest)
            .with_authenticity(OfflineSecurityEvidencePosture::Confirmed)
            .with_custody(OfflineSecurityEvidencePosture::Confirmed)
            .with_security_scope(security_scope))
    }
    pub fn path(&self) -> &Path {
        &self.path
    }
    pub const fn expected_digest(&self) -> Option<[u8; 32]> {
        self.expected_digest
    }
    pub const fn authenticity(&self) -> OfflineSecurityEvidencePosture {
        self.authenticity
    }
    pub const fn custody(&self) -> OfflineSecurityEvidencePosture {
        self.custody
    }
    pub const fn recovery_availability(&self) -> OfflineRecoveryAvailability {
        self.recovery_availability
    }
    pub const fn security_scope(&self) -> Option<StoreSecurityScopeIdentity> {
        self.security_scope
    }
    pub const fn security_scope_receipt(&self) -> Option<StoreSecurityScopeAdmissionReceiptId> {
        self.security_scope_receipt
    }

    const fn with_security_scope(mut self, receipt: StoreSecurityScopeAdmissionReceipt) -> Self {
        self.security_scope = Some(receipt.identity());
        self.security_scope_receipt = Some(receipt.receipt_id());
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OfflineTruthEvidenceSet {
    entries: Vec<OfflineFileTruthEvidence>,
}

impl OfflineTruthEvidenceSet {
    pub fn from_entries(
        entries: impl IntoIterator<Item = OfflineFileTruthEvidence>,
        maximum_owned_allocation_bytes: u64,
    ) -> Result<Self, OfflineTruthEvidenceAdmissionDenial> {
        let entries = entries.into_iter();
        let requested_rows = owned_rows_bytes(entries.size_hint().0)
            .ok_or(OfflineTruthEvidenceAdmissionDenial::AllocationFailed)?;
        enforce_owned_allocation(requested_rows, maximum_owned_allocation_bytes)?;
        let mut admitted = Vec::new();
        admitted
            .try_reserve_exact(entries.size_hint().0)
            .map_err(|_| OfflineTruthEvidenceAdmissionDenial::AllocationFailed)?;
        for entry in entries {
            let next_path_bytes = path_owned_allocation_bytes(&entry.path)
                .ok_or(OfflineTruthEvidenceAdmissionDenial::AllocationFailed)?;
            if admitted.len() == admitted.capacity() {
                let requested_capacity = admitted
                    .capacity()
                    .checked_add(1)
                    .ok_or(OfflineTruthEvidenceAdmissionDenial::AllocationFailed)?;
                let admitted_bytes = owned_rows_bytes(requested_capacity)
                    .and_then(|rows| {
                        rows.checked_add(evidence_entries_path_payload_bytes(&admitted)?)
                    })
                    .and_then(|bytes| bytes.checked_add(next_path_bytes))
                    .ok_or(OfflineTruthEvidenceAdmissionDenial::AllocationFailed)?;
                enforce_owned_allocation(admitted_bytes, maximum_owned_allocation_bytes)?;
                admitted
                    .try_reserve(1)
                    .map_err(|_| OfflineTruthEvidenceAdmissionDenial::AllocationFailed)?;
            }
            admitted.push(entry);
            let actual_owned_allocation_bytes = evidence_entries_owned_allocation_bytes(&admitted)
                .ok_or(OfflineTruthEvidenceAdmissionDenial::AllocationFailed)?;
            enforce_owned_allocation(
                actual_owned_allocation_bytes,
                maximum_owned_allocation_bytes,
            )?;
        }
        Self::from_owned_entries(admitted, maximum_owned_allocation_bytes)
    }

    pub(crate) fn from_owned_entries(
        mut admitted: Vec<OfflineFileTruthEvidence>,
        maximum_owned_allocation_bytes: u64,
    ) -> Result<Self, OfflineTruthEvidenceAdmissionDenial> {
        let owned_allocation_bytes = evidence_entries_owned_allocation_bytes(&admitted)
            .ok_or(OfflineTruthEvidenceAdmissionDenial::AllocationFailed)?;
        enforce_owned_allocation(owned_allocation_bytes, maximum_owned_allocation_bytes)?;
        admitted.sort_by(|left, right| left.path.cmp(&right.path));
        if admitted.windows(2).any(|pair| pair[0].path == pair[1].path) {
            return Err(OfflineTruthEvidenceAdmissionDenial::DuplicatePhysicalSource);
        }
        Ok(Self { entries: admitted })
    }
    pub(crate) fn for_path(&self, path: &Path) -> Option<&OfflineFileTruthEvidence> {
        self.entries
            .binary_search_by(|entry| entry.path().cmp(path))
            .ok()
            .map(|index| &self.entries[index])
    }

    pub(crate) fn owned_allocation_bytes(&self) -> Option<u64> {
        let rows = u64::try_from(self.entries.capacity())
            .ok()?
            .checked_mul(std::mem::size_of::<OfflineFileTruthEvidence>() as u64)?;
        self.entries.iter().try_fold(rows, |total, entry| {
            total.checked_add(path_owned_allocation_bytes(&entry.path)?)
        })
    }
}

fn evidence_entries_owned_allocation_bytes(entries: &Vec<OfflineFileTruthEvidence>) -> Option<u64> {
    entries
        .iter()
        .try_fold(owned_rows_bytes(entries.capacity())?, |total, entry| {
            total.checked_add(path_owned_allocation_bytes(&entry.path)?)
        })
}

fn evidence_entries_path_payload_bytes(entries: &[OfflineFileTruthEvidence]) -> Option<u64> {
    entries.iter().try_fold(0_u64, |total, entry| {
        total.checked_add(path_owned_allocation_bytes(&entry.path)?)
    })
}

fn owned_rows_bytes(capacity: usize) -> Option<u64> {
    u64::try_from(capacity)
        .ok()?
        .checked_mul(std::mem::size_of::<OfflineFileTruthEvidence>() as u64)
}

fn enforce_owned_allocation(
    admitted: u64,
    limit: u64,
) -> Result<(), OfflineTruthEvidenceAdmissionDenial> {
    if admitted > limit {
        Err(OfflineTruthEvidenceAdmissionDenial::OwnedAllocationBudgetExceeded { admitted, limit })
    } else {
        Ok(())
    }
}

#[cfg(windows)]
fn path_owned_allocation_bytes(path: &Path) -> Option<u64> {
    use std::os::windows::ffi::OsStrExt;
    u64::try_from(path.as_os_str().encode_wide().count())
        .ok()?
        .checked_mul(2)
}

#[cfg(unix)]
fn path_owned_allocation_bytes(path: &Path) -> Option<u64> {
    use std::os::unix::ffi::OsStrExt;
    u64::try_from(path.as_os_str().as_bytes().len()).ok()
}

#[cfg(not(any(windows, unix)))]
fn path_owned_allocation_bytes(path: &Path) -> Option<u64> {
    u64::try_from(path.to_string_lossy().len()).ok()
}
