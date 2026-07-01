use super::residue_manifest::{
    current_spatial_consumer_residue_manifest, SpatialConsumerResidueDisposition,
    SpatialConsumerResidueOwner,
};

pub(crate) fn require_exact_spatial_consumer_closeout() {
    let residue = current_spatial_consumer_residue_manifest();

    assert_eq!(residue.len(), 2);
    assert_eq!(
        residue[0].source_path(),
        "crates/worth-spatial/src/workload_platform/evidence_lookup_public_closeout/current_source.rs"
    );
    assert_eq!(
        residue[0].current_surface(),
        "current_evidence_lookup_public_closeout"
    );
    assert_eq!(
        residue[0].owner(),
        SpatialConsumerResidueOwner::WorthSpatial
    );
    assert_eq!(
        residue[0].disposition(),
        SpatialConsumerResidueDisposition::ExplicitResidue
    );
    assert_eq!(
        residue[1].source_path(),
        "crates/worth-spatial/src/workload_platform/evidence_lookup_public_closeout/current_source.rs"
    );
    assert_eq!(
        residue[1].current_surface(),
        "current_evidence_lookup_public_closeout_assembly_input"
    );
    assert_eq!(
        residue[1].owner(),
        SpatialConsumerResidueOwner::WorthSpatial
    );
    assert_eq!(
        residue[1].disposition(),
        SpatialConsumerResidueDisposition::CertificationOnly
    );
    assert!(residue.iter().all(|row| {
        row.disposition() != SpatialConsumerResidueDisposition::AuthoritativeOrdinaryConsumer
    }));
}
