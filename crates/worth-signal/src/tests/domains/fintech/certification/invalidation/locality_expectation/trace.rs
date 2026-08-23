use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

mod actions;
mod cause_storage;
mod checkpoints;
mod derivation;
mod indices;
mod structural;
use super::{
    ExpectedCanonicalWork, ExpectedDependencyCause, ExpectedGraphBinding,
    ExpectedSealedOriginBinding, ExpectedWorkIdentity,
};
use crate::tests::domains::fintech::world::{
    FinancialAspect, FinancialLocalityActionTrace, FinancialLocalityDefinition,
    FinancialLocalityOutput, FinancialLocalityScenario, FinancialLocalitySourceObligation,
    FinancialStructuralMutation, LocalityScope, LocalitySemanticOutputId,
};
pub(super) use actions::InterpretedLifecycleEvent;
use actions::{interpret_actions, InterpretedCheckpoint, InterpretedCommit};
use derivation::trace_commit_group;
use structural::apply_structural_trace;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct ExpectedAspectDelta {
    pub(super) aspect: FinancialAspect,
    pub(super) scope: Option<LocalityScope>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ExpectedProducerDelta {
    pub(super) action_ordinal: u32,
    pub(super) admission_wave: u32,
    pub(super) producer: LocalitySemanticOutputId,
    pub(super) output_commit_ordinal: u64,
    pub(super) cached_version: u64,
    pub(super) committed_version: u64,
    pub(super) changes: Vec<ExpectedAspectDelta>,
    pub(super) outputs: Arc<[FinancialLocalityOutput]>,
    pub(super) dependency_revisions: Arc<BTreeMap<LocalitySemanticOutputId, u64>>,
    pub(super) missing_snapshot_consumers: BTreeSet<LocalitySemanticOutputId>,
}

pub(super) struct ExpectedTrace {
    pub(super) evaluations: BTreeSet<LocalitySemanticOutputId>,
    pub(super) stops: BTreeSet<LocalitySemanticOutputId>,
    pub(super) deltas: Vec<ExpectedProducerDelta>,
    work_records: Vec<ExpectedWorkRecord>,
    versions: BTreeMap<(LocalitySemanticOutputId, FinancialAspect), u64>,
    pub(super) lifecycle: Vec<(u32, InterpretedLifecycleEvent)>,
    pub(super) retries: u64,
    pub(super) stale_denials: u64,
    pub(super) topology_revalidations: u64,
    pub(super) rejected_topology_mutations: u64,
    pub(super) final_readiness_epoch: u64,
    pub(super) evaluation_occurrences: u64,
    pub(super) stop_occurrences: u64,
    pub(super) commit_group_count: u64,
    pub(super) final_dependency_revisions: BTreeMap<LocalitySemanticOutputId, u64>,
    checkpoints: Vec<InterpretedCheckpoint>,
    current_source_bases: Vec<FinancialLocalitySourceObligation>,
    next_output_commit_ordinal: u64,
    next_readiness_epoch: u64,
    cause_slot_generations: Vec<u32>,
    free_cause_slots: Vec<u32>,
    cause_store_generation: u32,
    pending_cause_slots: BTreeMap<LocalitySemanticOutputId, (u32, u64)>,
    subscribers_by_producer: BTreeMap<LocalitySemanticOutputId, Vec<LocalitySemanticOutputId>>,
    commit_ordinals_by_wave_producer: BTreeMap<(u32, LocalitySemanticOutputId), Vec<u64>>,
}

#[derive(Clone)]
pub(super) struct ExpectedWorkRecord {
    pub(super) target: LocalitySemanticOutputId,
    pub(super) dependency_revision: u64,
    pub(super) readiness_epoch: u64,
    pub(super) stage_order: u32,
    pub(super) sealed_origin: ExpectedSealedOriginBinding,
}

pub(super) fn derive_expected_trace_for(
    definition: &FinancialLocalityDefinition,
    action_trace: &FinancialLocalityActionTrace,
) -> ExpectedTrace {
    let interpreted = interpret_actions(definition, action_trace);
    let mut trace = empty_trace(definition);
    let commit_groups = group_commits(definition.scenario(), &interpreted.commits);
    trace.commit_group_count = commit_groups.len() as u64;
    let mut timeline = interpreted
        .structural
        .iter()
        .filter(|_| definition.scenario() != FinancialLocalityScenario::PortfolioDependencyChurn)
        .map(|(ordinal, structural)| (*ordinal, TimelineEvent::Structural(*structural)))
        .chain(commit_groups.into_iter().enumerate().map(|(wave, group)| {
            (
                group[0].action_ordinal,
                TimelineEvent::CommitGroup(wave as u32, group),
            )
        }))
        .chain(
            action_trace
                .actions()
                .iter()
                .enumerate()
                .filter_map(|(ordinal, action)| {
                    let crate::tests::domains::fintech::world::FinancialLocalityAction::StagePreRewireWork { binding, .. } = action else {
                        return None;
                    };
                    Some((ordinal as u32, TimelineEvent::StagePreRewire(binding.target)))
                }),
        )
        .collect::<Vec<_>>();
    timeline.sort_by_key(|(ordinal, _)| *ordinal);
    for (_, event) in timeline {
        match event {
            TimelineEvent::Structural(structural) => apply_structural_trace(structural, &mut trace),
            TimelineEvent::CommitGroup(wave, group) => {
                trace_commit_group(definition, wave, &group, &mut trace)
            }
            TimelineEvent::StagePreRewire(target) => trace.stage_pre_rewire_work(target),
        }
    }
    trace.lifecycle = interpreted.lifecycle;
    trace.retries = interpreted.retries;
    trace.stale_denials = interpreted.stale_denials;
    trace.topology_revalidations = interpreted.topology_revalidations;
    trace.rejected_topology_mutations = interpreted.rejected_topology_mutations;
    trace.final_readiness_epoch = interpreted.final_readiness_epoch;
    trace.checkpoints = interpreted.checkpoints;
    trace.current_source_bases = interpreted.current_source_bases;
    trace.final_dependency_revisions = interpreted.final_dependency_revisions;
    trace
}

enum TimelineEvent {
    Structural(FinancialStructuralMutation),
    CommitGroup(u32, Vec<InterpretedCommit>),
    StagePreRewire(LocalitySemanticOutputId),
}

fn group_commits(
    scenario: FinancialLocalityScenario,
    commits: &[InterpretedCommit],
) -> Vec<Vec<InterpretedCommit>> {
    if scenario == FinancialLocalityScenario::PortfolioDependencyChurn {
        return commits.iter().cloned().map(|commit| vec![commit]).collect();
    }
    let mut groups: Vec<Vec<InterpretedCommit>> = Vec::new();
    for commit in commits {
        match groups.last_mut() {
            Some(group)
                if group[0].outputs == commit.outputs
                    && group[0].dependency_revisions == commit.dependency_revisions =>
            {
                group.push(commit.clone())
            }
            _ => groups.push(vec![commit.clone()]),
        }
    }
    groups
}

fn empty_trace(definition: &FinancialLocalityDefinition) -> ExpectedTrace {
    ExpectedTrace {
        evaluations: BTreeSet::new(),
        stops: BTreeSet::new(),
        deltas: Vec::new(),
        work_records: Vec::new(),
        versions: definition
            .outputs()
            .iter()
            .flat_map(|output| {
                output
                    .produced_aspects()
                    .into_iter()
                    .map(move |aspect| ((output.id, aspect), 1))
            })
            .collect(),
        lifecycle: Vec::new(),
        retries: 0,
        stale_denials: 0,
        topology_revalidations: 0,
        rejected_topology_mutations: 0,
        final_readiness_epoch: 1,
        evaluation_occurrences: 0,
        stop_occurrences: 0,
        commit_group_count: 0,
        final_dependency_revisions: definition
            .outputs()
            .iter()
            .map(|output| (output.id, u64::from(!output.subscriptions.is_empty())))
            .collect(),
        checkpoints: Vec::new(),
        current_source_bases: Vec::new(),
        next_output_commit_ordinal: definition.outputs().len() as u64 + 1,
        next_readiness_epoch: definition.workload().release_waves().len() as u64 + 1,
        cause_slot_generations: Vec::new(),
        free_cause_slots: Vec::new(),
        cause_store_generation: 0,
        pending_cause_slots: BTreeMap::new(),
        subscribers_by_producer: BTreeMap::new(),
        commit_ordinals_by_wave_producer: BTreeMap::new(),
    }
}

pub(super) fn scopes_overlap(left: Option<LocalityScope>, right: Option<LocalityScope>) -> bool {
    match (left, right) {
        (None, _) | (_, None) => true,
        (Some(left), Some(right)) if left.region != right.region => false,
        (Some(left), Some(right)) => {
            left.detail.is_none() || right.detail.is_none() || left.detail == right.detail
        }
    }
}

impl ExpectedTrace {
    pub(super) fn canonical_work(
        &self,
        definition: &FinancialLocalityDefinition,
        graph_instance: u64,
    ) -> ExpectedCanonicalWork {
        let graph = ExpectedGraphBinding {
            graph_instance,
            seed: definition.seed(),
            scale: definition.scale(),
        };
        let current_revisions = self.work_records.iter().fold(
            BTreeMap::<LocalitySemanticOutputId, u64>::new(),
            |mut revisions, record| {
                revisions
                    .entry(record.target)
                    .and_modify(|revision| *revision = (*revision).max(record.dependency_revision))
                    .or_insert(record.dependency_revision);
                revisions
            },
        );
        let mut work = ExpectedCanonicalWork::new();
        for record in &self.work_records {
            if current_revisions[&record.target] != record.dependency_revision {
                continue;
            }
            let identity = ExpectedWorkIdentity {
                graph,
                target: record.target,
                dependency_revision: record.dependency_revision,
                readiness_epoch: record.readiness_epoch,
                stage_order: record.stage_order,
            };
            work.entry(identity)
                .or_default()
                .insert(record.sealed_origin.clone());
        }
        work
    }

    pub(super) fn peak_ready_width(&self, _definition: &FinancialLocalityDefinition) -> u64 {
        let mut widths = BTreeMap::<_, BTreeSet<_>>::new();
        for record in &self.work_records {
            widths
                .entry((record.readiness_epoch, record.stage_order))
                .or_default()
                .insert(record.target);
        }
        widths
            .values()
            .map(|targets| targets.len() as u64)
            .max()
            .unwrap_or(0)
    }

    pub(super) fn ready_batch_allocation_count(&self) -> u64 {
        self.work_records
            .iter()
            .map(|record| (record.readiness_epoch, record.stage_order))
            .collect::<BTreeSet<_>>()
            .len() as u64
    }

    pub(super) fn requires_reconstruction(&self) -> bool {
        !self.lifecycle.is_empty()
    }

    pub(super) fn reconstructed_work(
        &self,
        definition: &FinancialLocalityDefinition,
        graph_instance: u64,
        readiness_epoch: u64,
        causes: &BTreeSet<ExpectedDependencyCause>,
    ) -> ExpectedCanonicalWork {
        let graph = ExpectedGraphBinding {
            graph_instance,
            seed: definition.seed(),
            scale: definition.scale(),
        };
        let mut work = ExpectedCanonicalWork::new();
        for basis in &self.current_source_bases {
            let identity = ExpectedWorkIdentity {
                graph,
                target: basis.source,
                dependency_revision: basis.dependency_revision,
                readiness_epoch,
                stage_order: 0,
            };
            work.entry(identity).or_default().insert(
                ExpectedSealedOriginBinding::SourceRecompute {
                    admission_generation: basis.admission_generation,
                },
            );
        }
        let mut cause_groups = BTreeMap::new();
        for cause in causes {
            cause_groups
                .entry((cause.consumer, cause.dependency_revision))
                .or_insert_with(Vec::new)
                .push(cause.output_commit_ordinal);
        }
        for ((target, dependency_revision), mut producer_commit_ordinals) in cause_groups {
            producer_commit_ordinals.sort_unstable();
            producer_commit_ordinals.dedup();
            let identity = ExpectedWorkIdentity {
                graph,
                target,
                dependency_revision,
                readiness_epoch,
                stage_order: 0,
            };
            work.entry(identity).or_default().insert(
                ExpectedSealedOriginBinding::DependencyCommit {
                    cause_set_generation: 0,
                    producer_commit_ordinals,
                },
            );
        }
        work
    }

    pub(super) fn allocate_output_commit_ordinal(&mut self) -> u64 {
        let ordinal = self.next_output_commit_ordinal;
        self.next_output_commit_ordinal += 1;
        ordinal
    }

    pub(super) fn allocate_readiness_epoch(&mut self) -> u64 {
        let epoch = self.next_readiness_epoch;
        self.next_readiness_epoch += 1;
        epoch
    }
}

fn declared_stage(
    definition: &FinancialLocalityDefinition,
    target: LocalitySemanticOutputId,
) -> u32 {
    definition
        .workload()
        .release_waves()
        .iter()
        .position(|wave| wave.contains(&target))
        .expect("every expected target belongs to a declared release wave") as u32
}
