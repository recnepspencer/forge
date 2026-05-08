use std::collections::BTreeSet;

use forge_relational::facade::identity::{EntityId, RelationId};
use schema::facade::{
    Aspect, CreateKey, EntityReference, MutationOrigin, TopologyEntityKind, TopologyMutation,
};
use serde::{Deserialize, Serialize};

use super::{
    BoundaryMembershipKind, LoopEndpointKind, LoopSuccessorKind, ShellOrWireMembershipKind,
    TopologyDerivedRegion, TopologyEditChangedScope, TopologyEditFamily, TopologyEditNamingOutcome,
    TopologyEditNamingReport, TopologyEditNamingRow, TopologyEditNamingScope,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TopologyEditAction {
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyEditContract {
    pub family: TopologyEditFamily,
    pub action: TopologyEditAction,
    pub touched_aspects: BTreeSet<Aspect>,
    pub changed_scopes: Vec<TopologyEditChangedScope>,
    pub naming_scopes: Vec<TopologyEditNamingScope>,
    pub derived_regions: Vec<TopologyDerivedRegion>,
    pub lowered_mutations: Vec<TopologyMutation>,
}

impl TopologyEditContract {
    pub fn mutation_origin_for(
        mode: &super::super::facade::TopologyEditApplicationMode,
    ) -> MutationOrigin {
        match mode {
            super::super::facade::TopologyEditApplicationMode::Mainline => {
                MutationOrigin::LocalEdit
            }
            super::super::facade::TopologyEditApplicationMode::BranchLocal(_) => {
                MutationOrigin::BranchLocalApplication
            }
        }
    }

    pub fn touched_aspects(&self) -> &BTreeSet<Aspect> {
        &self.touched_aspects
    }

    pub fn changed_scopes(&self) -> &[TopologyEditChangedScope] {
        &self.changed_scopes
    }

    pub fn naming_scopes(&self) -> &[TopologyEditNamingScope] {
        &self.naming_scopes
    }

    pub fn derived_regions(&self) -> &[TopologyDerivedRegion] {
        &self.derived_regions
    }

    pub fn lowered_mutations(&self) -> &[TopologyMutation] {
        &self.lowered_mutations
    }

    pub fn naming_report(&self) -> TopologyEditNamingReport {
        let rows = self
            .naming_scopes
            .iter()
            .copied()
            .map(|scope| match self.family {
                TopologyEditFamily::CreateTopologyEntity => TopologyEditNamingRow {
                    family: self.family,
                    scope,
                    outcome: TopologyEditNamingOutcome::Preserved,
                    reason: "new topology entity publishes with one attached persistent name".into(),
                },
                TopologyEditFamily::RetireTopologyEntity => TopologyEditNamingRow {
                    family: self.family,
                    scope,
                    outcome: TopologyEditNamingOutcome::Rejected,
                    reason:
                        "retired topology entity does not preserve one canonical successor naming target"
                            .into(),
                },
                TopologyEditFamily::AttachBoundaryMembership
                | TopologyEditFamily::DetachBoundaryMembership
                | TopologyEditFamily::RewireLoopSuccessor
                | TopologyEditFamily::RewireLoopEndpoint
                | TopologyEditFamily::AttachShellOrWireMembership
                | TopologyEditFamily::DetachShellOrWireMembership
                | TopologyEditFamily::SpliceRadialAdjacency
                | TopologyEditFamily::DetachRadialAdjacency => TopologyEditNamingRow {
                    family: self.family,
                    scope,
                    outcome: TopologyEditNamingOutcome::Ambiguous,
                    reason:
                        "topology neighborhood changed without a declared canonical continuity mapping"
                            .into(),
                },
            })
            .collect();
        TopologyEditNamingReport { rows }
    }
}
