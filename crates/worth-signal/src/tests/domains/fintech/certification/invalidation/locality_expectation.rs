use std::collections::BTreeSet;

use crate::tests::domains::fintech::world::{
    FinancialAspect, FinancialLocalityDefinition, FinancialLocalityScenario, LocalityScope,
    LocalitySemanticOutputId,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::tests::domains::fintech) struct ExpectedBucketKey {
    pub(in crate::tests::domains::fintech) producer: LocalitySemanticOutputId,
    pub(in crate::tests::domains::fintech) aspect: FinancialAspect,
    pub(in crate::tests::domains::fintech) scope: Option<LocalityScope>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::tests::domains::fintech) struct ExpectedDependencyDeclaration {
    pub(in crate::tests::domains::fintech) producer: LocalitySemanticOutputId,
    pub(in crate::tests::domains::fintech) consumer: LocalitySemanticOutputId,
    pub(in crate::tests::domains::fintech) aspect: FinancialAspect,
    pub(in crate::tests::domains::fintech) scope: Option<LocalityScope>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::tests::domains::fintech) struct ExpectedDependencyCause {
    pub(in crate::tests::domains::fintech) dependency: ExpectedDependencyDeclaration,
    pub(in crate::tests::domains::fintech) producer_commit_ordinal: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::tests::domains::fintech) enum ExpectedWorkOrigin {
    SourceRecompute,
    DependencyCommit,
    StructuralRecompute,
}

impl ExpectedWorkOrigin {
    const ALL: [Self; 3] = [
        Self::SourceRecompute,
        Self::DependencyCommit,
        Self::StructuralRecompute,
    ];
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::tests::domains::fintech) struct ExpectedWorkIdentity {
    pub(in crate::tests::domains::fintech) target: LocalitySemanticOutputId,
    pub(in crate::tests::domains::fintech) dependency_revision: u64,
    pub(in crate::tests::domains::fintech) readiness_epoch: u64,
    pub(in crate::tests::domains::fintech) stage_order: u32,
    pub(in crate::tests::domains::fintech) origin: ExpectedWorkOrigin,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct FinancialLocalityExpectationManifest {
    scenario: FinancialLocalityScenario,
    queried_bucket_keys: BTreeSet<ExpectedBucketKey>,
    candidate_dependencies: BTreeSet<ExpectedDependencyDeclaration>,
    canonical_causes: BTreeSet<ExpectedDependencyCause>,
    canonical_work: BTreeSet<ExpectedWorkIdentity>,
    necessary_evaluations: BTreeSet<LocalitySemanticOutputId>,
    unchanged_output_stops: BTreeSet<LocalitySemanticOutputId>,
    peak_ready_width: u64,
}

struct FinancialLocalityExpectationParts {
    scenario: FinancialLocalityScenario,
    queried_bucket_keys: BTreeSet<ExpectedBucketKey>,
    candidate_dependencies: BTreeSet<ExpectedDependencyDeclaration>,
    canonical_causes: BTreeSet<ExpectedDependencyCause>,
    canonical_work: BTreeSet<ExpectedWorkIdentity>,
    necessary_evaluations: BTreeSet<LocalitySemanticOutputId>,
    unchanged_output_stops: BTreeSet<LocalitySemanticOutputId>,
    peak_ready_width: u64,
}

impl FinancialLocalityExpectationManifest {
    pub(in crate::tests::domains::fintech) fn derive(
        definition: &FinancialLocalityDefinition,
    ) -> Self {
        let candidate_dependencies = expected_candidate_dependencies(definition);
        let canonical_causes = expected_canonical_causes(definition, &candidate_dependencies);
        let necessary_evaluations = expected_evaluations(definition);
        let unchanged_output_stops = expected_stops(definition);
        let canonical_work = expected_work(definition, &necessary_evaluations);
        let peak_ready_width = expected_peak_width(definition, &necessary_evaluations);

        Self::from_parts(FinancialLocalityExpectationParts {
            scenario: definition.scenario(),
            queried_bucket_keys: expected_bucket_keys(definition),
            candidate_dependencies,
            canonical_causes,
            canonical_work,
            necessary_evaluations,
            unchanged_output_stops,
            peak_ready_width,
        })
    }

    fn from_parts(parts: FinancialLocalityExpectationParts) -> Self {
        Self {
            scenario: parts.scenario,
            queried_bucket_keys: parts.queried_bucket_keys,
            candidate_dependencies: parts.candidate_dependencies,
            canonical_causes: parts.canonical_causes,
            canonical_work: parts.canonical_work,
            necessary_evaluations: parts.necessary_evaluations,
            unchanged_output_stops: parts.unchanged_output_stops,
            peak_ready_width: parts.peak_ready_width,
        }
    }

    pub(in crate::tests::domains::fintech) const fn scenario(&self) -> FinancialLocalityScenario {
        self.scenario
    }

    pub(in crate::tests::domains::fintech) fn queried_bucket_keys(
        &self,
    ) -> &BTreeSet<ExpectedBucketKey> {
        &self.queried_bucket_keys
    }

    pub(in crate::tests::domains::fintech) fn candidate_dependencies(
        &self,
    ) -> &BTreeSet<ExpectedDependencyDeclaration> {
        &self.candidate_dependencies
    }

    pub(in crate::tests::domains::fintech) fn canonical_causes(
        &self,
    ) -> &BTreeSet<ExpectedDependencyCause> {
        &self.canonical_causes
    }

    pub(in crate::tests::domains::fintech) fn canonical_work(
        &self,
    ) -> &BTreeSet<ExpectedWorkIdentity> {
        &self.canonical_work
    }

    pub(in crate::tests::domains::fintech) fn necessary_evaluations(
        &self,
    ) -> &BTreeSet<LocalitySemanticOutputId> {
        &self.necessary_evaluations
    }

    pub(in crate::tests::domains::fintech) fn unchanged_output_stops(
        &self,
    ) -> &BTreeSet<LocalitySemanticOutputId> {
        &self.unchanged_output_stops
    }

    pub(in crate::tests::domains::fintech) const fn peak_ready_width(&self) -> u64 {
        self.peak_ready_width
    }
}

fn expected_bucket_keys(definition: &FinancialLocalityDefinition) -> BTreeSet<ExpectedBucketKey> {
    let mutation = definition.mutation();
    let mut keys = BTreeSet::from([ExpectedBucketKey {
        producer: mutation.producer,
        aspect: mutation.aspect,
        scope: None,
    }]);
    if let Some(scope) = mutation.scope {
        keys.insert(ExpectedBucketKey {
            producer: mutation.producer,
            aspect: mutation.aspect,
            scope: Some(LocalityScope::partition(scope.region)),
        });
        if scope.detail.is_some() {
            keys.insert(ExpectedBucketKey {
                producer: mutation.producer,
                aspect: mutation.aspect,
                scope: Some(scope),
            });
        }
    }
    keys
}

fn expected_candidate_dependencies(
    definition: &FinancialLocalityDefinition,
) -> BTreeSet<ExpectedDependencyDeclaration> {
    let mutation = definition.mutation();
    definition
        .outputs()
        .iter()
        .flat_map(|output| {
            output.dependencies.iter().filter_map(move |dependency| {
                (dependency.producer == mutation.producer
                    && dependency.aspect == mutation.aspect
                    && scopes_overlap(dependency.edge_scope, mutation.scope))
                .then_some(ExpectedDependencyDeclaration {
                    producer: dependency.producer,
                    consumer: output.id,
                    aspect: dependency.aspect,
                    scope: dependency.edge_scope,
                })
            })
        })
        .collect()
}

fn expected_evaluations(
    definition: &FinancialLocalityDefinition,
) -> BTreeSet<LocalitySemanticOutputId> {
    definition
        .outputs()
        .iter()
        .filter(|output| output.expected_for_mutation)
        .map(|output| output.id)
        .collect()
}

fn expected_canonical_causes(
    definition: &FinancialLocalityDefinition,
    candidates: &BTreeSet<ExpectedDependencyDeclaration>,
) -> BTreeSet<ExpectedDependencyCause> {
    let mutation = definition.mutation();
    candidates
        .iter()
        .filter(|candidate| {
            definition
                .outputs()
                .iter()
                .find(|output| output.id == candidate.consumer)
                .and_then(|output| {
                    output.dependencies.iter().find(|dependency| {
                        dependency.producer == candidate.producer
                            && dependency.aspect == candidate.aspect
                            && dependency.edge_scope == candidate.scope
                    })
                })
                .is_some_and(|dependency| scopes_overlap(dependency.contract_scope, mutation.scope))
        })
        .copied()
        .map(|dependency| ExpectedDependencyCause {
            dependency,
            producer_commit_ordinal: 1,
        })
        .collect()
}

fn expected_stops(definition: &FinancialLocalityDefinition) -> BTreeSet<LocalitySemanticOutputId> {
    definition
        .outputs()
        .iter()
        .filter(|output| output.unchanged_output_stop)
        .map(|output| output.id)
        .collect()
}

fn expected_work(
    definition: &FinancialLocalityDefinition,
    necessary: &BTreeSet<LocalitySemanticOutputId>,
) -> BTreeSet<ExpectedWorkIdentity> {
    let mutation = definition.mutation();
    definition
        .outputs()
        .iter()
        .filter(|output| necessary.contains(&output.id))
        .map(|output| ExpectedWorkIdentity {
            target: output.id,
            dependency_revision: u64::from(!output.dependencies.is_empty()),
            readiness_epoch: 1,
            stage_order: output.id.ordinal(),
            origin: if output.id == mutation.producer {
                ExpectedWorkOrigin::SourceRecompute
            } else {
                ExpectedWorkOrigin::DependencyCommit
            },
        })
        .collect()
}

fn scopes_overlap(left: Option<LocalityScope>, right: Option<LocalityScope>) -> bool {
    match (left, right) {
        (None, _) | (_, None) => true,
        (Some(left), Some(right)) if left.region != right.region => false,
        (Some(left), Some(right)) => {
            left.detail.is_none() || right.detail.is_none() || left.detail == right.detail
        }
    }
}

fn expected_peak_width(
    definition: &FinancialLocalityDefinition,
    necessary: &BTreeSet<LocalitySemanticOutputId>,
) -> u64 {
    let mut depths = std::collections::BTreeMap::new();
    let mut widths = std::collections::BTreeMap::<u32, u64>::new();
    for output in definition
        .outputs()
        .iter()
        .filter(|output| necessary.contains(&output.id))
    {
        let depth = output
            .dependencies
            .iter()
            .filter_map(|dependency| depths.get(&dependency.producer).copied())
            .max()
            .map_or(0, |depth| depth + 1);
        depths.insert(output.id, depth);
        *widths.entry(depth).or_default() += 1;
    }
    widths.into_values().max().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_retains_all_seven_independent_expectation_dimensions() {
        let source = LocalitySemanticOutputId::new(1);
        let target = LocalitySemanticOutputId::new(2);
        let dependency = ExpectedDependencyDeclaration {
            producer: source,
            consumer: target,
            aspect: FinancialAspect::Price,
            scope: None,
        };
        let work = ExpectedWorkIdentity {
            target,
            dependency_revision: 3,
            readiness_epoch: 5,
            stage_order: 7,
            origin: ExpectedWorkOrigin::DependencyCommit,
        };
        let manifest =
            FinancialLocalityExpectationManifest::from_parts(FinancialLocalityExpectationParts {
                scenario: FinancialLocalityScenario::SparseBookFanout,
                queried_bucket_keys: [ExpectedBucketKey {
                    producer: source,
                    aspect: FinancialAspect::Price,
                    scope: None,
                }]
                .into_iter()
                .collect(),
                candidate_dependencies: [dependency].into_iter().collect(),
                canonical_causes: [ExpectedDependencyCause {
                    dependency,
                    producer_commit_ordinal: 11,
                }]
                .into_iter()
                .collect(),
                canonical_work: [work].into_iter().collect(),
                necessary_evaluations: [target].into_iter().collect(),
                unchanged_output_stops: [source].into_iter().collect(),
                peak_ready_width: 1,
            });

        assert_eq!(
            manifest.scenario(),
            FinancialLocalityScenario::SparseBookFanout
        );
        assert_eq!(manifest.queried_bucket_keys().len(), 1);
        assert_eq!(manifest.candidate_dependencies().len(), 1);
        assert_eq!(manifest.canonical_causes().len(), 1);
        assert_eq!(manifest.canonical_work(), &BTreeSet::from([work]));
        assert_eq!(manifest.necessary_evaluations(), &BTreeSet::from([target]));
        assert_eq!(manifest.unchanged_output_stops(), &BTreeSet::from([source]));
        assert_eq!(manifest.peak_ready_width(), 1);
        assert_eq!(ExpectedWorkOrigin::ALL.len(), 3);
    }

    #[test]
    fn independent_manifest_owner_does_not_import_runtime_routing_or_scheduling() {
        let source = include_str!("locality_expectation.rs");
        let forbidden = [
            ["logic", "invalidation", "routing"].join("::"),
            ["logic", "invalidation", "scheduling"].join("::"),
            ["Frontier", "Plan"].concat(),
            ["Invalidation", "ReadyQueue"].concat(),
        ];
        for symbol in forbidden {
            assert!(
                !source.contains(&symbol),
                "oracle imports production symbol {symbol}"
            );
        }
    }
}
