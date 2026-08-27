use std::collections::HashMap;

use crate::durability::data::{DurableCheckpoint, DurableStore};
use crate::history::data::{CommitId, PositionedCanonicalCommit};
use crate::runtime::state::subsystems::RuntimeSubsystem;
use crate::runtime::RelationalRuntimeConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CheckpointEnvelopeLocation {
    pub(crate) checkpoint_index: usize,
    pub(crate) envelope_index: usize,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct DurabilitySubsystem {
    pub(crate) log: Vec<PositionedCanonicalCommit>,
    pub(crate) checkpoints: Vec<DurableCheckpoint>,
    pub(crate) log_commit_index: HashMap<CommitId, usize>,
    pub(crate) checkpoint_commit_index: HashMap<CommitId, CheckpointEnvelopeLocation>,
    pub(crate) store: Option<DurableStore>,
    #[cfg(any(test, feature = "test-durability-faults"))]
    pub(crate) fail_next_append: bool,
}

impl DurabilitySubsystem {
    fn build_from_config(config: &RelationalRuntimeConfig) -> Self {
        Self {
            log: Vec::new(),
            checkpoints: Vec::new(),
            log_commit_index: HashMap::new(),
            checkpoint_commit_index: HashMap::new(),
            store: config
                .durability
                .policy
                .store_layout
                .clone()
                .map(|layout| DurableStore {
                    layout,
                    segments: Vec::new(),
                    checkpoints: Vec::new(),
                }),
            #[cfg(any(test, feature = "test-durability-faults"))]
            fail_next_append: false,
        }
    }

    pub(crate) fn push_log_envelope(&mut self, commit: PositionedCanonicalCommit) {
        let commit_id = commit.envelope().commit.commit_id;
        self.log_commit_index.insert(commit_id, self.log.len());
        self.log.push(commit);
    }

    pub(crate) fn push_checkpoint(&mut self, checkpoint: DurableCheckpoint) {
        let checkpoint_index = self.checkpoints.len();
        for (envelope_index, envelope) in checkpoint.envelopes.iter().enumerate() {
            self.checkpoint_commit_index.insert(
                envelope.envelope().commit.commit_id,
                CheckpointEnvelopeLocation {
                    checkpoint_index,
                    envelope_index,
                },
            );
        }
        self.checkpoints.push(checkpoint);
    }

    pub(crate) fn set_log(&mut self, log: Vec<PositionedCanonicalCommit>) {
        self.log = log;
        self.rebuild_log_commit_index();
    }

    pub(crate) fn rebuild_log_commit_index(&mut self) {
        self.log_commit_index.clear();
        for (index, envelope) in self.log.iter().enumerate() {
            self.log_commit_index
                .insert(envelope.envelope().commit.commit_id, index);
        }
    }

    #[cfg(test)]
    pub(crate) fn remove_log_commit(&mut self, commit_id: CommitId) -> bool {
        let before = self.log.len();
        self.log
            .retain(|entry| entry.envelope().commit.commit_id != commit_id);
        let changed = before != self.log.len();
        if changed {
            self.rebuild_log_commit_index();
        }
        changed
    }

    pub(crate) fn trim_log_front(&mut self, count: usize) {
        if count == 0 {
            return;
        }
        self.log.drain(0..count);
        self.rebuild_log_commit_index();
    }

    pub(crate) fn durable_log_envelope(
        &self,
        commit_id: CommitId,
    ) -> Option<&PositionedCanonicalCommit> {
        self.log_commit_index
            .get(&commit_id)
            .and_then(|index| self.log.get(*index))
    }

    pub(crate) fn checkpoint_envelope(
        &self,
        commit_id: CommitId,
    ) -> Option<&PositionedCanonicalCommit> {
        let location = self.checkpoint_commit_index.get(&commit_id)?;
        self.checkpoints
            .get(location.checkpoint_index)
            .and_then(|checkpoint| checkpoint.envelopes.get(location.envelope_index))
    }
}

impl RuntimeSubsystem for DurabilitySubsystem {
    type Config = RelationalRuntimeConfig;

    fn new(config: &Self::Config) -> Self {
        Self::build_from_config(config)
    }

    fn fork(&self) -> Self {
        self.clone()
    }
}
