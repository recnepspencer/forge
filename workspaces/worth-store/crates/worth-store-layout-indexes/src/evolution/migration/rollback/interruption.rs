use super::{LayoutBindingWitness, LayoutEvolutionDenial, LayoutRollbackExecutionFingerprint};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutRollbackInterruptionBoundary {
    SourceBound,
    TargetPublished,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutRollbackInterruptionState {
    fingerprint: LayoutRollbackExecutionFingerprint,
    binding: LayoutBindingWitness,
    boundary: LayoutRollbackInterruptionBoundary,
}

impl LayoutRollbackInterruptionState {
    pub(super) const fn source_bound(
        fingerprint: LayoutRollbackExecutionFingerprint,
        binding: LayoutBindingWitness,
    ) -> Self {
        Self {
            fingerprint,
            binding,
            boundary: LayoutRollbackInterruptionBoundary::SourceBound,
        }
    }

    pub(super) const fn target_published(
        fingerprint: LayoutRollbackExecutionFingerprint,
        binding: LayoutBindingWitness,
    ) -> Self {
        Self {
            fingerprint,
            binding,
            boundary: LayoutRollbackInterruptionBoundary::TargetPublished,
        }
    }

    pub const fn fingerprint(&self) -> LayoutRollbackExecutionFingerprint {
        self.fingerprint
    }

    pub const fn binding(&self) -> &LayoutBindingWitness {
        &self.binding
    }

    pub const fn boundary(&self) -> LayoutRollbackInterruptionBoundary {
        self.boundary
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutRollbackInterruptionPosture {
    ResumeFromSource,
    TargetAlreadyPublished,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum LayoutRollbackInterruptionCase {
    Classified(LayoutRollbackInterruptionPosture),
    Denied(Box<LayoutEvolutionDenial>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutRollbackInterruptionOutcome {
    case: LayoutRollbackInterruptionCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutRollbackInterruptionView<'a> {
    Classified(LayoutRollbackInterruptionPosture),
    Denied(&'a LayoutEvolutionDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayoutRollbackInterruptionCaseId(&'static str);

impl LayoutRollbackInterruptionCaseId {
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

pub fn layout_rollback_interruption_cases() -> impl Iterator<Item = LayoutRollbackInterruptionCaseId>
{
    [
        LayoutRollbackInterruptionCaseId("layout.rollback.interruption.resume_source"),
        LayoutRollbackInterruptionCaseId("layout.rollback.interruption.target_published"),
        LayoutRollbackInterruptionCaseId("layout.rollback.interruption.denied.execution_mismatch"),
    ]
    .into_iter()
}

impl LayoutRollbackInterruptionOutcome {
    fn classified(posture: LayoutRollbackInterruptionPosture) -> Self {
        Self {
            case: LayoutRollbackInterruptionCase::Classified(posture),
        }
    }

    fn denied(denial: LayoutEvolutionDenial) -> Self {
        Self {
            case: LayoutRollbackInterruptionCase::Denied(Box::new(denial)),
        }
    }

    pub const fn view(&self) -> LayoutRollbackInterruptionView<'_> {
        match &self.case {
            LayoutRollbackInterruptionCase::Classified(posture) => {
                LayoutRollbackInterruptionView::Classified(*posture)
            }
            LayoutRollbackInterruptionCase::Denied(denial) => {
                LayoutRollbackInterruptionView::Denied(denial)
            }
        }
    }

    pub const fn case_id(&self) -> LayoutRollbackInterruptionCaseId {
        match self.case {
            LayoutRollbackInterruptionCase::Classified(
                LayoutRollbackInterruptionPosture::ResumeFromSource,
            ) => LayoutRollbackInterruptionCaseId("layout.rollback.interruption.resume_source"),
            LayoutRollbackInterruptionCase::Classified(
                LayoutRollbackInterruptionPosture::TargetAlreadyPublished,
            ) => LayoutRollbackInterruptionCaseId("layout.rollback.interruption.target_published"),
            LayoutRollbackInterruptionCase::Denied(_) => LayoutRollbackInterruptionCaseId(
                "layout.rollback.interruption.denied.execution_mismatch",
            ),
        }
    }

    pub fn into_result(self) -> Result<LayoutRollbackInterruptionPosture, LayoutEvolutionDenial> {
        match self.case {
            LayoutRollbackInterruptionCase::Classified(posture) => Ok(posture),
            LayoutRollbackInterruptionCase::Denied(denial) => Err(*denial),
        }
    }
}

pub(super) fn classify_rollback_interruption(
    expected: LayoutRollbackExecutionFingerprint,
    interruption: LayoutRollbackInterruptionState,
) -> LayoutRollbackInterruptionOutcome {
    if interruption.fingerprint != expected {
        return LayoutRollbackInterruptionOutcome::denied(
            LayoutEvolutionDenial::RollbackInterruptStateDoesNotMatchExecution {
                expected: Box::new(expected),
                actual: Box::new(interruption.fingerprint),
            },
        );
    }

    LayoutRollbackInterruptionOutcome::classified(match interruption.boundary {
        LayoutRollbackInterruptionBoundary::SourceBound => {
            LayoutRollbackInterruptionPosture::ResumeFromSource
        }
        LayoutRollbackInterruptionBoundary::TargetPublished => {
            LayoutRollbackInterruptionPosture::TargetAlreadyPublished
        }
    })
}
