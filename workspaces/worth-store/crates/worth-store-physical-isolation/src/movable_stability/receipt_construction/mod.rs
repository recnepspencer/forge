pub mod physical_execution_receipt;

#[cfg(any(test, feature = "certification-authority"))]
pub use physical_execution_receipt::physical_placement_movement_execution_for_certification_test;
pub use physical_execution_receipt::PhysicalPlacementMovementExecutionReceipt;
