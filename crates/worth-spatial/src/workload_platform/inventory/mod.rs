mod classification;
mod decision;
mod registry;
mod report;
mod validation;

pub use classification::{
    LegacyFixtureClassification, ReceiptPosture, SurfaceAuthority, SurfaceKind, SurfaceScope,
    TopologyPosture, WorkloadSurfaceId,
};
pub use decision::InventoryDecision;
pub use registry::existing_seed_inventory_rows;
pub use report::{SeedInventoryCounters, SeedInventoryReport, SeedInventoryRow};
pub use validation::{InventoryValidationError, InventoryValidationErrorKind};
