pub use crate::migration::LayoutMigrationFacade;

pub const fn layout_migration() -> LayoutMigrationFacade {
    crate::migration::layout_migration()
}
