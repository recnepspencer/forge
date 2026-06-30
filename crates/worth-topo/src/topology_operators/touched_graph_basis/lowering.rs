use forge_query::facade::{
    ForgeQueryAspectMutationOperation, ForgeQueryGraphTouchDescriptor,
    ForgeQueryGraphTouchDescriptorDenial, ForgeQueryGraphTouchLifecycleFamily,
    ForgeQueryMutationFamily,
};
use schema::facade::platform::authority::{EntityReference, TopologyMutation};
use schema::facade::platform::relations::TopologyRelationKind;

use crate::topology_operators::{
    TopologyDeclaredMutationActionRef, TopologyDeclaredMutationSequence, TopologyMutationFamily,
    TOPOLOGY_OPERATOR_RELATION_COLLECTION,
};
#[cfg(any(test, feature = "test-support-lowering"))]
use crate::topology_operators::{
    TopologyMutationChangedScope, TopologyRewireLoopEndpointDeclaration,
    TopologySpliceRadialAdjacencyDeclaration,
};

use super::aspect_touch_lowering::query_aspect_touch;
use super::basis::TopologyTouchedGraphBasisInput;
use super::{
    topology_lifecycle_posture_from_mutation_family, topology_touched_aspect_from_schema_aspect,
    topology_touched_scope_from_changed_scope, TopologyGraphLifecyclePosture,
    TopologyTouchedEntity, TopologyTouchedGraphBasis, TopologyTouchedOperatingWorld,
    TopologyTouchedRelation,
};
#[cfg(any(test, feature = "test-support-lowering"))]
use super::{TopologyTouchedAspect, TopologyTouchedScope};

#[cfg(any(test, feature = "test-support-lowering"))]
pub(crate) fn topology_rewire_loop_endpoint_touched_graph_basis(
    declaration: &TopologyRewireLoopEndpointDeclaration,
    operating_world: TopologyTouchedOperatingWorld,
) -> TopologyTouchedGraphBasis {
    let relation_kind = declaration.endpoint().relation_kind();
    retarget_relation_basis(
        declaration.relation_id(),
        relation_kind,
        TopologyMutationFamily::RewireLoopEndpoint,
        [declaration.half_edge_id(), declaration.vertex_id()]
            .into_iter()
            .map(TopologyTouchedEntity::new)
            .collect(),
        [topology_touched_scope_from_changed_scope(
            TopologyMutationChangedScope::Loop,
        )],
        operating_world,
    )
}

#[cfg(any(test, feature = "test-support-lowering"))]
pub(crate) fn topology_splice_radial_adjacency_touched_graph_basis(
    declaration: &TopologySpliceRadialAdjacencyDeclaration,
    operating_world: TopologyTouchedOperatingWorld,
) -> TopologyTouchedGraphBasis {
    retarget_relation_basis(
        declaration.relation_id(),
        TopologyRelationKind::HalfEdgeRadialNext,
        TopologyMutationFamily::SpliceRadialAdjacency,
        [
            declaration.half_edge_id(),
            declaration.radial_next_half_edge_id(),
        ]
        .into_iter()
        .map(TopologyTouchedEntity::new)
        .collect(),
        [topology_touched_scope_from_changed_scope(
            TopologyMutationChangedScope::RadialNeighborhood,
        )],
        operating_world,
    )
}

pub(crate) fn topology_touched_graph_basis_from_mutation_sequence(
    sequence: &TopologyDeclaredMutationSequence,
    operating_world: TopologyTouchedOperatingWorld,
) -> TopologyTouchedGraphBasis {
    let mut entities = Vec::new();
    let mut relations = Vec::new();
    let mut relation_kinds = Vec::new();
    let mut aspects = Vec::new();
    let mut scopes = Vec::new();
    let families = sequence.families();

    for member in sequence.members() {
        let record = member.record();
        relation_kinds.extend(relation_kinds_for_action(member.action_ref()));
        scopes.extend(
            record
                .changed_scopes()
                .iter()
                .copied()
                .map(topology_touched_scope_from_changed_scope),
        );
        aspects.extend(
            record
                .touched_aspects()
                .iter()
                .copied()
                .map(topology_touched_aspect_from_schema_aspect),
        );
        collect_action_entities_and_relations(member.action_ref(), &mut entities, &mut relations);
        for mutation in member.lowered_mutations() {
            collect_lowered_mutation_entities_and_relations(
                mutation,
                &mut entities,
                &mut relations,
                &mut relation_kinds,
            );
        }
    }

    TopologyTouchedGraphBasis::from_input(TopologyTouchedGraphBasisInput {
        entities,
        relations,
        relation_kinds,
        aspects,
        topology_scopes: scopes,
        lifecycle_posture: lifecycle_posture_for_families(families),
        operating_world,
    })
}

pub fn topology_operator_touch_descriptor_from_touched_graph_basis(
    basis: &TopologyTouchedGraphBasis,
) -> Result<ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchDescriptorDenial> {
    ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
        TOPOLOGY_OPERATOR_RELATION_COLLECTION,
        mutation_family_for_lifecycle(basis.lifecycle_posture()),
        Some(lifecycle_family_for_lifecycle(basis.lifecycle_posture())),
        basis
            .aspects()
            .iter()
            .copied()
            .map(|aspect| ForgeQueryAspectMutationOperation::set(query_aspect_touch(aspect))),
        basis.aspects().iter().copied().map(query_aspect_touch),
    )
}

fn collect_action_entities_and_relations(
    action: TopologyDeclaredMutationActionRef<'_>,
    entities: &mut Vec<TopologyTouchedEntity>,
    relations: &mut Vec<TopologyTouchedRelation>,
) {
    match action {
        TopologyDeclaredMutationActionRef::AttachBoundaryMembership { owner, member, .. }
        | TopologyDeclaredMutationActionRef::AttachShellOrWireMembership {
            owner, member, ..
        } => {
            collect_entity_reference(owner, entities);
            collect_entity_reference(member, entities);
        }
        TopologyDeclaredMutationActionRef::CreateTopologyEntity { .. } => {}
        TopologyDeclaredMutationActionRef::DetachBoundaryMembership { relation_id, .. }
        | TopologyDeclaredMutationActionRef::DetachRadialAdjacency { relation_id }
        | TopologyDeclaredMutationActionRef::DetachShellOrWireMembership { relation_id, .. } => {
            relations.push(TopologyTouchedRelation::new(relation_id));
        }
        TopologyDeclaredMutationActionRef::RewireLoopEndpoint {
            relation_id,
            half_edge_id,
            vertex_id,
            ..
        } => {
            relations.push(TopologyTouchedRelation::new(relation_id));
            entities.push(TopologyTouchedEntity::new(half_edge_id));
            entities.push(TopologyTouchedEntity::new(vertex_id));
        }
        TopologyDeclaredMutationActionRef::RewireLoopSuccessor {
            relation_id,
            half_edge_id,
            successor_half_edge_id,
            ..
        }
        | TopologyDeclaredMutationActionRef::SpliceRadialAdjacency {
            relation_id,
            half_edge_id,
            radial_next_half_edge_id: successor_half_edge_id,
        } => {
            relations.push(TopologyTouchedRelation::new(relation_id));
            entities.push(TopologyTouchedEntity::new(half_edge_id));
            entities.push(TopologyTouchedEntity::new(successor_half_edge_id));
        }
        TopologyDeclaredMutationActionRef::RetireTopologyEntity { entity_id, .. } => {
            entities.push(TopologyTouchedEntity::new(entity_id));
        }
    }
}

fn collect_lowered_mutation_entities_and_relations(
    mutation: &TopologyMutation,
    entities: &mut Vec<TopologyTouchedEntity>,
    relations: &mut Vec<TopologyTouchedRelation>,
    relation_kinds: &mut Vec<TopologyRelationKind>,
) {
    match mutation {
        TopologyMutation::CreateEntity { .. } => {}
        TopologyMutation::CreateRelation {
            kind,
            source,
            target,
            ..
        } => {
            if let schema::facade::platform::relations::RelationKind::Topology(kind) = kind {
                relation_kinds.push(*kind);
            }
            collect_entity_reference(source, entities);
            collect_entity_reference(target, entities);
        }
        TopologyMutation::UpsertEntity { entity_id, .. }
        | TopologyMutation::RemoveEntity { entity_id } => {
            entities.push(TopologyTouchedEntity::new(*entity_id));
        }
        TopologyMutation::UpsertRelation {
            relation_id,
            kind,
            source,
            target,
        } => {
            if let schema::facade::platform::relations::RelationKind::Topology(kind) = kind {
                relation_kinds.push(*kind);
            }
            relations.push(TopologyTouchedRelation::new(*relation_id));
            entities.push(TopologyTouchedEntity::new(*source));
            entities.push(TopologyTouchedEntity::new(*target));
        }
        TopologyMutation::RemoveRelation { relation_id } => {
            relations.push(TopologyTouchedRelation::new(*relation_id));
        }
    }
}

fn collect_entity_reference(
    reference: &EntityReference,
    entities: &mut Vec<TopologyTouchedEntity>,
) {
    if let EntityReference::Existing(entity_id) = reference {
        entities.push(TopologyTouchedEntity::new(*entity_id));
    }
}

fn relation_kinds_for_action(
    action: TopologyDeclaredMutationActionRef<'_>,
) -> Vec<TopologyRelationKind> {
    match action {
        TopologyDeclaredMutationActionRef::AttachBoundaryMembership { kind, .. }
        | TopologyDeclaredMutationActionRef::DetachBoundaryMembership { kind, .. } => {
            vec![kind.relation_kind()]
        }
        TopologyDeclaredMutationActionRef::AttachShellOrWireMembership { kind, .. }
        | TopologyDeclaredMutationActionRef::DetachShellOrWireMembership { kind, .. } => {
            vec![kind.relation_kind()]
        }
        TopologyDeclaredMutationActionRef::RewireLoopEndpoint { endpoint, .. } => {
            vec![endpoint.relation_kind()]
        }
        TopologyDeclaredMutationActionRef::RewireLoopSuccessor { kind, .. } => {
            vec![kind.relation_kind()]
        }
        TopologyDeclaredMutationActionRef::SpliceRadialAdjacency { .. }
        | TopologyDeclaredMutationActionRef::DetachRadialAdjacency { .. } => {
            vec![TopologyRelationKind::HalfEdgeRadialNext]
        }
        TopologyDeclaredMutationActionRef::CreateTopologyEntity { .. }
        | TopologyDeclaredMutationActionRef::RetireTopologyEntity { .. } => Vec::new(),
    }
}

fn lifecycle_posture_for_families(
    families: &[TopologyMutationFamily],
) -> TopologyGraphLifecyclePosture {
    if families
        .iter()
        .copied()
        .any(|family| family == TopologyMutationFamily::CreateTopologyEntity)
    {
        return TopologyGraphLifecyclePosture::EntityCreation;
    }
    if families
        .iter()
        .copied()
        .any(|family| family == TopologyMutationFamily::RetireTopologyEntity)
    {
        return TopologyGraphLifecyclePosture::EntityRetirement;
    }
    if families.iter().copied().any(|family| {
        topology_lifecycle_posture_from_mutation_family(family)
            == TopologyGraphLifecyclePosture::ExistingRelationCreate
    }) {
        return TopologyGraphLifecyclePosture::ExistingRelationCreate;
    }
    if families.iter().copied().any(|family| {
        topology_lifecycle_posture_from_mutation_family(family)
            == TopologyGraphLifecyclePosture::ExistingRelationRemoval
    }) {
        return TopologyGraphLifecyclePosture::ExistingRelationRemoval;
    }
    TopologyGraphLifecyclePosture::ExistingRelationRetarget
}

#[cfg(any(test, feature = "test-support-lowering"))]
fn retarget_relation_basis(
    relation_id: forge_relational::facade::identity::RelationId,
    relation_kind: TopologyRelationKind,
    mutation_family: TopologyMutationFamily,
    entities: Vec<TopologyTouchedEntity>,
    scopes: impl IntoIterator<Item = TopologyTouchedScope>,
    operating_world: TopologyTouchedOperatingWorld,
) -> TopologyTouchedGraphBasis {
    TopologyTouchedGraphBasis::from_input(TopologyTouchedGraphBasisInput {
        entities,
        relations: vec![TopologyTouchedRelation::new(relation_id)],
        relation_kinds: vec![relation_kind],
        aspects: touched_aspects_for_family(mutation_family),
        topology_scopes: scopes.into_iter().collect(),
        lifecycle_posture: topology_lifecycle_posture_from_mutation_family(mutation_family),
        operating_world,
    })
}

#[cfg(any(test, feature = "test-support-lowering"))]
fn touched_aspects_for_family(family: TopologyMutationFamily) -> Vec<TopologyTouchedAspect> {
    match family {
        TopologyMutationFamily::RewireLoopEndpoint => vec![
            TopologyTouchedAspect::TopologyStructure,
            TopologyTouchedAspect::TopologyBoundary,
        ],
        TopologyMutationFamily::SpliceRadialAdjacency => vec![
            TopologyTouchedAspect::TopologyStructure,
            TopologyTouchedAspect::TopologyRadial,
        ],
        TopologyMutationFamily::CreateTopologyEntity
        | TopologyMutationFamily::RetireTopologyEntity
        | TopologyMutationFamily::AttachBoundaryMembership
        | TopologyMutationFamily::DetachBoundaryMembership
        | TopologyMutationFamily::RewireLoopSuccessor
        | TopologyMutationFamily::AttachShellOrWireMembership
        | TopologyMutationFamily::DetachShellOrWireMembership
        | TopologyMutationFamily::DetachRadialAdjacency => {
            vec![TopologyTouchedAspect::TopologyStructure]
        }
    }
}

fn mutation_family_for_lifecycle(
    lifecycle: TopologyGraphLifecyclePosture,
) -> ForgeQueryMutationFamily {
    match lifecycle {
        TopologyGraphLifecyclePosture::EntityCreation
        | TopologyGraphLifecyclePosture::ExistingRelationCreate => ForgeQueryMutationFamily::Insert,
        TopologyGraphLifecyclePosture::EntityRetirement
        | TopologyGraphLifecyclePosture::ExistingRelationRemoval => {
            ForgeQueryMutationFamily::Delete
        }
        TopologyGraphLifecyclePosture::ExistingRelationRetarget => ForgeQueryMutationFamily::Update,
    }
}

fn lifecycle_family_for_lifecycle(
    lifecycle: TopologyGraphLifecyclePosture,
) -> ForgeQueryGraphTouchLifecycleFamily {
    match lifecycle {
        TopologyGraphLifecyclePosture::EntityCreation
        | TopologyGraphLifecyclePosture::ExistingRelationCreate => {
            ForgeQueryGraphTouchLifecycleFamily::VerifiedExistingTargetFollowup
        }
        TopologyGraphLifecyclePosture::EntityRetirement
        | TopologyGraphLifecyclePosture::ExistingRelationRemoval => {
            ForgeQueryGraphTouchLifecycleFamily::VerifiedExistingTargetRetirement
        }
        TopologyGraphLifecyclePosture::ExistingRelationRetarget => {
            ForgeQueryGraphTouchLifecycleFamily::VerifiedExistingTargetRetarget
        }
    }
}

#[cfg(any(test, feature = "test-support-lowering"))]
pub(crate) fn test_basis_from_parts(
    entities: Vec<TopologyTouchedEntity>,
    relations: Vec<TopologyTouchedRelation>,
    relation_kinds: Vec<TopologyRelationKind>,
    aspects: Vec<TopologyTouchedAspect>,
    scopes: Vec<TopologyTouchedScope>,
) -> TopologyTouchedGraphBasis {
    TopologyTouchedGraphBasis::from_input(TopologyTouchedGraphBasisInput {
        entities,
        relations,
        relation_kinds,
        aspects,
        topology_scopes: scopes,
        lifecycle_posture: TopologyGraphLifecyclePosture::ExistingRelationRetarget,
        operating_world: TopologyTouchedOperatingWorld::mainline(),
    })
}
