mod compile_boundary;
mod compile_boundary_certification;
mod consumer_audit;
mod definition;
mod hot_reload_admission;
mod inventory;
mod inventory_data;
mod query_owned_inventory;
mod query_projection;
mod slice_lowering;

pub use compile_boundary::WorthUiSemanticCompileBoundary;
pub use compile_boundary_certification::{
    WorthUiCompileBoundaryCertification, WorthUiCompileBoundaryPosture,
};
pub use consumer_audit::{
    WorthUiSemanticConsumerAuditFinding, WorthUiSemanticConsumerAuditFindingKind,
    WorthUiSemanticConsumerAuditReceipt,
};
pub use definition::{
    WorthUiSemanticMeaningClass, WorthUiSemanticSliceConsumers, WorthUiSemanticSliceFactMapping,
    WorthUiSemanticSliceId, WorthUiSemanticSliceOwner,
};
pub use hot_reload_admission::{
    WorthUiAdmittedHotReloadableSemanticSliceSet, WorthUiHotReloadableSemanticSlice,
    WorthUiSemanticHotReloadAdmissionDenial,
};
pub use inventory::{WorthUiSemanticSliceDescriptor, WorthUiSemanticSliceInventory};
pub use query_owned_inventory::WorthUiQueryOwnedSemanticSliceInventory;
pub use query_projection::WorthUiQuerySemanticSliceProjection;
pub use slice_lowering::{
    WorthUiSemanticChangedSliceRow, WorthUiSemanticChangedSliceSet,
    WorthUiSemanticSliceLoweringCause,
};

#[cfg(test)]
mod tests;
