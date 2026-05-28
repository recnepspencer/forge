use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::platform::authority::DerivedTopologyReadBasis;
use schema::facade::platform::relations::TopologyRelationKind;

use crate::projection::read_views::domain::parity::{
    build_domain_query_view_parity_artifact, TopologyDomainQueryViewParityArtifact,
    TopologyDomainQueryViewRef,
};
use crate::projection::read_views::domain::TopologyDomainQuery;

use super::support::{current_lookup_rows, snapshot_basis_workspace};

pub(super) fn local_rewire_parity_artifact(
    domain_query: &TopologyDomainQuery,
    runtime: &RelationalRuntime,
    stem: &str,
    read_basis: &DerivedTopologyReadBasis,
) -> TopologyDomainQueryViewParityArtifact {
    let (mut workspace, assembly) = snapshot_basis_workspace(runtime, stem, read_basis);
    let lookup_rows = current_lookup_rows(&mut workspace, &assembly);
    let moved_identity = lookup_rows
        .lookup()
        .first_source_identity_for_relation_kind(TopologyRelationKind::HalfEdgeNext)
        .expect("sheet disk should expose successor source");
    let local_rewire = domain_query
        .local_rewire_neighborhood(&mut workspace, &moved_identity, 6)
        .expect("local rewire neighborhood should load");
    build_domain_query_view_parity_artifact(
        read_basis,
        TopologyDomainQueryViewRef::LocalRewire(&local_rewire),
    )
}

pub(super) fn loop_cycle_parity_artifact(
    domain_query: &TopologyDomainQuery,
    runtime: &RelationalRuntime,
    stem: &str,
    read_basis: &DerivedTopologyReadBasis,
    depth: usize,
) -> TopologyDomainQueryViewParityArtifact {
    let (mut workspace, assembly) = snapshot_basis_workspace(runtime, stem, read_basis);
    let lookup_rows = current_lookup_rows(&mut workspace, &assembly);
    let start_identity = lookup_rows
        .lookup()
        .first_source_identity_for_relation_kind(TopologyRelationKind::HalfEdgeNext)
        .expect("wire should expose successor source");
    let loop_cycle = domain_query
        .loop_cycle(&mut workspace, &start_identity, depth)
        .expect("loop cycle should load");
    build_domain_query_view_parity_artifact(
        read_basis,
        TopologyDomainQueryViewRef::LoopCycle(&loop_cycle),
    )
}

pub(super) fn radial_parity_artifact(
    domain_query: &TopologyDomainQuery,
    runtime: &RelationalRuntime,
    stem: &str,
    read_basis: &DerivedTopologyReadBasis,
) -> TopologyDomainQueryViewParityArtifact {
    let (mut workspace, assembly) = snapshot_basis_workspace(runtime, stem, read_basis);
    let lookup_rows = current_lookup_rows(&mut workspace, &assembly);
    let source_identity = lookup_rows
        .lookup()
        .first_source_identity_for_relation_kind(TopologyRelationKind::HalfEdgeRadialNext)
        .expect("edge fan should expose radial source");
    let radial = domain_query
        .radial_half_edge_neighborhood(&mut workspace, &source_identity)
        .expect("radial neighborhood should load");
    build_domain_query_view_parity_artifact(read_basis, TopologyDomainQueryViewRef::Radial(&radial))
}




