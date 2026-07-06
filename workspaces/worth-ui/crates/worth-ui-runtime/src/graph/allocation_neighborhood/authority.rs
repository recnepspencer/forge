use crate::evidence::{
    UiAllocationNeighborhood, UiAllocationNeighborhoodClass, UiMeasurementBasis,
};
use crate::graph::allocation_neighborhood::membership::{
    derive_members, layout_participates_in_planning,
};
use crate::graph::{UiAllocationNeighborhoodDenial, UiGraphParticipationAxis, UiGraphSnapshot};
use crate::obligations::selection::UiSelectedObligationSet;

pub(crate) fn admit_allocation_neighborhood_for_basis(
    snapshot: &UiGraphSnapshot,
    basis: &UiMeasurementBasis,
) -> Result<UiAllocationNeighborhood, UiAllocationNeighborhoodDenial> {
    let expected = basis.graph_node_identity();
    let Some(root_record) = snapshot.lookup().graph_node(expected) else {
        return Err(UiAllocationNeighborhoodDenial::UnknownRootGraphNode {
            graph_node_identity: expected,
        });
    };
    let root_layout_participation = root_record
        .value()
        .participation_posture()
        .axis(UiGraphParticipationAxis::Layout);
    if !layout_participates_in_planning(snapshot, expected, root_layout_participation) {
        return Err(UiAllocationNeighborhoodDenial::RootNotLayoutParticipant {
            graph_node_identity: expected,
        });
    }

    let neighborhood_class =
        UiAllocationNeighborhoodClass::from_measurement_hint(basis.neighborhood_class_hint());
    let membership_rule =
        basis.allocation_neighborhood_membership_rule(root_record.value().operator_kind());
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
        basis.allocation_neighborhood_operator_contract(&root_record.value(), root_topology),
        basis.dependency_map().clone(),
        neighborhood_class,
        membership_rule,
        members,
    ))
}

pub(crate) fn admit_allocation_neighborhood_for_selected(
    snapshot: &UiGraphSnapshot,
    selected: &UiSelectedObligationSet,
    basis: &UiMeasurementBasis,
) -> Result<UiAllocationNeighborhood, UiAllocationNeighborhoodDenial> {
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
    admit_allocation_neighborhood_for_basis(snapshot, basis)
}
