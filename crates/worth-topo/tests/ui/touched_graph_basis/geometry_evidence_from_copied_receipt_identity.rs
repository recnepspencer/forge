use forge_query::facade::ForgeQuerySessionLabel;
use topology::facade::{
    topology_geometry_only_evidence_touched_graph_basis, TopologyGeometryOnlyEvidenceDigest,
    TopologyTouchedOperatingWorld,
};

fn main() {
    let identity = ForgeQuerySessionLabel::scoped_strs(
        "worth-spatial.geometry-only-evidence",
        ["Split", "copied-spatial-receipt"],
    )
    .unwrap();
    let digest =
        TopologyGeometryOnlyEvidenceDigest::from_query_evidence_identity(identity.identity_digest());
    let _basis = topology_geometry_only_evidence_touched_graph_basis(
        digest,
        TopologyTouchedOperatingWorld::mainline(),
    );
}
