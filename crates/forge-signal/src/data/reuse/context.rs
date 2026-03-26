use serde::{Deserialize, Serialize};

use crate::data::comparator::VersionComparatorPolicy;
use crate::data::core_profile::StableHashValue;
use crate::data::dependency::DependencySnapshotId;
use crate::data::handle::NodeId;
use crate::data::node::ContextRequirement;
use crate::data::output::PartitionSubscription;
use crate::data::performance::AuthorityPolicy;
use crate::data::proof::PartitionScopeSet;

/// Compact runtime evidence needed to certify artifact reuse across semantic boundaries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReuseBoundaryContext {
    pub topology_regime: u32,
    pub tolerance_regime: VersionComparatorPolicy,
    pub semantic_region: ReuseSemanticRegionIdentity,
    pub authority_policy: AuthorityPolicy,
    #[serde(default)]
    pub artifact_family: Option<ArtifactFamilyId>,
    #[serde(default)]
    pub structural_dependency_basis: DependencySnapshotId,
    #[serde(default)]
    pub partition_region_basis: PartitionScopeSet,
    #[serde(default)]
    pub strategy_detail: ReuseStrategyBoundaryContext,
}

/// Compact hot-lane authority for deterministic reuse/replay decisions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ReuseBoundaryAuthority {
    pub topology_regime: u32,
    pub tolerance_regime: VersionComparatorPolicy,
    pub semantic_region_digest: StableHashValue,
    pub authority_policy: AuthorityPolicy,
    #[serde(default)]
    pub artifact_family: Option<ArtifactFamilyId>,
    #[serde(default)]
    pub structural_dependency_basis: DependencySnapshotId,
    #[serde(default)]
    pub partition_region_basis_digest: StableHashValue,
    #[serde(default)]
    pub partition_region_basis_count: u32,
    #[serde(default)]
    pub strategy_detail: ReuseStrategyBoundaryAuthority,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ReuseStrategyBoundaryAuthority {
    #[default]
    None,
    CrossIdentity {
        persistent_correspondence_kind: PersistentCorrespondenceKind,
        persistent_correspondence_digest: StableHashValue,
        persistent_correspondence_valid: bool,
    },
    PartialArtifactSplice {
        composition_region_digest: StableHashValue,
        composition_region_count: u32,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ArtifactFamilyId(String);

impl ArtifactFamilyId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ReuseStrategyBoundaryContext {
    #[default]
    None,
    CrossIdentity {
        persistent_correspondence: PersistentCorrespondenceEvidence,
    },
    PartialArtifactSplice {
        composition_regions: PartitionScopeSet,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PersistentCorrespondenceKind {
    Unknown,
    HostSuppliedKey,
    ContractDeclaredBasis,
    LineageBackedMapping,
    RegionIdentityBasis,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PersistentCorrespondenceEvidence {
    HostSuppliedKey(String),
    ContractDeclaredBasis(String),
    LineageBackedMapping(String),
    RegionIdentityBasis(String),
}

impl PersistentCorrespondenceEvidence {
    pub fn host_supplied_key(value: impl Into<String>) -> Self {
        Self::HostSuppliedKey(value.into())
    }

    pub fn contract_declared_basis(value: impl Into<String>) -> Self {
        Self::ContractDeclaredBasis(value.into())
    }

    pub fn lineage_backed_mapping(value: impl Into<String>) -> Self {
        Self::LineageBackedMapping(value.into())
    }

    pub fn region_identity_basis(value: impl Into<String>) -> Self {
        Self::RegionIdentityBasis(value.into())
    }

    pub fn kind(&self) -> PersistentCorrespondenceKind {
        match self {
            Self::HostSuppliedKey(_) => PersistentCorrespondenceKind::HostSuppliedKey,
            Self::ContractDeclaredBasis(_) => PersistentCorrespondenceKind::ContractDeclaredBasis,
            Self::LineageBackedMapping(_) => PersistentCorrespondenceKind::LineageBackedMapping,
            Self::RegionIdentityBasis(_) => PersistentCorrespondenceKind::RegionIdentityBasis,
        }
    }

    pub fn is_structurally_valid(&self) -> bool {
        match self {
            Self::HostSuppliedKey(value) => !value.trim().is_empty(),
            Self::ContractDeclaredBasis(value) => {
                let trimmed = value.trim();
                trimmed.starts_with("contract:")
                    && trimmed.len() > "contract:".len()
                    && !trimmed.contains('|')
            }
            Self::LineageBackedMapping(value) => {
                let trimmed = value.trim();
                if !trimmed.starts_with("lineage-map:") || trimmed.contains('|') {
                    return false;
                }
                let mapping = &trimmed["lineage-map:".len()..];
                let mut segments = mapping.split("->");
                let Some(left) = segments.next() else {
                    return false;
                };
                let Some(right) = segments.next() else {
                    return false;
                };
                segments.next().is_none() && !left.trim().is_empty() && !right.trim().is_empty()
            }
            Self::RegionIdentityBasis(value) => {
                let trimmed = value.trim();
                trimmed.starts_with("region:")
                    && trimmed.len() > "region:".len()
                    && !trimmed.contains('|')
            }
        }
    }
}

impl ReuseBoundaryContext {
    pub fn persistent_correspondence(&self) -> Option<&PersistentCorrespondenceEvidence> {
        match &self.strategy_detail {
            ReuseStrategyBoundaryContext::CrossIdentity {
                persistent_correspondence,
            } => Some(persistent_correspondence),
            ReuseStrategyBoundaryContext::None
            | ReuseStrategyBoundaryContext::PartialArtifactSplice { .. } => None,
        }
    }

    pub fn composition_regions(&self) -> Option<&PartitionScopeSet> {
        match &self.strategy_detail {
            ReuseStrategyBoundaryContext::PartialArtifactSplice {
                composition_regions,
            } => Some(composition_regions),
            ReuseStrategyBoundaryContext::None
            | ReuseStrategyBoundaryContext::CrossIdentity { .. } => None,
        }
    }

    pub fn authority(&self) -> ReuseBoundaryAuthority {
        ReuseBoundaryAuthority {
            topology_regime: self.topology_regime,
            tolerance_regime: self.tolerance_regime.clone(),
            semantic_region_digest: stable_semantic_region_digest(&self.semantic_region),
            authority_policy: self.authority_policy,
            artifact_family: self.artifact_family.clone(),
            structural_dependency_basis: self.structural_dependency_basis,
            partition_region_basis_digest: stable_partition_scope_digest(
                &self.partition_region_basis,
            ),
            partition_region_basis_count: self.partition_region_basis.len() as u32,
            strategy_detail: match &self.strategy_detail {
                ReuseStrategyBoundaryContext::None => ReuseStrategyBoundaryAuthority::None,
                ReuseStrategyBoundaryContext::CrossIdentity {
                    persistent_correspondence,
                } => ReuseStrategyBoundaryAuthority::CrossIdentity {
                    persistent_correspondence_kind: persistent_correspondence.kind(),
                    persistent_correspondence_digest: stable_persistent_correspondence_digest(
                        persistent_correspondence,
                    ),
                    persistent_correspondence_valid: persistent_correspondence
                        .is_structurally_valid(),
                },
                ReuseStrategyBoundaryContext::PartialArtifactSplice {
                    composition_regions,
                } => ReuseStrategyBoundaryAuthority::PartialArtifactSplice {
                    composition_region_digest: stable_partition_scope_digest(composition_regions),
                    composition_region_count: composition_regions.len() as u32,
                },
            },
        }
    }
}

impl ReuseBoundaryAuthority {
    pub fn persistent_correspondence_kind(&self) -> Option<PersistentCorrespondenceKind> {
        match self.strategy_detail {
            ReuseStrategyBoundaryAuthority::CrossIdentity {
                persistent_correspondence_kind,
                ..
            } => Some(persistent_correspondence_kind),
            ReuseStrategyBoundaryAuthority::None
            | ReuseStrategyBoundaryAuthority::PartialArtifactSplice { .. } => None,
        }
    }

    pub fn composition_region_count(&self) -> u32 {
        match self.strategy_detail {
            ReuseStrategyBoundaryAuthority::PartialArtifactSplice {
                composition_region_count,
                ..
            } => composition_region_count,
            ReuseStrategyBoundaryAuthority::None
            | ReuseStrategyBoundaryAuthority::CrossIdentity { .. } => 0,
        }
    }
}

/// Stable node-local semantic region identity for one artifact family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReuseSemanticRegionIdentity {
    pub node: NodeId,
    pub partitioned_output: bool,
    #[serde(default)]
    pub partition_scope: Vec<PartitionSubscription>,
    #[serde(default)]
    pub required_context: ContextRequirement,
}

impl ReuseSemanticRegionIdentity {
    pub fn new(
        node: NodeId,
        partitioned_output: bool,
        partition_scope: impl Into<Vec<PartitionSubscription>>,
        required_context: ContextRequirement,
    ) -> Self {
        let mut partition_scope = partition_scope.into();
        if partition_scope.len() > 1 {
            partition_scope.sort_unstable();
            partition_scope.dedup();
        }
        Self {
            node,
            partitioned_output,
            partition_scope,
            required_context,
        }
    }
}

fn stable_hash_seed() -> StableHashValue {
    0xcbf29ce484222325_u64 as StableHashValue
}

fn hash_u64(mut hash: StableHashValue, value: u64) -> StableHashValue {
    for byte in value.to_le_bytes() {
        hash ^= byte as StableHashValue;
        hash = hash.wrapping_mul(0x100000001b3_u64 as StableHashValue);
    }
    hash
}

fn hash_bool(hash: StableHashValue, value: bool) -> StableHashValue {
    hash_u64(hash, u64::from(value))
}

fn hash_str(mut hash: StableHashValue, value: &str) -> StableHashValue {
    for byte in value.as_bytes() {
        hash ^= *byte as StableHashValue;
        hash = hash.wrapping_mul(0x100000001b3_u64 as StableHashValue);
    }
    hash
}

fn stable_semantic_region_digest(region: &ReuseSemanticRegionIdentity) -> StableHashValue {
    stable_semantic_region_digest_from_parts(
        region.node,
        region.partitioned_output,
        &region.partition_scope,
        region.required_context,
    )
}

pub(crate) fn stable_semantic_region_digest_from_parts(
    node: NodeId,
    partitioned_output: bool,
    partition_scope: &[PartitionSubscription],
    required_context: ContextRequirement,
) -> StableHashValue {
    let mut hash = stable_hash_seed();
    hash = hash_u64(hash, node.index() as u64);
    hash = hash_u64(hash, node.generation() as u64);
    hash = hash_bool(hash, partitioned_output);
    for scope in partition_scope {
        hash = hash_partition_subscription(hash, scope);
    }
    hash_context_requirement(hash, required_context)
}

fn stable_partition_scope_digest(scopes: &PartitionScopeSet) -> StableHashValue {
    stable_partition_scope_digest_from_slice(scopes.as_slice())
}

pub(crate) fn stable_partition_scope_digest_from_slice(
    scopes: &[PartitionSubscription],
) -> StableHashValue {
    scopes.iter().fold(stable_hash_seed(), |hash, scope| {
        hash_partition_subscription(hash, scope)
    })
}

pub(crate) fn stable_persistent_correspondence_digest(
    evidence: &PersistentCorrespondenceEvidence,
) -> StableHashValue {
    let (tag, value) = match evidence {
        PersistentCorrespondenceEvidence::HostSuppliedKey(value) => (1_u64, value.as_str()),
        PersistentCorrespondenceEvidence::ContractDeclaredBasis(value) => (2_u64, value.as_str()),
        PersistentCorrespondenceEvidence::LineageBackedMapping(value) => (3_u64, value.as_str()),
        PersistentCorrespondenceEvidence::RegionIdentityBasis(value) => (4_u64, value.as_str()),
    };
    hash_str(hash_u64(stable_hash_seed(), tag), value)
}

fn hash_partition_subscription(
    mut hash: StableHashValue,
    scope: &PartitionSubscription,
) -> StableHashValue {
    hash = hash_str(hash, scope.partition.0.as_str());
    hash = hash_str(hash, scope.detail.as_deref().unwrap_or(""));
    hash_u64(hash, scope.match_mode as u64)
}

fn hash_context_requirement(
    hash: StableHashValue,
    requirement: ContextRequirement,
) -> StableHashValue {
    let tag = match requirement {
        ContextRequirement::None => 0_u64,
        ContextRequirement::DomainContext => 1_u64,
        ContextRequirement::RelationalSnapshot => 2_u64,
    };
    hash_u64(hash, tag)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::proof::PartitionScopeSet;

    #[test]
    fn strategy_detail_accessors_are_mutually_exclusive() {
        let cross_identity = ReuseBoundaryContext {
            topology_regime: 1,
            tolerance_regime: VersionComparatorPolicy::Exact,
            semantic_region: ReuseSemanticRegionIdentity::new(
                NodeId::new(1, 0),
                false,
                Vec::new(),
                ContextRequirement::None,
            ),
            authority_policy: AuthorityPolicy::SpeculativeThenReconcile,
            artifact_family: None,
            structural_dependency_basis: crate::data::dependency::DependencySnapshotId::EMPTY,
            partition_region_basis: PartitionScopeSet::default(),
            strategy_detail: ReuseStrategyBoundaryContext::CrossIdentity {
                persistent_correspondence: PersistentCorrespondenceEvidence::host_supplied_key(
                    "mesh-001",
                ),
            },
        };
        assert!(cross_identity.persistent_correspondence().is_some());
        assert!(cross_identity.composition_regions().is_none());

        let partial_splice = ReuseBoundaryContext {
            strategy_detail: ReuseStrategyBoundaryContext::PartialArtifactSplice {
                composition_regions: PartitionScopeSet::new([
                    crate::data::output::PartitionSubscription::whole_partition("wing"),
                ]),
            },
            ..cross_identity.clone()
        };
        assert!(partial_splice.persistent_correspondence().is_none());
        assert_eq!(
            partial_splice
                .composition_regions()
                .map(|regions| regions.len()),
            Some(1)
        );
    }
}
