use super::super::materialization::WorthQueryDeclarationEntryOrchestrationMaterializationTier;
use super::super::sequencing::{
    automation_step_for_stage, WorthQueryDeclarationEntryOrchestrationAutomationStep,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationEntryOrchestrationStage {
    AdmittedHandle,
    DeclarationReviewed,
    LegalityEstablished,
    ProgressionAdmitted,
    ProgressionResolved,
    FoundationalDescribed,
    RoutePlanned,
    ReceiptIssued,
    EnvelopeConstructed,
}

impl WorthQueryDeclarationEntryOrchestrationStage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AdmittedHandle => "admitted_handle",
            Self::DeclarationReviewed => "declaration_reviewed",
            Self::LegalityEstablished => "legality_established",
            Self::ProgressionAdmitted => "progression_admitted",
            Self::ProgressionResolved => "progression_resolved",
            Self::FoundationalDescribed => "foundational_described",
            Self::RoutePlanned => "route_planned",
            Self::ReceiptIssued => "receipt_issued",
            Self::EnvelopeConstructed => "envelope_constructed",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationEntryOrchestrationStepDisposition {
    Admitted,
    Automated,
    Deferred,
    Denied,
    Refused,
    Failed,
    ExplicitForCaller,
    TerminalSuccess,
}

impl WorthQueryDeclarationEntryOrchestrationStepDisposition {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Automated => "automated",
            Self::Deferred => "deferred",
            Self::Denied => "denied",
            Self::Refused => "refused",
            Self::Failed => "failed",
            Self::ExplicitForCaller => "explicit_for_caller",
            Self::TerminalSuccess => "terminal_success",
        }
    }

    fn is_terminal_stop(self) -> bool {
        matches!(
            self,
            Self::Deferred | Self::Denied | Self::Refused | Self::Failed | Self::ExplicitForCaller
        )
    }
}

pub struct WorthQueryDeclarationEntryOrchestrationStageRecord {
    stage: WorthQueryDeclarationEntryOrchestrationStage,
    disposition: WorthQueryDeclarationEntryOrchestrationStepDisposition,
    retained_digest: Option<String>,
    reason: Option<&'static str>,
    materialization_tier: Option<WorthQueryDeclarationEntryOrchestrationMaterializationTier>,
}

impl WorthQueryDeclarationEntryOrchestrationStageRecord {
    pub(crate) fn admitted(
        stage: WorthQueryDeclarationEntryOrchestrationStage,
        retained_digest: Option<String>,
    ) -> Self {
        Self::new(
            stage,
            WorthQueryDeclarationEntryOrchestrationStepDisposition::Admitted,
            retained_digest,
            None,
        )
    }

    pub(crate) fn automated(
        stage: WorthQueryDeclarationEntryOrchestrationStage,
        retained_digest: Option<String>,
    ) -> Self {
        Self::new(
            stage,
            WorthQueryDeclarationEntryOrchestrationStepDisposition::Automated,
            retained_digest,
            None,
        )
    }

    pub(crate) fn terminal_success(
        stage: WorthQueryDeclarationEntryOrchestrationStage,
        retained_digest: Option<String>,
    ) -> Self {
        Self::new(
            stage,
            WorthQueryDeclarationEntryOrchestrationStepDisposition::TerminalSuccess,
            retained_digest,
            None,
        )
    }

    pub(crate) fn deferred(
        stage: WorthQueryDeclarationEntryOrchestrationStage,
        retained_digest: Option<String>,
        reason: &'static str,
    ) -> Self {
        Self::new(
            stage,
            WorthQueryDeclarationEntryOrchestrationStepDisposition::Deferred,
            retained_digest,
            Some(reason),
        )
    }

    pub(crate) fn denied(
        stage: WorthQueryDeclarationEntryOrchestrationStage,
        retained_digest: Option<String>,
        reason: &'static str,
    ) -> Self {
        Self::new(
            stage,
            WorthQueryDeclarationEntryOrchestrationStepDisposition::Denied,
            retained_digest,
            Some(reason),
        )
    }

    #[cfg(test)]
    pub(crate) fn refused(
        stage: WorthQueryDeclarationEntryOrchestrationStage,
        retained_digest: Option<String>,
        reason: &'static str,
    ) -> Self {
        Self::new(
            stage,
            WorthQueryDeclarationEntryOrchestrationStepDisposition::Refused,
            retained_digest,
            Some(reason),
        )
    }

    pub(crate) fn failed(
        stage: WorthQueryDeclarationEntryOrchestrationStage,
        retained_digest: Option<String>,
        reason: &'static str,
    ) -> Self {
        Self::new(
            stage,
            WorthQueryDeclarationEntryOrchestrationStepDisposition::Failed,
            retained_digest,
            Some(reason),
        )
    }

    #[cfg(test)]
    pub(crate) fn explicit_for_caller(
        stage: WorthQueryDeclarationEntryOrchestrationStage,
        retained_digest: Option<String>,
        reason: &'static str,
    ) -> Self {
        Self::new(
            stage,
            WorthQueryDeclarationEntryOrchestrationStepDisposition::ExplicitForCaller,
            retained_digest,
            Some(reason),
        )
    }

    fn new(
        stage: WorthQueryDeclarationEntryOrchestrationStage,
        disposition: WorthQueryDeclarationEntryOrchestrationStepDisposition,
        retained_digest: Option<String>,
        reason: Option<&'static str>,
    ) -> Self {
        Self {
            stage,
            disposition,
            retained_digest,
            reason,
            materialization_tier: None,
        }
    }

    pub(crate) fn with_materialization_tier(
        mut self,
        materialization_tier: WorthQueryDeclarationEntryOrchestrationMaterializationTier,
    ) -> Self {
        self.materialization_tier = Some(materialization_tier);
        self
    }

    pub fn stage(&self) -> WorthQueryDeclarationEntryOrchestrationStage {
        self.stage
    }

    pub fn automation_step(&self) -> WorthQueryDeclarationEntryOrchestrationAutomationStep {
        automation_step_for_stage(self.stage)
    }

    pub fn disposition(&self) -> WorthQueryDeclarationEntryOrchestrationStepDisposition {
        self.disposition
    }

    pub fn retained_digest(&self) -> Option<&str> {
        self.retained_digest.as_deref()
    }

    pub fn reason(&self) -> Option<&'static str> {
        self.reason
    }

    pub fn materialization_tier(
        &self,
    ) -> Option<WorthQueryDeclarationEntryOrchestrationMaterializationTier> {
        self.materialization_tier
    }

    pub fn is_reached(&self) -> bool {
        !self.disposition.is_terminal_stop()
    }

    pub fn is_stop(&self) -> bool {
        self.disposition.is_terminal_stop()
    }

    pub fn is_terminal(&self) -> bool {
        self.disposition.is_terminal_stop()
            || self.disposition
                == WorthQueryDeclarationEntryOrchestrationStepDisposition::TerminalSuccess
    }
}

pub type WorthQueryDeclarationEntryOrchestrationStepRecord =
    WorthQueryDeclarationEntryOrchestrationStageRecord;
