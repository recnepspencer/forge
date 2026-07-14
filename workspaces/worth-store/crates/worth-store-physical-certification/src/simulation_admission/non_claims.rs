#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SimulationHarnessNonClaim {
    NoPhysicalIsolationCorrectnessClaim,
    NoS6IoQosClaim,
    NoBlobLifecycleClaim,
    NoS10RepairForensicsClaim,
    NoS11SecurityAuthenticityClaim,
    NoS12FullDatabaseCertificationClaim,
}

pub(crate) const REQUIRED_S45_ENTRY_NON_CLAIMS: [SimulationHarnessNonClaim; 6] = [
    SimulationHarnessNonClaim::NoPhysicalIsolationCorrectnessClaim,
    SimulationHarnessNonClaim::NoS6IoQosClaim,
    SimulationHarnessNonClaim::NoBlobLifecycleClaim,
    SimulationHarnessNonClaim::NoS10RepairForensicsClaim,
    SimulationHarnessNonClaim::NoS11SecurityAuthenticityClaim,
    SimulationHarnessNonClaim::NoS12FullDatabaseCertificationClaim,
];
