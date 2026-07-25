use std::sync::{Arc, Weak};

use super::{PhysicalEffectActivity, PhysicalSubmissionState};

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

    pub(in crate::physical_runtime::work) fn begin_dispatch(
        &self,
    ) -> Option<PhysicalEffectActivity> {
        self.state
            .upgrade()
            .and_then(|state| state.begin_effect(self.identity))
    }

    pub(in crate::physical_runtime::work) fn mark_pressure(
        &self,
        pressure: crate::physical_runtime::PhysicalWorkPressureClass,
    ) -> bool {
        self.state
            .upgrade()
            .is_some_and(|state| state.commands.mark_pressure(self.identity, pressure))
    }

    pub(in crate::physical_runtime::work) fn bind_signal(
        &self,
        signal_request: worth_signal::facade::ResourceRequestHandle,
        route: crate::physical_runtime::PhysicalSignalAspectBindingDigest,
        superseded: Option<worth_signal::facade::ResourceRequestHandle>,
    ) -> bool {
        self.state.upgrade().is_some_and(|state| {
            state
                .commands
                .bind_signal(self.identity, signal_request, route, superseded)
        })
    }

    pub(in crate::physical_runtime::work) fn register_signal_locality(
        &self,
        route: crate::physical_runtime::PhysicalSignalAspectBindingDigest,
    ) -> bool {
        self.state.upgrade().is_some_and(|state| {
            state
                .commands
                .register_signal_locality(self.identity, route)
        })
    }

    pub(in crate::physical_runtime::work) fn release_settled(
        &self,
        fate: crate::physical_runtime::PhysicalWorkEffectFate,
        recovery: crate::physical_runtime::PhysicalWorkRecoveryDisposition,
    ) {
        if !self.release.claim_release() {
            return;
        }
        let Some(state) = self.state.upgrade() else {
            return;
        };
        if let Some(released) = state.commands.release(self.identity) {
            state.release_capacity(released.scope_members, released.semantic_bytes);
            state
                .accounting
                .record_terminal(released.operation, released.pressure);
            state
                .terminal_ledger
                .record(super::super::PhysicalWorkTerminalEvent::Settled {
                    identity: self.identity,
                    fate,
                    recovery,
                    consumer_cancelled: released.consumer_cancelled,
                });
        }
    }

    pub(in crate::physical_runtime::work) fn mark_retry_pending(&self) {
        if let Some(state) = self.state.upgrade() {
            state.commands.mark_retry_pending(self.identity);
        }
    }

    pub(in crate::physical_runtime::work) fn is_cancelled(&self) -> bool {
        self.release.is_cancelled()
    }
}

impl Drop for PhysicalWorkCapacityLease {
    fn drop(&mut self) {
        let Some(state) = self.state.upgrade() else {
            return;
        };
        let Some(registration) = state.commands.signal_registration(self.identity) else {
            if self.release.claim_release() {
                state.release_abandoned(self.identity, false);
            }
            return;
        };
        if !self.release.claim_abandonment() {
            return;
        }
        if !state.abandonment.publish(
            &state,
            self.identity,
            registration,
            Arc::clone(&self.release),
        ) {
            state.signal_admission.revoke();
        }
    }
}
