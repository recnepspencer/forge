use forge_query::facade::ForgeQuerySessionLabel;
use topology::facade::TopologyTouchedOperatingWorld;
use topology::spatial_sealed_receipt_admission::{
    topology_geometry_only_evidence_touched_graph_basis_from_spatial_sealed_receipt,
    TopologyGeometryOnlyEvidenceDigest,
};

fn main() {
    let copied_receipt_identity = ForgeQuerySessionLabel::scoped_strs(
        "worth-spatial.geometry-only-evidence",
        ["Split".to_owned(), "copied-spatial-receipt".to_owned()],
    )
    .expect("public Query labels are not topology geometry authority");
    let copied_digest =
        TopologyGeometryOnlyEvidenceDigest::from_spatial_sealed_boolean_receipt_identity(
            copied_receipt_identity.identity_digest(),
        );
    let _ = topology_geometry_only_evidence_touched_graph_basis_from_spatial_sealed_receipt(
        copied_digest,
        TopologyTouchedOperatingWorld::mainline(),
    );
}
