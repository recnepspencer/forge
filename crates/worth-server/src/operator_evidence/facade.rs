use crate::config::WorthServerOperatorEvidenceConfig;

use super::{
    evidence_record::WorthServerOperatorEvidenceRecord,
    input::WorthServerEvidenceInput,
    plan::{WorthServerOperatorEvidenceMaterializationError, WorthServerOperatorEvidencePlan},
    transform::WorthServerEvidenceTransform,
};

#[derive(Clone, Debug)]
pub struct WorthServerOperatorEvidenceFacade {
    config: WorthServerOperatorEvidenceConfig,
}

impl WorthServerOperatorEvidenceFacade {
    pub(crate) fn new(config: WorthServerOperatorEvidenceConfig) -> Self {
        Self { config }
    }

    pub fn plan(
        &self,
        input: WorthServerEvidenceInput,
        transform: WorthServerEvidenceTransform,
    ) -> WorthServerOperatorEvidencePlan {
        WorthServerOperatorEvidencePlan::new(&self.config, input, Some(transform))
    }

    pub fn record(
        &self,
        input: WorthServerEvidenceInput,
        transform: WorthServerEvidenceTransform,
    ) -> Result<WorthServerOperatorEvidenceRecord, WorthServerOperatorEvidenceMaterializationError>
    {
        self.plan(input, transform).materialize()
    }

    pub fn record_with_defaults(
        &self,
        input: WorthServerEvidenceInput,
    ) -> Result<WorthServerOperatorEvidenceRecord, WorthServerOperatorEvidenceMaterializationError>
    {
        WorthServerOperatorEvidencePlan::new(&self.config, input, None).materialize()
    }
}
