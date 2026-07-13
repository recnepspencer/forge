mod compatibility_window;
mod denial;
mod evolution_declaration;
mod interruption;
mod migration_plan;
mod planning;
mod rollback_plan;
mod stale_rebind;
#[cfg(test)]
pub(crate) mod test_support;
#[cfg(test)]
pub(crate) mod tests;
mod transition_outcome;
mod version;

pub use compatibility_window::{
    LayoutCompatibilityWindow, LayoutReadCompatibilityPosture, LayoutWriteCompatibilityPosture,
};
pub use denial::LayoutEvolutionDenial;
pub use evolution_declaration::{LayoutBindingWitness, LayoutEvolutionDeclaration};
pub use interruption::{
    LayoutInterruptedMigrationDisposition, LayoutInterruptionPolicy, LayoutInterruptionState,
};
pub use migration_plan::{LayoutMigrationPlan, LayoutMigrationRequest, LayoutPlanFingerprint};
pub use planning::{layout_migration, LayoutMigrationFacade};
pub use rollback_plan::{LayoutRollbackPlan, LayoutRollbackRequest};
pub use stale_rebind::{LayoutRebindRequired, LayoutStaleBinding};
pub use transition_outcome::{
    migration_planning_cases, rollback_planning_cases, MigrationPlanningCaseId,
    MigrationPlanningOutcome, MigrationPlanningView, RollbackPlanningCaseId,
    RollbackPlanningOutcome, RollbackPlanningView,
};
pub use version::LayoutVersion;
