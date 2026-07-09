mod compatibility_window;
mod denial;
mod evolution_declaration;
mod facade;
mod interruption;
mod migration_plan;
mod rollback_plan;
mod stale_rebind;
#[cfg(test)]
mod test_support;
#[cfg(test)]
mod tests;
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
pub use version::LayoutVersion;
