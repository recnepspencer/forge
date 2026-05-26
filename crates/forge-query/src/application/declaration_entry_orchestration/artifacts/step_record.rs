use super::super::materialization::ForgeQueryDeclarationEntryOrchestrationMaterializationTier;
use super::super::sequencing::{
    automation_step_for_stage, ForgeQueryDeclarationEntryOrchestrationAutomationStep,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationEntryOrchestrationStage {
    AdmittedHandle,
    DeclarationReviewed,
    LegalityEstablished,
    ProgressionResolved,
    FoundationalDescribed,
    RoutePlanned,
    ReceiptIssued,
    EnvelopeConstructed,
}

impl ForgeQueryDeclarationEntryOrchestrationStage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AdmittedHandle => "admitted_handle",
            Self::DeclarationReviewed => "declaration_reviewed",
            Self::LegalityEstablished => "legality_established",
            Self::ProgressionResolved => "progression_resolved",
            Self::FoundationalDescribed => "foundational_described",
            Self::RoutePlanned => "route_planned",
            Self::ReceiptIssued => "receipt_issued",
            Self::EnvelopeConstructed => "envelope_constructed",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationEntryOrchestrationStepDisposition {
    Admitted,
    Automated,
    Deferred,
    Denied,
    Refused,
    Failed,
    ExplicitForCaller,
    TerminalSuccess,
}

impl ForgeQueryDeclarationEntryOrchestrationStepDisposition {
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

pub struct ForgeQueryDeclarationEntryOrchestrationStageRecord {
    stage: ForgeQueryDeclarationEntryOrchestrationStage,
    disposition: ForgeQueryDeclarationEntryOrchestrationStepDisposition,
    retained_digest: Option<String>,
    reason: Option<&'static str>,
    materialization_tier: Option<ForgeQueryDeclarationEntryOrchestrationMaterializationTier>,
}

impl ForgeQueryDeclarationEntryOrchestrationStageRecord {
    pub(crate) fn admitted(
        stage: ForgeQueryDeclarationEntryOrchestrationStage,
        retained_digest: Option<String>,
    ) -> Self {
        Self::new(
            stage,
            ForgeQueryDeclarationEntryOrchestrationStepDisposition::Admitted,
            retained_digest,
            None,
        )
    }

    pub(crate) fn automated(
        stage: ForgeQueryDeclarationEntryOrchestrationStage,
        retained_digest: Option<String>,
    ) -> Self {
        Self::new(
            stage,
            ForgeQueryDeclarationEntryOrchestrationStepDisposition::Automated,
            retained_digest,
            None,
        )
    }

    pub(crate) fn terminal_success(
        stage: ForgeQueryDeclarationEntryOrchestrationStage,
        retained_digest: Option<String>,
    ) -> Self {
        Self::new(
            stage,
            ForgeQueryDeclarationEntryOrchestrationStepDisposition::TerminalSuccess,
            retained_digest,
            None,
        )
    }

    pub(crate) fn deferred(
        stage: ForgeQueryDeclarationEntryOrchestrationStage,
        retained_digest: Option<String>,
        reason: &'static str,
    ) -> Self {
        Self::new(
            stage,
            ForgeQueryDeclarationEntryOrchestrationStepDisposition::Deferred,
            retained_digest,
            Some(reason),
        )
    }

    pub(crate) fn denied(
        stage: ForgeQueryDeclarationEntryOrchestrationStage,
        retained_digest: Option<String>,
        reason: &'static str,
    ) -> Self {
        Self::new(
            stage,
            ForgeQueryDeclarationEntryOrchestrationStepDisposition::Denied,
            retained_digest,
            Some(reason),
        )
    }

    pub(crate) fn refused(
        stage: ForgeQueryDeclarationEntryOrchestrationStage,
        retained_digest: Option<String>,
        reason: &'static str,
    ) -> Self {
        Self::new(
            stage,
            ForgeQueryDeclarationEntryOrchestrationStepDisposition::Refused,
            retained_digest,
            Some(reason),
        )
    }

    pub(crate) fn failed(
        stage: ForgeQueryDeclarationEntryOrchestrationStage,
        retained_digest: Option<String>,
        reason: &'static str,
    ) -> Self {
        Self::new(
            stage,
            ForgeQueryDeclarationEntryOrchestrationStepDisposition::Failed,
            retained_digest,
            Some(reason),
        )
    }

    pub(crate) fn explicit_for_caller(
        stage: ForgeQueryDeclarationEntryOrchestrationStage,
        retained_digest: Option<String>,
        reason: &'static str,
    ) -> Self {
        Self::new(
            stage,
            ForgeQueryDeclarationEntryOrchestrationStepDisposition::ExplicitForCaller,
            retained_digest,
            Some(reason),
        )
    }

    fn new(
        stage: ForgeQueryDeclarationEntryOrchestrationStage,
        disposition: ForgeQueryDeclarationEntryOrchestrationStepDisposition,
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
        materialization_tier: ForgeQueryDeclarationEntryOrchestrationMaterializationTier,
    ) -> Self {
        self.materialization_tier = Some(materialization_tier);
        self
    }

    pub fn stage(&self) -> ForgeQueryDeclarationEntryOrchestrationStage {
        self.stage
    }

    pub fn automation_step(&self) -> ForgeQueryDeclarationEntryOrchestrationAutomationStep {
        automation_step_for_stage(self.stage)
    }

    pub fn disposition(&self) -> ForgeQueryDeclarationEntryOrchestrationStepDisposition {
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
    ) -> Option<ForgeQueryDeclarationEntryOrchestrationMaterializationTier> {
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
                == ForgeQueryDeclarationEntryOrchestrationStepDisposition::TerminalSuccess
    }
}

pub type ForgeQueryDeclarationEntryOrchestrationStepRecord =
    ForgeQueryDeclarationEntryOrchestrationStageRecord;
