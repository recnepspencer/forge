//! Physical scenario harness drivers, observers, and oracles.

#[cfg(test)]
pub(crate) mod test_support;

pub use crate::drivers::{PhysicalScenarioDriverKind, PhysicalScenarioDriverRequirement};
pub use crate::harness::{PhysicalScenarioHarnessDenial, PhysicalScenarioQualityHarness};
pub use crate::observers::{PhysicalScenarioObserverKind, PhysicalScenarioObserverRequirement};
pub use crate::oracles::{
    PhysicalOracleDenialKind, PhysicalOracleJudgment, PhysicalOracleOutcome,
    PhysicalProofOracleKind, PhysicalProofOracleVerdict,
};