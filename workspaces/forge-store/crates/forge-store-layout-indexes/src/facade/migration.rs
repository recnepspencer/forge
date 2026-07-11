pub use crate::evolution::migration::LayoutMigrationFacade;

pub const fn layout_migration() -> LayoutMigrationFacade {
    crate::evolution::migration::layout_migration()
}
