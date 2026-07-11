mod compatibility_window;
mod denial;
mod evolution_declaration;
mod facade;
mod interruption;
mod migration_plan;
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
pub use facade::{layout_migration, LayoutMigrationFacade};
pub use interruption::{
    LayoutInterruptedMigrationDisposition, LayoutInterruptionPolicy, LayoutInterruptionState,
};
pub use migration_plan::{
    LayoutMigrationOutcome, LayoutMigrationPlan, LayoutMigrationRequest, LayoutPlanFingerprint,
};
pub use rollback_plan::{LayoutRollbackOutcome, LayoutRollbackPlan, LayoutRollbackRequest};
pub use stale_rebind::{S8LayoutRebindRequired, S8LayoutStaleBinding};
pub use transition_outcome::{
    S8MigrationPlanningOutcome, S8MigrationPlanningView, S8RollbackPlanningOutcome,
    S8RollbackPlanningView,
};
pub use version::LayoutVersion;
