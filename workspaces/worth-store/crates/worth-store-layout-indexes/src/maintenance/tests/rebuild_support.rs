use crate::strategy::tests_support::{admitted_page_key_bytes, admitted_wal_key_bytes};
use crate::{
    access_shapes, AccessLaneClassification, DerivedIndexParityBasis, DerivedIndexParityRow,
};
use worth_store_physical_format::PhysicalRootManifestRebuildWitness;
use worth_store_wal::BlobWalReplayRebuildWitness;

pub(super) fn root_rebuild_setup(
    family: crate::AdmittedPhysicalArtifactFamily,
    source: &worth_store_physical_format::PhysicalRootManifestRebuildSource,
) -> (
    crate::access::shape::AccessShapeContract,
    crate::AdmittedLayoutMaterialization,
) {
    let shape = access_shapes()
        .rebuild_read(AccessLaneClassification::Maintenance)
        .unwrap();
    (
        shape,
        crate::maintenance::test_support::root_materialization(family, source),
    )
}

pub(super) fn wal_rebuild_setup(
    family: crate::AdmittedPhysicalArtifactFamily,
) -> (
    crate::access::shape::AccessShapeContract,
    crate::AdmittedLayoutMaterialization,
) {
    let shape = access_shapes()
        .rebuild_read(AccessLaneClassification::Maintenance)
        .unwrap();
    (
        shape,
        crate::maintenance::test_support::wal_materialization(family),
    )
}

pub(super) fn root_rebuilt_parity_basis_with_value(
    coverage: crate::LayoutCoverageWitness,
    source_witness: &PhysicalRootManifestRebuildWitness,
    value: &str,
) -> DerivedIndexParityBasis {
    let row = &source_witness.rows()[0];
    DerivedIndexParityBasis::new(
        vec![DerivedIndexParityRow::new(
            admitted_page_key_bytes(row.segment_id().get(), row.page_id().get()),
            value,
        )],
        coverage,
        true,
        source_witness.counter_shape().to_vec(),
    )
    .unwrap()
}

pub(super) fn wal_rebuilt_parity_basis_with_value(
    coverage: crate::LayoutCoverageWitness,
    source_witness: &BlobWalReplayRebuildWitness,
    value: &str,
) -> DerivedIndexParityBasis {
    DerivedIndexParityBasis::new(
        vec![DerivedIndexParityRow::new(
            admitted_wal_key_bytes(source_witness.record().identity().sequence()),
            value,
        )],
        coverage,
        true,
        source_witness.counter_shape().to_vec(),
    )
    .unwrap()
}
