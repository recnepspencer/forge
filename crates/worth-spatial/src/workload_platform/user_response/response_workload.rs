use super::{
    source::WorthUserResponseSourceKind, HumanReadableResponse, WorthUserOutcome,
    WorthUserOutcomeCause, WorthUserOutcomeCauseKind, WorthUserResponseEvidence,
    WorthUserResponseSource,
};
use crate::workload_platform::vocabulary::{
    certify_stage, ResponseWorkloadReceipt, SpatialWorkloadStage, WorkloadStageDenial,
    WorkloadStageIdentity, WorkloadStageSupport,
};

pub struct WorthUserResponseWorkload {
    source: WorthUserResponseSource,
    declaration: String,
}

impl WorthUserResponseWorkload {
    pub fn from_source(source: WorthUserResponseSource) -> Self {
        Self {
            source,
            declaration: "worth user response workload".to_string(),
        }
    }

    pub fn declared(mut self, declaration: impl Into<String>) -> Self {
        self.declaration = declaration.into();
        self
    }

    pub fn respond(self) -> Result<WorthUserResponseReceipt, WorkloadStageDenial> {
        let outcome = response_outcome_from_source(&self.source);
        let stage_receipt = certify_stage(
            SpatialWorkloadStage::Response,
            self.declaration,
            outcome.evidence().source_identity().to_string(),
            WorkloadStageSupport::Admitted,
            "Worth user response workload is admitted.",
        )?;
        Ok(WorthUserResponseReceipt {
            stage_receipt: ResponseWorkloadReceipt::from_envelope(stage_receipt),
            outcome,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUserResponseReceipt {
    stage_receipt: ResponseWorkloadReceipt,
    outcome: WorthUserOutcome,
}

impl WorthUserResponseReceipt {
    pub fn stage_identity(&self) -> &WorkloadStageIdentity {
        self.stage_receipt.identity()
    }

    pub fn stage_receipt(&self) -> &ResponseWorkloadReceipt {
        &self.stage_receipt
    }

    pub fn outcome(&self) -> &WorthUserOutcome {
        &self.outcome
    }

    pub fn human_response(&self) -> &HumanReadableResponse {
        self.outcome.human_response()
    }

    pub fn evidence(&self) -> &WorthUserResponseEvidence {
        self.outcome.evidence()
    }
}

fn response_outcome_from_source(source: &WorthUserResponseSource) -> WorthUserOutcome {
    match source.kind() {
        WorthUserResponseSourceKind::Admitted {
            message,
            evidence_digest,
            source_identity,
        } => WorthUserOutcome::admitted(
            HumanReadableResponse::from_source_summary(message),
            WorthUserResponseEvidence::new(evidence_digest, source_identity),
        ),
        WorthUserResponseSourceKind::PolicyRequired {
            message,
            evidence_digest,
            source_identity,
            choices,
        } => WorthUserOutcome::policy_required(
            WorthUserOutcomeCause::new(WorthUserOutcomeCauseKind::PolicyRequired, message),
            HumanReadableResponse::from_source_summary(message),
            WorthUserResponseEvidence::new(evidence_digest, source_identity),
            choices.clone(),
        ),
        WorthUserResponseSourceKind::NoOptions {
            cause_kind,
            message,
            evidence_digest,
            source_identity,
        } => WorthUserOutcome::no_options(
            WorthUserOutcomeCause::new(*cause_kind, message),
            HumanReadableResponse::from_source_summary(message),
            WorthUserResponseEvidence::new(evidence_digest, source_identity),
        ),
    }
}
