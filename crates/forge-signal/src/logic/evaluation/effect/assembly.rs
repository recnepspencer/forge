use crate::data::output::{ChangedRegion, MemoizedResultOrigin, OutputIdentity};
use crate::data::reuse::{ReuseBoundaryContext, ReuseCertificationRecord};
use crate::data::trace::CausalityMetadata;
use crate::logic::prepared::PreparedKeyedContext;

use super::{DiagnosticEnvelope, OperationalEffect};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EvaluationEffect {
    pub operational: OperationalEffect,
    pub diagnostics: Option<DiagnosticEnvelope>,
    pub runtime_metadata: EffectRuntimeMetadata,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct EffectRuntimeMetadata {
    pub memoized_origin: MemoizedResultOrigin,
    pub recomputed: bool,
    pub keyed_context: Option<PreparedKeyedContext>,
    pub causality: Option<CausalityMetadata>,
    pub reuse_certification: Option<ReuseCertificationRecord>,
    pub reuse_boundary_detail: Option<ReuseBoundaryContext>,
}

impl EvaluationEffect {
    pub(crate) fn output_identity(&self) -> Option<&OutputIdentity> {
        self.diagnostics
            .as_ref()
            .and_then(DiagnosticEnvelope::output_identity)
    }

    pub(crate) fn continuity_token(&self) -> Option<&crate::data::output::ArtifactContinuityToken> {
        self.diagnostics
            .as_ref()
            .and_then(DiagnosticEnvelope::continuity_token)
    }

    pub(crate) fn changed_regions(&self) -> &[ChangedRegion] {
        self.diagnostics
            .as_ref()
            .map(DiagnosticEnvelope::changed_regions)
            .unwrap_or(&[])
    }

    pub(crate) fn labels(&self) -> &[String] {
        self.diagnostics
            .as_ref()
            .map(DiagnosticEnvelope::labels)
            .unwrap_or(&[])
    }

    pub(crate) fn memoized_origin(&self) -> MemoizedResultOrigin {
        self.runtime_metadata.memoized_origin
    }

    pub(crate) fn recomputed(&self) -> bool {
        self.runtime_metadata.recomputed
    }

    pub(crate) fn keyed_context(&self) -> Option<&PreparedKeyedContext> {
        self.runtime_metadata.keyed_context.as_ref()
    }

    pub(crate) fn take_causality(&mut self) -> Option<CausalityMetadata> {
        self.runtime_metadata.causality.take()
    }

    pub(crate) fn reuse_certification(&self) -> Option<&ReuseCertificationRecord> {
        self.runtime_metadata.reuse_certification.as_ref()
    }

    pub(crate) fn reuse_boundary_detail(&self) -> Option<&ReuseBoundaryContext> {
        self.runtime_metadata.reuse_boundary_detail.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::data::aspect::AspectVersion;
    use crate::data::dependency::{
        CommittedSnapshotUpdate, DependencySnapshot, ReplacementSnapshotUpdate,
        SharedDependencySnapshot, SnapshotDeltaRecord,
    };
    use crate::data::handle::NodeId;
    use crate::data::output::MemoizedResultOrigin;
    use crate::data::output::OutputChange;
    use crate::data::reuse::{
        ReuseBasis, ReuseBoundaryContext, ReuseOrigin, ReuseSemanticRegionIdentity,
    };
    use crate::logic::evaluation::{EffectRuntimeMetadata, EvaluationEffect, OperationalEffect};

    #[test]
    fn memoized_origin_is_runtime_metadata_not_diagnostic_payload() {
        let mut shape_store = crate::data::dependency::DependencySnapshotShapeStore::default();
        let effect = EvaluationEffect {
            operational: OperationalEffect {
                node: NodeId::new(0, 0),
                verdict: crate::logic::evaluation::EvaluationVerdict::Recomputed,
                aspect_version: AspectVersion::zero(),
                output_change: OutputChange::Replaced,
                reuse_basis: ReuseBasis::fresh_compute(),
                reuse_origin: ReuseOrigin::FreshCompute,
                reuse_boundary_authority: ReuseBoundaryContext {
                    topology_regime: 0,
                    tolerance_regime: crate::data::comparator::VersionComparatorPolicy::Exact,
                    semantic_region: ReuseSemanticRegionIdentity::new(
                        NodeId::new(0, 0),
                        false,
                        Vec::new(),
                        crate::data::node::ContextRequirement::None,
                    ),
                    authority_policy:
                        crate::data::performance::AuthorityPolicy::SpeculativeThenReconcile,
                    artifact_family: None,
                    structural_dependency_basis:
                        crate::data::dependency::DependencySnapshotId::EMPTY,
                    partition_region_basis: crate::data::proof::PartitionScopeSet::default(),
                    strategy_detail: crate::data::reuse::ReuseStrategyBoundaryContext::None,
                }
                .authority(),
                dependency_snapshot_update: CommittedSnapshotUpdate::Replace(
                    ReplacementSnapshotUpdate::from_snapshot(
                        DependencySnapshot::empty(),
                        &mut shape_store,
                    ),
                ),
                snapshot_delta: SnapshotDeltaRecord::between(
                    NodeId::new(0, 0),
                    &DependencySnapshot::empty(),
                    &SharedDependencySnapshot::empty(),
                ),
                meaningful_input_changes: 0,
            },
            diagnostics: None,
            runtime_metadata: EffectRuntimeMetadata {
                memoized_origin: MemoizedResultOrigin::MemoizedFromCache,
                ..EffectRuntimeMetadata::default()
            },
        };

        assert_eq!(
            effect.memoized_origin(),
            MemoizedResultOrigin::MemoizedFromCache
        );
    }
}
