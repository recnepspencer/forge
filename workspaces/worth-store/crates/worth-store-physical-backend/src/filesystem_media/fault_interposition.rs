use std::sync::OnceLock;
use std::sync::{Arc, Mutex};

use super::{operation_counters::MediaCounterCells, MediaOperationRole, MediaPauseGate};

#[derive(Debug, Clone)]
pub enum MediaFaultDirective {
    FailBefore {
        kind: std::io::ErrorKind,
        raw_os_error: Option<i32>,
    },
    AllowPrefix {
        bytes: u64,
    },
    IndeterminateAfterEffect,
    FailBarrier {
        kind: std::io::ErrorKind,
        raw_os_error: Option<i32>,
    },
    PauseBefore(MediaPauseGate),
    PauseAfter(MediaPauseGate),
    InterruptReplacementObservation,
}

#[derive(Debug, Clone)]
pub struct MediaFaultRule {
    role: MediaOperationRole,
    ordinal: u64,
    directive: MediaFaultDirective,
    owner: Option<super::MediaOwnerIdentity>,
    store: Option<worth_store_physical_format::store_namespace::StableStoreIdentity>,
    operation: Option<super::MediaOperationIdentity>,
    runtime_incarnation: Option<u64>,
}

impl MediaFaultRule {
    #[cfg(any(test, feature = "certification-test-authority"))]
    pub(crate) fn for_certification(
        role: MediaOperationRole,
        ordinal: u64,
        directive: MediaFaultDirective,
    ) -> Self {
        Self {
            role,
            ordinal,
            directive,
            owner: None,
            store: None,
            operation: None,
            runtime_incarnation: None,
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub fn for_owner(mut self, owner: super::MediaOwnerIdentity) -> Self {
        self.owner = Some(owner);
        self
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub fn for_store(
        mut self,
        store: worth_store_physical_format::store_namespace::StableStoreIdentity,
    ) -> Self {
        self.store = Some(store);
        self
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub fn for_operation(mut self, operation: super::MediaOperationIdentity) -> Self {
        self.operation = Some(operation);
        self
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub fn for_runtime_incarnation(mut self, runtime_incarnation: u64) -> Self {
        self.runtime_incarnation = Some(runtime_incarnation);
        self
    }

    fn matches(&self, context: super::MediaOperationContext) -> bool {
        self.role == context.role()
            && self.ordinal == context.role_ordinal()
            && self
                .owner
                .is_none_or(|owner| context.owner() == Some(owner))
            && self
                .store
                .is_none_or(|store| context.store() == Some(store))
            && self
                .operation
                .is_none_or(|operation| context.operation() == Some(operation))
            && self
                .runtime_incarnation
                .is_none_or(|runtime| context.runtime_incarnation() == Some(runtime))
    }
}

#[derive(Debug, Clone, Default)]
pub struct MediaFaultSchedule {
    rules: Arc<[MediaFaultRule]>,
    lease_release_pause: Option<MediaPauseGate>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaFaultScheduleDenial {
    ZeroOrdinal,
    DuplicateSemanticMatch,
    DirectiveRoleMismatch,
}

impl MediaFaultSchedule {
    #[cfg(any(test, feature = "certification-test-authority"))]
    pub(crate) fn for_certification(
        rules: Vec<MediaFaultRule>,
    ) -> Result<Self, MediaFaultScheduleDenial> {
        for (index, rule) in rules.iter().enumerate() {
            if rule.ordinal == 0 {
                return Err(MediaFaultScheduleDenial::ZeroOrdinal);
            }
            if !super::fault_schedule_validation::directive_matches_role(rule.role, &rule.directive)
            {
                return Err(MediaFaultScheduleDenial::DirectiveRoleMismatch);
            }
            if rules[..index].iter().any(|prior| {
                prior.role == rule.role
                    && prior.ordinal == rule.ordinal
                    && prior.owner == rule.owner
                    && prior.store == rule.store
                    && prior.operation == rule.operation
                    && prior.runtime_incarnation == rule.runtime_incarnation
            }) {
                return Err(MediaFaultScheduleDenial::DuplicateSemanticMatch);
            }
        }
        Ok(Self {
            rules: rules.into(),
            lease_release_pause: None,
        })
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub fn pause_before_lease_release(mut self, gate: MediaPauseGate) -> Self {
        self.lease_release_pause = Some(gate);
        self
    }
}

#[derive(Debug)]
pub(super) struct MediaFaultInterposer {
    schedule: MediaFaultSchedule,
    counters: Arc<MediaCounterCells>,
    role_ordinals: Mutex<[u64; MediaOperationRole::ALL.len()]>,
    owner: OnceLock<super::MediaOwnerIdentity>,
    runtime_incarnation: OnceLock<u64>,
    store: OnceLock<worth_store_physical_format::store_namespace::StableStoreIdentity>,
}

impl MediaFaultInterposer {
    pub(super) fn new(schedule: MediaFaultSchedule, counters: Arc<MediaCounterCells>) -> Self {
        Self {
            schedule,
            counters,
            role_ordinals: Mutex::new([0; MediaOperationRole::ALL.len()]),
            owner: OnceLock::new(),
            runtime_incarnation: OnceLock::new(),
            store: OnceLock::new(),
        }
    }

    pub(super) fn begin(
        &self,
        role: MediaOperationRole,
        requested_bytes: u64,
    ) -> MediaBoundaryAttempt<'_> {
        self.begin_operation(
            role,
            requested_bytes,
            super::MediaOperationCoordinates::unbound(),
        )
    }

    pub(super) fn begin_operation(
        &self,
        role: MediaOperationRole,
        requested_bytes: u64,
        coordinates: super::MediaOperationCoordinates,
    ) -> MediaBoundaryAttempt<'_> {
        self.counters.begin(role, requested_bytes);
        let ordinal = {
            let mut ordinals = self
                .role_ordinals
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            let slot = &mut ordinals[role.index()];
            *slot = slot.saturating_add(1);
            *slot
        };
        let context = super::MediaOperationContext::new(
            super::MediaOperationIdentityBinding {
                owner: self.owner.get().copied(),
                runtime_incarnation: self.runtime_incarnation.get().copied(),
                store: self.store.get().copied(),
            },
            role,
            requested_bytes,
            coordinates,
            ordinal,
        );
        let directive = self
            .schedule
            .rules
            .iter()
            .find(|rule| rule.matches(context))
            .map(|rule| rule.directive.clone());
        if directive.is_some() {
            self.counters.record_fault_match(context);
        }
        if let Some(MediaFaultDirective::PauseBefore(gate)) = &directive {
            gate.pause(Some(context));
        }
        MediaBoundaryAttempt {
            interposer: self,
            context,
            directive,
            terminal: false,
        }
    }

    pub(super) fn bind_runtime_incarnation(&self, runtime_incarnation: u64) {
        let _ = self.runtime_incarnation.set(runtime_incarnation);
    }

    pub(super) fn bind_owner(&self, owner: super::MediaOwnerIdentity) {
        let _ = self.owner.set(owner);
    }

    pub(super) fn bind_store(
        &self,
        store: worth_store_physical_format::store_namespace::StableStoreIdentity,
    ) {
        let _ = self.store.set(store);
    }

    pub(super) fn counters(&self) -> &MediaCounterCells {
        &self.counters
    }

    pub(super) fn shared_counters(&self) -> &Arc<MediaCounterCells> {
        &self.counters
    }

    pub(super) fn begin_lease_release(&self) -> MediaBoundaryAttempt<'_> {
        let attempt = self.begin(MediaOperationRole::ReleaseMutationLease, 0);
        if let Some(gate) = &self.schedule.lease_release_pause {
            gate.pause(Some(attempt.context));
        }
        attempt
    }
}

pub(super) struct MediaBoundaryAttempt<'boundary> {
    interposer: &'boundary MediaFaultInterposer,
    context: super::MediaOperationContext,
    directive: Option<MediaFaultDirective>,
    terminal: bool,
}

impl MediaBoundaryAttempt<'_> {
    pub(super) fn fail_before_error(&self) -> Option<std::io::Error> {
        match &self.directive {
            Some(MediaFaultDirective::FailBefore { kind, raw_os_error }) => {
                Some(injected_error(*kind, *raw_os_error))
            }
            _ => None,
        }
    }

    pub(super) fn barrier_error(&self) -> Option<std::io::Error> {
        match &self.directive {
            Some(MediaFaultDirective::FailBarrier { kind, raw_os_error }) => {
                Some(injected_error(*kind, *raw_os_error))
            }
            _ => None,
        }
    }

    pub(super) fn transfer_limit(&self, requested: u64) -> u64 {
        match &self.directive {
            Some(MediaFaultDirective::AllowPrefix { bytes }) => requested.min(*bytes),
            _ => requested,
        }
    }

    pub(super) fn effect_observation_is_indeterminate(&self) -> bool {
        matches!(
            self.directive,
            Some(MediaFaultDirective::IndeterminateAfterEffect)
                | Some(MediaFaultDirective::InterruptReplacementObservation)
        )
    }

    pub(super) fn completed(mut self, completed_bytes: u64) {
        self.after_boundary();
        self.record_fault_terminal(super::MediaCounterTerminal::Completed, completed_bytes);
        self.interposer
            .counters
            .completed(self.context.role(), completed_bytes);
        self.terminal = true;
    }

    pub(super) fn denied(mut self) {
        self.after_boundary();
        self.record_fault_terminal(super::MediaCounterTerminal::DeniedBeforeEffect, 0);
        self.interposer.counters.denied(self.context.role());
        self.terminal = true;
    }

    pub(super) fn confinement_denied(mut self) {
        self.after_boundary();
        self.record_fault_terminal(super::MediaCounterTerminal::DeniedBeforeEffect, 0);
        self.interposer
            .counters
            .confinement_denied(self.context.role());
        self.terminal = true;
    }

    pub(super) fn stale_handle_denied(mut self) {
        self.after_boundary();
        self.record_fault_terminal(super::MediaCounterTerminal::DeniedBeforeEffect, 0);
        self.interposer
            .counters
            .stale_handle_denied(self.context.role());
        self.terminal = true;
    }

    pub(super) fn unsupported_capability(mut self) {
        self.after_boundary();
        self.record_fault_terminal(super::MediaCounterTerminal::DeniedBeforeEffect, 0);
        self.interposer
            .counters
            .unsupported_capability(self.context.role());
        self.terminal = true;
    }

    pub(super) fn partial(mut self, completed_bytes: u64) {
        self.after_boundary();
        self.record_fault_terminal(super::MediaCounterTerminal::PartialEffect, completed_bytes);
        self.interposer
            .counters
            .partial(self.context.role(), completed_bytes);
        self.terminal = true;
    }

    pub(super) fn indeterminate(mut self, completed_bytes: u64) {
        self.after_boundary();
        self.record_fault_terminal(
            super::MediaCounterTerminal::IndeterminateEffect,
            completed_bytes,
        );
        self.interposer
            .counters
            .indeterminate(self.context.role(), completed_bytes);
        self.terminal = true;
    }

    fn after_boundary(&self) {
        if let Some(MediaFaultDirective::PauseAfter(gate)) = &self.directive {
            gate.pause(Some(self.context));
        }
    }

    fn record_fault_terminal(&self, terminal: super::MediaCounterTerminal, completed_bytes: u64) {
        if self.directive.is_some() {
            self.interposer
                .counters
                .record_fault_terminal(self.context, terminal, completed_bytes);
        }
    }
}

fn injected_error(kind: std::io::ErrorKind, raw_os_error: Option<i32>) -> std::io::Error {
    raw_os_error
        .map(std::io::Error::from_raw_os_error)
        .unwrap_or_else(|| std::io::Error::from(kind))
}

impl Drop for MediaBoundaryAttempt<'_> {
    fn drop(&mut self) {
        if !self.terminal {
            self.record_fault_terminal(super::MediaCounterTerminal::IndeterminateEffect, 0);
            self.interposer
                .counters
                .indeterminate(self.context.role(), 0);
        }
    }
}
