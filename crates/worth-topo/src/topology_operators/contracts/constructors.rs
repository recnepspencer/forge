use std::collections::BTreeSet;

use forge_relational::facade::identity::{EntityId, RelationId};
use schema::facade::platform::aspects::{Aspect, DiagnosticsAspect, NamingAspect, TopologyAspect};
use schema::facade::platform::authority::{CreateKey, EntityReference, TopologyMutation};
use schema::facade::platform::entities::TopologyEntityKind;
use schema::facade::platform::relations::{RelationKind, TopologyRelationKind};

use super::{
    BoundaryMembershipKind, LoopEndpointKind, LoopSuccessorKind, ShellOrWireMembershipKind,
    TopologyDerivedRegion, TopologyEditAction, TopologyEditChangedScope, TopologyEditContract,
    TopologyEditDerivedFallbackPolicy, TopologyEditFamily, TopologyEditNamingScope,
};

impl TopologyEditContract {
    pub fn create_topology_entity(create_key: impl Into<String>, kind: TopologyEntityKind) -> Self {
        let create_key = CreateKey::new(create_key.into());
        let persistent_name_key =
            CreateKey::new(format!("{}.persistent_name", create_key.as_str()));
        let persistent_name_relation_key =
            CreateKey::new(format!("{}.targets", persistent_name_key.as_str()));
        let touched_aspects = BTreeSet::from([
            Aspect::Topology(TopologyAspect::Structure),
            Aspect::Naming(NamingAspect::PersistentName),
            Aspect::Diagnostics(DiagnosticsAspect::Decisions),
        ]);
        let lowered_mutations = vec![
            TopologyMutation::CreateEntity {
                create_key: create_key.clone(),
                kind: schema::facade::platform::entities::EntityKind::Topology(kind),
            },
            TopologyMutation::CreateEntity {
                create_key: persistent_name_key.clone(),
                kind: schema::facade::platform::entities::EntityKind::Naming(
                    schema::facade::platform::entities::NamingEntityKind::PersistentName,
                ),
            },
            TopologyMutation::CreateRelation {
                create_key: persistent_name_relation_key.clone(),
                kind: RelationKind::Naming(
                    schema::facade::platform::relations::NamingRelationKind::PersistentNameTargetsEntity,
                ),
                source: EntityReference::Created(persistent_name_key.clone()),
                target: EntityReference::Created(create_key.clone()),
            },
        ];
        Self {
            family: TopologyEditFamily::CreateTopologyEntity,
            action: TopologyEditAction::CreateTopologyEntity {
                create_key,
                kind,
                persistent_name_key,
                persistent_name_relation_key,
            },
            touched_aspects,
            changed_scopes: vec![
                TopologyEditChangedScope::Entity,
                TopologyEditChangedScope::Naming,
            ],
            naming_scopes: vec![TopologyEditNamingScope::EditedEntityNames],
            derived_regions: vec![
                TopologyDerivedRegion::EditLocalNeighborhoodRegion,
                TopologyDerivedRegion::NamingContinuityRegion,
            ],
            derived_fallback_policy: TopologyEditDerivedFallbackPolicy::AllowExplicitFallback,
            lowered_mutations,
        }
    }

    pub fn retire_topology_entity(entity_id: EntityId, kind: TopologyEntityKind) -> Self {
        let touched_aspects = BTreeSet::from([
            Aspect::Topology(TopologyAspect::Structure),
            Aspect::Diagnostics(DiagnosticsAspect::Decisions),
        ]);
        Self {
            family: TopologyEditFamily::RetireTopologyEntity,
            action: TopologyEditAction::RetireTopologyEntity { entity_id, kind },
            touched_aspects,
            changed_scopes: vec![
                TopologyEditChangedScope::Entity,
                TopologyEditChangedScope::LocalNeighborhood,
            ],
            naming_scopes: vec![TopologyEditNamingScope::EditedEntityNames],
            derived_regions: vec![
                TopologyDerivedRegion::EditLocalNeighborhoodRegion,
                TopologyDerivedRegion::NamingContinuityRegion,
            ],
            derived_fallback_policy: TopologyEditDerivedFallbackPolicy::AllowExplicitFallback,
            lowered_mutations: vec![TopologyMutation::RemoveEntity { entity_id }],
        }
    }

    pub fn attach_boundary_membership(
        create_key: impl Into<String>,
        kind: BoundaryMembershipKind,
        owner: impl Into<EntityReference>,
        member: impl Into<EntityReference>,
    ) -> Self {
        let owner = owner.into();
        let member = member.into();
        let touched_aspects = BTreeSet::from([
            Aspect::Topology(TopologyAspect::Boundary),
            Aspect::Diagnostics(DiagnosticsAspect::Decisions),
        ]);
        let lowered_mutations = vec![TopologyMutation::CreateRelation {
            create_key: CreateKey::new(create_key.into()),
            kind: RelationKind::Topology(kind.relation_kind()),
            source: owner.clone(),
            target: member.clone(),
        }];
        Self {
            family: TopologyEditFamily::AttachBoundaryMembership,
            action: TopologyEditAction::AttachBoundaryMembership {
                create_key: match &lowered_mutations[0] {
                    TopologyMutation::CreateRelation { create_key, .. } => create_key.clone(),
                    _ => unreachable!(),
                },
                kind,
                owner,
                member,
            },
            touched_aspects,
            changed_scopes: vec![
                TopologyEditChangedScope::Relation,
                TopologyEditChangedScope::Loop,
                TopologyEditChangedScope::LocalNeighborhood,
            ],
            naming_scopes: vec![TopologyEditNamingScope::AdjacentEntityNames],
            derived_regions: vec![
                TopologyDerivedRegion::LoopRegion,
                TopologyDerivedRegion::EditLocalNeighborhoodRegion,
                TopologyDerivedRegion::NamingContinuityRegion,
            ],
            derived_fallback_policy: TopologyEditDerivedFallbackPolicy::AllowExplicitFallback,
            lowered_mutations,
        }
    }

    pub fn detach_boundary_membership(
        relation_id: RelationId,
        kind: BoundaryMembershipKind,
    ) -> Self {
        let touched_aspects = BTreeSet::from([
            Aspect::Topology(TopologyAspect::Boundary),
            Aspect::Diagnostics(DiagnosticsAspect::Decisions),
        ]);
        Self {
            family: TopologyEditFamily::DetachBoundaryMembership,
            action: TopologyEditAction::DetachBoundaryMembership { relation_id, kind },
            touched_aspects,
            changed_scopes: vec![
                TopologyEditChangedScope::Relation,
                TopologyEditChangedScope::Loop,
                TopologyEditChangedScope::LocalNeighborhood,
            ],
            naming_scopes: vec![TopologyEditNamingScope::AdjacentEntityNames],
            derived_regions: vec![
                TopologyDerivedRegion::LoopRegion,
                TopologyDerivedRegion::EditLocalNeighborhoodRegion,
                TopologyDerivedRegion::NamingContinuityRegion,
            ],
            derived_fallback_policy: TopologyEditDerivedFallbackPolicy::AllowExplicitFallback,
            lowered_mutations: vec![TopologyMutation::RemoveRelation { relation_id }],
        }
    }

    pub fn rewire_loop_successor(
        relation_id: RelationId,
        kind: LoopSuccessorKind,
        half_edge_id: EntityId,
        successor_half_edge_id: EntityId,
    ) -> Self {
        let touched_aspects = BTreeSet::from([
            Aspect::Topology(TopologyAspect::Boundary),
            Aspect::Diagnostics(DiagnosticsAspect::Decisions),
        ]);
        Self {
            family: TopologyEditFamily::RewireLoopSuccessor,
            action: TopologyEditAction::RewireLoopSuccessor {
                relation_id,
                kind,
                half_edge_id,
                successor_half_edge_id,
            },
            touched_aspects,
            changed_scopes: vec![
                TopologyEditChangedScope::Relation,
                TopologyEditChangedScope::Loop,
                TopologyEditChangedScope::LocalNeighborhood,
            ],
            naming_scopes: vec![TopologyEditNamingScope::AdjacentEntityNames],
            derived_regions: vec![
                TopologyDerivedRegion::LoopRegion,
                TopologyDerivedRegion::EditLocalNeighborhoodRegion,
                TopologyDerivedRegion::NamingContinuityRegion,
            ],
            derived_fallback_policy: TopologyEditDerivedFallbackPolicy::AllowExplicitFallback,
            lowered_mutations: vec![TopologyMutation::UpsertRelation {
                relation_id,
                kind: RelationKind::Topology(kind.relation_kind()),
                source: half_edge_id,
                target: successor_half_edge_id,
            }],
        }
    }

    pub fn rewire_loop_endpoint(
        relation_id: RelationId,
        endpoint: LoopEndpointKind,
        half_edge_id: EntityId,
        vertex_id: EntityId,
    ) -> Self {
        let touched_aspects = BTreeSet::from([
            Aspect::Topology(TopologyAspect::Boundary),
            Aspect::Diagnostics(DiagnosticsAspect::Decisions),
        ]);
        Self {
            family: TopologyEditFamily::RewireLoopEndpoint,
            action: TopologyEditAction::RewireLoopEndpoint {
                relation_id,
                endpoint,
                half_edge_id,
                vertex_id,
            },
            touched_aspects,
            changed_scopes: vec![
                TopologyEditChangedScope::Relation,
                TopologyEditChangedScope::Loop,
                TopologyEditChangedScope::LocalNeighborhood,
            ],
            naming_scopes: vec![TopologyEditNamingScope::AdjacentEntityNames],
            derived_regions: vec![
                TopologyDerivedRegion::LoopRegion,
                TopologyDerivedRegion::EditLocalNeighborhoodRegion,
                TopologyDerivedRegion::NamingContinuityRegion,
            ],
            derived_fallback_policy: TopologyEditDerivedFallbackPolicy::AllowExplicitFallback,
            lowered_mutations: vec![TopologyMutation::UpsertRelation {
                relation_id,
                kind: RelationKind::Topology(endpoint.relation_kind()),
                source: half_edge_id,
                target: vertex_id,
            }],
        }
    }

    pub fn attach_shell_or_wire_membership(
        create_key: impl Into<String>,
        kind: ShellOrWireMembershipKind,
        owner: impl Into<EntityReference>,
        member: impl Into<EntityReference>,
    ) -> Self {
        let create_key = CreateKey::new(create_key.into());
        let owner = owner.into();
        let member = member.into();
        let changed_scope = match kind {
            ShellOrWireMembershipKind::RegionOwnsShell
            | ShellOrWireMembershipKind::ShellOwnsFace => TopologyEditChangedScope::Shell,
            ShellOrWireMembershipKind::WireOwnsHalfEdge => TopologyEditChangedScope::Wire,
        };
        let derived_region = match kind {
            ShellOrWireMembershipKind::RegionOwnsShell
            | ShellOrWireMembershipKind::ShellOwnsFace => TopologyDerivedRegion::ShellRegion,
            ShellOrWireMembershipKind::WireOwnsHalfEdge => TopologyDerivedRegion::WireRegion,
        };
        Self {
            family: TopologyEditFamily::AttachShellOrWireMembership,
            action: TopologyEditAction::AttachShellOrWireMembership {
                create_key: create_key.clone(),
                kind,
                owner: owner.clone(),
                member: member.clone(),
            },
            touched_aspects: BTreeSet::from([
                Aspect::Topology(TopologyAspect::Ownership),
                Aspect::Diagnostics(DiagnosticsAspect::Decisions),
            ]),
            changed_scopes: vec![
                TopologyEditChangedScope::Relation,
                changed_scope,
                TopologyEditChangedScope::LocalNeighborhood,
            ],
            naming_scopes: vec![TopologyEditNamingScope::AdjacentEntityNames],
            derived_regions: vec![
                derived_region,
                TopologyDerivedRegion::EditLocalNeighborhoodRegion,
                TopologyDerivedRegion::NamingContinuityRegion,
            ],
            derived_fallback_policy: TopologyEditDerivedFallbackPolicy::AllowExplicitFallback,
            lowered_mutations: vec![TopologyMutation::CreateRelation {
                create_key,
                kind: RelationKind::Topology(kind.relation_kind()),
                source: owner,
                target: member,
            }],
        }
    }

    pub fn detach_shell_or_wire_membership(
        relation_id: RelationId,
        kind: ShellOrWireMembershipKind,
    ) -> Self {
        let changed_scope = match kind {
            ShellOrWireMembershipKind::RegionOwnsShell
            | ShellOrWireMembershipKind::ShellOwnsFace => TopologyEditChangedScope::Shell,
            ShellOrWireMembershipKind::WireOwnsHalfEdge => TopologyEditChangedScope::Wire,
        };
        let derived_region = match kind {
            ShellOrWireMembershipKind::RegionOwnsShell
            | ShellOrWireMembershipKind::ShellOwnsFace => TopologyDerivedRegion::ShellRegion,
            ShellOrWireMembershipKind::WireOwnsHalfEdge => TopologyDerivedRegion::WireRegion,
        };
        Self {
            family: TopologyEditFamily::DetachShellOrWireMembership,
            action: TopologyEditAction::DetachShellOrWireMembership { relation_id, kind },
            touched_aspects: BTreeSet::from([
                Aspect::Topology(TopologyAspect::Ownership),
                Aspect::Diagnostics(DiagnosticsAspect::Decisions),
            ]),
            changed_scopes: vec![
                TopologyEditChangedScope::Relation,
                changed_scope,
                TopologyEditChangedScope::LocalNeighborhood,
            ],
            naming_scopes: vec![TopologyEditNamingScope::AdjacentEntityNames],
            derived_regions: vec![
                derived_region,
                TopologyDerivedRegion::EditLocalNeighborhoodRegion,
                TopologyDerivedRegion::NamingContinuityRegion,
            ],
            derived_fallback_policy: TopologyEditDerivedFallbackPolicy::AllowExplicitFallback,
            lowered_mutations: vec![TopologyMutation::RemoveRelation { relation_id }],
        }
    }

    pub fn splice_radial_adjacency(
        relation_id: RelationId,
        half_edge_id: EntityId,
        radial_next_half_edge_id: EntityId,
    ) -> Self {
        Self {
            family: TopologyEditFamily::SpliceRadialAdjacency,
            action: TopologyEditAction::SpliceRadialAdjacency {
                relation_id,
                half_edge_id,
                radial_next_half_edge_id,
            },
            touched_aspects: BTreeSet::from([
                Aspect::Topology(TopologyAspect::Radial),
                Aspect::Diagnostics(DiagnosticsAspect::Decisions),
            ]),
            changed_scopes: vec![
                TopologyEditChangedScope::Relation,
                TopologyEditChangedScope::RadialNeighborhood,
                TopologyEditChangedScope::LocalNeighborhood,
            ],
            naming_scopes: vec![TopologyEditNamingScope::AdjacentEntityNames],
            derived_regions: vec![
                TopologyDerivedRegion::RadialNeighborhoodRegion,
                TopologyDerivedRegion::EditLocalNeighborhoodRegion,
                TopologyDerivedRegion::NamingContinuityRegion,
            ],
            derived_fallback_policy: TopologyEditDerivedFallbackPolicy::AllowExplicitFallback,
            lowered_mutations: vec![TopologyMutation::UpsertRelation {
                relation_id,
                kind: RelationKind::Topology(TopologyRelationKind::HalfEdgeRadialNext),
                source: half_edge_id,
                target: radial_next_half_edge_id,
            }],
        }
    }

    pub fn detach_radial_adjacency(relation_id: RelationId) -> Self {
        Self {
            family: TopologyEditFamily::DetachRadialAdjacency,
            action: TopologyEditAction::DetachRadialAdjacency { relation_id },
            touched_aspects: BTreeSet::from([
                Aspect::Topology(TopologyAspect::Radial),
                Aspect::Diagnostics(DiagnosticsAspect::Decisions),
            ]),
            changed_scopes: vec![
                TopologyEditChangedScope::Relation,
                TopologyEditChangedScope::RadialNeighborhood,
                TopologyEditChangedScope::LocalNeighborhood,
            ],
            naming_scopes: vec![TopologyEditNamingScope::AdjacentEntityNames],
            derived_regions: vec![
                TopologyDerivedRegion::RadialNeighborhoodRegion,
                TopologyDerivedRegion::EditLocalNeighborhoodRegion,
                TopologyDerivedRegion::NamingContinuityRegion,
            ],
            derived_fallback_policy: TopologyEditDerivedFallbackPolicy::AllowExplicitFallback,
            lowered_mutations: vec![TopologyMutation::RemoveRelation { relation_id }],
        }
    }
}
