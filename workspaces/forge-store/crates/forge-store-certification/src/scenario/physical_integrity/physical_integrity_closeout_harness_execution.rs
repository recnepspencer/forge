use crate::{
    PhysicalIntegrityCloseoutDenial, PhysicalScenarioPlan, PhysicalStoryTranscript,
    S3ExecutedBoundaryDenialEvidence, S3ExecutedCorruptionLocalizationEvidence,
    S3LineCapCompositionEvidence, S3S4HandoffCloseoutEvidence,
    SyntheticCloseoutShortcutRejectionReport,
};
use crate::{S3AcceptanceSuiteKind, S3CloseoutEvidenceFamily};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S3CloseoutExecutedOutputKind {
    CorruptionLocalization,
    BoundaryDenial,
    HarnessTranscript,
    SyntheticShortcutRejection,
    S4IntegrityHandoff,
    LineCapComposition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S3CloseoutHarnessExecutionEvidence {
    acceptance_suite: S3AcceptanceSuiteKind,
    output_kind: S3CloseoutExecutedOutputKind,
    output_count: u32,
}

impl S3CloseoutHarnessExecutionEvidence {
    pub(crate) fn corruption_localization(
        rows: &[S3ExecutedCorruptionLocalizationEvidence],
    ) -> Self {
        Self::new(
            S3AcceptanceSuiteKind::CorruptionLocalization,
            S3CloseoutExecutedOutputKind::CorruptionLocalization,
            rows.len(),
        )
    }

    pub(crate) fn boundary_denial(rows: &[S3ExecutedBoundaryDenialEvidence]) -> Self {
        Self::new(
            S3AcceptanceSuiteKind::BoundaryDenial,
            S3CloseoutExecutedOutputKind::BoundaryDenial,
            rows.len(),
        )
    }

    pub(crate) const fn harness_transcript(output_count: usize) -> Self {
        Self::new(
            S3AcceptanceSuiteKind::HarnessTranscript,
            S3CloseoutExecutedOutputKind::HarnessTranscript,
            output_count,
        )
    }

    pub(crate) fn synthetic_rejection(rows: &[SyntheticCloseoutShortcutRejectionReport]) -> Self {
        Self::new(
            S3AcceptanceSuiteKind::SyntheticShortcutRejection,
            S3CloseoutExecutedOutputKind::SyntheticShortcutRejection,
            rows.len(),
        )
    }

    pub(crate) fn recovery_handoff(evidence: &S3S4HandoffCloseoutEvidence) -> Self {
        let output_count = usize::from(evidence.proves_no_raw_bytes_crossed());
        Self::new(
            S3AcceptanceSuiteKind::S4IntegrityHandoff,
            S3CloseoutExecutedOutputKind::S4IntegrityHandoff,
            output_count,
        )
    }

    pub(crate) fn line_cap_composition(evidence: &S3LineCapCompositionEvidence) -> Self {
        Self::new(
            S3AcceptanceSuiteKind::LineCapComposition,
            S3CloseoutExecutedOutputKind::LineCapComposition,
            evidence.checked_surface_count(),
        )
    }

    pub const fn acceptance_suite(self) -> S3AcceptanceSuiteKind {
        self.acceptance_suite
    }

    pub const fn output_kind(self) -> S3CloseoutExecutedOutputKind {
        self.output_kind
    }

    pub const fn output_count(self) -> u32 {
        self.output_count
    }

    pub(crate) fn require_suite(
        self,
        suite: S3AcceptanceSuiteKind,
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
        acceptance_suite: S3AcceptanceSuiteKind,
        output_kind: S3CloseoutExecutedOutputKind,
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
pub(crate) struct S3ExecutedCloseoutHarnessRun {
    acceptance_suite: S3AcceptanceSuiteKind,
    plan: PhysicalScenarioPlan,
    transcript: PhysicalStoryTranscript,
    executed_output: S3CloseoutHarnessExecutionEvidence,
}

impl S3ExecutedCloseoutHarnessRun {
    pub(crate) fn from_executed_output(
        acceptance_suite: S3AcceptanceSuiteKind,
        plan: PhysicalScenarioPlan,
        transcript: PhysicalStoryTranscript,
        executed_output: S3CloseoutHarnessExecutionEvidence,
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

    pub const fn acceptance_suite(&self) -> S3AcceptanceSuiteKind {
        self.acceptance_suite
    }

    pub const fn plan(&self) -> &PhysicalScenarioPlan {
        &self.plan
    }

    pub const fn transcript(&self) -> &PhysicalStoryTranscript {
        &self.transcript
    }

    pub const fn executed_output(&self) -> S3CloseoutHarnessExecutionEvidence {
        self.executed_output
    }
}

impl From<S3AcceptanceSuiteKind> for S3CloseoutEvidenceFamily {
    fn from(kind: S3AcceptanceSuiteKind) -> Self {
        match kind {
            S3AcceptanceSuiteKind::CorruptionLocalization => Self::CorruptionLocalization,
            S3AcceptanceSuiteKind::BoundaryDenial => Self::BoundaryDenial,
            S3AcceptanceSuiteKind::HarnessTranscript => Self::HarnessTranscript,
            S3AcceptanceSuiteKind::SyntheticShortcutRejection => Self::SyntheticShortcutRejection,
            S3AcceptanceSuiteKind::S4IntegrityHandoff => Self::S4IntegrityHandoff,
            S3AcceptanceSuiteKind::LineCapComposition => Self::LineCapComposition,
        }
    }
}
