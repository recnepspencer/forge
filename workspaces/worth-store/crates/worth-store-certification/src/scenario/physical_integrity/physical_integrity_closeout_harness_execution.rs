use crate::{
    ExecutedCorruptionLocalizationEvidence, ExecutedIntegrityBoundaryDenialEvidence,
    IntegrityCompositionEvidence, IntegrityRecoveryHandoffCloseoutEvidence,
    PhysicalIntegrityCloseoutDenial, PhysicalScenarioPlan, PhysicalStoryTranscript,
    SyntheticCloseoutShortcutRejectionReport,
};
use crate::{IntegrityCloseoutEvidenceFamily, PhysicalIntegrityAcceptanceSuite};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrityCloseoutExecutedOutputKind {
    CorruptionLocalization,
    BoundaryDenial,
    HarnessTranscript,
    SyntheticShortcutRejection,
    RecoveryIntegrityHandoff,
    LineCapComposition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntegrityHarnessExecutionEvidence {
    acceptance_suite: PhysicalIntegrityAcceptanceSuite,
    output_kind: IntegrityCloseoutExecutedOutputKind,
    output_count: u32,
}

impl IntegrityHarnessExecutionEvidence {
    pub(crate) fn corruption_localization(rows: &[ExecutedCorruptionLocalizationEvidence]) -> Self {
        Self::new(
            PhysicalIntegrityAcceptanceSuite::CorruptionLocalization,
            IntegrityCloseoutExecutedOutputKind::CorruptionLocalization,
            rows.len(),
        )
    }

    pub(crate) fn boundary_denial(rows: &[ExecutedIntegrityBoundaryDenialEvidence]) -> Self {
        Self::new(
            PhysicalIntegrityAcceptanceSuite::BoundaryDenial,
            IntegrityCloseoutExecutedOutputKind::BoundaryDenial,
            rows.len(),
        )
    }

    pub(crate) const fn harness_transcript(output_count: usize) -> Self {
        Self::new(
            PhysicalIntegrityAcceptanceSuite::HarnessTranscript,
            IntegrityCloseoutExecutedOutputKind::HarnessTranscript,
            output_count,
        )
    }

    pub(crate) fn synthetic_rejection(rows: &[SyntheticCloseoutShortcutRejectionReport]) -> Self {
        Self::new(
            PhysicalIntegrityAcceptanceSuite::SyntheticShortcutRejection,
            IntegrityCloseoutExecutedOutputKind::SyntheticShortcutRejection,
            rows.len(),
        )
    }

    pub(crate) fn recovery_handoff(evidence: &IntegrityRecoveryHandoffCloseoutEvidence) -> Self {
        let output_count = usize::from(evidence.proves_no_raw_bytes_crossed());
        Self::new(
            PhysicalIntegrityAcceptanceSuite::RecoveryIntegrityHandoff,
            IntegrityCloseoutExecutedOutputKind::RecoveryIntegrityHandoff,
            output_count,
        )
    }

    pub(crate) fn line_cap_composition(evidence: &IntegrityCompositionEvidence) -> Self {
        Self::new(
            PhysicalIntegrityAcceptanceSuite::LineCapComposition,
            IntegrityCloseoutExecutedOutputKind::LineCapComposition,
            evidence.checked_surface_count(),
        )
    }

    pub const fn acceptance_suite(self) -> PhysicalIntegrityAcceptanceSuite {
        self.acceptance_suite
    }

    pub const fn output_kind(self) -> IntegrityCloseoutExecutedOutputKind {
        self.output_kind
    }

    pub const fn output_count(self) -> u32 {
        self.output_count
    }

    pub(crate) fn require_suite(
        self,
        suite: PhysicalIntegrityAcceptanceSuite,
    ) -> Result<(), PhysicalIntegrityCloseoutDenial> {
        if self.acceptance_suite != suite || self.output_count == 0 {
            Err(PhysicalIntegrityCloseoutDenial::MissingExecutedSuiteOutput(
                suite,
            ))
        } else {
            Ok(())
        }
    }

    const fn new(
        acceptance_suite: PhysicalIntegrityAcceptanceSuite,
        output_kind: IntegrityCloseoutExecutedOutputKind,
        output_count: usize,
    ) -> Self {
        Self {
            acceptance_suite,
            output_kind,
            output_count: output_count as u32,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExecutedIntegrityCloseoutHarnessRun {
    acceptance_suite: PhysicalIntegrityAcceptanceSuite,
    plan: PhysicalScenarioPlan,
    transcript: PhysicalStoryTranscript,
    executed_output: IntegrityHarnessExecutionEvidence,
}

impl ExecutedIntegrityCloseoutHarnessRun {
    pub(crate) fn from_executed_output(
        acceptance_suite: PhysicalIntegrityAcceptanceSuite,
        plan: PhysicalScenarioPlan,
        transcript: PhysicalStoryTranscript,
        executed_output: IntegrityHarnessExecutionEvidence,
    ) -> Result<Self, PhysicalIntegrityCloseoutDenial> {
        executed_output.require_suite(acceptance_suite)?;
        if transcript.plan_identity() != plan.identity() {
            return Err(PhysicalIntegrityCloseoutDenial::MissingHarnessTranscript(
                acceptance_suite,
            ));
        }
        Ok(Self {
            acceptance_suite,
            plan,
            transcript,
            executed_output,
        })
    }

    pub const fn acceptance_suite(&self) -> PhysicalIntegrityAcceptanceSuite {
        self.acceptance_suite
    }

    pub const fn plan(&self) -> &PhysicalScenarioPlan {
        &self.plan
    }

    pub const fn transcript(&self) -> &PhysicalStoryTranscript {
        &self.transcript
    }

    pub const fn executed_output(&self) -> IntegrityHarnessExecutionEvidence {
        self.executed_output
    }
}

impl From<PhysicalIntegrityAcceptanceSuite> for IntegrityCloseoutEvidenceFamily {
    fn from(kind: PhysicalIntegrityAcceptanceSuite) -> Self {
        match kind {
            PhysicalIntegrityAcceptanceSuite::CorruptionLocalization => {
                Self::CorruptionLocalization
            }
            PhysicalIntegrityAcceptanceSuite::BoundaryDenial => Self::BoundaryDenial,
            PhysicalIntegrityAcceptanceSuite::HarnessTranscript => Self::HarnessTranscript,
            PhysicalIntegrityAcceptanceSuite::SyntheticShortcutRejection => {
                Self::SyntheticShortcutRejection
            }
            PhysicalIntegrityAcceptanceSuite::RecoveryIntegrityHandoff => {
                Self::RecoveryIntegrityHandoff
            }
            PhysicalIntegrityAcceptanceSuite::LineCapComposition => Self::LineCapComposition,
        }
    }
}
