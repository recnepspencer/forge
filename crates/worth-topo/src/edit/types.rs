use std::collections::BTreeSet;

use forge_relational::facade::identity::{EntityId, RelationId};
use serde::{Deserialize, Serialize};
use worth_schema::facade::{
    WorthAspect, WorthCreateKey, WorthDiagnosticsAspect, WorthEntityReference, WorthMutationOrigin,
    WorthNamingAspect, WorthRelationKind, WorthTopologyAspect, WorthTopologyEntityKind,
    WorthTopologyMutation, WorthTopologyRelationKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthTopologyEditFamily {
    CreateTopologyEntity,
    RetireTopologyEntity,
    AttachBoundaryMembership,
    DetachBoundaryMembership,
    RewireLoopSuccessor,
    RewireLoopEndpoint,
    AttachShellOrWireMembership,
    DetachShellOrWireMembership,
    SpliceRadialAdjacency,
    DetachRadialAdjacency,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthTopologyEditChangedScope {
    Entity,
    Relation,
    LocalNeighborhood,
    Loop,
    Wire,
    Shell,
    RadialNeighborhood,
    Naming,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthTopologyEditNamingScope {
    EditedEntityNames,
    AdjacentEntityNames,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthTopologyDerivedRegion {
    LoopRegion,
    WireRegion,
    ShellRegion,
    RadialNeighborhoodRegion,
    EditLocalNeighborhoodRegion,
    NamingContinuityRegion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthTopologyEditNamingOutcome {
    Preserved,
    Ambiguous,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthTopologyEditNamingRow {
    pub family: WorthTopologyEditFamily,
    pub scope: WorthTopologyEditNamingScope,
    pub outcome: WorthTopologyEditNamingOutcome,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthTopologyEditNamingReport {
    pub rows: Vec<WorthTopologyEditNamingRow>,
}

impl WorthTopologyEditNamingReport {
    pub fn rejected(mut self, reason: impl Into<String>) -> Self {
        let reason = reason.into();
        for row in &mut self.rows {
            row.outcome = WorthTopologyEditNamingOutcome::Rejected;
            row.reason = reason.clone();
        }
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthBoundaryMembershipKind {
    FaceOuterLoop,
    FaceInnerLoop,
    LoopOwnsHalfEdge,
}

impl WorthBoundaryMembershipKind {
    pub const fn relation_kind(self) -> WorthTopologyRelationKind {
        match self {
            Self::FaceOuterLoop => WorthTopologyRelationKind::FaceOuterLoop,
            Self::FaceInnerLoop => WorthTopologyRelationKind::FaceInnerLoop,
            Self::LoopOwnsHalfEdge => WorthTopologyRelationKind::LoopOwnsHalfEdge,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthLoopSuccessorKind {
    Next,
    Prev,
}

impl WorthLoopSuccessorKind {
    pub const fn relation_kind(self) -> WorthTopologyRelationKind {
        match self {
            Self::Next => WorthTopologyRelationKind::HalfEdgeNext,
            Self::Prev => WorthTopologyRelationKind::HalfEdgePrev,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthLoopEndpointKind {
    Start,
    End,
}

impl WorthLoopEndpointKind {
    pub const fn relation_kind(self) -> WorthTopologyRelationKind {
        match self {
            Self::Start => WorthTopologyRelationKind::HalfEdgeStartsAtVertex,
            Self::End => WorthTopologyRelationKind::HalfEdgeEndsAtVertex,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthShellOrWireMembershipKind {
    RegionOwnsShell,
    ShellOwnsFace,
    WireOwnsHalfEdge,
}

impl WorthShellOrWireMembershipKind {
    pub const fn relation_kind(self) -> WorthTopologyRelationKind {
        match self {
            Self::RegionOwnsShell => WorthTopologyRelationKind::RegionOwnsShell,
            Self::ShellOwnsFace => WorthTopologyRelationKind::ShellOwnsFace,
            Self::WireOwnsHalfEdge => WorthTopologyRelationKind::WireOwnsHalfEdge,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorthTopologyEditAction {
    CreateTopologyEntity {
        create_key: WorthCreateKey,
        kind: WorthTopologyEntityKind,
        persistent_name_key: WorthCreateKey,
        persistent_name_relation_key: WorthCreateKey,
    },
    RetireTopologyEntity {
        entity_id: EntityId,
        kind: WorthTopologyEntityKind,
    },
    AttachBoundaryMembership {
        create_key: WorthCreateKey,
        kind: WorthBoundaryMembershipKind,
        owner: WorthEntityReference,
        member: WorthEntityReference,
    },
    DetachBoundaryMembership {
        relation_id: RelationId,
        kind: WorthBoundaryMembershipKind,
    },
    RewireLoopSuccessor {
        relation_id: RelationId,
        kind: WorthLoopSuccessorKind,
        half_edge_id: EntityId,
        successor_half_edge_id: EntityId,
    },
    RewireLoopEndpoint {
        relation_id: RelationId,
        endpoint: WorthLoopEndpointKind,
        half_edge_id: EntityId,
        vertex_id: EntityId,
    },
    AttachShellOrWireMembership {
        create_key: WorthCreateKey,
        kind: WorthShellOrWireMembershipKind,
        owner: WorthEntityReference,
        member: WorthEntityReference,
    },
    DetachShellOrWireMembership {
        relation_id: RelationId,
        kind: WorthShellOrWireMembershipKind,
    },
    SpliceRadialAdjacency {
        relation_id: RelationId,
        half_edge_id: EntityId,
        radial_next_half_edge_id: EntityId,
    },
    DetachRadialAdjacency {
        relation_id: RelationId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthTopologyEditContract {
    pub family: WorthTopologyEditFamily,
    pub action: WorthTopologyEditAction,
    pub touched_aspects: BTreeSet<WorthAspect>,
    pub changed_scopes: Vec<WorthTopologyEditChangedScope>,
    pub naming_scopes: Vec<WorthTopologyEditNamingScope>,
    pub derived_regions: Vec<WorthTopologyDerivedRegion>,
    pub lowered_mutations: Vec<WorthTopologyMutation>,
}

impl WorthTopologyEditContract {
    pub fn mutation_origin_for(
        mode: &super::facade::WorthTopologyEditApplicationMode,
    ) -> WorthMutationOrigin {
        match mode {
            super::facade::WorthTopologyEditApplicationMode::Mainline => {
                WorthMutationOrigin::LocalEdit
            }
            super::facade::WorthTopologyEditApplicationMode::BranchLocal(_) => {
                WorthMutationOrigin::BranchLocalApplication
            }
        }
    }

    pub fn touched_aspects(&self) -> &BTreeSet<WorthAspect> {
        &self.touched_aspects
    }

    pub fn changed_scopes(&self) -> &[WorthTopologyEditChangedScope] {
        &self.changed_scopes
    }

    pub fn naming_scopes(&self) -> &[WorthTopologyEditNamingScope] {
        &self.naming_scopes
    }

    pub fn derived_regions(&self) -> &[WorthTopologyDerivedRegion] {
        &self.derived_regions
    }

    pub fn lowered_mutations(&self) -> &[WorthTopologyMutation] {
        &self.lowered_mutations
    }

    pub fn naming_report(&self) -> WorthTopologyEditNamingReport {
        let rows = self
            .naming_scopes
            .iter()
            .copied()
            .map(|scope| match self.family {
                WorthTopologyEditFamily::CreateTopologyEntity => WorthTopologyEditNamingRow {
                    family: self.family,
                    scope,
                    outcome: WorthTopologyEditNamingOutcome::Preserved,
                    reason: "new topology entity publishes with one attached persistent name".into(),
                },
                WorthTopologyEditFamily::RetireTopologyEntity => WorthTopologyEditNamingRow {
                    family: self.family,
                    scope,
                    outcome: WorthTopologyEditNamingOutcome::Rejected,
                    reason:
                        "retired topology entity does not preserve one canonical successor naming target"
                            .into(),
                },
                WorthTopologyEditFamily::AttachBoundaryMembership
                | WorthTopologyEditFamily::DetachBoundaryMembership
                | WorthTopologyEditFamily::RewireLoopSuccessor
                | WorthTopologyEditFamily::RewireLoopEndpoint
                | WorthTopologyEditFamily::AttachShellOrWireMembership
                | WorthTopologyEditFamily::DetachShellOrWireMembership
                | WorthTopologyEditFamily::SpliceRadialAdjacency
                | WorthTopologyEditFamily::DetachRadialAdjacency => WorthTopologyEditNamingRow {
                    family: self.family,
                    scope,
                    outcome: WorthTopologyEditNamingOutcome::Ambiguous,
                    reason:
                        "topology neighborhood changed without a declared canonical continuity mapping"
                            .into(),
                },
            })
            .collect();
        WorthTopologyEditNamingReport { rows }
    }

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
        let lowered_mutations = vec![WorthTopologyMutation::RemoveEntity { entity_id }];
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
            lowered_mutations,
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
        let lowered_mutations = vec![WorthTopologyMutation::RemoveRelation { relation_id }];
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
            lowered_mutations,
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
        let lowered_mutations = vec![WorthTopologyMutation::UpsertRelation {
            relation_id,
            kind: WorthRelationKind::Topology(kind.relation_kind()),
            source: half_edge_id,
            target: successor_half_edge_id,
        }];
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
            lowered_mutations,
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
        let lowered_mutations = vec![WorthTopologyMutation::UpsertRelation {
            relation_id,
            kind: WorthRelationKind::Topology(endpoint.relation_kind()),
            source: half_edge_id,
            target: vertex_id,
        }];
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
            lowered_mutations,
        }
    }

    pub fn attach_shell_or_wire_membership(
        create_key: impl Into<String>,
        kind: WorthShellOrWireMembershipKind,
        owner: impl Into<WorthEntityReference>,
        member: impl Into<WorthEntityReference>,
    ) -> Self {
        let owner = owner.into();
        let member = member.into();
        let touched_aspects = BTreeSet::from([
            WorthAspect::Topology(match kind {
                WorthShellOrWireMembershipKind::WireOwnsHalfEdge => WorthTopologyAspect::Ownership,
                _ => WorthTopologyAspect::Ownership,
            }),
            WorthAspect::Diagnostics(WorthDiagnosticsAspect::Decisions),
        ]);
        let lowered_mutations = vec![WorthTopologyMutation::CreateRelation {
            create_key: WorthCreateKey::new(create_key.into()),
            kind: WorthRelationKind::Topology(kind.relation_kind()),
            source: owner.clone(),
            target: member.clone(),
        }];
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
                changed_scope,
                WorthTopologyEditChangedScope::LocalNeighborhood,
            ],
            naming_scopes: vec![WorthTopologyEditNamingScope::AdjacentEntityNames],
            derived_regions: vec![
                derived_region,
                WorthTopologyDerivedRegion::EditLocalNeighborhoodRegion,
                WorthTopologyDerivedRegion::NamingContinuityRegion,
            ],
            lowered_mutations,
        }
    }

    pub fn detach_shell_or_wire_membership(
        relation_id: RelationId,
        kind: WorthShellOrWireMembershipKind,
    ) -> Self {
        let touched_aspects = BTreeSet::from([
            WorthAspect::Topology(WorthTopologyAspect::Ownership),
            WorthAspect::Diagnostics(WorthDiagnosticsAspect::Decisions),
        ]);
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
        let lowered_mutations = vec![WorthTopologyMutation::RemoveRelation { relation_id }];
        Self {
            family: WorthTopologyEditFamily::DetachShellOrWireMembership,
            action: WorthTopologyEditAction::DetachShellOrWireMembership { relation_id, kind },
            touched_aspects,
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
            lowered_mutations,
        }
    }

    pub fn splice_radial_adjacency(
        relation_id: RelationId,
        half_edge_id: EntityId,
        radial_next_half_edge_id: EntityId,
    ) -> Self {
        let touched_aspects = BTreeSet::from([
            WorthAspect::Topology(WorthTopologyAspect::Radial),
            WorthAspect::Diagnostics(WorthDiagnosticsAspect::Decisions),
        ]);
        let lowered_mutations = vec![WorthTopologyMutation::UpsertRelation {
            relation_id,
            kind: WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeRadialNext),
            source: half_edge_id,
            target: radial_next_half_edge_id,
        }];
        Self {
            family: WorthTopologyEditFamily::SpliceRadialAdjacency,
            action: WorthTopologyEditAction::SpliceRadialAdjacency {
                relation_id,
                half_edge_id,
                radial_next_half_edge_id,
            },
            touched_aspects,
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
            lowered_mutations,
        }
    }

    pub fn detach_radial_adjacency(relation_id: RelationId) -> Self {
        let touched_aspects = BTreeSet::from([
            WorthAspect::Topology(WorthTopologyAspect::Radial),
            WorthAspect::Diagnostics(WorthDiagnosticsAspect::Decisions),
        ]);
        let lowered_mutations = vec![WorthTopologyMutation::RemoveRelation { relation_id }];
        Self {
            family: WorthTopologyEditFamily::DetachRadialAdjacency,
            action: WorthTopologyEditAction::DetachRadialAdjacency { relation_id },
            touched_aspects,
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
            lowered_mutations,
        }
    }
}
