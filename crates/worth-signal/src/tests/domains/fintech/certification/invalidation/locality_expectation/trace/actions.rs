use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

mod topology;

use super::super::ExpectedActionCheckpointKind;
use crate::tests::domains::fintech::world::{
    FinancialLocalityAction, FinancialLocalityActionTrace, FinancialLocalityDefinition,
    FinancialLocalityMutation, FinancialLocalityOutput, FinancialLocalitySourceObligation,
    FinancialLocalityStagedWork, FinancialStructuralMutation, LocalitySemanticOutputId,
};

#[derive(Clone)]
pub(super) struct InterpretedCommit {
    pub(super) action_ordinal: u32,
    pub(super) mutation: FinancialLocalityMutation,
    pub(super) outputs: Arc<[FinancialLocalityOutput]>,
    pub(super) dependency_revisions: Arc<BTreeMap<LocalitySemanticOutputId, u64>>,
    pub(super) settles_dependencies: bool,
    pub(super) structural_origin: Option<FinancialStructuralMutation>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech::certification::invalidation::locality_expectation) enum InterpretedLifecycleEvent
{
    CaptureBranch,
    CaptureCheckpoint,
    DestroyDerivedState,
    ReadmitFreshRuntime,
    ReplayCanonicalTrace,
    DeterministicRerun,
}

pub(super) struct InterpretedTrace {
    pub(super) commits: Vec<InterpretedCommit>,
    pub(super) structural: Vec<(u32, FinancialStructuralMutation)>,
    pub(super) lifecycle: Vec<(u32, InterpretedLifecycleEvent)>,
    pub(super) retries: u64,
    pub(super) stale_denials: u64,
    pub(super) topology_revalidations: u64,
    pub(super) rejected_topology_mutations: u64,
    pub(super) final_readiness_epoch: u64,
    pub(super) checkpoints: Vec<InterpretedCheckpoint>,
    pub(super) current_source_bases: Vec<FinancialLocalitySourceObligation>,
    pub(super) final_dependency_revisions: BTreeMap<LocalitySemanticOutputId, u64>,
}

pub(super) struct InterpretedCheckpoint {
    pub(super) action_ordinal: u32,
    pub(super) kind: ExpectedActionCheckpointKind,
    pub(super) runtime_epoch: u64,
    pub(super) persisted_runtime_epoch: u64,
    pub(super) current_source_bases: Vec<FinancialLocalitySourceObligation>,
    pub(super) persisted_source_bases: Vec<FinancialLocalitySourceObligation>,
}

pub(super) fn interpret_actions(
    definition: &FinancialLocalityDefinition,
    trace: &FinancialLocalityActionTrace,
) -> InterpretedTrace {
    let mut state = InterpreterState::new(definition);
    for (action_ordinal, action) in trace.actions().iter().enumerate() {
        let settles_dependencies = !trace
            .actions()
            .get(action_ordinal + 1)
            .is_some_and(|next| matches!(next, FinancialLocalityAction::StagePreRewireWork { .. }));
        state.apply(action_ordinal as u32, *action, settles_dependencies);
    }
    state.finish()
}

struct InterpreterState {
    outputs: Vec<FinancialLocalityOutput>,
    revisions: BTreeMap<LocalitySemanticOutputId, u64>,
    staged: BTreeMap<u16, FinancialLocalityStagedWork>,
    current_source_bases: BTreeMap<LocalitySemanticOutputId, FinancialLocalitySourceObligation>,
    persisted_source_bases: BTreeMap<LocalitySemanticOutputId, FinancialLocalitySourceObligation>,
    commits: Vec<InterpretedCommit>,
    structural: Vec<(u32, FinancialStructuralMutation)>,
    lifecycle: Vec<(u32, InterpretedLifecycleEvent)>,
    retries: u64,
    stale_denials: u64,
    topology_revalidations: u64,
    rejected_topology_mutations: u64,
    readiness_epoch: u64,
    persisted_runtime_epoch: u64,
    lifecycle_stage: u8,
    last_topology_ordinal: u64,
    publication_orders: BTreeSet<u32>,
    checkpoints: Vec<InterpretedCheckpoint>,
    pending_structural: Option<FinancialStructuralMutation>,
}

impl InterpreterState {
    fn new(definition: &FinancialLocalityDefinition) -> Self {
        Self {
            outputs: definition.outputs().to_vec(),
            revisions: definition
                .outputs()
                .iter()
                .map(|output| (output.id, u64::from(!output.subscriptions.is_empty())))
                .collect(),
            staged: BTreeMap::new(),
            current_source_bases: BTreeMap::new(),
            persisted_source_bases: BTreeMap::new(),
            commits: Vec::new(),
            structural: Vec::new(),
            lifecycle: Vec::new(),
            retries: 0,
            stale_denials: 0,
            topology_revalidations: 0,
            rejected_topology_mutations: 0,
            readiness_epoch: 1,
            persisted_runtime_epoch: 1,
            lifecycle_stage: 0,
            last_topology_ordinal: 0,
            publication_orders: BTreeSet::new(),
            checkpoints: Vec::new(),
            pending_structural: None,
        }
    }

    fn apply(
        &mut self,
        action_ordinal: u32,
        action: FinancialLocalityAction,
        settles_dependencies: bool,
    ) {
        match action {
            FinancialLocalityAction::CommitFactor(mutation) => {
                assert!(self.publication_orders.insert(mutation.publication_order));
                self.commits.push(InterpretedCommit {
                    action_ordinal,
                    mutation,
                    outputs: Arc::from(self.outputs.clone()),
                    dependency_revisions: Arc::new(self.revisions.clone()),
                    settles_dependencies,
                    structural_origin: settles_dependencies
                        .then(|| self.pending_structural.take())
                        .flatten(),
                });
            }
            FinancialLocalityAction::RetryAdmission { .. } => self.retries += 1,
            FinancialLocalityAction::StageSourceRecompute { obligation } => {
                assert_eq!(
                    self.revisions[&obligation.source],
                    obligation.dependency_revision
                );
                assert!(self
                    .output(obligation.source)
                    .produced_aspects()
                    .contains(&obligation.aspect));
                self.current_source_bases
                    .entry(obligation.source)
                    .and_modify(|current| {
                        assert_eq!(current.aspect, obligation.aspect);
                        assert_eq!(current.scope, obligation.scope);
                        assert!(current.admission_generation < obligation.admission_generation);
                        *current = obligation;
                    })
                    .or_insert(obligation);
            }
            FinancialLocalityAction::StagePreRewireWork { round, binding } => {
                self.stage_pre_rewire(action_ordinal, round, binding)
            }
            FinancialLocalityAction::AcceptedOwnerMove { round, change } => {
                self.accept_owner_move(action_ordinal, round, change)
            }
            FinancialLocalityAction::RejectStaleWork {
                round,
                stale,
                current_dependency_revision,
            } => self.reject_stale_work(action_ordinal, round, stale, current_dependency_revision),
            FinancialLocalityAction::AcceptedDependencyRemoval {
                round,
                owner,
                removed_subscription,
                structural,
            } => self.accept_dependency_removal(
                action_ordinal,
                round,
                owner,
                removed_subscription,
                structural,
            ),
            FinancialLocalityAction::AcceptedDependencyRecreation {
                round,
                owner,
                subscription,
                structural,
            } => self.accept_dependency_recreation(
                action_ordinal,
                round,
                owner,
                subscription,
                structural,
            ),
            FinancialLocalityAction::RejectedCycle {
                target,
                attempted_subscription,
                attempted_topology_ordinal,
                retained_dependency_revision,
                ..
            } => self.reject_cycle(
                action_ordinal,
                target,
                attempted_subscription,
                attempted_topology_ordinal,
                retained_dependency_revision,
            ),
            FinancialLocalityAction::CaptureBranch { branch_ordinal } => {
                assert_eq!(branch_ordinal, 1);
                self.lifecycle(action_ordinal, 1, InterpretedLifecycleEvent::CaptureBranch);
                self.persisted_source_bases = self.current_source_bases.clone();
                self.persisted_runtime_epoch = self.readiness_epoch;
                self.checkpoint(action_ordinal, ExpectedActionCheckpointKind::BranchCaptured);
            }
            FinancialLocalityAction::CaptureCheckpoint { checkpoint_ordinal } => {
                assert_eq!(checkpoint_ordinal, 1);
                self.lifecycle(
                    action_ordinal,
                    2,
                    InterpretedLifecycleEvent::CaptureCheckpoint,
                );
                assert_eq!(self.persisted_source_bases, self.current_source_bases);
                self.checkpoint(
                    action_ordinal,
                    ExpectedActionCheckpointKind::CheckpointCaptured,
                );
            }
            FinancialLocalityAction::DestroyDerivedState {
                destruction_ordinal,
            } => {
                assert_eq!(destruction_ordinal, 1);
                self.lifecycle(
                    action_ordinal,
                    3,
                    InterpretedLifecycleEvent::DestroyDerivedState,
                );
                self.current_source_bases.clear();
                self.checkpoint(
                    action_ordinal,
                    ExpectedActionCheckpointKind::DerivedStateDestroyed,
                );
            }
            FinancialLocalityAction::ReadmitFreshRuntime { runtime_epoch } => {
                assert_eq!(runtime_epoch, self.readiness_epoch + 1);
                self.readiness_epoch = runtime_epoch;
                self.lifecycle(
                    action_ordinal,
                    4,
                    InterpretedLifecycleEvent::ReadmitFreshRuntime,
                );
                self.current_source_bases = self.persisted_source_bases.clone();
                self.checkpoint(
                    action_ordinal,
                    ExpectedActionCheckpointKind::CausesReadmitted,
                );
            }
            FinancialLocalityAction::ReplayCanonicalTrace { replay_ordinal } => {
                assert_eq!(replay_ordinal, 1);
                self.lifecycle(
                    action_ordinal,
                    5,
                    InterpretedLifecycleEvent::ReplayCanonicalTrace,
                );
                self.checkpoint(
                    action_ordinal,
                    ExpectedActionCheckpointKind::ReadyWorkReconstructed,
                );
            }
            FinancialLocalityAction::DeterministicRerun { rerun_ordinal } => {
                assert_eq!(rerun_ordinal, 1);
                self.lifecycle(
                    action_ordinal,
                    6,
                    InterpretedLifecycleEvent::DeterministicRerun,
                );
                self.checkpoint(
                    action_ordinal,
                    ExpectedActionCheckpointKind::DeterministicRerun,
                );
            }
        }
    }

    fn checkpoint(&mut self, action_ordinal: u32, kind: ExpectedActionCheckpointKind) {
        self.checkpoints.push(InterpretedCheckpoint {
            action_ordinal,
            kind,
            runtime_epoch: self.readiness_epoch,
            persisted_runtime_epoch: self.persisted_runtime_epoch,
            current_source_bases: self.current_source_bases.values().copied().collect(),
            persisted_source_bases: self.persisted_source_bases.values().copied().collect(),
        });
    }

    fn lifecycle(&mut self, ordinal: u32, stage: u8, event: InterpretedLifecycleEvent) {
        assert_eq!(stage, self.lifecycle_stage + 1);
        self.lifecycle_stage = stage;
        self.lifecycle.push((ordinal, event));
    }

    fn output(&self, id: LocalitySemanticOutputId) -> &FinancialLocalityOutput {
        &self.outputs[id.ordinal() as usize]
    }

    fn output_mut(&mut self, id: LocalitySemanticOutputId) -> &mut FinancialLocalityOutput {
        &mut self.outputs[id.ordinal() as usize]
    }

    fn finish(self) -> InterpretedTrace {
        assert!(self.staged.is_empty());
        assert!(self.pending_structural.is_none());
        InterpretedTrace {
            commits: self.commits,
            structural: self.structural,
            lifecycle: self.lifecycle,
            retries: self.retries,
            stale_denials: self.stale_denials,
            topology_revalidations: self.topology_revalidations,
            rejected_topology_mutations: self.rejected_topology_mutations,
            final_readiness_epoch: self.readiness_epoch,
            checkpoints: self.checkpoints,
            current_source_bases: self.current_source_bases.values().copied().collect(),
            final_dependency_revisions: self.revisions,
        }
    }
}
