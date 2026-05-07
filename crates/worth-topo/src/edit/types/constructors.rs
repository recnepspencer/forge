use std::collections::BTreeSet;

use forge_relational::facade::identity::{EntityId, RelationId};
use worth_schema::facade::{
    WorthAspect, WorthCreateKey, WorthDiagnosticsAspect, WorthEntityReference, WorthNamingAspect,
    WorthRelationKind, WorthTopologyAspect, WorthTopologyEntityKind, WorthTopologyMutation,
    WorthTopologyRelationKind,
};

use super::{
    WorthBoundaryMembershipKind, WorthLoopEndpointKind, WorthLoopSuccessorKind,
    WorthShellOrWireMembershipKind, WorthTopologyDerivedRegion, WorthTopologyEditAction,
    WorthTopologyEditChangedScope, WorthTopologyEditContract, WorthTopologyEditFamily,
    WorthTopologyEditNamingScope,
};

impl WorthTopologyEditContract {
    pub fn create_topology_entity(
        create_key: impl Into<String>,
        kind: WorthTopologyEntityKind,
    ) -> Self {
        let create_key = WorthCreateKey::new(create_key.into());
        let persistent_name_key =
            WorthCreateKey::new(format!("{}.persistent_name", create_key.as_str()));
        let persistent_name_relation_key =
            WorthCreateKey::new(format!("{}.targets", persistent_name_key.as_str()));
        let touched_aspects = BTreeSet::from([
            WorthAspect::Topology(WorthTopologyAspect::Structure),
            WorthAspect::Naming(WorthNamingAspect::PersistentName),
            WorthAspect::Diagnostics(WorthDiagnosticsAspect::Decisions),
        ]);
        let lowered_mutations = vec![
            WorthTopologyMutation::CreateEntity {
                create_key: create_key.clone(),
                kind: worth_schema::facade::WorthEntityKind::Topology(kind),
            },
            WorthTopologyMutation::CreateEntity {
                create_key: persistent_name_key.clone(),
                kind: worth_schema::facade::WorthEntityKind::Naming(
                    worth_schema::facade::WorthNamingEntityKind::PersistentName,
                ),
            },
            WorthTopologyMutation::CreateRelation {
                create_key: persistent_name_relation_key.clone(),
                kind: WorthRelationKind::Naming(
                    worth_schema::facade::WorthNamingRelationKind::PersistentNameTargetsEntity,
                ),
                source: WorthEntityReference::Created(persistent_name_key.clone()),
                target: WorthEntityReference::Created(create_key.clone()),
            },
        ];
        Self {
            family: WorthTopologyEditFamily::CreateTopologyEntity,
            action: WorthTopologyEditAction::CreateTopologyEntity {
                create_key,
                kind,
                persistent_name_key,
                persistent_name_relation_key,
            },
            touched_aspects,
            changed_scopes: vec![
                WorthTopologyEditChangedScope::Entity,
                WorthTopologyEditChangedScope::Naming,
            ],
            naming_scopes: vec![WorthTopologyEditNamingScope::EditedEntityNames],
            derived_regions: vec![
                WorthTopologyDerivedRegion::EditLocalNeighborhoodRegion,
                WorthTopologyDerivedRegion::NamingContinuityRegion,
            ],
            lowered_mutations,
        }
    }

    pub fn retire_topology_entity(entity_id: EntityId, kind: WorthTopologyEntityKind) -> Self {
        let touched_aspects = BTreeSet::from([
            WorthAspect::Topology(WorthTopologyAspect::Structure),
            WorthAspect::Diagnostics(WorthDiagnosticsAspect::Decisions),
        ]);
        Self {
            family: WorthTopologyEditFamily::RetireTopologyEntity,
            action: WorthTopologyEditAction::RetireTopologyEntity { entity_id, kind },
            touched_aspects,
            changed_scopes: vec![
                WorthTopologyEditChangedScope::Entity,
                WorthTopologyEditChangedScope::LocalNeighborhood,
            ],
            naming_scopes: vec![WorthTopologyEditNamingScope::EditedEntityNames],
            derived_regions: vec![
                WorthTopologyDerivedRegion::EditLocalNeighborhoodRegion,
                WorthTopologyDerivedRegion::NamingContinuityRegion,
            ],
            lowered_mutations: vec![WorthTopologyMutation::RemoveEntity { entity_id }],
        }
    }

    pub fn attach_boundary_membership(
        create_key: impl Into<String>,
        kind: WorthBoundaryMembershipKind,
        owner: impl Into<WorthEntityReference>,
        member: impl Into<WorthEntityReference>,
    ) -> Self {
        let owner = owner.into();
        let member = member.into();
        let touched_aspects = BTreeSet::from([
            WorthAspect::Topology(WorthTopologyAspect::Boundary),
            WorthAspect::Diagnostics(WorthDiagnosticsAspect::Decisions),
        ]);
        let lowered_mutations = vec![WorthTopologyMutation::CreateRelation {
            create_key: WorthCreateKey::new(create_key.into()),
            kind: WorthRelationKind::Topology(kind.relation_kind()),
            source: owner.clone(),
            target: member.clone(),
        }];
        Self {
            family: WorthTopologyEditFamily::AttachBoundaryMembership,
            action: WorthTopologyEditAction::AttachBoundaryMembership {
                create_key: match &lowered_mutations[0] {
                    WorthTopologyMutation::CreateRelation { create_key, .. } => create_key.clone(),
                    _ => unreachable!(),
                },
                kind,
                owner,
                member,
            },
            touched_aspects,
            changed_scopes: vec![
                WorthTopologyEditChangedScope::Relation,
                WorthTopologyEditChangedScope::Loop,
                WorthTopologyEditChangedScope::LocalNeighborhood,
            ],
            naming_scopes: vec![WorthTopologyEditNamingScope::AdjacentEntityNames],
            derived_regions: vec![
                WorthTopologyDerivedRegion::LoopRegion,
                WorthTopologyDerivedRegion::EditLocalNeighborhoodRegion,
                WorthTopologyDerivedRegion::NamingContinuityRegion,
            ],
            lowered_mutations,
        }
    }

    pub fn detach_boundary_membership(
        relation_id: RelationId,
        kind: WorthBoundaryMembershipKind,
    ) -> Self {
        let touched_aspects = BTreeSet::from([
            WorthAspect::Topology(WorthTopologyAspect::Boundary),
            WorthAspect::Diagnostics(WorthDiagnosticsAspect::Decisions),
        ]);
        Self {
            family: WorthTopologyEditFamily::DetachBoundaryMembership,
            action: WorthTopologyEditAction::DetachBoundaryMembership { relation_id, kind },
            touched_aspects,
            changed_scopes: vec![
                WorthTopologyEditChangedScope::Relation,
                WorthTopologyEditChangedScope::Loop,
                WorthTopologyEditChangedScope::LocalNeighborhood,
            ],
            naming_scopes: vec![WorthTopologyEditNamingScope::AdjacentEntityNames],
            derived_regions: vec![
                WorthTopologyDerivedRegion::LoopRegion,
                WorthTopologyDerivedRegion::EditLocalNeighborhoodRegion,
                WorthTopologyDerivedRegion::NamingContinuityRegion,
            ],
            lowered_mutations: vec![WorthTopologyMutation::RemoveRelation { relation_id }],
        }
    }

    pub fn rewire_loop_successor(
        relation_id: RelationId,
        kind: WorthLoopSuccessorKind,
        half_edge_id: EntityId,
        successor_half_edge_id: EntityId,
    ) -> Self {
        let touched_aspects = BTreeSet::from([
            WorthAspect::Topology(WorthTopologyAspect::Boundary),
            WorthAspect::Diagnostics(WorthDiagnosticsAspect::Decisions),
        ]);
        Self {
            family: WorthTopologyEditFamily::RewireLoopSuccessor,
            action: WorthTopologyEditAction::RewireLoopSuccessor {
                relation_id,
                kind,
                half_edge_id,
                successor_half_edge_id,
            },
            touched_aspects,
            changed_scopes: vec![
                WorthTopologyEditChangedScope::Relation,
                WorthTopologyEditChangedScope::Loop,
                WorthTopologyEditChangedScope::LocalNeighborhood,
            ],
            naming_scopes: vec![WorthTopologyEditNamingScope::AdjacentEntityNames],
            derived_regions: vec![
                WorthTopologyDerivedRegion::LoopRegion,
                WorthTopologyDerivedRegion::EditLocalNeighborhoodRegion,
                WorthTopologyDerivedRegion::NamingContinuityRegion,
            ],
            lowered_mutations: vec![WorthTopologyMutation::UpsertRelation {
                relation_id,
                kind: WorthRelationKind::Topology(kind.relation_kind()),
                source: half_edge_id,
                target: successor_half_edge_id,
            }],
        }
    }

    pub fn rewire_loop_endpoint(
        relation_id: RelationId,
        endpoint: WorthLoopEndpointKind,
        half_edge_id: EntityId,
        vertex_id: EntityId,
    ) -> Self {
        let touched_aspects = BTreeSet::from([
            WorthAspect::Topology(WorthTopologyAspect::Boundary),
            WorthAspect::Diagnostics(WorthDiagnosticsAspect::Decisions),
        ]);
        Self {
            family: WorthTopologyEditFamily::RewireLoopEndpoint,
            action: WorthTopologyEditAction::RewireLoopEndpoint {
                relation_id,
                endpoint,
                half_edge_id,
                vertex_id,
            },
            touched_aspects,
            changed_scopes: vec![
                WorthTopologyEditChangedScope::Relation,
                WorthTopologyEditChangedScope::Loop,
                WorthTopologyEditChangedScope::LocalNeighborhood,
            ],
            naming_scopes: vec![WorthTopologyEditNamingScope::AdjacentEntityNames],
            derived_regions: vec![
                WorthTopologyDerivedRegion::LoopRegion,
                WorthTopologyDerivedRegion::EditLocalNeighborhoodRegion,
                WorthTopologyDerivedRegion::NamingContinuityRegion,
            ],
            lowered_mutations: vec![WorthTopologyMutation::UpsertRelation {
                relation_id,
                kind: WorthRelationKind::Topology(endpoint.relation_kind()),
                source: half_edge_id,
                target: vertex_id,
            }],
        }
    }

    pub fn attach_shell_or_wire_membership(
        create_key: impl Into<String>,
        kind: WorthShellOrWireMembershipKind,
        owner: impl Into<WorthEntityReference>,
        member: impl Into<WorthEntityReference>,
    ) -> Self {
        let create_key = WorthCreateKey::new(create_key.into());
        let owner = owner.into();
        let member = member.into();
        let changed_scope = match kind {
            WorthShellOrWireMembershipKind::RegionOwnsShell
            | WorthShellOrWireMembershipKind::ShellOwnsFace => WorthTopologyEditChangedScope::Shell,
            WorthShellOrWireMembershipKind::WireOwnsHalfEdge => WorthTopologyEditChangedScope::Wire,
        };
        let derived_region = match kind {
            WorthShellOrWireMembershipKind::RegionOwnsShell
            | WorthShellOrWireMembershipKind::ShellOwnsFace => {
                WorthTopologyDerivedRegion::ShellRegion
            }
            WorthShellOrWireMembershipKind::WireOwnsHalfEdge => {
                WorthTopologyDerivedRegion::WireRegion
            }
        };
        Self {
            family: WorthTopologyEditFamily::AttachShellOrWireMembership,
            action: WorthTopologyEditAction::AttachShellOrWireMembership {
                create_key: create_key.clone(),
                kind,
                owner: owner.clone(),
                member: member.clone(),
            },
            touched_aspects: BTreeSet::from([
                WorthAspect::Topology(WorthTopologyAspect::Ownership),
                WorthAspect::Diagnostics(WorthDiagnosticsAspect::Decisions),
            ]),
            changed_scopes: vec![
                WorthTopologyEditChangedScope::Relation,
                changed_scope,
                WorthTopologyEditChangedScope::LocalNeighborhood,
            ],
            naming_scopes: vec![WorthTopologyEditNamingScope::AdjacentEntityNames],
            derived_regions: vec![
                derived_region,
                WorthTopologyDerivedRegion::EditLocalNeighborhoodRegion,
                WorthTopologyDerivedRegion::NamingContinuityRegion,
            ],
            lowered_mutations: vec![WorthTopologyMutation::CreateRelation {
                create_key,
                kind: WorthRelationKind::Topology(kind.relation_kind()),
                source: owner,
                target: member,
            }],
        }
    }

    pub fn detach_shell_or_wire_membership(
        relation_id: RelationId,
        kind: WorthShellOrWireMembershipKind,
    ) -> Self {
        let changed_scope = match kind {
            WorthShellOrWireMembershipKind::RegionOwnsShell
            | WorthShellOrWireMembershipKind::ShellOwnsFace => WorthTopologyEditChangedScope::Shell,
            WorthShellOrWireMembershipKind::WireOwnsHalfEdge => WorthTopologyEditChangedScope::Wire,
        };
        let derived_region = match kind {
            WorthShellOrWireMembershipKind::RegionOwnsShell
            | WorthShellOrWireMembershipKind::ShellOwnsFace => {
                WorthTopologyDerivedRegion::ShellRegion
            }
            WorthShellOrWireMembershipKind::WireOwnsHalfEdge => {
                WorthTopologyDerivedRegion::WireRegion
            }
        };
        Self {
            family: WorthTopologyEditFamily::DetachShellOrWireMembership,
            action: WorthTopologyEditAction::DetachShellOrWireMembership { relation_id, kind },
            touched_aspects: BTreeSet::from([
                WorthAspect::Topology(WorthTopologyAspect::Ownership),
                WorthAspect::Diagnostics(WorthDiagnosticsAspect::Decisions),
            ]),
            changed_scopes: vec![
                WorthTopologyEditChangedScope::Relation,
                changed_scope,
                WorthTopologyEditChangedScope::LocalNeighborhood,
            ],
            naming_scopes: vec![WorthTopologyEditNamingScope::AdjacentEntityNames],
            derived_regions: vec![
                derived_region,
                WorthTopologyDerivedRegion::EditLocalNeighborhoodRegion,
                WorthTopologyDerivedRegion::NamingContinuityRegion,
            ],
            lowered_mutations: vec![WorthTopologyMutation::RemoveRelation { relation_id }],
        }
    }

    pub fn splice_radial_adjacency(
        relation_id: RelationId,
        half_edge_id: EntityId,
        radial_next_half_edge_id: EntityId,
    ) -> Self {
        Self {
            family: WorthTopologyEditFamily::SpliceRadialAdjacency,
            action: WorthTopologyEditAction::SpliceRadialAdjacency {
                relation_id,
                half_edge_id,
                radial_next_half_edge_id,
            },
            touched_aspects: BTreeSet::from([
                WorthAspect::Topology(WorthTopologyAspect::Radial),
                WorthAspect::Diagnostics(WorthDiagnosticsAspect::Decisions),
            ]),
            changed_scopes: vec![
                WorthTopologyEditChangedScope::Relation,
                WorthTopologyEditChangedScope::RadialNeighborhood,
                WorthTopologyEditChangedScope::LocalNeighborhood,
            ],
            naming_scopes: vec![WorthTopologyEditNamingScope::AdjacentEntityNames],
            derived_regions: vec![
                WorthTopologyDerivedRegion::RadialNeighborhoodRegion,
                WorthTopologyDerivedRegion::EditLocalNeighborhoodRegion,
                WorthTopologyDerivedRegion::NamingContinuityRegion,
            ],
            lowered_mutations: vec![WorthTopologyMutation::UpsertRelation {
                relation_id,
                kind: WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeRadialNext),
                source: half_edge_id,
                target: radial_next_half_edge_id,
            }],
        }
    }

    pub fn detach_radial_adjacency(relation_id: RelationId) -> Self {
        Self {
            family: WorthTopologyEditFamily::DetachRadialAdjacency,
            action: WorthTopologyEditAction::DetachRadialAdjacency { relation_id },
            touched_aspects: BTreeSet::from([
                WorthAspect::Topology(WorthTopologyAspect::Radial),
                WorthAspect::Diagnostics(WorthDiagnosticsAspect::Decisions),
            ]),
            changed_scopes: vec![
                WorthTopologyEditChangedScope::Relation,
                WorthTopologyEditChangedScope::RadialNeighborhood,
                WorthTopologyEditChangedScope::LocalNeighborhood,
            ],
            naming_scopes: vec![WorthTopologyEditNamingScope::AdjacentEntityNames],
            derived_regions: vec![
                WorthTopologyDerivedRegion::RadialNeighborhoodRegion,
                WorthTopologyDerivedRegion::EditLocalNeighborhoodRegion,
                WorthTopologyDerivedRegion::NamingContinuityRegion,
            ],
            lowered_mutations: vec![WorthTopologyMutation::RemoveRelation { relation_id }],
        }
    }
}
