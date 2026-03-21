use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::config::data::CrossContextPolicy;
use crate::identity::data::KindId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct RelationIntegrityPlanRevision(pub u128);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RelationIntegrityDeclarations {
    pub plan_revision: RelationIntegrityPlanRevision,
    pub endpoint_kind_contracts: Vec<EndpointKindContractDeclaration>,
    pub cardinality_contracts: Vec<CardinalityContractDeclaration>,
    pub uniqueness_contracts: Vec<UniquenessContractDeclaration>,
    pub symmetry_contracts: Vec<SymmetryContractDeclaration>,
    pub endpoint_deletion_integrity_contracts: Vec<EndpointDeletionIntegrityDeclaration>,
}

impl RelationIntegrityDeclarations {
    pub fn new(
        endpoint_kind_contracts: Vec<EndpointKindContractDeclaration>,
        cardinality_contracts: Vec<CardinalityContractDeclaration>,
        uniqueness_contracts: Vec<UniquenessContractDeclaration>,
        symmetry_contracts: Vec<SymmetryContractDeclaration>,
        endpoint_deletion_integrity_contracts: Vec<EndpointDeletionIntegrityDeclaration>,
    ) -> Self {
        Self {
            plan_revision: RelationIntegrityPlanRevision(0),
            endpoint_kind_contracts,
            cardinality_contracts,
            uniqueness_contracts,
            symmetry_contracts,
            endpoint_deletion_integrity_contracts,
        }
    }

    pub fn contract_count(&self) -> usize {
        self.endpoint_kind_contracts.len()
            + self.cardinality_contracts.len()
            + self.uniqueness_contracts.len()
            + self.symmetry_contracts.len()
            + self.endpoint_deletion_integrity_contracts.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndpointKindContractDeclaration {
    pub contract_id: String,
    pub allowed_source_kinds: Vec<KindId>,
    pub allowed_target_kinds: Vec<KindId>,
    pub self_edges_allowed: bool,
    pub cross_context_policy: CrossContextPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardinalityContractDeclaration {
    pub contract_id: String,
    pub source_max: Option<usize>,
    pub target_max: Option<usize>,
    pub pair_max: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum UniquenessScope {
    DirectedSemanticEdge,
    NormalizedSymmetricEdge,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UniquenessContractDeclaration {
    pub contract_id: String,
    pub scope: UniquenessScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SymmetryMode {
    CanonicalUndirected,
    PairedInverseRequired,
    InverseProhibited,
    PairedTwinRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymmetryContractDeclaration {
    pub contract_id: String,
    pub mode: SymmetryMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EndpointDeletionIntegrityMode {
    RejectDeleteWithLiveRelations,
    RequireRelationDeletionInSameCommit,
    RequireRelationRetirement,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndpointDeletionIntegrityDeclaration {
    pub contract_id: String,
    pub mode: EndpointDeletionIntegrityMode,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RelationIntegrityPlanCatalog {
    pub relation_plans: BTreeMap<KindId, LoweredRelationIntegrityPlan>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoweredRelationIntegrityPlan {
    pub kind_id: KindId,
    pub plan_revision: RelationIntegrityPlanRevision,
    pub endpoint_kind_contracts: Vec<LoweredEndpointKindContract>,
    pub cardinality_contracts: Vec<LoweredCardinalityContract>,
    pub uniqueness_contracts: Vec<LoweredUniquenessContract>,
    pub symmetry_contracts: Vec<LoweredSymmetryContract>,
    pub endpoint_deletion_integrity_contracts: Vec<LoweredEndpointDeletionIntegrityContract>,
}

impl LoweredRelationIntegrityPlan {
    pub fn contract_count(&self) -> usize {
        self.endpoint_kind_contracts.len()
            + self.cardinality_contracts.len()
            + self.uniqueness_contracts.len()
            + self.symmetry_contracts.len()
            + self.endpoint_deletion_integrity_contracts.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LoweredEndpointKindContract {
    pub contract_id: String,
    pub relation_kind_id: KindId,
    pub allowed_source_kinds: Vec<KindId>,
    pub allowed_target_kinds: Vec<KindId>,
    pub self_edges_allowed: bool,
    pub cross_context_policy: CrossContextPolicy,
    pub plan_revision: RelationIntegrityPlanRevision,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LoweredCardinalityContract {
    pub contract_id: String,
    pub relation_kind_id: KindId,
    pub source_max: Option<usize>,
    pub target_max: Option<usize>,
    pub pair_max: Option<usize>,
    pub plan_revision: RelationIntegrityPlanRevision,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LoweredUniquenessContract {
    pub contract_id: String,
    pub relation_kind_id: KindId,
    pub scope: UniquenessScope,
    pub plan_revision: RelationIntegrityPlanRevision,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LoweredSymmetryContract {
    pub contract_id: String,
    pub relation_kind_id: KindId,
    pub mode: SymmetryMode,
    pub plan_revision: RelationIntegrityPlanRevision,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LoweredEndpointDeletionIntegrityContract {
    pub contract_id: String,
    pub relation_kind_id: KindId,
    pub mode: EndpointDeletionIntegrityMode,
    pub plan_revision: RelationIntegrityPlanRevision,
}
