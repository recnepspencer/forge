use super::super::capability::Roadmap2SequenceId;
use super::super::harness::{HarnessMaturityLevel, HarnessSubsystemMaturity};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct SequenceHarnessDependency {
    sequence_id: Roadmap2SequenceId,
    subsystem: HarnessSubsystemMaturity,
    minimum_level: HarnessMaturityLevel,
}

impl SequenceHarnessDependency {
    pub fn new(
        sequence_id: Roadmap2SequenceId,
        subsystem: HarnessSubsystemMaturity,
        minimum_level: HarnessMaturityLevel,
    ) -> Self {
        Self {
            sequence_id,
            subsystem,
            minimum_level,
        }
    }
}
