use forge_query::facade::runtime::{ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchReadVerb};

use super::{
    deny_copied_receipt_fields_as_spatial_query_lowering_authority,
    deny_query_descriptor_as_spatial_query_lowering_authority,
    deny_raw_row_as_spatial_query_lowering_authority,
    deny_topology_touched_basis_as_spatial_query_lowering_authority,
    SpatialEvidenceQueryLoweringDenialKind,
};

#[test]
fn query_descriptor_denial_rejects_lower_authority_substitutions_before_lowering() {
    let raw_row = deny_raw_row_as_spatial_query_lowering_authority("WorkloadEvidenceRow");
    assert_eq!(
        raw_row.kind(),
        SpatialEvidenceQueryLoweringDenialKind::RawRowSubstitution
    );
    assert!(raw_row.detail().contains("without spatial touch authority"));

    let copied_receipt =
        deny_copied_receipt_fields_as_spatial_query_lowering_authority("CopiedReceiptFields");
    assert_eq!(
        copied_receipt.kind(),
        SpatialEvidenceQueryLoweringDenialKind::CopiedReceiptSubstitution
    );

    let topology = deny_topology_touched_basis_as_spatial_query_lowering_authority(
        "TopologyDeclaredTouchedGraphBasisProof",
    );
    assert_eq!(
        topology.kind(),
        SpatialEvidenceQueryLoweringDenialKind::TopologyTouchedBasisSubstitution
    );

    let descriptor = ForgeQueryGraphTouchDescriptor::read_family(
        "worth.spatial.evidence_touch",
        [ForgeQueryGraphTouchReadVerb::ObservesCollection],
    )
    .expect("fixture Query descriptor should construct");
    let query_descriptor = deny_query_descriptor_as_spatial_query_lowering_authority(&descriptor);
    assert_eq!(
        query_descriptor.kind(),
        SpatialEvidenceQueryLoweringDenialKind::QueryDescriptorSubstitution
    );
    assert!(query_descriptor
        .detail()
        .contains("cannot lower itself or reconstruct spatial touch authority"));
}
