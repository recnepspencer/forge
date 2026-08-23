use std::collections::{BTreeMap, BTreeSet};

use super::trace::{scopes_overlap, ExpectedAspectDelta, ExpectedTrace};
use super::{
    ExpectedBucketKey, ExpectedCandidateOccurrence, ExpectedDependencyCause,
    ExpectedDependencyDeclaration, ExpectedGraphBinding,
};
use crate::tests::domains::fintech::world::{
    FinancialLocalityDefinition, LocalityScope, LocalitySemanticOutputId,
};

pub(super) struct ExpectedCandidateManifest {
    pub(super) queried_bucket_keys: BTreeSet<ExpectedBucketKey>,
    pub(super) queried_bucket_occurrences: u64,
    pub(super) candidate_dependencies: Vec<ExpectedCandidateOccurrence>,
    pub(super) canonical_causes: BTreeSet<ExpectedDependencyCause>,
    pub(super) admitted_candidate_occurrences: u64,
    pub(super) scope_rejections: u64,
    pub(super) causality_rejections: u64,
}

pub(super) struct ExpectedAdmittedCause {
    pub(super) cause: ExpectedDependencyCause,
}

pub(super) fn derive_candidate_manifest(
    definition: &FinancialLocalityDefinition,
    graph_instance: u64,
    trace: &ExpectedTrace,
) -> ExpectedCandidateManifest {
    let graph = ExpectedGraphBinding {
        graph_instance,
        seed: definition.seed(),
        scale: definition.scale(),
    };
    let mut queried_bucket_keys = BTreeSet::new();
    let mut candidate_dependencies = Vec::new();
    let mut admitted_causes = Vec::new();
    let mut admitted_cause_keys = BTreeSet::new();
    let mut declarations_by_snapshot = BTreeMap::new();
    let mut scope_rejections = 0_u64;
    let mut admitted_candidate_occurrences = 0_u64;
    let mut causality_rejections = 0_u64;
    let mut query_ordinal = 0_u32;
    for delta in &trace.deltas {
        let snapshot_key = delta.outputs.as_ptr() as usize;
        let declarations_by_producer = declarations_by_snapshot
            .entry(snapshot_key)
            .or_insert_with(|| dependency_declarations_by_producer(delta));
        let declarations = declarations_by_producer
            .get(&delta.producer)
            .map(Vec::as_slice)
            .unwrap_or_default();
        for change in &delta.changes {
            for (bucket, membership) in bucket_keys(delta.producer, *change) {
                queried_bucket_keys.insert(bucket);
                if matches!(membership, ExpectedBucketMembership::PartitionAggregate) {
                    queried_bucket_keys.extend(declarations.iter().filter_map(|dependency| {
                        let scope = dependency.edge_scope?;
                        (dependency.producer == bucket.producer
                            && dependency.aspect == bucket.aspect
                            && scope.region == bucket.scope?.region
                            && scope.detail.is_some())
                        .then_some(ExpectedBucketKey {
                            producer: bucket.producer,
                            aspect: bucket.aspect,
                            scope: Some(scope),
                        })
                    }));
                }
                for dependency in declarations.iter().filter(|dependency| {
                    declaration_belongs_to_bucket(dependency, bucket, membership)
                }) {
                    let occurrence = ExpectedCandidateOccurrence {
                        query_ordinal,
                        bucket,
                        dependency: *dependency,
                        output_commit_ordinal: delta.output_commit_ordinal,
                    };
                    if !contract_accepts_normalized_cause(dependency) {
                        scope_rejections += 1;
                    } else if !delta
                        .missing_snapshot_consumers
                        .contains(&dependency.consumer)
                    {
                        admitted_candidate_occurrences += 1;
                        let cause = expected_cause(graph, occurrence, delta);
                        if admitted_cause_keys.insert((delta.admission_wave, cause.clone())) {
                            admitted_causes.push(ExpectedAdmittedCause { cause });
                        }
                    } else {
                        causality_rejections += 1;
                    }
                    candidate_dependencies.push(occurrence);
                }
                query_ordinal += 1;
            }
        }
    }
    let canonical_causes =
        reconcile_current_causes(&admitted_causes, &trace.final_dependency_revisions);
    ExpectedCandidateManifest {
        queried_bucket_keys,
        queried_bucket_occurrences: u64::from(query_ordinal),
        candidate_dependencies,
        canonical_causes,
        admitted_candidate_occurrences,
        scope_rejections,
        causality_rejections,
    }
}

fn contract_accepts_normalized_cause(dependency: &ExpectedDependencyDeclaration) -> bool {
    dependency
        .edge_scope
        .is_none_or(|scope| scopes_overlap(dependency.contract_scope, Some(scope)))
}

fn dependency_declarations_by_producer(
    delta: &super::trace::ExpectedProducerDelta,
) -> BTreeMap<LocalitySemanticOutputId, Vec<ExpectedDependencyDeclaration>> {
    let mut declarations = BTreeMap::<_, Vec<_>>::new();
    for output in delta.outputs.iter() {
        for subscription in &output.subscriptions {
            declarations.entry(subscription.upstream).or_default().push(
                ExpectedDependencyDeclaration {
                    producer: subscription.upstream,
                    consumer: output.id,
                    aspect: subscription.input_aspect,
                    edge_scope: subscription.edge_scope,
                    contract_scope: subscription.eligibility_scope,
                    dependency_revision: delta.dependency_revisions[&output.id],
                },
            );
        }
    }
    declarations
}

#[derive(Clone, Copy)]
enum ExpectedBucketMembership {
    Exact,
    PartitionAggregate,
    WholeAspect,
}

fn bucket_keys(
    producer: LocalitySemanticOutputId,
    change: ExpectedAspectDelta,
) -> Vec<(ExpectedBucketKey, ExpectedBucketMembership)> {
    let key = |scope| ExpectedBucketKey {
        producer,
        aspect: change.aspect,
        scope,
    };
    match change.scope {
        Some(scope) if scope.detail.is_some() => vec![
            (key(None), ExpectedBucketMembership::Exact),
            (
                key(Some(LocalityScope::partition(scope.region))),
                ExpectedBucketMembership::Exact,
            ),
            (key(Some(scope)), ExpectedBucketMembership::Exact),
        ],
        Some(scope) => vec![
            (key(None), ExpectedBucketMembership::Exact),
            (
                key(Some(scope)),
                ExpectedBucketMembership::PartitionAggregate,
            ),
        ],
        None => vec![(key(None), ExpectedBucketMembership::WholeAspect)],
    }
}

fn declaration_belongs_to_bucket(
    declaration: &ExpectedDependencyDeclaration,
    bucket: ExpectedBucketKey,
    membership: ExpectedBucketMembership,
) -> bool {
    if declaration.producer != bucket.producer || declaration.aspect != bucket.aspect {
        return false;
    }
    match membership {
        ExpectedBucketMembership::Exact => declaration.edge_scope == bucket.scope,
        ExpectedBucketMembership::PartitionAggregate => {
            let Some(bucket_scope) = bucket.scope else {
                return false;
            };
            declaration
                .edge_scope
                .is_some_and(|scope| scope.region == bucket_scope.region)
        }
        ExpectedBucketMembership::WholeAspect => true,
    }
}

fn expected_cause(
    graph: ExpectedGraphBinding,
    occurrence: ExpectedCandidateOccurrence,
    delta: &super::trace::ExpectedProducerDelta,
) -> ExpectedDependencyCause {
    let dependency = occurrence.dependency;
    ExpectedDependencyCause {
        graph,
        consumer: dependency.consumer,
        dependency_revision: dependency.dependency_revision,
        producer: dependency.producer,
        aspect: dependency.aspect,
        edge_scope: dependency.edge_scope,
        cached_version: delta.cached_version,
        output_commit_ordinal: occurrence.output_commit_ordinal,
        committed_version: delta.committed_version,
        changed_scopes: dependency.edge_scope.into_iter().collect(),
    }
}

fn reconcile_current_causes(
    admitted: &[ExpectedAdmittedCause],
    current_revisions: &std::collections::BTreeMap<LocalitySemanticOutputId, u64>,
) -> BTreeSet<ExpectedDependencyCause> {
    type CauseKey = (
        LocalitySemanticOutputId,
        super::FinancialAspect,
        Option<LocalityScope>,
    );
    let mut current = BTreeMap::<
        LocalitySemanticOutputId,
        (u64, BTreeMap<CauseKey, ExpectedDependencyCause>),
    >::new();
    for event in admitted {
        let cause = &event.cause;
        let consumer_causes = current
            .entry(cause.consumer)
            .or_insert_with(|| (cause.dependency_revision, BTreeMap::new()));
        if consumer_causes.0 != cause.dependency_revision {
            consumer_causes.0 = cause.dependency_revision;
            consumer_causes.1.clear();
        }
        consumer_causes.1.insert(
            (cause.producer, cause.aspect, cause.edge_scope),
            cause.clone(),
        );
    }
    current
        .into_iter()
        .filter(|(consumer, (revision, _))| current_revisions[consumer] == *revision)
        .flat_map(|(_, (_, causes))| causes.into_values())
        .collect()
}
