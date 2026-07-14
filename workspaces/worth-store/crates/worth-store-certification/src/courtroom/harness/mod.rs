//! Physical scenario harness drivers, observers, and oracles.

#[cfg(test)]
pub(crate) mod test_support;

pub use crate::courtroom::cross_cutting::observers::{
    PhysicalScenarioObserverKind, PhysicalScenarioObserverRequirement,
};
pub use crate::courtroom::cross_cutting::oracles::{
    PhysicalOracleDenialKind, PhysicalOracleJudgment, PhysicalOracleOutcome,
    PhysicalProofOracleKind, PhysicalProofOracleVerdict,
};
pub use crate::scenario::cross_cutting::drivers::{
    PhysicalScenarioDriverKind, PhysicalScenarioDriverRequirement,
};
pub use crate::scenario::cross_cutting::harness::{
    PhysicalScenarioHarnessDenial, PhysicalScenarioQualityHarness,
};
