use std::{
    num::NonZeroU64,
    sync::{
        atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
        Arc, Condvar, Mutex, Weak,
    },
};

use worth_proof::TransitionOutcome;
use worth_store_physical_format::store_namespace::StableStoreIdentity;

use crate::physical_runtime::{
    instance::PhysicalSignalAdmissionStatus, lifecycle::LifecycleState, LifecycleGeneration,
    RuntimeIdentity,
};

use super::{
    command_storage::PhysicalCommandArena,
    observation::{PhysicalWorkAccounting, PhysicalWorkObservationOwner},
    PhysicalMutationWorkRequest, PhysicalOperationIdentity, PhysicalReadWorkRequest,
    PhysicalSignalAspectBindingSet, PhysicalSignalProfileIdentity, PhysicalWorkCapacity,
    PhysicalWorkDeclarationDenial, PhysicalWorkDurabilityRequirement, PhysicalWorkGeneration,
    PhysicalWorkIdentity, PhysicalWorkIntent, PhysicalWorkIntentParts, PhysicalWorkObservation,
    PhysicalWorkOperationFamily, PhysicalWorkRecoveryDisposition, PhysicalWorkShutdownObservation,
    PhysicalWorkTerminalDisposition,
};

mod outcome;
mod reservation;
#[cfg(feature = "certification-test-authority")]
mod yieldpoint;

pub use outcome::{
    PhysicalWorkCapacityDimension, PhysicalWorkSubmissionDeferred, PhysicalWorkSubmissionDenial,
    PhysicalWorkSubmissionFailure, PhysicalWorkSubmissionOutcome, PhysicalWorkSubmissionReceipt,
    PhysicalWorkSubmissionStale,
};
pub(super) use reservation::PhysicalWorkCapacityLease;

#[derive(Clone)]
pub struct PhysicalReadSubmission {
    shared: Weak<PhysicalSubmissionState>,
    generation: LifecycleGeneration,
}

#[derive(Clone)]
pub struct PhysicalMutationSubmission {
    shared: Weak<PhysicalSubmissionState>,
    generation: LifecycleGeneration,
}

pub(in crate::physical_runtime) struct PhysicalWorkSubmissionOwner {
    shared: Arc<PhysicalSubmissionState>,
    observation: PhysicalWorkObservationOwner,
}

pub(in crate::physical_runtime) struct PhysicalWorkSubmissionFoundation {
    pub(in crate::physical_runtime) store: StableStoreIdentity,
    pub(in crate::physical_runtime) runtime: RuntimeIdentity,
    pub(in crate::physical_runtime) generation: LifecycleGeneration,
    pub(in crate::physical_runtime) lifecycle: Arc<LifecycleState>,
    pub(in crate::physical_runtime) signal_profile: PhysicalSignalProfileIdentity,
    pub(in crate::physical_runtime) bindings: Arc<PhysicalSignalAspectBindingSet>,
    pub(in crate::physical_runtime) signal_admission: PhysicalSignalAdmissionStatus,
}

pub(in crate::physical_runtime) enum PhysicalWorkStopKind {
    Close,
    Abort,
    Drop,
}

pub(super) struct PhysicalSubmissionState {
    accepting: AtomicBool,
    terminal_published: AtomicBool,
    store: StableStoreIdentity,
    runtime: RuntimeIdentity,
    generation: LifecycleGeneration,
    lifecycle: Arc<LifecycleState>,
    signal_admission: PhysicalSignalAdmissionStatus,
    signal_profile: PhysicalSignalProfileIdentity,
    bindings: Arc<PhysicalSignalAspectBindingSet>,
    capacity: PhysicalWorkCapacity,
    next_operation: AtomicU64,
    active_submissions: AtomicUsize,
    active_wait: Mutex<()>,
    active_changed: Condvar,
    reserved_commands: AtomicUsize,
    reserved_scope_members: AtomicUsize,
    reserved_semantic_bytes: AtomicUsize,
    commands: PhysicalCommandArena,
    accounting: PhysicalWorkAccounting,
}

impl PhysicalWorkSubmissionOwner {
    pub(in crate::physical_runtime) fn new(
        foundation: PhysicalWorkSubmissionFoundation,
    ) -> Self {
        let capacity = foundation.bindings.capacity();
        Self {
            shared: Arc::new(PhysicalSubmissionState {
                accepting: AtomicBool::new(true),
                terminal_published: AtomicBool::new(false),
                store: foundation.store,
                runtime: foundation.runtime,
                generation: foundation.generation,
                lifecycle: foundation.lifecycle,
                signal_admission: foundation.signal_admission,
                signal_profile: foundation.signal_profile,
                bindings: foundation.bindings,
                capacity,
                next_operation: AtomicU64::new(1),
                active_submissions: AtomicUsize::new(0),
                active_wait: Mutex::new(()),
                active_changed: Condvar::new(),
                reserved_commands: AtomicUsize::new(0),
                reserved_scope_members: AtomicUsize::new(0),
                reserved_semantic_bytes: AtomicUsize::new(0),
                commands: PhysicalCommandArena::bounded(capacity.commands()),
                accounting: PhysicalWorkAccounting::new(),
            }),
            observation: PhysicalWorkObservationOwner::new(),
        }
    }

    pub(in crate::physical_runtime) fn read_submission(&self) -> PhysicalReadSubmission {
        PhysicalReadSubmission {
            shared: Arc::downgrade(&self.shared),
            generation: self.shared.generation,
        }
    }

    pub(in crate::physical_runtime) fn mutation_submission(&self) -> PhysicalMutationSubmission {
        PhysicalMutationSubmission {
            shared: Arc::downgrade(&self.shared),
            generation: self.shared.generation,
        }
    }

    pub(in crate::physical_runtime) fn observation(&self) -> PhysicalWorkObservation {
        self.observation.handle()
    }

    pub(super) fn state(&self) -> &Arc<PhysicalSubmissionState> {
        &self.shared
    }

    #[cfg(feature = "certification-test-authority")]
    pub(in crate::physical_runtime) fn pause_after_command_shard_lock_for_certification(
        &self,
    ) -> yieldpoint::CertificationPhysicalSubmissionPauseGate {
        self.shared
            .commands
            .pause_after_shard_lock_for_certification()
    }

    pub(in crate::physical_runtime) fn stop(
        &self,
        kind: PhysicalWorkStopKind,
    ) -> PhysicalWorkShutdownObservation {
        self.shared.accepting.store(false, Ordering::Release);
        self.shared.await_idle();
        let disposition = match kind {
            PhysicalWorkStopKind::Close => PhysicalWorkTerminalDisposition::ClosedBeforeReadiness,
            PhysicalWorkStopKind::Abort => PhysicalWorkTerminalDisposition::AbortedBeforeReadiness,
            PhysicalWorkStopKind::Drop => PhysicalWorkTerminalDisposition::DroppedBeforeReadiness,
        };
        let drained = self.shared.commands.drain_active();
        for command in &drained {
            if command.release.claim_release() {
                self.shared
                    .release_capacity(command.scope_members, command.semantic_bytes);
            }
        }
        let observation = PhysicalWorkShutdownObservation::from_active(
            self.shared.accounting.declared(),
            self.shared.accounting.safe_pre_effect_terminal(),
            drained
                .into_iter()
                .map(|command| (command.identity, command.stage)),
            disposition,
        );
        if !self.shared.terminal_published.swap(true, Ordering::AcqRel) {
            self.observation.publish(observation.clone());
        }
        observation
    }
}

impl PhysicalSubmissionState {
    pub(super) const fn store(&self) -> StableStoreIdentity {
        self.store
    }

    pub(super) const fn runtime(&self) -> RuntimeIdentity {
        self.runtime
    }

    pub(super) const fn generation(&self) -> LifecycleGeneration {
        self.generation
    }

    pub(super) fn lifecycle_snapshot(
        &self,
    ) -> crate::physical_runtime::lifecycle::LifecycleStateSnapshot {
        self.lifecycle.snapshot()
    }

    pub(super) const fn signal_profile(&self) -> PhysicalSignalProfileIdentity {
        self.signal_profile
    }

    pub(super) fn signal_available(&self) -> bool {
        self.signal_admission.is_available()
    }

    pub(super) fn bindings(&self) -> &PhysicalSignalAspectBindingSet {
        &self.bindings
    }

    pub(super) fn admit_declared(
        self: &Arc<Self>,
        identity: PhysicalWorkIdentity,
    ) -> Option<(PhysicalWorkIntent, PhysicalWorkCapacityLease)> {
        let admitted = self.commands.admit_declared(identity)?;
        let lease =
            PhysicalWorkCapacityLease::new(self, identity, admitted.release);
        Some((admitted.intent, lease))
    }

    fn release_capacity(&self, scope_members: usize, semantic_bytes: usize) {
        self.reserved_semantic_bytes
            .fetch_sub(semantic_bytes, Ordering::AcqRel);
        self.reserved_scope_members
            .fetch_sub(scope_members, Ordering::AcqRel);
        self.reserved_commands.fetch_sub(1, Ordering::AcqRel);
    }
}

impl Drop for PhysicalWorkSubmissionOwner {
    fn drop(&mut self) {
        if self.shared.accepting.load(Ordering::Acquire) {
            let _ = self.stop(PhysicalWorkStopKind::Drop);
        }
    }
}

impl PhysicalReadSubmission {
    pub fn submit(&self, request: PhysicalReadWorkRequest) -> PhysicalWorkSubmissionOutcome {
        submit(
            &self.shared,
            self.generation,
            PhysicalWorkIntentRequest {
                operation: PhysicalWorkOperationFamily::ArtifactRangeRead,
                scope: request.scope,
                semantic_basis: request.semantic_basis,
                security: request.security,
                effect: super::PhysicalWorkEffectClass::ReadOnly,
                durability: PhysicalWorkDurabilityRequirement::ReadOnly,
                recovery: PhysicalWorkRecoveryDisposition::NoEffect,
            },
        )
    }
}

impl PhysicalMutationSubmission {
    pub fn submit(&self, request: PhysicalMutationWorkRequest) -> PhysicalWorkSubmissionOutcome {
        submit(
            &self.shared,
            self.generation,
            PhysicalWorkIntentRequest {
                operation: request.operation,
                scope: request.scope,
                semantic_basis: request.semantic_basis,
                security: request.security,
                effect: request.effect,
                durability: PhysicalWorkDurabilityRequirement::ArtifactRangeWrite(
                    request.durability,
                ),
                recovery: request.recovery,
            },
        )
    }
}

struct PhysicalWorkIntentRequest {
    operation: PhysicalWorkOperationFamily,
    scope: super::PhysicalWorkScope,
    semantic_basis: super::PhysicalWorkSemanticBasis,
    security: worth_store_security::StoreAuthorityBoundSecurityScopeReceipt,
    effect: super::PhysicalWorkEffectClass,
    durability: PhysicalWorkDurabilityRequirement,
    recovery: PhysicalWorkRecoveryDisposition,
}

fn submit(
    weak: &Weak<PhysicalSubmissionState>,
    generation: LifecycleGeneration,
    request: PhysicalWorkIntentRequest,
) -> PhysicalWorkSubmissionOutcome {
    let Some(shared) = weak.upgrade() else {
        return TransitionOutcome::stale(PhysicalWorkSubmissionStale::OwnerReleased).into();
    };
    let _activity = match shared.enter(generation) {
        Ok(activity) => activity,
        Err(stale) => return TransitionOutcome::stale(stale).into(),
    };
    if let Err(denial) = admit_submission_contracts(&shared, &request) {
        return TransitionOutcome::denied(denial).into();
    }
    let scope_members = request.scope.coordinates().len();
    let semantic_bytes = request.semantic_basis.semantic_byte_width();
    let reservation = match shared.reserve(scope_members, semantic_bytes) {
        Ok(reservation) => reservation,
        Err(deferred) => return TransitionOutcome::deferred(deferred).into(),
    };
    let identity = match allocate_operation_identity(&shared) {
        Ok(identity) => identity,
        Err(failure) => return TransitionOutcome::failed(failure).into(),
    };
    let intent = match PhysicalWorkIntent::from_instance_owner(PhysicalWorkIntentParts {
        identity,
        operation: request.operation,
        scope: request.scope,
        semantic_basis: request.semantic_basis,
        security: request.security,
        effect: request.effect,
        durability: request.durability,
        signal_profile: shared.signal_profile,
        recovery: request.recovery,
    }) {
        Ok(intent) => intent,
        Err(denial) => {
            return TransitionOutcome::denied(PhysicalWorkSubmissionDenial::Declaration(denial))
                .into()
        }
    };
    let reservation = reservation.commit();
    shared.commands.push_declared(
        intent,
        reservation.scope_members,
        reservation.semantic_bytes,
    );
    shared.accounting.record_declared();
    TransitionOutcome::success(PhysicalWorkSubmissionReceipt {
        identity,
        signal_profile: shared.signal_profile,
    })
    .into()
}

fn admit_submission_contracts(
    shared: &PhysicalSubmissionState,
    request: &PhysicalWorkIntentRequest,
) -> Result<(), PhysicalWorkSubmissionDenial> {
    if !shared.bindings.admits(
        request.semantic_basis.aspect_identity(),
        request.semantic_basis.binding_stamp(),
    ) {
        return Err(PhysicalWorkSubmissionDenial::SemanticContractNotInstalled);
    }
    if !shared.bindings.admits_security(request.security) {
        return Err(PhysicalWorkSubmissionDenial::SecurityAuthorityMismatch);
    }
    Ok(())
}

fn allocate_operation_identity(
    shared: &PhysicalSubmissionState,
) -> Result<PhysicalWorkIdentity, PhysicalWorkSubmissionFailure> {
    let sequence = shared
        .next_operation
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |value| {
            value.checked_add(1)
        })
        .map_err(|_| PhysicalWorkSubmissionFailure::OperationIdentityExhausted)?;
    let sequence = NonZeroU64::new(sequence)
        .expect("physical operation sequence starts at one and only increments");
    Ok(PhysicalWorkIdentity::from_instance_owner(
        shared.store,
        shared.runtime,
        PhysicalWorkGeneration::from_lifecycle(shared.generation),
        PhysicalOperationIdentity::from_owner_sequence(sequence),
    ))
}

#[cfg(feature = "certification-test-authority")]
pub use yieldpoint::CertificationPhysicalSubmissionPauseGate;
