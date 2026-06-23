use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::platform::relations::TopologyRelationKind;
use schema::facade::topology_authoring::DerivedTopologyReadBasis;

use crate::certification::support::read_proof_harness::TopologyReadProofHarness;
use crate::projection::read_views::domain::parity::{
    build_topology_read_view_parity_artifact, TopologyReadViewParityArtifact, TopologyReadViewRef,
};

use super::support::{current_lookup_rows, snapshot_basis_workspace};

pub(super) fn local_rewire_parity_artifact(
    topology_read: &TopologyReadProofHarness,
    runtime: &RelationalRuntime,
    stem: &str,
    read_basis: &DerivedTopologyReadBasis,
) -> TopologyReadViewParityArtifact {
    let (mut workspace, surfaces) = snapshot_basis_workspace(runtime, stem, read_basis);
    let lookup_rows = current_lookup_rows(&mut workspace, &surfaces);
    let moved_identity = lookup_rows
        .lookup()
        .first_source_identity_for_relation_kind(TopologyRelationKind::HalfEdgeNext)
        .expect("sheet disk should expose successor source");
    let local_rewire = topology_read
        .local_rewire_neighborhood(&mut workspace, &moved_identity, 4)
        .expect("local rewire neighborhood should load");
    build_topology_read_view_parity_artifact(
        read_basis,
        TopologyReadViewRef::LocalRewire(&local_rewire),
    )
}

pub(super) fn loop_cycle_parity_artifact(
    topology_read: &TopologyReadProofHarness,
    runtime: &RelationalRuntime,
    stem: &str,
    read_basis: &DerivedTopologyReadBasis,
    depth: usize,
) -> TopologyReadViewParityArtifact {
    let (mut workspace, surfaces) = snapshot_basis_workspace(runtime, stem, read_basis);
    let lookup_rows = current_lookup_rows(&mut workspace, &surfaces);
    let start_identity = lookup_rows
        .lookup()
        .first_source_identity_for_relation_kind(TopologyRelationKind::HalfEdgeNext)
        .expect("wire should expose successor source");
    let loop_cycle = topology_read
        .loop_cycle(&mut workspace, &start_identity, depth)
        .expect("loop cycle should load");
    build_topology_read_view_parity_artifact(
        read_basis,
        TopologyReadViewRef::LoopCycle(&loop_cycle),
    )
}

pub(super) fn radial_parity_artifact(
    topology_read: &TopologyReadProofHarness,
    runtime: &RelationalRuntime,
    stem: &str,
    read_basis: &DerivedTopologyReadBasis,
) -> TopologyReadViewParityArtifact {
    let (mut workspace, surfaces) = snapshot_basis_workspace(runtime, stem, read_basis);
    let lookup_rows = current_lookup_rows(&mut workspace, &surfaces);
    let source_identity = lookup_rows
        .lookup()
        .first_source_identity_for_relation_kind(TopologyRelationKind::HalfEdgeRadialNext)
        .expect("edge fan should expose radial source");
    let radial = topology_read
        .radial_half_edge_neighborhood(&mut workspace, &source_identity)
        .expect("radial neighborhood should load");
    build_topology_read_view_parity_artifact(read_basis, TopologyReadViewRef::Radial(&radial))
}
