use super::{
    super::{
        PhysicalWorkCourtroomRunBinding, PhysicalWorkOracleEvidence, PhysicalWorkSourceBinding,
    },
    PhysicalWorkFreshReopenEvidence, PhysicalWorkHostileArtifactEvidence,
    PhysicalWorkHostileProcessEvidence, PhysicalWorkHostileTruthComparison,
    PhysicalWorkHostileTruthScenario, PhysicalWorkHostileTruthVerdict,
};
use crate::physical_runtime::record_serving::evidence::physical_work::hostile_validation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkHostileTruthCaseBinding {
    scenario: PhysicalWorkHostileTruthScenario,
    run: PhysicalWorkCourtroomRunBinding,
    observer_binary: PhysicalWorkSourceBinding,
    processes: PhysicalWorkHostileProcessEvidence,
}

impl PhysicalWorkHostileTruthCaseBinding {
    pub const fn new(
        scenario: PhysicalWorkHostileTruthScenario,
        run: PhysicalWorkCourtroomRunBinding,
        observer_binary: PhysicalWorkSourceBinding,
        processes: PhysicalWorkHostileProcessEvidence,
    ) -> Self {
        Self {
            scenario,
            run,
            observer_binary,
            processes,
        }
    }

    pub fn finish(
        self,
        comparison: PhysicalWorkHostileTruthComparison,
        artifacts: impl IntoIterator<Item = PhysicalWorkHostileArtifactEvidence>,
        reopener: PhysicalWorkFreshReopenEvidence,
        oracle: PhysicalWorkOracleEvidence,
    ) -> PhysicalWorkHostileTruthCaseEvidence {
        let artifacts = artifacts.into_iter().collect::<Vec<_>>().into_boxed_slice();
        let findings =
            hostile_validation::validate_case(&self, comparison, &artifacts, reopener, &oracle);
        PhysicalWorkHostileTruthCaseEvidence {
            binding: self,
            comparison,
            artifacts,
            reopener,
            oracle,
            verdict: PhysicalWorkHostileTruthVerdict::from_findings(findings),
        }
    }

    pub const fn scenario(&self) -> PhysicalWorkHostileTruthScenario {
        self.scenario
    }

    pub const fn run(&self) -> &PhysicalWorkCourtroomRunBinding {
        &self.run
    }

    pub const fn observer_binary(&self) -> &PhysicalWorkSourceBinding {
        &self.observer_binary
    }

    pub const fn processes(&self) -> &PhysicalWorkHostileProcessEvidence {
        &self.processes
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkHostileTruthCaseEvidence {
    binding: PhysicalWorkHostileTruthCaseBinding,
    comparison: PhysicalWorkHostileTruthComparison,
    artifacts: Box<[PhysicalWorkHostileArtifactEvidence]>,
    reopener: PhysicalWorkFreshReopenEvidence,
    oracle: PhysicalWorkOracleEvidence,
    verdict: PhysicalWorkHostileTruthVerdict,
}

impl PhysicalWorkHostileTruthCaseEvidence {
    pub const fn binding(&self) -> &PhysicalWorkHostileTruthCaseBinding {
        &self.binding
    }

    pub const fn comparison(&self) -> PhysicalWorkHostileTruthComparison {
        self.comparison
    }

    pub const fn artifacts(&self) -> &[PhysicalWorkHostileArtifactEvidence] {
        &self.artifacts
    }

    pub const fn reopener(&self) -> PhysicalWorkFreshReopenEvidence {
        self.reopener
    }

    pub const fn oracle(&self) -> &PhysicalWorkOracleEvidence {
        &self.oracle
    }

    pub const fn verdict(&self) -> &PhysicalWorkHostileTruthVerdict {
        &self.verdict
    }
}
