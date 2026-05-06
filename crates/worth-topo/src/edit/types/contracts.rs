use std::collections::BTreeSet;

use forge_relational::facade::identity::{EntityId, RelationId};
use serde::{Deserialize, Serialize};
use worth_schema::facade::{
    WorthAspect, WorthCreateKey, WorthEntityReference, WorthMutationOrigin,
    WorthTopologyEntityKind, WorthTopologyMutation,
};

use super::{
    WorthBoundaryMembershipKind, WorthLoopEndpointKind, WorthLoopSuccessorKind,
    WorthShellOrWireMembershipKind, WorthTopologyDerivedRegion, WorthTopologyEditChangedScope,
    WorthTopologyEditFamily, WorthTopologyEditNamingOutcome, WorthTopologyEditNamingReport,
    WorthTopologyEditNamingRow, WorthTopologyEditNamingScope,
};

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
        mode: &super::super::facade::WorthTopologyEditApplicationMode,
    ) -> WorthMutationOrigin {
        match mode {
            super::super::facade::WorthTopologyEditApplicationMode::Mainline => {
                WorthMutationOrigin::LocalEdit
            }
            super::super::facade::WorthTopologyEditApplicationMode::BranchLocal(_) => {
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
}
