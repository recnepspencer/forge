use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use super::actions::InterpretedCommit;
use super::{
    declared_stage, scopes_overlap, ExpectedAspectDelta, ExpectedProducerDelta,
    ExpectedSealedOriginBinding, ExpectedTrace, ExpectedWorkRecord,
};
use crate::tests::domains::fintech::world::{
    FinancialAspect, FinancialLocalityDefinition, FinancialLocalityFormula,
    FinancialLocalityOutput, FinancialLocalitySubscription, FinancialStructuralMutation,
    LocalitySemanticOutputId,
};

struct DependencyTraceInput<'a> {
    output: &'a FinancialLocalityOutput,
    triggering: &'a [&'a FinancialLocalitySubscription],
    action_ordinal: u32,
    admission_wave: u32,
    outputs: &'a Arc<[FinancialLocalityOutput]>,
    dependency_revisions: &'a Arc<BTreeMap<LocalitySemanticOutputId, u64>>,
    structural_origin: Option<FinancialStructuralMutation>,
    readiness_epoch: u64,
    cause_set_generation: Option<u64>,
}

pub(super) fn trace_commit_group(
    definition: &FinancialLocalityDefinition,
    admission_wave: u32,
    commits: &[InterpretedCommit],
    trace: &mut ExpectedTrace,
) {
    trace.refresh_subscriber_index(&commits[0].outputs);
    let delta_start = trace.deltas.len();
    let mut ordered_commits = commits.iter().collect::<Vec<_>>();
    ordered_commits.sort_by_key(|commit| commit.mutation.publication_order);
    let mut producer_commits = BTreeMap::<LocalitySemanticOutputId, Vec<&InterpretedCommit>>::new();
    for commit in &ordered_commits {
        producer_commits
            .entry(commit.mutation.producer)
            .or_default()
            .push(commit);
    }
    let mut seeded_producers = BTreeSet::new();
    for commit in &ordered_commits {
        if seeded_producers.insert(commit.mutation.producer) {
            seed_source_trace(
                trace,
                admission_wave,
                &producer_commits[&commit.mutation.producer],
            );
        }
    }
    let producers = ordered_commits
        .iter()
        .map(|commit| commit.mutation.producer)
        .collect::<BTreeSet<_>>();
    let last = ordered_commits
        .last()
        .expect("commit group must not be empty");
    if ordered_commits
        .iter()
        .all(|commit| commit.settles_dependencies)
    {
        trace_dependency_outputs(
            definition,
            &last.outputs,
            &last.dependency_revisions,
            &producers,
            last.action_ordinal,
            admission_wave,
            delta_start,
            last.structural_origin,
            trace,
        );
    }
}

fn seed_source_trace(
    trace: &mut ExpectedTrace,
    admission_wave: u32,
    commits: &[&InterpretedCommit],
) {
    let first = commits[0];
    let producer = first.mutation.producer;
    let changes = commits
        .iter()
        .map(|commit| ExpectedAspectDelta {
            aspect: commit.mutation.aspect,
            scope: commit.mutation.scope,
        })
        .collect::<Vec<_>>();
    let mut aspect_versions = changes
        .iter()
        .map(|change| change.aspect)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .map(|aspect| advance_version(trace, producer, aspect));
    let (cached_version, committed_version) = aspect_versions
        .next()
        .expect("a source commit must change at least one aspect");
    assert!(aspect_versions.all(|version| version == (cached_version, committed_version)));
    trace.evaluation_occurrences += 1;
    trace.evaluations.insert(producer);
    let readiness_epoch = trace.allocate_readiness_epoch();
    trace.work_records.push(ExpectedWorkRecord {
        target: producer,
        dependency_revision: first.dependency_revisions[&producer],
        readiness_epoch,
        stage_order: 0,
        sealed_origin: ExpectedSealedOriginBinding::SourceRecompute {
            admission_generation: commits
                .iter()
                .map(|commit| commit.mutation.admission_generation)
                .max()
                .expect("a source commit group is not empty"),
        },
    });
    let output_commit_ordinal = trace.allocate_output_commit_ordinal();
    trace.record_delta(ExpectedProducerDelta {
        action_ordinal: commits
            .iter()
            .map(|commit| commit.action_ordinal)
            .max()
            .expect("a source commit group is not empty"),
        admission_wave,
        producer,
        output_commit_ordinal,
        cached_version,
        committed_version,
        changes,
        outputs: first.outputs.clone(),
        dependency_revisions: first.dependency_revisions.clone(),
        missing_snapshot_consumers: commits
            .iter()
            .filter_map(|commit| commit.structural_origin)
            .map(|structural| structural.target)
            .collect(),
    });
    admit_latest_delta_causes(trace);
}

fn trace_dependency_outputs(
    definition: &FinancialLocalityDefinition,
    outputs: &Arc<[FinancialLocalityOutput]>,
    dependency_revisions: &Arc<BTreeMap<LocalitySemanticOutputId, u64>>,
    producers: &BTreeSet<LocalitySemanticOutputId>,
    action_ordinal: u32,
    admission_wave: u32,
    delta_start: usize,
    structural_origin: Option<FinancialStructuralMutation>,
    trace: &mut ExpectedTrace,
) {
    let mut outputs_by_stage = BTreeMap::<u32, Vec<_>>::new();
    for output in outputs
        .iter()
        .filter(|output| !producers.contains(&output.id))
    {
        let economic_stage = declared_stage(definition, output.id);
        outputs_by_stage
            .entry(economic_stage)
            .or_default()
            .push(output);
    }
    for stage_outputs in outputs_by_stage.into_values() {
        let triggered_outputs = stage_outputs
            .into_iter()
            .filter_map(|output| {
                let triggering = output
                    .subscriptions
                    .iter()
                    .filter(|dependency| {
                        dependency_is_triggered(dependency, &trace.deltas[delta_start..])
                    })
                    .collect::<Vec<_>>();
                let structurally_admitted =
                    structural_origin.is_some_and(|origin| origin.target == output.id);
                (!triggering.is_empty()
                    && (structurally_admitted || trace.has_pending_cause(output.id)))
                .then_some((output, triggering))
            })
            .collect::<Vec<_>>();
        if triggered_outputs.is_empty() {
            continue;
        }
        let readiness_epoch = trace.allocate_readiness_epoch();
        let stage_generations = triggered_outputs
            .iter()
            .filter(|(output, _)| match structural_origin {
                Some(origin) => origin.target != output.id,
                None => true,
            })
            .map(|(output, _)| (output.id, trace.pending_cause_generation(output.id)))
            .collect::<BTreeMap<_, _>>();
        for (output, triggering) in triggered_outputs {
            let is_structural = structural_origin.is_some_and(|origin| origin.target == output.id);
            let cause_set_generation = if is_structural {
                None
            } else {
                Some(stage_generations[&output.id])
            };
            if !is_structural {
                trace.settle_pending_cause(output.id);
            }
            trace_dependency_output(
                DependencyTraceInput {
                    output,
                    triggering: &triggering,
                    action_ordinal,
                    admission_wave,
                    outputs,
                    dependency_revisions,
                    structural_origin,
                    readiness_epoch,
                    cause_set_generation,
                },
                trace,
            );
        }
    }
}

fn trace_dependency_output(input: DependencyTraceInput<'_>, trace: &mut ExpectedTrace) {
    let DependencyTraceInput {
        output,
        triggering,
        action_ordinal,
        admission_wave,
        outputs,
        dependency_revisions,
        structural_origin,
        readiness_epoch,
        cause_set_generation,
    } = input;
    trace.evaluations.insert(output.id);
    trace.evaluation_occurrences += 1;
    let sealed_origin = match structural_origin.filter(|origin| origin.target == output.id) {
        Some(origin) => ExpectedSealedOriginBinding::StructuralRecompute {
            structural_generation: origin.resulting_dependency_revision,
        },
        None => ExpectedSealedOriginBinding::DependencyCommit {
            cause_set_generation: cause_set_generation
                .expect("dependency work must carry an allocated cause-set generation"),
            producer_commit_ordinals: triggering_commit_ordinals(triggering, admission_wave, trace),
        },
    };
    trace.work_records.push(ExpectedWorkRecord {
        target: output.id,
        dependency_revision: dependency_revisions[&output.id],
        readiness_epoch,
        stage_order: 0,
        sealed_origin,
    });
    match output.formula {
        FinancialLocalityFormula::StableControl { .. } => {
            trace.stops.insert(output.id);
            trace.stop_occurrences += 1;
        }
        FinancialLocalityFormula::LinearDependency { .. } => append_changed_output_delta(
            output,
            action_ordinal,
            admission_wave,
            outputs,
            dependency_revisions,
            trace,
        ),
        FinancialLocalityFormula::MarketSource { .. } => {
            panic!("dependency output cannot be a market source")
        }
    }
}

fn triggering_commit_ordinals(
    triggering: &[&FinancialLocalitySubscription],
    admission_wave: u32,
    trace: &ExpectedTrace,
) -> Vec<u64> {
    let mut ordinals = triggering
        .iter()
        .flat_map(|dependency| {
            trace
                .commit_ordinals_by_wave_producer
                .get(&(admission_wave, dependency.upstream))
                .into_iter()
                .flatten()
                .copied()
        })
        .collect::<Vec<_>>();
    ordinals.sort_unstable();
    ordinals.dedup();
    ordinals
}

fn append_changed_output_delta(
    output: &FinancialLocalityOutput,
    action_ordinal: u32,
    admission_wave: u32,
    outputs: &Arc<[FinancialLocalityOutput]>,
    dependency_revisions: &Arc<BTreeMap<LocalitySemanticOutputId, u64>>,
    trace: &mut ExpectedTrace,
) {
    let aspects = output.produced_aspects();
    let mut versions = aspects
        .iter()
        .map(|aspect| advance_version(trace, output.id, *aspect));
    let (cached_version, committed_version) = versions
        .next()
        .expect("every locality output produces an aspect");
    assert!(
        versions.all(|versions| versions == (cached_version, committed_version)),
        "one semantic output commit must advance all produced aspects from the same version"
    );
    let output_commit_ordinal = trace.allocate_output_commit_ordinal();
    trace.record_delta(ExpectedProducerDelta {
        action_ordinal,
        admission_wave,
        producer: output.id,
        output_commit_ordinal,
        cached_version,
        committed_version,
        changes: aspects
            .iter()
            .map(|aspect| ExpectedAspectDelta {
                aspect: *aspect,
                scope: None,
            })
            .collect(),
        outputs: Arc::clone(outputs),
        dependency_revisions: Arc::clone(dependency_revisions),
        missing_snapshot_consumers: BTreeSet::new(),
    });
    admit_latest_delta_causes(trace);
}

fn admit_latest_delta_causes(trace: &mut ExpectedTrace) {
    let delta = trace
        .deltas
        .last()
        .expect("cause admission requires a produced delta");
    let candidates = trace
        .subscribers_by_producer
        .get(&delta.producer)
        .cloned()
        .unwrap_or_default();
    let targets = candidates
        .into_iter()
        .map(|target| &delta.outputs[target.ordinal() as usize])
        .filter(|output| !delta.missing_snapshot_consumers.contains(&output.id))
        .filter(|output| {
            output
                .subscriptions
                .iter()
                .any(|dependency| dependency_is_triggered(dependency, std::slice::from_ref(delta)))
        })
        .map(|output| output.id)
        .collect::<Vec<_>>();
    for target in targets {
        trace.admit_pending_cause(target);
    }
}

fn advance_version(
    trace: &mut ExpectedTrace,
    output: LocalitySemanticOutputId,
    aspect: FinancialAspect,
) -> (u64, u64) {
    let version = trace
        .versions
        .get_mut(&(output, aspect))
        .expect("declared output aspect must have a baseline version");
    let cached = *version;
    *version += 1;
    (cached, *version)
}

fn dependency_is_triggered(
    dependency: &FinancialLocalitySubscription,
    deltas: &[ExpectedProducerDelta],
) -> bool {
    deltas
        .iter()
        .filter(|delta| delta.producer == dependency.upstream)
        .flat_map(|delta| &delta.changes)
        .any(|change| {
            change.aspect == dependency.input_aspect
                && scopes_overlap(dependency.edge_scope, change.scope)
                && dependency
                    .edge_scope
                    .is_none_or(|scope| scopes_overlap(dependency.eligibility_scope, Some(scope)))
        })
}
