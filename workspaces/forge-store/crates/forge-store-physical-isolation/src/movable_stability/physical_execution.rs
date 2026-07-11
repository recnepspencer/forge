#[cfg(any(test, feature = "certification-authority"))]
pub use super::receipt_construction::physical_placement_movement_execution_for_certification_test;
pub use super::receipt_construction::PhysicalPlacementMovementExecutionReceipt;
