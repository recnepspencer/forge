use worth_foundational::facade::DiagnosticRichnessProfile;

use crate::{config::WorthServerOperatorEvidenceConfig, WorthServerResponseEnvelope};

use super::{
    attachments::{build_attachment_bundle, WorthServerOperatorEvidenceAttachmentError},
    classification::WorthServerOperatorEvidenceClass,
    counters::{build_counter_receipt, WorthServerOperatorEvidenceCounterError},
    evidence_record::WorthServerOperatorEvidenceRecord,
    input::WorthServerEvidenceInput,
    transform::WorthServerEvidenceTransform,
};

#[derive(Clone, Debug)]
pub struct WorthServerOperatorEvidencePlan {
    planned: PlannedOperatorEvidence,
}

#[derive(Clone, Debug)]
struct PlannedOperatorEvidence {
    transform: WorthServerEvidenceTransform,
    diagnostics_profile: DiagnosticRichnessProfile,
    response: WorthServerResponseEnvelope,
    classification: WorthServerOperatorEvidenceClass,
}

impl WorthServerOperatorEvidencePlan {
    pub(crate) fn new(
        config: &WorthServerOperatorEvidenceConfig,
        input: WorthServerEvidenceInput,
        transform: Option<WorthServerEvidenceTransform>,
    ) -> Self {
        let planned = match input {
            WorthServerEvidenceInput::ResponseEnvelope(response) => {
                let diagnostics_profile = response
                    .diagnostics_profile()
                    .max(config.minimum_diagnostics_profile());
                PlannedOperatorEvidence {
                    transform: transform.unwrap_or(config.default_transform()),
                    diagnostics_profile,
                    classification: WorthServerOperatorEvidenceClass::from_response_envelope(
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
    ) -> Result<WorthServerOperatorEvidenceRecord, WorthServerOperatorEvidenceMaterializationError>
    {
        let counter_receipt = build_counter_receipt(&self.planned.classification)
            .map_err(WorthServerOperatorEvidenceMaterializationError::CounterReceipt)?;
        let (attachment_bundle, materialized_attachment_bundle) = build_attachment_bundle(
            &self.planned.classification,
            &self.planned.response,
            self.planned.diagnostics_profile,
        )
        .map_err(WorthServerOperatorEvidenceMaterializationError::AttachmentBundle)?;

        Ok(WorthServerOperatorEvidenceRecord::new(
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
pub enum WorthServerOperatorEvidenceMaterializationError {
    CounterReceipt(WorthServerOperatorEvidenceCounterError),
    AttachmentBundle(WorthServerOperatorEvidenceAttachmentError),
}
