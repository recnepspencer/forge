use topology::facade::{
    current_topology_undo_family_catalog, TopologyUndoFamilyCatalog, TopologyUndoFamilyDeclaration,
    TopologyUndoFamilyIdentity, TopologyUndoFamilyLocalityPosture,
    TopologyUndoFamilyPriorProofPosture, TopologyUndoFamilyScopeProductPosture,
    TopologyUndoFamilyStageIndexPosture, TopologyUndoFamilyWorkloadDependencyPosture,
};
use worth_spatial::facade::undo_family_catalog::{
    current_spatial_undo_family_catalog, SpatialUndoFamilyCatalog, SpatialUndoFamilyDeclaration,
    SpatialUndoFamilyIdentity, SpatialUndoFamilyLocalityPosture,
    SpatialUndoFamilyPriorProofPosture, SpatialUndoFamilyScopeProductPosture,
    SpatialUndoFamilyStageIndexPosture, SpatialUndoFamilyWorkloadDependencyPosture,
};

use super::consumer_binding::UndoFamilyConsumerRequirement;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum UndoFamilyIdentity {
    TopologyTraversalViewsRollback,
    TopologyMaterializedGraphRollback,
    SpatialBooleanEventLedgerRollback,
    SpatialProjectionReceiptRollback,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UndoFamilyDomain {
    Topology,
    Spatial,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UndoFamilyLocalityPosture {
    RequiresTouchedClosure,
    RequiresSpatialTouchAuthority,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UndoFamilyPriorProofPosture {
    RequiresInvalidationExecutionReceipt,
    RequiresEvidenceLookupExecutionReceipt,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UndoFamilyStageIndexPosture {
    RequiresStageIndexIdentity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UndoFamilyWorkloadDependencyPosture {
    TopologyOnly,
    LookupReceiptOnly,
    RequiresLookupConsumedWorkload,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UndoFamilyScopeProductPosture {
    RequiresTopologyUndoScopeProduct,
    RequiresSpatialUndoScopeProduct,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UndoFamilyDeclaration {
    identity: UndoFamilyIdentity,
    domain: UndoFamilyDomain,
    locality_posture: UndoFamilyLocalityPosture,
    prior_proof_posture: UndoFamilyPriorProofPosture,
    stage_index_posture: UndoFamilyStageIndexPosture,
    workload_dependency_posture: UndoFamilyWorkloadDependencyPosture,
    scope_product_posture: UndoFamilyScopeProductPosture,
}

impl UndoFamilyDeclaration {
    fn from_topology(declaration: &TopologyUndoFamilyDeclaration) -> Self {
        Self {
            identity: match declaration.identity() {
                TopologyUndoFamilyIdentity::TraversalViewsRollback => {
                    UndoFamilyIdentity::TopologyTraversalViewsRollback
                }
                TopologyUndoFamilyIdentity::MaterializedGraphRollback => {
                    UndoFamilyIdentity::TopologyMaterializedGraphRollback
                }
            },
            domain: UndoFamilyDomain::Topology,
            locality_posture: match declaration.locality_posture() {
                TopologyUndoFamilyLocalityPosture::RequiresTouchedClosure => {
                    UndoFamilyLocalityPosture::RequiresTouchedClosure
                }
            },
            prior_proof_posture: match declaration.prior_proof_posture() {
                TopologyUndoFamilyPriorProofPosture::RequiresInvalidationExecutionReceipt => {
                    UndoFamilyPriorProofPosture::RequiresInvalidationExecutionReceipt
                }
            },
            stage_index_posture: match declaration.stage_index_posture() {
                TopologyUndoFamilyStageIndexPosture::RequiresStageIndexIdentity => {
                    UndoFamilyStageIndexPosture::RequiresStageIndexIdentity
                }
            },
            workload_dependency_posture: match declaration.workload_dependency_posture() {
                TopologyUndoFamilyWorkloadDependencyPosture::TopologyOnly => {
                    UndoFamilyWorkloadDependencyPosture::TopologyOnly
                }
            },
            scope_product_posture: match declaration.scope_product_posture() {
                TopologyUndoFamilyScopeProductPosture::RequiresTopologyUndoScopeProduct => {
                    UndoFamilyScopeProductPosture::RequiresTopologyUndoScopeProduct
                }
            },
        }
    }

    fn from_spatial(declaration: &SpatialUndoFamilyDeclaration) -> Self {
        Self {
            identity: match declaration.identity() {
                SpatialUndoFamilyIdentity::BooleanEventLedgerRollback => {
                    UndoFamilyIdentity::SpatialBooleanEventLedgerRollback
                }
                SpatialUndoFamilyIdentity::ProjectionReceiptRollback => {
                    UndoFamilyIdentity::SpatialProjectionReceiptRollback
                }
            },
            domain: UndoFamilyDomain::Spatial,
            locality_posture: match declaration.locality_posture() {
                SpatialUndoFamilyLocalityPosture::RequiresSpatialTouchAuthority => {
                    UndoFamilyLocalityPosture::RequiresSpatialTouchAuthority
                }
            },
            prior_proof_posture: match declaration.prior_proof_posture() {
                SpatialUndoFamilyPriorProofPosture::RequiresEvidenceLookupExecutionReceipt => {
                    UndoFamilyPriorProofPosture::RequiresEvidenceLookupExecutionReceipt
                }
            },
            stage_index_posture: match declaration.stage_index_posture() {
                SpatialUndoFamilyStageIndexPosture::RequiresStageIndexIdentity => {
                    UndoFamilyStageIndexPosture::RequiresStageIndexIdentity
                }
            },
            workload_dependency_posture: match declaration.workload_dependency_posture() {
                SpatialUndoFamilyWorkloadDependencyPosture::LookupReceiptOnly => {
                    UndoFamilyWorkloadDependencyPosture::LookupReceiptOnly
                }
                SpatialUndoFamilyWorkloadDependencyPosture::RequiresLookupConsumedWorkload => {
                    UndoFamilyWorkloadDependencyPosture::RequiresLookupConsumedWorkload
                }
            },
            scope_product_posture: match declaration.scope_product_posture() {
                SpatialUndoFamilyScopeProductPosture::RequiresSpatialUndoScopeProduct => {
                    UndoFamilyScopeProductPosture::RequiresSpatialUndoScopeProduct
                }
            },
        }
    }

    pub const fn identity(&self) -> UndoFamilyIdentity {
        self.identity
    }

    pub const fn domain(&self) -> UndoFamilyDomain {
        self.domain
    }

    pub const fn locality_posture(&self) -> UndoFamilyLocalityPosture {
        self.locality_posture
    }

    pub const fn prior_proof_posture(&self) -> UndoFamilyPriorProofPosture {
        self.prior_proof_posture
    }

    pub const fn stage_index_posture(&self) -> UndoFamilyStageIndexPosture {
        self.stage_index_posture
    }

    pub const fn workload_dependency_posture(&self) -> UndoFamilyWorkloadDependencyPosture {
        self.workload_dependency_posture
    }

    pub const fn scope_product_posture(&self) -> UndoFamilyScopeProductPosture {
        self.scope_product_posture
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UndoFamilyCatalogCounters {
    topology_family_count: usize,
    spatial_family_count: usize,
}

impl UndoFamilyCatalogCounters {
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
pub struct UndoFamilyCatalog {
    declarations: Vec<UndoFamilyDeclaration>,
    counters: UndoFamilyCatalogCounters,
}

impl UndoFamilyCatalog {
    fn new(topology: &TopologyUndoFamilyCatalog, spatial: &SpatialUndoFamilyCatalog) -> Self {
        let mut declarations = Vec::new();
        declarations.extend(
            topology
                .declarations()
                .iter()
                .map(UndoFamilyDeclaration::from_topology),
        );
        declarations.extend(
            spatial
                .declarations()
                .iter()
                .map(UndoFamilyDeclaration::from_spatial),
        );
        Self {
            declarations,
            counters: UndoFamilyCatalogCounters::new(
                topology.declarations().len(),
                spatial.declarations().len(),
            ),
        }
    }

    pub fn declarations(&self) -> &[UndoFamilyDeclaration] {
        &self.declarations
    }

    pub const fn counters(&self) -> &UndoFamilyCatalogCounters {
        &self.counters
    }

    pub fn require_family(&self, identity: UndoFamilyIdentity) -> Option<&UndoFamilyDeclaration> {
        self.declarations
            .iter()
            .find(|declaration| declaration.identity() == identity)
    }

    pub fn require_family_for_consumer(
        &self,
        requirement: &UndoFamilyConsumerRequirement,
    ) -> Option<&UndoFamilyDeclaration> {
        self.require_family(requirement.required_family())
    }

    pub fn families_for_domain(&self, domain: UndoFamilyDomain) -> Vec<&UndoFamilyDeclaration> {
        self.declarations
            .iter()
            .filter(|declaration| declaration.domain() == domain)
            .collect()
    }
}

pub fn current_undo_family_catalog() -> UndoFamilyCatalog {
    let topology = current_topology_undo_family_catalog();
    let spatial = current_spatial_undo_family_catalog();
    UndoFamilyCatalog::new(&topology, &spatial)
}
