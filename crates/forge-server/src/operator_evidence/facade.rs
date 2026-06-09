use crate::config::ForgeServerOperatorEvidenceConfig;

use super::{
    evidence_record::ForgeServerOperatorEvidenceRecord,
    input::ForgeServerEvidenceInput,
    plan::{ForgeServerOperatorEvidenceMaterializationError, ForgeServerOperatorEvidencePlan},
    transform::ForgeServerEvidenceTransform,
};

#[derive(Clone, Debug)]
pub struct ForgeServerOperatorEvidenceFacade {
    config: ForgeServerOperatorEvidenceConfig,
}

impl ForgeServerOperatorEvidenceFacade {
    pub(crate) fn new(config: ForgeServerOperatorEvidenceConfig) -> Self {
        Self { config }
    }

    pub fn plan(
        &self,
        input: ForgeServerEvidenceInput,
        transform: ForgeServerEvidenceTransform,
    ) -> ForgeServerOperatorEvidencePlan {
        ForgeServerOperatorEvidencePlan::new(&self.config, input, Some(transform))
    }

    pub fn record(
        &self,
        input: ForgeServerEvidenceInput,
        transform: ForgeServerEvidenceTransform,
    ) -> Result<ForgeServerOperatorEvidenceRecord, ForgeServerOperatorEvidenceMaterializationError>
    {
        self.plan(input, transform).materialize()
    }

    pub fn record_with_defaults(
        &self,
        input: ForgeServerEvidenceInput,
    ) -> Result<ForgeServerOperatorEvidenceRecord, ForgeServerOperatorEvidenceMaterializationError>
    {
        ForgeServerOperatorEvidencePlan::new(&self.config, input, None).materialize()
    }
}
