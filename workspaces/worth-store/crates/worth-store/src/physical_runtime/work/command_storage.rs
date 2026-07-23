use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use super::{PhysicalWorkIdentity, PhysicalWorkIntent, PhysicalWorkTerminalStage};

const COMMAND_SHARDS: usize = 32;

pub(super) struct PhysicalCommandArena {
    capacity: usize,
    declared: Box<[Mutex<Vec<PhysicalCommandEntry>>]>,
    #[cfg(feature = "certification-test-authority")]
    certification_shard_gate: Mutex<Option<super::CertificationPhysicalSubmissionPauseGate>>,
}

struct PhysicalCommandEntry {
    identity: PhysicalWorkIdentity,
    intent: Option<PhysicalWorkIntent>,
    scope_members: usize,
    semantic_bytes: usize,
    stage: PhysicalWorkTerminalStage,
    release: Arc<PhysicalCommandRelease>,
}

pub(super) struct PhysicalCommandAdmission {
    pub(super) intent: PhysicalWorkIntent,
    pub(super) release: Arc<PhysicalCommandRelease>,
}

pub(super) struct ReleasedPhysicalCommand {
    pub(super) scope_members: usize,
    pub(super) semantic_bytes: usize,
}

pub(super) struct DrainedPhysicalCommand {
    pub(super) identity: PhysicalWorkIdentity,
    pub(super) scope_members: usize,
    pub(super) semantic_bytes: usize,
    pub(super) stage: PhysicalWorkTerminalStage,
    pub(super) release: Arc<PhysicalCommandRelease>,
}

pub(super) struct PhysicalCommandRelease {
    released: AtomicBool,
}

impl PhysicalCommandRelease {
    fn new() -> Self {
        Self {
            released: AtomicBool::new(false),
        }
    }

    pub(super) fn claim_release(&self) -> bool {
        !self.released.swap(true, Ordering::AcqRel)
    }
}

impl PhysicalCommandArena {
    pub(super) fn bounded(capacity: usize) -> Self {
        Self {
            capacity,
            declared: (0..COMMAND_SHARDS)
                .map(|_| Mutex::new(Vec::new()))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            #[cfg(feature = "certification-test-authority")]
            certification_shard_gate: Mutex::new(None),
        }
    }

    pub(super) const fn capacity(&self) -> usize {
        self.capacity
    }

    pub(super) fn push_declared(
        &self,
        intent: PhysicalWorkIntent,
        scope_members: usize,
        semantic_bytes: usize,
    ) {
        let shard = intent.identity().operation().get() as usize % self.declared.len();
        let mut declared = self.declared[shard]
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        #[cfg(feature = "certification-test-authority")]
        self.pause_after_shard_lock();
        declared.push(PhysicalCommandEntry {
            identity: intent.identity(),
            intent: Some(intent),
            scope_members,
            semantic_bytes,
            stage: PhysicalWorkTerminalStage::Declared,
            release: Arc::new(PhysicalCommandRelease::new()),
        });
    }

    pub(super) fn admit_declared(
        &self,
        identity: PhysicalWorkIdentity,
    ) -> Option<PhysicalCommandAdmission> {
        let shard = identity.operation().get() as usize % self.declared.len();
        let mut declared = self.declared[shard]
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let entry = declared
            .iter_mut()
            .find(|entry| entry.identity == identity)?;
        let intent = entry.intent.take()?;
        Some(PhysicalCommandAdmission {
            intent,
            release: Arc::clone(&entry.release),
        })
    }

    pub(super) fn mark_stage(
        &self,
        identity: PhysicalWorkIdentity,
        stage: PhysicalWorkTerminalStage,
    ) {
        let shard = identity.operation().get() as usize % self.declared.len();
        let mut declared = self.declared[shard]
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(entry) = declared.iter_mut().find(|entry| entry.identity == identity) {
            entry.stage = stage;
        }
    }

    pub(super) fn release(
        &self,
        identity: PhysicalWorkIdentity,
    ) -> Option<ReleasedPhysicalCommand> {
        let shard = identity.operation().get() as usize % self.declared.len();
        let mut declared = self.declared[shard]
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let position = declared.iter().position(|entry| entry.identity == identity)?;
        let entry = declared.swap_remove(position);
        Some(ReleasedPhysicalCommand {
            scope_members: entry.scope_members,
            semantic_bytes: entry.semantic_bytes,
        })
    }

    #[cfg(feature = "certification-test-authority")]
    pub(super) fn pause_after_shard_lock_for_certification(
        &self,
    ) -> super::CertificationPhysicalSubmissionPauseGate {
        let gate = super::CertificationPhysicalSubmissionPauseGate::new();
        *self
            .certification_shard_gate
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(gate.clone());
        gate
    }

    #[cfg(feature = "certification-test-authority")]
    fn pause_after_shard_lock(&self) {
        let gate = self
            .certification_shard_gate
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone();
        if let Some(gate) = gate {
            gate.arrive_and_wait();
        }
    }

    pub(super) fn drain_active(&self) -> Vec<DrainedPhysicalCommand> {
        let mut drained = Vec::new();
        for shard in &self.declared {
            drained.extend(
                shard
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner())
                    .drain(..)
                    .map(|entry| DrainedPhysicalCommand {
                        identity: entry.identity,
                        scope_members: entry.scope_members,
                        semantic_bytes: entry.semantic_bytes,
                        stage: entry.stage,
                        release: entry.release,
                    }),
            );
        }
        drained.sort_by_key(|entry| entry.identity.operation().get());
        drained
    }
}
