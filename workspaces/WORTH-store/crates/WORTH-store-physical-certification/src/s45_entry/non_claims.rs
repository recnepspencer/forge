#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum S45HarnessNonClaim {
    NoS5PhysicalIsolationCorrectnessClaim,
    NoS6IoQosClaim,
    NoS7BlobLifecycleClaim,
    NoS10RepairForensicsClaim,
    NoS11SecurityAuthenticityClaim,
    NoS12FullDatabaseCertificationClaim,
}

pub(crate) const REQUIRED_S45_ENTRY_NON_CLAIMS: [S45HarnessNonClaim; 6] = [
    S45HarnessNonClaim::NoS5PhysicalIsolationCorrectnessClaim,
    S45HarnessNonClaim::NoS6IoQosClaim,
    S45HarnessNonClaim::NoS7BlobLifecycleClaim,
    S45HarnessNonClaim::NoS10RepairForensicsClaim,
    S45HarnessNonClaim::NoS11SecurityAuthenticityClaim,
    S45HarnessNonClaim::NoS12FullDatabaseCertificationClaim,
];
