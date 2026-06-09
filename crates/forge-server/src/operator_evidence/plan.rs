use forge_foundational::facade::DiagnosticRichnessProfile;

use crate::{config::ForgeServerOperatorEvidenceConfig, ForgeServerResponseEnvelope};

use super::{
    attachments::{build_attachment_bundle, ForgeServerOperatorEvidenceAttachmentError},
    classification::ForgeServerOperatorEvidenceClass,
    counters::{build_counter_receipt, ForgeServerOperatorEvidenceCounterError},
    evidence_record::ForgeServerOperatorEvidenceRecord,
    input::ForgeServerEvidenceInput,
    transform::ForgeServerEvidenceTransform,
};

#[derive(Clone, Debug)]
pub struct ForgeServerOperatorEvidencePlan {
    planned: PlannedOperatorEvidence,
}

#[derive(Clone, Debug)]
struct PlannedOperatorEvidence {
    transform: ForgeServerEvidenceTransform,
    diagnostics_profile: DiagnosticRichnessProfile,
    response: ForgeServerResponseEnvelope,
    classification: ForgeServerOperatorEvidenceClass,
}

impl ForgeServerOperatorEvidencePlan {
    pub(crate) fn new(
        config: &ForgeServerOperatorEvidenceConfig,
        input: ForgeServerEvidenceInput,
        transform: Option<ForgeServerEvidenceTransform>,
    ) -> Self {
        let planned = match input {
            ForgeServerEvidenceInput::ResponseEnvelope(response) => {
                let diagnostics_profile = response
                    .diagnostics_profile()
                    .max(config.minimum_diagnostics_profile());
                PlannedOperatorEvidence {
                    transform: transform.unwrap_or(config.default_transform()),
                    diagnostics_profile,
                    classification: ForgeServerOperatorEvidenceClass::from_response_envelope(
                        &response,
                    ),
                    response,
                }
            }
        };
        Self { planned }
    }

    pub fn materialize(
        self,
    ) -> Result<ForgeServerOperatorEvidenceRecord, ForgeServerOperatorEvidenceMaterializationError>
    {
        let counter_receipt = build_counter_receipt(&self.planned.classification)
            .map_err(ForgeServerOperatorEvidenceMaterializationError::CounterReceipt)?;
        let (attachment_bundle, materialized_attachment_bundle) = build_attachment_bundle(
            &self.planned.classification,
            &self.planned.response,
            self.planned.diagnostics_profile,
        )
        .map_err(ForgeServerOperatorEvidenceMaterializationError::AttachmentBundle)?;

        Ok(ForgeServerOperatorEvidenceRecord::new(
            self.planned.transform,
            self.planned.diagnostics_profile,
            self.planned.response.canonical_digest().to_string(),
            self.planned.classification,
            counter_receipt,
            attachment_bundle,
            materialized_attachment_bundle,
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerOperatorEvidenceMaterializationError {
    CounterReceipt(ForgeServerOperatorEvidenceCounterError),
    AttachmentBundle(ForgeServerOperatorEvidenceAttachmentError),
}
