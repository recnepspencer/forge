use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use super::context::ReuseBoundaryContext;

/// The operational shortcut admitted by prepared/runtime reuse planning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ReuseStrategy {
    #[default]
    OutputSuppression,
    MemoizedArtifactReuse,
    SnapshotRestoreReuse,
    ReconciliationAdoption,
    CrossIdentityPersistentMatch,
    PartialArtifactSplicing,
}

/// The realized runtime outcome after apply/execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ReuseOrigin {
    #[default]
    FreshCompute,
    OutputSuppressed,
    MemoizedArtifactReuse,
    SnapshotRestore,
    ReconciliationAdoption,
    CrossIdentityPersistentReuse,
    PartialArtifactSplice,
}

/// Lowered compact admission packet used on the hot path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ReuseBasis {
    #[serde(default)]
    pub strategy: Option<ReuseStrategy>,
    #[serde(default)]
    pub source: ReuseSource,
    #[serde(default)]
    pub crossing: ReuseCrossing,
    #[serde(default)]
    pub dependency_snapshot_basis: Option<u32>,
    #[serde(default)]
    pub topology_regime_basis: Option<u32>,
    #[serde(default)]
    pub structural_dependency_basis: Option<u32>,
    #[serde(default)]
    pub artifact_family_basis: Option<u32>,
    #[serde(default)]
    pub partition_region_basis_count: u32,
}

impl ReuseBasis {
    pub fn fresh_compute() -> Self {
        Self::default()
    }

    pub fn strategy(strategy: ReuseStrategy, source: ReuseSource, crossing: ReuseCrossing) -> Self {
        Self {
            strategy: Some(strategy),
            source,
            crossing,
            ..Self::default()
        }
    }

    pub fn output_suppression() -> Self {
        Self::strategy(
            ReuseStrategy::OutputSuppression,
            ReuseSource::None,
            ReuseCrossing::None,
        )
    }

    pub fn memoized_artifact_reuse() -> Self {
        Self::strategy(
            ReuseStrategy::MemoizedArtifactReuse,
            ReuseSource::MemoizedArtifact,
            ReuseCrossing::None,
        )
    }

    pub fn snapshot_restore_reuse() -> Self {
        Self::strategy(
            ReuseStrategy::SnapshotRestoreReuse,
            ReuseSource::SnapshotArtifact,
            ReuseCrossing::SnapshotRestore,
        )
    }

    pub fn reconciliation_adoption() -> Self {
        Self::strategy(
            ReuseStrategy::ReconciliationAdoption,
            ReuseSource::AuthorityReconciliation,
            ReuseCrossing::AuthorityBoundary,
        )
    }

    pub fn cross_identity_persistent_match() -> Self {
        Self::strategy(
            ReuseStrategy::CrossIdentityPersistentMatch,
            ReuseSource::PersistentCorrespondence,
            ReuseCrossing::PersistentIdentityBoundary,
        )
    }

    pub fn partial_artifact_splicing() -> Self {
        Self::strategy(
            ReuseStrategy::PartialArtifactSplicing,
            ReuseSource::PartialComposition,
            ReuseCrossing::CompositionBoundary,
        )
    }

    pub fn from_boundary_context(
        strategy: ReuseStrategy,
        source: ReuseSource,
        crossing: ReuseCrossing,
        context: &ReuseBoundaryContext,
    ) -> Self {
        Self {
            strategy: Some(strategy),
            source,
            crossing,
            dependency_snapshot_basis: Some(context.structural_dependency_basis),
            topology_regime_basis: Some(context.topology_regime),
            structural_dependency_basis: Some(context.structural_dependency_basis),
            artifact_family_basis: context.artifact_family.as_ref().map(compact_hash),
            partition_region_basis_count: context.partition_region_basis.len() as u32,
        }
    }

    pub fn is_fresh_compute(&self) -> bool {
        self.strategy.is_none() && self.source == ReuseSource::None
    }
}

fn compact_hash(value: &String) -> u32 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    (hasher.finish() & u32::MAX as u64) as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::comparator::VersionComparatorPolicy;
    use crate::data::handle::NodeId;
    use crate::data::node::ContextRequirement;
    use crate::data::performance::AuthorityPolicy;
    use crate::data::proof::PartitionScopeSet;
    use crate::data::reuse::ReuseSemanticRegionIdentity;

    #[test]
    fn reuse_strategy_and_origin_are_distinct() {
        assert_ne!(
            ReuseOrigin::OutputSuppressed,
            ReuseOrigin::MemoizedArtifactReuse
        );
        assert_eq!(ReuseBasis::fresh_compute().strategy, None);
        assert_eq!(ReuseBasis::fresh_compute().source, ReuseSource::None);
    }

    #[test]
    fn reuse_basis_is_compact_but_not_opaque() {
        let context = ReuseBoundaryContext {
            topology_regime: 11,
            tolerance_regime: VersionComparatorPolicy::Exact,
            semantic_region: ReuseSemanticRegionIdentity::new(
                NodeId::new(0, 0),
                true,
                Vec::new(),
                ContextRequirement::None,
            ),
            authority_policy: AuthorityPolicy::SpeculativeThenReconcile,
            artifact_family: Some("mesh".to_string()),
            structural_dependency_basis: 19,
            partition_region_basis: PartitionScopeSet::default(),
            persistent_correspondence: None,
            composition_regions: PartitionScopeSet::default(),
        };

        let basis = ReuseBasis::from_boundary_context(
            ReuseStrategy::MemoizedArtifactReuse,
            ReuseSource::MemoizedArtifact,
            ReuseCrossing::None,
            &context,
        );

        assert_eq!(basis.strategy, Some(ReuseStrategy::MemoizedArtifactReuse));
        assert_eq!(basis.topology_regime_basis, Some(11));
        assert_eq!(basis.structural_dependency_basis, Some(19));
        assert!(basis.artifact_family_basis.is_some());
    }
}

/// The source lane from which an existing artifact was reused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ReuseSource {
    #[default]
    None,
    MemoizedArtifact,
    SnapshotArtifact,
    AuthorityReconciliation,
    PersistentCorrespondence,
    PartialComposition,
}

/// The runtime boundary crossed, if any, while reusing an artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ReuseCrossing {
    #[default]
    None,
    SnapshotRestore,
    AuthorityBoundary,
    PersistentIdentityBoundary,
    CompositionBoundary,
}
