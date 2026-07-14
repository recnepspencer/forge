use super::{
    LayoutBindingWitness, LayoutEvolutionDeclaration, LayoutEvolutionDenial, LayoutPlanFingerprint,
    LayoutRollbackRequest,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutInterruptionPolicy {
    ResumeDeclaredMigration,
    RollbackDeclaredMigration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutInterruptionBoundary {
    SourceBound,
    TargetPublished,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutInterruptionFingerprint {
    Plan(Box<LayoutPlanFingerprint>),
    MigrationExecution(Box<super::LayoutMigrationExecutionFingerprint>),
}

impl LayoutInterruptionFingerprint {
    pub(in crate::evolution::migration) fn plan(fingerprint: LayoutPlanFingerprint) -> Self {
        Self::Plan(Box::new(fingerprint))
    }

    pub(in crate::evolution::migration) fn migration_execution(
        fingerprint: super::LayoutMigrationExecutionFingerprint,
    ) -> Self {
        Self::MigrationExecution(Box::new(fingerprint))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutInterruptionState {
    fingerprint: LayoutInterruptionFingerprint,
    binding: LayoutBindingWitness,
    boundary: LayoutInterruptionBoundary,
}

impl LayoutInterruptionState {
    pub(crate) const fn new(
        fingerprint: LayoutInterruptionFingerprint,
        binding: LayoutBindingWitness,
        boundary: LayoutInterruptionBoundary,
    ) -> Self {
        Self {
            fingerprint,
            binding,
            boundary,
        }
    }

    pub fn fingerprint(&self) -> LayoutInterruptionFingerprint {
        self.fingerprint.clone()
    }

    pub const fn binding(&self) -> &LayoutBindingWitness {
        &self.binding
    }

    pub const fn boundary(&self) -> LayoutInterruptionBoundary {
        self.boundary
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutInterruptedMigrationDisposition {
    Resume(LayoutInterruptionState),
    RemainAtSource(LayoutInterruptionState),
    Rollback(LayoutRollbackRequest),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum LayoutMigrationInterruptionCase {
    Resume(LayoutInterruptionState),
    RemainAtSource(LayoutInterruptionState),
    Rollback(LayoutRollbackRequest),
    Denied(LayoutEvolutionDenial),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutMigrationInterruptionOutcome {
    case: LayoutMigrationInterruptionCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutMigrationInterruptionView<'a> {
    Resume(&'a LayoutInterruptionState),
    RemainAtSource(&'a LayoutInterruptionState),
    Rollback(&'a LayoutRollbackRequest),
    Denied(&'a LayoutEvolutionDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayoutMigrationInterruptionCaseId(&'static str);

impl LayoutMigrationInterruptionCaseId {
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

pub fn layout_migration_interruption_cases(
) -> impl Iterator<Item = LayoutMigrationInterruptionCaseId> {
    [
        LayoutMigrationInterruptionCaseId("layout.migration.interruption.resume"),
        LayoutMigrationInterruptionCaseId("layout.migration.interruption.remain_at_source"),
        LayoutMigrationInterruptionCaseId("layout.migration.interruption.rollback"),
        LayoutMigrationInterruptionCaseId("layout.migration.interruption.denied.plan_mismatch"),
    ]
    .into_iter()
}

impl LayoutMigrationInterruptionOutcome {
    fn issue(case: LayoutMigrationInterruptionCase) -> Self {
        Self { case }
    }

    pub const fn view(&self) -> LayoutMigrationInterruptionView<'_> {
        match &self.case {
            LayoutMigrationInterruptionCase::Resume(value) => {
                LayoutMigrationInterruptionView::Resume(value)
            }
            LayoutMigrationInterruptionCase::RemainAtSource(value) => {
                LayoutMigrationInterruptionView::RemainAtSource(value)
            }
            LayoutMigrationInterruptionCase::Rollback(value) => {
                LayoutMigrationInterruptionView::Rollback(value)
            }
            LayoutMigrationInterruptionCase::Denied(value) => {
                LayoutMigrationInterruptionView::Denied(value)
            }
        }
    }

    pub const fn case_id(&self) -> LayoutMigrationInterruptionCaseId {
        match self.case {
            LayoutMigrationInterruptionCase::Resume(_) => {
                LayoutMigrationInterruptionCaseId("layout.migration.interruption.resume")
            }
            LayoutMigrationInterruptionCase::RemainAtSource(_) => {
                LayoutMigrationInterruptionCaseId("layout.migration.interruption.remain_at_source")
            }
            LayoutMigrationInterruptionCase::Rollback(_) => {
                LayoutMigrationInterruptionCaseId("layout.migration.interruption.rollback")
            }
            LayoutMigrationInterruptionCase::Denied(_) => LayoutMigrationInterruptionCaseId(
                "layout.migration.interruption.denied.plan_mismatch",
            ),
        }
    }

    pub fn into_result(
        self,
    ) -> Result<LayoutInterruptedMigrationDisposition, LayoutEvolutionDenial> {
        match self.case {
            LayoutMigrationInterruptionCase::Resume(value) => {
                Ok(LayoutInterruptedMigrationDisposition::Resume(value))
            }
            LayoutMigrationInterruptionCase::RemainAtSource(value) => {
                Ok(LayoutInterruptedMigrationDisposition::RemainAtSource(value))
            }
            LayoutMigrationInterruptionCase::Rollback(value) => {
                Ok(LayoutInterruptedMigrationDisposition::Rollback(value))
            }
            LayoutMigrationInterruptionCase::Denied(denial) => Err(denial),
        }
    }
}

pub(super) fn classify_migration_interruption(
    expected: LayoutInterruptionFingerprint,
    declaration: LayoutEvolutionDeclaration,
    interruption: LayoutInterruptionState,
) -> LayoutMigrationInterruptionOutcome {
    if interruption.fingerprint() != expected {
        return LayoutMigrationInterruptionOutcome::issue(LayoutMigrationInterruptionCase::Denied(
            LayoutEvolutionDenial::InterruptStateDoesNotMatchPlan {
                expected: Box::new(expected),
                actual: Box::new(interruption.fingerprint()),
            },
        ));
    }

    LayoutMigrationInterruptionOutcome::issue(
        match (declaration.interruption_policy(), interruption.boundary()) {
            (LayoutInterruptionPolicy::ResumeDeclaredMigration, _) => {
                LayoutMigrationInterruptionCase::Resume(interruption)
            }
            (
                LayoutInterruptionPolicy::RollbackDeclaredMigration,
                LayoutInterruptionBoundary::SourceBound,
            ) => LayoutMigrationInterruptionCase::RemainAtSource(interruption),
            (
                LayoutInterruptionPolicy::RollbackDeclaredMigration,
                LayoutInterruptionBoundary::TargetPublished,
            ) => {
                let current_family = interruption.binding.admitted_family();
                LayoutMigrationInterruptionCase::Rollback(LayoutRollbackRequest::new(
                    declaration,
                    interruption.binding,
                    current_family,
                ))
            }
        },
    )
}
