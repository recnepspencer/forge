use topology::facade::{
    current_topology_replay_family_catalog, TopologyReplayFamilyCatalog,
    TopologyReplayFamilyDeclaration, TopologyReplayFamilyIdentity,
    TopologyReplayFamilyLocalityPosture, TopologyReplayFamilyPriorProofPosture,
    TopologyReplayFamilyScopeProductPosture, TopologyReplayFamilyStageIndexPosture,
    TopologyReplayFamilyWorkloadDependencyPosture,
};
use worth_spatial::facade::replay_family_catalog::{
    current_spatial_replay_family_catalog, SpatialReplayFamilyCatalog,
    SpatialReplayFamilyDeclaration, SpatialReplayFamilyIdentity,
    SpatialReplayFamilyLocalityPosture, SpatialReplayFamilyPriorProofPosture,
    SpatialReplayFamilyScopeProductPosture, SpatialReplayFamilyStageIndexPosture,
    SpatialReplayFamilyWorkloadDependencyPosture,
};

use super::consumer_binding::ReplayFamilyConsumerRequirement;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ReplayFamilyIdentity {
    TopologyTraversalViewsReplay,
    TopologyMaterializedGraphReplay,
    SpatialBooleanEventLedgerReplay,
    SpatialProjectionReceiptReplay,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplayFamilyDomain {
    Topology,
    Spatial,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplayFamilyLocalityPosture {
    RequiresTouchedClosure,
    RequiresSpatialTouchAuthority,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplayFamilyPriorProofPosture {
    RequiresInvalidationSelectedPlanAndExecutionReceipt,
    RequiresEvidenceLookupExecutionReceipt,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplayFamilyStageIndexPosture {
    RequiresStageIndexIdentity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplayFamilyWorkloadDependencyPosture {
    TopologyOnly,
    LookupReceiptOnly,
    RequiresLookupConsumedWorkloadAndRetainedReplay,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplayFamilyScopeProductPosture {
    RequiresTopologyReplayScopeProduct,
    RequiresSpatialReplayScopeProduct,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayFamilyDeclaration {
    identity: ReplayFamilyIdentity,
    domain: ReplayFamilyDomain,
    locality_posture: ReplayFamilyLocalityPosture,
    prior_proof_posture: ReplayFamilyPriorProofPosture,
    stage_index_posture: ReplayFamilyStageIndexPosture,
    workload_dependency_posture: ReplayFamilyWorkloadDependencyPosture,
    scope_product_posture: ReplayFamilyScopeProductPosture,
}

impl ReplayFamilyDeclaration {
    fn from_topology(declaration: &TopologyReplayFamilyDeclaration) -> Self {
        Self {
            identity: match declaration.identity() {
                TopologyReplayFamilyIdentity::TraversalViewsReplay => {
                    ReplayFamilyIdentity::TopologyTraversalViewsReplay
                }
                TopologyReplayFamilyIdentity::MaterializedGraphReplay => {
                    ReplayFamilyIdentity::TopologyMaterializedGraphReplay
                }
            },
            domain: ReplayFamilyDomain::Topology,
            locality_posture: match declaration.locality_posture() {
                TopologyReplayFamilyLocalityPosture::RequiresTouchedClosure => {
                    ReplayFamilyLocalityPosture::RequiresTouchedClosure
                }
            },
            prior_proof_posture: match declaration.prior_proof_posture() {
                TopologyReplayFamilyPriorProofPosture::RequiresInvalidationSelectedPlanAndExecutionReceipt => {
                    ReplayFamilyPriorProofPosture::RequiresInvalidationSelectedPlanAndExecutionReceipt
                }
            },
            stage_index_posture: match declaration.stage_index_posture() {
                TopologyReplayFamilyStageIndexPosture::RequiresStageIndexIdentity => {
                    ReplayFamilyStageIndexPosture::RequiresStageIndexIdentity
                }
            },
            workload_dependency_posture: match declaration.workload_dependency_posture() {
                TopologyReplayFamilyWorkloadDependencyPosture::TopologyOnly => {
                    ReplayFamilyWorkloadDependencyPosture::TopologyOnly
                }
            },
            scope_product_posture: match declaration.scope_product_posture() {
                TopologyReplayFamilyScopeProductPosture::RequiresTopologyReplayScopeProduct => {
                    ReplayFamilyScopeProductPosture::RequiresTopologyReplayScopeProduct
                }
            },
        }
    }

    fn from_spatial(declaration: &SpatialReplayFamilyDeclaration) -> Self {
        Self {
            identity: match declaration.identity() {
                SpatialReplayFamilyIdentity::BooleanEventLedgerReplay => {
                    ReplayFamilyIdentity::SpatialBooleanEventLedgerReplay
                }
                SpatialReplayFamilyIdentity::ProjectionReceiptReplay => {
                    ReplayFamilyIdentity::SpatialProjectionReceiptReplay
                }
            },
            domain: ReplayFamilyDomain::Spatial,
            locality_posture: match declaration.locality_posture() {
                SpatialReplayFamilyLocalityPosture::RequiresSpatialTouchAuthority => {
                    ReplayFamilyLocalityPosture::RequiresSpatialTouchAuthority
                }
            },
            prior_proof_posture: match declaration.prior_proof_posture() {
                SpatialReplayFamilyPriorProofPosture::RequiresEvidenceLookupExecutionReceipt => {
                    ReplayFamilyPriorProofPosture::RequiresEvidenceLookupExecutionReceipt
                }
            },
            stage_index_posture: match declaration.stage_index_posture() {
                SpatialReplayFamilyStageIndexPosture::RequiresStageIndexIdentity => {
                    ReplayFamilyStageIndexPosture::RequiresStageIndexIdentity
                }
            },
            workload_dependency_posture: match declaration.workload_dependency_posture() {
                SpatialReplayFamilyWorkloadDependencyPosture::LookupReceiptOnly => {
                    ReplayFamilyWorkloadDependencyPosture::LookupReceiptOnly
                }
                SpatialReplayFamilyWorkloadDependencyPosture::RequiresLookupConsumedWorkloadAndRetainedReplay => {
                    ReplayFamilyWorkloadDependencyPosture::RequiresLookupConsumedWorkloadAndRetainedReplay
                }
            },
            scope_product_posture: match declaration.scope_product_posture() {
                SpatialReplayFamilyScopeProductPosture::RequiresSpatialReplayScopeProduct => {
                    ReplayFamilyScopeProductPosture::RequiresSpatialReplayScopeProduct
                }
            },
        }
    }

    pub const fn identity(&self) -> ReplayFamilyIdentity {
        self.identity
    }

    pub const fn domain(&self) -> ReplayFamilyDomain {
        self.domain
    }

    pub const fn locality_posture(&self) -> ReplayFamilyLocalityPosture {
        self.locality_posture
    }

    pub const fn prior_proof_posture(&self) -> ReplayFamilyPriorProofPosture {
        self.prior_proof_posture
    }

    pub const fn stage_index_posture(&self) -> ReplayFamilyStageIndexPosture {
        self.stage_index_posture
    }

    pub const fn workload_dependency_posture(&self) -> ReplayFamilyWorkloadDependencyPosture {
        self.workload_dependency_posture
    }

    pub const fn scope_product_posture(&self) -> ReplayFamilyScopeProductPosture {
        self.scope_product_posture
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayFamilyCatalogCounters {
    topology_family_count: usize,
    spatial_family_count: usize,
}

impl ReplayFamilyCatalogCounters {
    fn new(topology_family_count: usize, spatial_family_count: usize) -> Self {
        Self {
            topology_family_count,
            spatial_family_count,
        }
    }

    pub const fn topology_family_count(&self) -> usize {
        self.topology_family_count
    }

    pub const fn spatial_family_count(&self) -> usize {
        self.spatial_family_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayFamilyCatalog {
    declarations: Vec<ReplayFamilyDeclaration>,
    counters: ReplayFamilyCatalogCounters,
}

impl ReplayFamilyCatalog {
    fn new(topology: &TopologyReplayFamilyCatalog, spatial: &SpatialReplayFamilyCatalog) -> Self {
        let mut declarations = Vec::new();
        declarations.extend(
            topology
                .declarations()
                .iter()
                .map(ReplayFamilyDeclaration::from_topology),
        );
        declarations.extend(
            spatial
                .declarations()
                .iter()
                .map(ReplayFamilyDeclaration::from_spatial),
        );

        Self {
            declarations,
            counters: ReplayFamilyCatalogCounters::new(
                topology.declarations().len(),
                spatial.declarations().len(),
            ),
        }
    }

    pub fn declarations(&self) -> &[ReplayFamilyDeclaration] {
        &self.declarations
    }

    pub const fn counters(&self) -> &ReplayFamilyCatalogCounters {
        &self.counters
    }

    pub fn require_family(
        &self,
        identity: ReplayFamilyIdentity,
    ) -> Option<&ReplayFamilyDeclaration> {
        self.declarations
            .iter()
            .find(|declaration| declaration.identity() == identity)
    }

    pub fn require_family_for_consumer(
        &self,
        requirement: &ReplayFamilyConsumerRequirement,
    ) -> Option<&ReplayFamilyDeclaration> {
        self.require_family(requirement.required_family())
    }

    pub fn families_for_domain(&self, domain: ReplayFamilyDomain) -> Vec<&ReplayFamilyDeclaration> {
        self.declarations
            .iter()
            .filter(|declaration| declaration.domain() == domain)
            .collect()
    }

    pub fn families_requiring_retained_replay(&self) -> Vec<&ReplayFamilyDeclaration> {
        self.declarations
            .iter()
            .filter(|declaration| {
                declaration.workload_dependency_posture()
                    == ReplayFamilyWorkloadDependencyPosture::RequiresLookupConsumedWorkloadAndRetainedReplay
            })
            .collect()
    }
}

pub fn current_replay_family_catalog() -> ReplayFamilyCatalog {
    let topology = current_topology_replay_family_catalog();
    let spatial = current_spatial_replay_family_catalog();
    ReplayFamilyCatalog::new(&topology, &spatial)
}
