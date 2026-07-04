mod closeout;
#[cfg(test)]
mod helper_surface_inventory;
mod residue_manifest;
mod spatial_consumer_cluster;

#[cfg(test)]
mod tests;

pub use residue_manifest::{
    current_spatial_consumer_residue_manifest, SpatialConsumerResidueDisposition,
    SpatialConsumerResidueOwner, SpatialConsumerResidueRow,
};
pub use spatial_consumer_cluster::{
    admit_lookup_execution_handoff_match, admit_lookup_product_handoff_match,
    admit_retained_replay_capture, build_retained_replay_parity_report,
    lower_evidence_lookup_index_product, require_retained_capture_receipt,
    reuse_evidence_lookup_index_product, SpatialLookupConsumerRouteDenial,
    SpatialLookupConsumerRouteDenialKind,
};

#[cfg(test)]
pub(crate) use closeout::require_exact_spatial_consumer_closeout;
#[cfg(test)]
pub(crate) use helper_surface_inventory::{
    current_displaced_evidence_index_helper_surface_inventory,
    DisplacedEvidenceIndexHelperSurfaceDisposition,
};
