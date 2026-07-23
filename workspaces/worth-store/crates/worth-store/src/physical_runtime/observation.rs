use std::{fmt, sync::Arc};

use super::{
    availability::InstalledCapabilityStatus,
    lifecycle::{
        LifecycleGeneration, LifecycleState, LifecycleStateSnapshot, ObservedLifecyclePhase,
    },
    resource_lifecycle::ObservationLease,
    PhysicalCapability, RuntimeCounterSnapshot, RuntimeIdentity,
};

/// Cloneable read-only view into one admitted runtime incarnation.
pub struct ObservationHandle {
    runtime_identity: RuntimeIdentity,
    lifecycle: Arc<LifecycleState>,
    observed_generation: LifecycleGeneration,
    lease: ObservationLease,
}

impl ObservationHandle {
    pub(crate) fn new(
        runtime_identity: RuntimeIdentity,
        lifecycle: Arc<LifecycleState>,
        lease: ObservationLease,
    ) -> Self {
        let observed_generation = lifecycle.snapshot().generation;
        Self {
            runtime_identity,
            lifecycle,
            observed_generation,
            lease,
        }
    }

    pub const fn runtime_identity(&self) -> RuntimeIdentity {
        self.runtime_identity
    }

    pub fn snapshot(&self) -> Result<RuntimeObservation, ObservationError> {
        self.lease.counters().record_lifecycle_observation();
        let before = self.require_admitted_generation()?;
        let counters = self.lease.counters().snapshot(before.generation);
        let after = self.lifecycle.snapshot();
        if after != before {
            return Err(self.classify_invalid_observation(after));
        }

        Ok(RuntimeObservation {
            runtime_identity: self.runtime_identity,
            lifecycle: LifecycleObservation::Admitted {
                generation: after.generation,
            },
            root_admission: RootAdmissionObservation::Admitted,
            counters,
        })
    }

    pub fn installed_capabilities(&self) -> Result<InstalledCapabilityStatus, ObservationError> {
        let before = self.require_admitted_generation()?;
        self.lease
            .counters()
            .record_capability_observations(PhysicalCapability::FAMILY_COUNT);
        let after = self.lifecycle.snapshot();
        if after == before {
            Ok(InstalledCapabilityStatus::c3())
        } else {
            Err(self.classify_invalid_observation(after))
        }
    }

    fn require_admitted_generation(&self) -> Result<LifecycleStateSnapshot, ObservationError> {
        let current = self.lifecycle.snapshot();
        if current.phase == ObservedLifecyclePhase::Admitted
            && current.generation == self.observed_generation
        {
            Ok(current)
        } else {
            Err(self.classify_invalid_observation(current))
        }
    }

    fn classify_invalid_observation(&self, current: LifecycleStateSnapshot) -> ObservationError {
        match current.phase {
            ObservedLifecyclePhase::Closed => ObservationError::Closed {
                runtime_identity: self.runtime_identity,
                closed_generation: current.generation,
            },
            ObservedLifecyclePhase::Admitted
            | ObservedLifecyclePhase::MediaOwned
            | ObservedLifecyclePhase::RecordServing
            | ObservedLifecyclePhase::Terminating
            | ObservedLifecyclePhase::Aborted => ObservationError::Stale {
                runtime_identity: self.runtime_identity,
                observed_generation: self.observed_generation,
                current_generation: current.generation,
            },
        }
    }
}

impl Clone for ObservationHandle {
    fn clone(&self) -> Self {
        Self {
            runtime_identity: self.runtime_identity,
            lifecycle: Arc::clone(&self.lifecycle),
            observed_generation: self.observed_generation,
            lease: self.lease.clone(),
        }
    }
}

/// Immutable O(1) snapshot of the lifecycle facts installed by C.3.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeObservation {
    runtime_identity: RuntimeIdentity,
    lifecycle: LifecycleObservation,
    root_admission: RootAdmissionObservation,
    counters: RuntimeCounterSnapshot,
}

impl RuntimeObservation {
    pub const fn runtime_identity(self) -> RuntimeIdentity {
        self.runtime_identity
    }

    pub const fn lifecycle(self) -> LifecycleObservation {
        self.lifecycle
    }

    pub const fn root_admission(self) -> RootAdmissionObservation {
        self.root_admission
    }

    pub const fn counters(self) -> RuntimeCounterSnapshot {
        self.counters
    }
}

/// Immutable fact that this incarnation currently owns its process-local root declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RootAdmissionObservation {
    Admitted,
}

/// Lifecycle fact visible through a valid admitted-runtime observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleObservation {
    Admitted { generation: LifecycleGeneration },
}

impl LifecycleObservation {
    pub const fn generation(self) -> LifecycleGeneration {
        match self {
            Self::Admitted { generation } => generation,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObservationError {
    Closed {
        runtime_identity: RuntimeIdentity,
        closed_generation: LifecycleGeneration,
    },
    Stale {
        runtime_identity: RuntimeIdentity,
        observed_generation: LifecycleGeneration,
        current_generation: LifecycleGeneration,
    },
}

impl fmt::Display for ObservationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Closed {
                runtime_identity,
                closed_generation,
            } => write!(
                formatter,
                "runtime {} closed at lifecycle generation {}",
                runtime_identity.get(),
                closed_generation.get()
            ),
            Self::Stale {
                runtime_identity,
                observed_generation,
                current_generation,
            } => write!(
                formatter,
                "runtime {} observation from lifecycle generation {} is stale at generation {}",
                runtime_identity.get(),
                observed_generation.get(),
                current_generation.get()
            ),
        }
    }
}

impl std::error::Error for ObservationError {}
