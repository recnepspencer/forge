use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::required_stage::PlanarBooleanReadinessRequiredStage;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanReadinessStageCoverage {
    stages: Vec<PlanarBooleanReadinessRequiredStage>,
    coverage_digest: String,
}

impl PlanarBooleanReadinessStageCoverage {
    pub(crate) fn all_required() -> Self {
        Self::from_stages(PlanarBooleanReadinessRequiredStage::ALL.to_vec())
    }

    fn from_stages(stages: Vec<PlanarBooleanReadinessRequiredStage>) -> Self {
        let coverage_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &stages
                .iter()
                .map(|stage| format!("{stage:?}:{}", stage.human_name()))
                .collect::<Vec<_>>(),
        );
        Self {
            stages,
            coverage_digest,
        }
    }

    pub fn stages(&self) -> &[PlanarBooleanReadinessRequiredStage] {
        &self.stages
    }

    pub fn covers_all_required_stages(&self) -> bool {
        self.stages.as_slice() == PlanarBooleanReadinessRequiredStage::ALL
    }

    pub fn coverage_digest(&self) -> &str {
        &self.coverage_digest
    }
}
