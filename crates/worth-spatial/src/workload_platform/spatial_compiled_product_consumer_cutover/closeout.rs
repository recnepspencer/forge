use super::residue_manifest::{
    current_spatial_consumer_residue_manifest, SpatialConsumerResidueDisposition,
    SpatialConsumerResidueOwner,
};

pub(crate) fn require_exact_spatial_consumer_closeout() {
    let residue = current_spatial_consumer_residue_manifest();

    assert_eq!(residue.len(), 1);
    assert_eq!(
        residue[0].source_path(),
        "crates/worth-spatial/src/workload_platform/planner_owned_routing/public_closeout_route/current.rs"
    );
    assert_eq!(
        residue[0].current_surface(),
        "current_evidence_lookup_public_closeout_assembly_input"
    );
    assert_eq!(
        residue[0].owner(),
        SpatialConsumerResidueOwner::WorthSpatial
    );
    assert_eq!(
        residue[0].disposition(),
        SpatialConsumerResidueDisposition::CertificationOnly
    );
    assert!(residue.iter().all(|row| {
        row.disposition() != SpatialConsumerResidueDisposition::AuthoritativeOrdinaryConsumer
    }));
}
