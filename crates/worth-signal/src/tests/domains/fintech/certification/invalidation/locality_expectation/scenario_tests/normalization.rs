use std::collections::BTreeSet;

use super::{
    ExpectedSealedOriginBinding, FinancialAspect, FinancialLocalityExpectationManifest,
    LocalityScope, LocalitySemanticOutputId,
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum NormalizedOriginShape {
    Source(u64),
    Dependency { generation: u64, producers: usize },
    Structural(u64),
}

pub(super) fn normalized_cause_shape(
    manifest: &FinancialLocalityExpectationManifest,
) -> BTreeSet<(
    LocalitySemanticOutputId,
    u64,
    LocalitySemanticOutputId,
    FinancialAspect,
    Option<LocalityScope>,
    u64,
    u64,
    Vec<LocalityScope>,
)> {
    manifest
        .canonical_causes()
        .iter()
        .map(|cause| {
            (
                cause.consumer,
                cause.dependency_revision,
                cause.producer,
                cause.aspect,
                cause.edge_scope,
                cause.cached_version,
                cause.committed_version,
                cause.changed_scopes.clone(),
            )
        })
        .collect()
}

pub(super) fn normalized_work_shape(
    manifest: &FinancialLocalityExpectationManifest,
) -> BTreeSet<(
    LocalitySemanticOutputId,
    u64,
    u32,
    BTreeSet<NormalizedOriginShape>,
)> {
    manifest
        .canonical_work()
        .iter()
        .map(|(work, origins)| {
            let origins = origins
                .iter()
                .map(|origin| match origin {
                    ExpectedSealedOriginBinding::SourceRecompute {
                        admission_generation,
                    } => NormalizedOriginShape::Source(*admission_generation),
                    ExpectedSealedOriginBinding::DependencyCommit {
                        cause_set_generation,
                        producer_commit_ordinals,
                    } => NormalizedOriginShape::Dependency {
                        generation: *cause_set_generation,
                        producers: producer_commit_ordinals.len(),
                    },
                    ExpectedSealedOriginBinding::StructuralRecompute {
                        structural_generation,
                    } => NormalizedOriginShape::Structural(*structural_generation),
                })
                .collect();
            (
                work.target,
                work.dependency_revision,
                work.stage_order,
                origins,
            )
        })
        .collect()
}
