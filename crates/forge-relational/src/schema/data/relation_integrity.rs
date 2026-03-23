use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::fmt;
use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::config::data::CrossContextPolicy;
use crate::config::data::CascadeDeletePolicy;
use crate::identity::data::KindId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct RelationIntegrityPlanRevision(pub u128);

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
#[serde(transparent)]
pub struct ContractId(String);

impl ContractId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for ContractId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for ContractId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<ContractId> for String {
    fn from(value: ContractId) -> Self {
        value.0
    }
}

impl Borrow<str> for ContractId {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl Deref for ContractId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl fmt::Display for ContractId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl PartialEq<&str> for ContractId {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<String> for ContractId {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other
    }
}

pub(crate) fn derive_relation_integrity_plan_revision(
    endpoint_kind_contracts: &[EndpointKindContractDeclaration],
    cardinality_contracts: &[CardinalityContractDeclaration],
    uniqueness_contracts: &[UniquenessContractDeclaration],
    symmetry_contracts: &[SymmetryContractDeclaration],
    endpoint_deletion_integrity_contracts: &[EndpointDeletionIntegrityDeclaration],
) -> RelationIntegrityPlanRevision {
    const FNV_OFFSET: u128 = 0x6c62272e07bb014262b821756295c58d;
    const FNV_PRIME: u128 = 0x0000000001000000000000000000013B;

    fn mix_bytes(hash: &mut u128, bytes: &[u8]) {
        for byte in bytes {
            *hash ^= *byte as u128;
            *hash = hash.wrapping_mul(FNV_PRIME);
        }
        *hash ^= 0xff_u128;
        *hash = hash.wrapping_mul(FNV_PRIME);
    }

    fn mix_string(hash: &mut u128, value: &str) {
        mix_bytes(hash, value.as_bytes());
    }

    fn mix_kind_ids(hash: &mut u128, kinds: &[KindId]) {
        for kind in kinds {
            mix_bytes(hash, &kind.0.to_le_bytes());
        }
    }

    let mut hash = FNV_OFFSET;
    for declaration in endpoint_kind_contracts {
        mix_bytes(&mut hash, b"endpoint_kind");
        mix_string(&mut hash, declaration.contract_id.as_str());
        mix_kind_ids(&mut hash, &declaration.allowed_source_kinds);
        mix_kind_ids(&mut hash, &declaration.allowed_target_kinds);
        mix_bytes(&mut hash, &[u8::from(declaration.self_edges_allowed)]);
        mix_bytes(
            &mut hash,
            match declaration.cross_context_policy {
                crate::config::data::CrossContextPolicy::AllowExplicit => b"allow_explicit",
                crate::config::data::CrossContextPolicy::SchemaControlled => b"schema_controlled",
                crate::config::data::CrossContextPolicy::Forbid => b"forbid",
            },
        );
    }
    for declaration in cardinality_contracts {
        mix_bytes(&mut hash, b"cardinality");
        mix_string(&mut hash, declaration.contract_id.as_str());
        mix_bytes(&mut hash, &declaration.source_max.unwrap_or(u64::MAX).to_le_bytes());
        mix_bytes(&mut hash, &declaration.target_max.unwrap_or(u64::MAX).to_le_bytes());
        mix_bytes(&mut hash, &declaration.pair_max.unwrap_or(u64::MAX).to_le_bytes());
        mix_bytes(&mut hash, &declaration.source_min.unwrap_or(0).to_le_bytes());
        mix_bytes(&mut hash, &declaration.target_min.unwrap_or(0).to_le_bytes());
        mix_bytes(&mut hash, &declaration.pair_min.unwrap_or(0).to_le_bytes());
        mix_bytes(
            &mut hash,
            match declaration.pair_min_semantics {
                PairMinimumSemantics::ObservedDirectedPairs => b"observed_directed_pairs",
            },
        );
        mix_bytes(
            &mut hash,
            match declaration.minimum_enforcement {
                MinimumCardinalityEnforcement::CommitBoundary => b"minimum_commit_boundary",
                MinimumCardinalityEnforcement::CertificationBoundary => {
                    b"minimum_certification_boundary"
                }
            },
        );
    }
    for declaration in uniqueness_contracts {
        mix_bytes(&mut hash, b"uniqueness");
        mix_string(&mut hash, declaration.contract_id.as_str());
        mix_bytes(
            &mut hash,
            match declaration.scope {
                UniquenessScope::DirectedSemanticEdge => b"directed",
                UniquenessScope::NormalizedSymmetricEdge => b"normalized",
            },
        );
    }
    for declaration in symmetry_contracts {
        mix_bytes(&mut hash, b"symmetry");
        mix_string(&mut hash, declaration.contract_id.as_str());
        mix_bytes(
            &mut hash,
            match declaration.mode {
                SymmetryMode::CanonicalUndirected => b"canonical_undirected",
                SymmetryMode::PairedInverseRequired => b"paired_inverse_required",
                SymmetryMode::InverseProhibited => b"inverse_prohibited",
                SymmetryMode::PairedTwinRequired => b"paired_twin_required",
            },
        );
    }
    for declaration in endpoint_deletion_integrity_contracts {
        mix_bytes(&mut hash, b"endpoint_delete");
        mix_string(&mut hash, declaration.contract_id.as_str());
        mix_bytes(
            &mut hash,
            match declaration.mode {
                EndpointDeletionIntegrityMode::RejectDeleteWithLiveRelations => {
                    b"reject_delete_with_live_relations"
                }
                EndpointDeletionIntegrityMode::RequireRelationDeletionInSameCommit => {
                    b"require_relation_deletion_in_same_commit"
                }
                EndpointDeletionIntegrityMode::RequireRelationRetirement => {
                    b"require_relation_retirement"
                }
            },
        );
    }
    RelationIntegrityPlanRevision(hash)
}

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
            plan_revision: derive_relation_integrity_plan_revision(
                &endpoint_kind_contracts,
                &cardinality_contracts,
                &uniqueness_contracts,
                &symmetry_contracts,
                &endpoint_deletion_integrity_contracts,
            ),
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
    pub contract_id: ContractId,
    pub allowed_source_kinds: Vec<KindId>,
    pub allowed_target_kinds: Vec<KindId>,
    pub self_edges_allowed: bool,
    pub cross_context_policy: CrossContextPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardinalityContractDeclaration {
    pub contract_id: ContractId,
    pub source_max: Option<u64>,
    pub target_max: Option<u64>,
    pub pair_max: Option<u64>,
    pub source_min: Option<u64>,
    pub target_min: Option<u64>,
    pub pair_min: Option<u64>,
    pub pair_min_semantics: PairMinimumSemantics,
    pub minimum_enforcement: MinimumCardinalityEnforcement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PairMinimumSemantics {
    ObservedDirectedPairs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MinimumCardinalityEnforcement {
    CommitBoundary,
    CertificationBoundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[non_exhaustive]
pub enum UniquenessScope {
    DirectedSemanticEdge,
    NormalizedSymmetricEdge,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UniquenessContractDeclaration {
    pub contract_id: ContractId,
    pub scope: UniquenessScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SymmetryMode {
    CanonicalUndirected,
    PairedInverseRequired,
    InverseProhibited,
    PairedTwinRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymmetryContractDeclaration {
    pub contract_id: ContractId,
    pub mode: SymmetryMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[non_exhaustive]
pub enum EndpointDeletionIntegrityMode {
    RejectDeleteWithLiveRelations,
    RequireRelationDeletionInSameCommit,
    RequireRelationRetirement,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndpointDeletionIntegrityDeclaration {
    pub contract_id: ContractId,
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
    pub cardinality_maximum_contracts: Vec<LoweredCardinalityMaximumContract>,
    pub cardinality_minimum_contracts: Vec<LoweredCardinalityMinimumContract>,
    pub uniqueness_contracts: Vec<LoweredUniquenessContract>,
    pub symmetry_contracts: Vec<LoweredSymmetryContract>,
    pub endpoint_deletion_integrity_contracts: Vec<LoweredEndpointDeletionIntegrityContract>,
}

impl LoweredRelationIntegrityPlan {
    pub fn contract_count(&self) -> usize {
        self.endpoint_kind_contracts.len()
            + self.cardinality_maximum_contracts.len()
            + self.cardinality_minimum_contracts.len()
            + self.uniqueness_contracts.len()
            + self.symmetry_contracts.len()
            + self.endpoint_deletion_integrity_contracts.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LoweredEndpointKindContract {
    pub contract_id: ContractId,
    pub relation_kind_id: KindId,
    pub allowed_source_kinds: Vec<KindId>,
    pub allowed_target_kinds: Vec<KindId>,
    pub self_edges_allowed: bool,
    pub cross_context_policy: CrossContextPolicy,
    pub plan_revision: RelationIntegrityPlanRevision,
}

impl LoweredEndpointKindContract {
    pub fn allows_source_kind(&self, kind_id: KindId) -> bool {
        self.allowed_source_kinds.binary_search(&kind_id).is_ok()
    }

    pub fn allows_target_kind(&self, kind_id: KindId) -> bool {
        self.allowed_target_kinds.binary_search(&kind_id).is_ok()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LoweredCardinalityMaximumContract {
    pub contract_id: ContractId,
    pub relation_kind_id: KindId,
    pub source_max: Option<u64>,
    pub target_max: Option<u64>,
    pub pair_max: Option<u64>,
    pub plan_revision: RelationIntegrityPlanRevision,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LoweredCardinalityMinimumContract {
    pub contract_id: ContractId,
    pub relation_kind_id: KindId,
    pub source_min: Option<u64>,
    pub target_min: Option<u64>,
    pub pair_min: Option<u64>,
    pub pair_min_semantics: PairMinimumSemantics,
    pub candidate_source_kinds: Vec<KindId>,
    pub candidate_target_kinds: Vec<KindId>,
    pub minimum_enforcement: MinimumCardinalityEnforcement,
    pub plan_revision: RelationIntegrityPlanRevision,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LoweredUniquenessContract {
    pub contract_id: ContractId,
    pub relation_kind_id: KindId,
    pub scope: UniquenessScope,
    pub plan_revision: RelationIntegrityPlanRevision,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LoweredSymmetryContract {
    pub contract_id: ContractId,
    pub relation_kind_id: KindId,
    pub mode: SymmetryMode,
    pub plan_revision: RelationIntegrityPlanRevision,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LoweredEndpointDeletionIntegrityContract {
    pub contract_id: ContractId,
    pub relation_kind_id: KindId,
    pub mode: EndpointDeletionIntegrityMode,
    pub cascade_delete_policy: CascadeDeletePolicy,
    pub plan_revision: RelationIntegrityPlanRevision,
}
