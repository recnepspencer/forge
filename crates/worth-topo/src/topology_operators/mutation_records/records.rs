use std::collections::BTreeSet;

use forge_relational::facade::identity::{EntityId, RelationId};
use schema::facade::platform::aspects::Aspect;
use schema::facade::platform::authority::{CreateKey, EntityReference, TopologyMutation};
use schema::facade::platform::entities::TopologyEntityKind;
use serde::{Deserialize, Serialize};

use super::{
    BoundaryMembershipKind, LoopEndpointKind, LoopSuccessorKind, ShellOrWireMembershipKind,
    TopologyDerivedRegion, TopologyMutationChangedScope, TopologyMutationDerivedFallbackPolicy,
    TopologyMutationFamily, TopologyMutationNamingOutcome, TopologyMutationNamingReport,
    TopologyMutationNamingRow, TopologyMutationNamingScope,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TopologyDeclaredMutationAction {
    CreateTopologyEntity {
        create_key: CreateKey,
        kind: TopologyEntityKind,
        persistent_name_key: CreateKey,
        persistent_name_relation_key: CreateKey,
    },
    RetireTopologyEntity {
        entity_id: EntityId,
        kind: TopologyEntityKind,
    },
    AttachBoundaryMembership {
        create_key: CreateKey,
        kind: BoundaryMembershipKind,
        owner: EntityReference,
        member: EntityReference,
    },
    DetachBoundaryMembership {
        relation_id: RelationId,
        kind: BoundaryMembershipKind,
    },
    RewireLoopSuccessor {
        relation_id: RelationId,
        kind: LoopSuccessorKind,
        half_edge_id: EntityId,
        successor_half_edge_id: EntityId,
    },
    RewireLoopEndpoint {
        relation_id: RelationId,
        endpoint: LoopEndpointKind,
        half_edge_id: EntityId,
        vertex_id: EntityId,
    },
    AttachShellOrWireMembership {
        create_key: CreateKey,
        kind: ShellOrWireMembershipKind,
        owner: EntityReference,
        member: EntityReference,
    },
    DetachShellOrWireMembership {
        relation_id: RelationId,
        kind: ShellOrWireMembershipKind,
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

#[derive(Clone, Copy)]
pub(crate) enum TopologyDeclaredMutationActionRef<'a> {
    AttachBoundaryMembership {
        kind: BoundaryMembershipKind,
        owner: &'a EntityReference,
        member: &'a EntityReference,
    },
    AttachShellOrWireMembership {
        kind: ShellOrWireMembershipKind,
        owner: &'a EntityReference,
        member: &'a EntityReference,
    },
    CreateTopologyEntity {
        create_key: &'a str,
        kind: TopologyEntityKind,
    },
    DetachBoundaryMembership {
        relation_id: RelationId,
        kind: BoundaryMembershipKind,
    },
    DetachRadialAdjacency {
        relation_id: RelationId,
    },
    DetachShellOrWireMembership {
        relation_id: RelationId,
        kind: ShellOrWireMembershipKind,
    },
    RewireLoopEndpoint {
        relation_id: RelationId,
        endpoint: LoopEndpointKind,
        half_edge_id: EntityId,
        vertex_id: EntityId,
    },
    RewireLoopSuccessor {
        relation_id: RelationId,
        kind: LoopSuccessorKind,
        half_edge_id: EntityId,
        successor_half_edge_id: EntityId,
    },
    SpliceRadialAdjacency {
        relation_id: RelationId,
        half_edge_id: EntityId,
        radial_next_half_edge_id: EntityId,
    },
    RetireTopologyEntity {
        entity_id: EntityId,
        kind: TopologyEntityKind,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyDeclaredMutationRecord {
    pub family: TopologyMutationFamily,
    pub action: TopologyDeclaredMutationAction,
    pub touched_aspects: BTreeSet<Aspect>,
    pub changed_scopes: Vec<TopologyMutationChangedScope>,
    pub naming_scopes: Vec<TopologyMutationNamingScope>,
    pub derived_regions: Vec<TopologyDerivedRegion>,
    pub derived_fallback_policy: TopologyMutationDerivedFallbackPolicy,
    pub lowered_mutations: Vec<TopologyMutation>,
}

impl TopologyDeclaredMutationRecord {
    pub(crate) fn action_ref(&self) -> TopologyDeclaredMutationActionRef<'_> {
        match &self.action {
            TopologyDeclaredMutationAction::AttachBoundaryMembership {
                kind,
                owner,
                member,
                ..
            } => TopologyDeclaredMutationActionRef::AttachBoundaryMembership {
                kind: *kind,
                owner,
                member,
            },
            TopologyDeclaredMutationAction::AttachShellOrWireMembership {
                kind,
                owner,
                member,
                ..
            } => TopologyDeclaredMutationActionRef::AttachShellOrWireMembership {
                kind: *kind,
                owner,
                member,
            },
            TopologyDeclaredMutationAction::CreateTopologyEntity {
                create_key, kind, ..
            } => TopologyDeclaredMutationActionRef::CreateTopologyEntity {
                create_key: create_key.as_str(),
                kind: *kind,
            },
            TopologyDeclaredMutationAction::DetachBoundaryMembership { relation_id, kind } => {
                TopologyDeclaredMutationActionRef::DetachBoundaryMembership {
                    relation_id: *relation_id,
                    kind: *kind,
                }
            }
            TopologyDeclaredMutationAction::DetachRadialAdjacency { relation_id } => {
                TopologyDeclaredMutationActionRef::DetachRadialAdjacency {
                    relation_id: *relation_id,
                }
            }
            TopologyDeclaredMutationAction::DetachShellOrWireMembership { relation_id, kind } => {
                TopologyDeclaredMutationActionRef::DetachShellOrWireMembership {
                    relation_id: *relation_id,
                    kind: *kind,
                }
            }
            TopologyDeclaredMutationAction::RewireLoopEndpoint {
                relation_id,
                endpoint,
                half_edge_id,
                vertex_id,
            } => TopologyDeclaredMutationActionRef::RewireLoopEndpoint {
                relation_id: *relation_id,
                endpoint: *endpoint,
                half_edge_id: *half_edge_id,
                vertex_id: *vertex_id,
            },
            TopologyDeclaredMutationAction::RewireLoopSuccessor {
                relation_id,
                kind,
                half_edge_id,
                successor_half_edge_id,
            } => TopologyDeclaredMutationActionRef::RewireLoopSuccessor {
                relation_id: *relation_id,
                kind: *kind,
                half_edge_id: *half_edge_id,
                successor_half_edge_id: *successor_half_edge_id,
            },
            TopologyDeclaredMutationAction::SpliceRadialAdjacency {
                relation_id,
                half_edge_id,
                radial_next_half_edge_id,
            } => TopologyDeclaredMutationActionRef::SpliceRadialAdjacency {
                relation_id: *relation_id,
                half_edge_id: *half_edge_id,
                radial_next_half_edge_id: *radial_next_half_edge_id,
            },
            TopologyDeclaredMutationAction::RetireTopologyEntity {
                entity_id, kind, ..
            } => TopologyDeclaredMutationActionRef::RetireTopologyEntity {
                entity_id: *entity_id,
                kind: *kind,
            },
        }
    }

    pub fn touched_aspects(&self) -> &BTreeSet<Aspect> {
        &self.touched_aspects
    }

    pub fn changed_scopes(&self) -> &[TopologyMutationChangedScope] {
        &self.changed_scopes
    }

    pub fn naming_scopes(&self) -> &[TopologyMutationNamingScope] {
        &self.naming_scopes
    }

    pub fn derived_regions(&self) -> &[TopologyDerivedRegion] {
        &self.derived_regions
    }

    pub fn derived_fallback_policy(&self) -> TopologyMutationDerivedFallbackPolicy {
        self.derived_fallback_policy
    }

    #[cfg(test)]
    pub fn with_derived_fallback_policy(
        mut self,
        policy: TopologyMutationDerivedFallbackPolicy,
    ) -> Self {
        self.derived_fallback_policy = policy;
        self
    }

    pub fn lowered_mutations(&self) -> &[TopologyMutation] {
        &self.lowered_mutations
    }

    pub fn naming_report(&self) -> TopologyMutationNamingReport {
        let rows = self
            .naming_scopes
            .iter()
            .copied()
            .map(|scope| match self.family {
                TopologyMutationFamily::CreateTopologyEntity => TopologyMutationNamingRow {
                    family: self.family,
                    scope,
                    outcome: TopologyMutationNamingOutcome::Preserved,
                    reason: "new topology entity publishes with one attached persistent name".into(),
                },
                TopologyMutationFamily::RetireTopologyEntity => TopologyMutationNamingRow {
                    family: self.family,
                    scope,
                    outcome: TopologyMutationNamingOutcome::Rejected,
                    reason:
                        "retired topology entity does not preserve one canonical successor naming target"
                            .into(),
                },
                TopologyMutationFamily::AttachBoundaryMembership
                | TopologyMutationFamily::DetachBoundaryMembership
                | TopologyMutationFamily::RewireLoopSuccessor
                | TopologyMutationFamily::RewireLoopEndpoint
                | TopologyMutationFamily::AttachShellOrWireMembership
                | TopologyMutationFamily::DetachShellOrWireMembership
                | TopologyMutationFamily::SpliceRadialAdjacency
                | TopologyMutationFamily::DetachRadialAdjacency => TopologyMutationNamingRow {
                    family: self.family,
                    scope,
                    outcome: TopologyMutationNamingOutcome::Ambiguous,
                    reason:
                        "topology neighborhood changed without a declared canonical continuity mapping"
                            .into(),
                },
            })
            .collect();
        TopologyMutationNamingReport { rows }
    }
}
