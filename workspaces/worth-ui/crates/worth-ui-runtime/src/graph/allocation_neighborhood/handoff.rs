//! Graph-owned neighborhood handoff: snapshot + basis → allocation neighborhood authority.

use crate::evidence::{
    UiAllocationNeighborhood, UiAllocationNeighborhoodClass, UiMeasurementBasis,
};
use crate::graph::allocation_neighborhood::membership::{
    derive_members, layout_participates_in_planning,
};
use crate::graph::allocation_neighborhood::membership_rule::classify_allocation_neighborhood_membership_rule;
use crate::graph::allocation_neighborhood::operator_contract::construct_allocation_neighborhood_operator_contract;
use crate::graph::{
    UiAllocationNeighborhoodDenial, UiGraphLookup, UiGraphNodeRecord, UiGraphParticipationAxis,
    UiGraphSnapshot,
};
use crate::obligations::selection::UiSelectedObligationSet;

pub(crate) fn admit_neighborhood_from_graph(
    snapshot: &UiGraphSnapshot,
    basis: &UiMeasurementBasis,
) -> Result<UiAllocationNeighborhood, UiAllocationNeighborhoodDenial> {
    let Some(root_record) = collect_root_graph_node(snapshot, basis) else {
        return Err(UiAllocationNeighborhoodDenial::UnknownRootGraphNode {
            graph_node_identity: basis.graph_node_identity(),
        });
    };
    classify_layout_participation(
        snapshot,
        basis.graph_node_identity(),
        root_record.value_ref(),
    )?;
    construct_allocation_neighborhood(snapshot, basis, root_record.value_ref())
}

pub(crate) fn admit_neighborhood_for_touch(
    snapshot: &UiGraphSnapshot,
    selected: &UiSelectedObligationSet,
    basis: &UiMeasurementBasis,
) -> Result<UiAllocationNeighborhood, UiAllocationNeighborhoodDenial> {
    classify_touch_target(selected, basis)?;
    admit_neighborhood_from_graph(snapshot, basis)
}

fn collect_root_graph_node(
    snapshot: &UiGraphSnapshot,
    basis: &UiMeasurementBasis,
) -> Option<UiGraphLookup<UiGraphNodeRecord>> {
    snapshot.lookup().graph_node(basis.graph_node_identity())
}

fn classify_layout_participation(
    snapshot: &UiGraphSnapshot,
    graph_node_identity: crate::graph::UiGraphNodeIdentity,
    root_record: &UiGraphNodeRecord,
) -> Result<(), UiAllocationNeighborhoodDenial> {
    let root_layout_participation = root_record
        .participation_posture()
        .axis(UiGraphParticipationAxis::Layout);
    if layout_participates_in_planning(snapshot, graph_node_identity, root_layout_participation) {
        Ok(())
    } else {
        Err(UiAllocationNeighborhoodDenial::RootNotLayoutParticipant {
            graph_node_identity,
        })
    }
}

fn classify_touch_target(
    selected: &UiSelectedObligationSet,
    basis: &UiMeasurementBasis,
) -> Result<(), UiAllocationNeighborhoodDenial> {
    let observed = selected.touch().target().graph_node_identity();
    let expected = basis.graph_node_identity();
    if observed != expected {
        return Err(UiAllocationNeighborhoodDenial::TouchTargetMismatch { expected, observed });
    }
    if selected.touch().world().world_profile().identity_digest()
        != basis.world_profile().identity_digest()
    {
        return Err(UiAllocationNeighborhoodDenial::WrongWorld);
    }
    Ok(())
}

fn construct_allocation_neighborhood(
    snapshot: &UiGraphSnapshot,
    basis: &UiMeasurementBasis,
    root_record: &UiGraphNodeRecord,
) -> Result<UiAllocationNeighborhood, UiAllocationNeighborhoodDenial> {
    let expected = basis.graph_node_identity();
    let neighborhood_class =
        UiAllocationNeighborhoodClass::from_measurement_hint(basis.neighborhood_class_hint());
    let membership_rule =
        classify_allocation_neighborhood_membership_rule(basis, root_record.operator_kind());
    let members = derive_members(snapshot, expected, membership_rule);
    let root_topology = snapshot
        .topology()
        .node_topology(expected)
        .expect("graph lookup node must have topology");

    Ok(UiAllocationNeighborhood::new_with_authority(
        expected,
        snapshot.generation(),
        basis.world_profile().identity_digest(),
        basis.identity_digest(),
        construct_allocation_neighborhood_operator_contract(basis, root_record, root_topology),
        basis.dependency_map().clone(),
        neighborhood_class,
        membership_rule,
        members,
    ))
}
