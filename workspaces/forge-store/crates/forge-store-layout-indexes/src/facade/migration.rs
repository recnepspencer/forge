use crate::migration::S8LayoutMigration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutMigrationFacade;

impl LayoutMigrationFacade {
    pub const fn versioning(&self) -> S8LayoutMigration {
        S8LayoutMigration
    }
}

pub const fn layout_migration() -> LayoutMigrationFacade {
    LayoutMigrationFacade
}
