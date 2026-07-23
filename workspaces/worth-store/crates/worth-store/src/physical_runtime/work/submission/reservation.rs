use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Weak,
};

use crate::physical_runtime::lifecycle::ObservedLifecyclePhase;

use super::{
    PhysicalSubmissionState, PhysicalWorkCapacityDimension, PhysicalWorkSubmissionDeferred,
    PhysicalWorkSubmissionStale,
};

impl PhysicalSubmissionState {
    pub(super) fn enter(
        &self,
        generation: crate::physical_runtime::LifecycleGeneration,
    ) -> Result<SubmissionActivity<'_>, PhysicalWorkSubmissionStale> {
        self.require_admission(generation)?;
        self.active_submissions.fetch_add(1, Ordering::AcqRel);
        if let Err(stale) = self.require_admission(generation) {
            self.leave();
            return Err(stale);
        }
        Ok(SubmissionActivity { state: self })
    }

    fn require_admission(
        &self,
        generation: crate::physical_runtime::LifecycleGeneration,
    ) -> Result<(), PhysicalWorkSubmissionStale> {
        let lifecycle = self.lifecycle.snapshot();
        if lifecycle.generation != generation
            || lifecycle.phase != ObservedLifecyclePhase::RecordServing
        {
            return Err(PhysicalWorkSubmissionStale::LifecycleGenerationAdvanced);
        }
        if !self.accepting.load(Ordering::Acquire) {
            return Err(PhysicalWorkSubmissionStale::AdmissionStopped);
        }
        if !self.signal_admission.is_available() {
            return Err(PhysicalWorkSubmissionStale::SignalOwnerUnavailable);
        }
        Ok(())
    }

    fn leave(&self) {
        if self.active_submissions.fetch_sub(1, Ordering::AcqRel) == 1 {
            self.active_changed.notify_all();
        }
    }

    pub(super) fn await_idle(&self) {
        let mut guard = self
            .active_wait
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        while self.active_submissions.load(Ordering::Acquire) != 0 {
            guard = self
                .active_changed
                .wait(guard)
                .unwrap_or_else(|poisoned| poisoned.into_inner());
        }
    }

    pub(super) fn reserve(
        self: &Arc<Self>,
        scope_members: usize,
        semantic_bytes: usize,
    ) -> Result<SubmissionReservation, PhysicalWorkSubmissionDeferred> {
        if scope_members > self.capacity.scope_members_per_work() {
            return Err(deferred(
                PhysicalWorkCapacityDimension::ScopeMembersPerWork,
                self.capacity.scope_members_per_work(),
            ));
        }
        if semantic_bytes > self.capacity.semantic_bytes_per_work() {
            return Err(deferred(
                PhysicalWorkCapacityDimension::SemanticBytesPerWork,
                self.capacity.semantic_bytes_per_work(),
            ));
        }
        reserve_counter(&self.reserved_commands, 1, self.capacity.commands()).map_err(|_| {
            deferred(
                PhysicalWorkCapacityDimension::Commands,
                self.commands.capacity(),
            )
        })?;
        if reserve_counter(
            &self.reserved_scope_members,
            scope_members,
            self.capacity.total_scope_members(),
        )
        .is_err()
        {
            self.reserved_commands.fetch_sub(1, Ordering::AcqRel);
            return Err(deferred(
                PhysicalWorkCapacityDimension::TotalScopeMembers,
                self.capacity.total_scope_members(),
            ));
        }
        if reserve_counter(
            &self.reserved_semantic_bytes,
            semantic_bytes,
            self.capacity.total_semantic_bytes(),
        )
        .is_err()
        {
            self.reserved_scope_members
                .fetch_sub(scope_members, Ordering::AcqRel);
            self.reserved_commands.fetch_sub(1, Ordering::AcqRel);
            return Err(deferred(
                PhysicalWorkCapacityDimension::TotalSemanticBytes,
                self.capacity.total_semantic_bytes(),
            ));
        }
        Ok(SubmissionReservation {
            state: Arc::clone(self),
            scope_members,
            semantic_bytes,
            committed: false,
        })
    }
}

pub(super) struct SubmissionActivity<'a> {
    state: &'a PhysicalSubmissionState,
}

impl Drop for SubmissionActivity<'_> {
    fn drop(&mut self) {
        self.state.leave();
    }
}

pub(super) struct SubmissionReservation {
    state: Arc<PhysicalSubmissionState>,
    scope_members: usize,
    semantic_bytes: usize,
    committed: bool,
}

impl SubmissionReservation {
    pub(super) fn commit(mut self) -> CommittedSubmissionReservation {
        self.committed = true;
        CommittedSubmissionReservation {
            scope_members: self.scope_members,
            semantic_bytes: self.semantic_bytes,
        }
    }
}

impl Drop for SubmissionReservation {
    fn drop(&mut self) {
        if !self.committed {
            self.state
                .reserved_semantic_bytes
                .fetch_sub(self.semantic_bytes, Ordering::AcqRel);
            self.state
                .reserved_scope_members
                .fetch_sub(self.scope_members, Ordering::AcqRel);
            self.state.reserved_commands.fetch_sub(1, Ordering::AcqRel);
        }
    }
}

pub(super) struct CommittedSubmissionReservation {
    pub(super) scope_members: usize,
    pub(super) semantic_bytes: usize,
}

pub(in crate::physical_runtime::work) struct PhysicalWorkCapacityLease {
    state: Weak<PhysicalSubmissionState>,
    identity: super::PhysicalWorkIdentity,
    release: Arc<super::super::command_storage::PhysicalCommandRelease>,
}

impl PhysicalWorkCapacityLease {
    pub(super) fn new(
        state: &Arc<PhysicalSubmissionState>,
        identity: super::PhysicalWorkIdentity,
        release: Arc<super::super::command_storage::PhysicalCommandRelease>,
    ) -> Self {
        Self {
            state: Arc::downgrade(state),
            identity,
            release,
        }
    }

    pub(in crate::physical_runtime::work) fn mark_stage(
        &self,
        stage: crate::physical_runtime::PhysicalWorkTerminalStage,
    ) {
        if let Some(state) = self.state.upgrade() {
            state.commands.mark_stage(self.identity, stage);
        }
    }
}

impl Drop for PhysicalWorkCapacityLease {
    fn drop(&mut self) {
        if !self.release.claim_release() {
            return;
        }
        let Some(state) = self.state.upgrade() else {
            return;
        };
        if let Some(released) = state.commands.release(self.identity) {
            state.release_capacity(released.scope_members, released.semantic_bytes);
            state.accounting.record_safe_pre_effect_terminal();
        }
    }
}

fn reserve_counter(counter: &AtomicUsize, amount: usize, limit: usize) -> Result<(), ()> {
    counter
        .fetch_update(Ordering::AcqRel, Ordering::Acquire, |current| {
            current.checked_add(amount).filter(|next| *next <= limit)
        })
        .map(|_| ())
        .map_err(|_| ())
}

const fn deferred(
    dimension: PhysicalWorkCapacityDimension,
    capacity: usize,
) -> PhysicalWorkSubmissionDeferred {
    PhysicalWorkSubmissionDeferred {
        dimension,
        capacity,
    }
}
