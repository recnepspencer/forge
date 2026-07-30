use serde::{Deserialize, Serialize};
use worth_ui::facade::query_binding::{
    UiPresentProjection, UiProjectionAvailability, UiProjectionObservation,
    UiProjectionUnavailableKind,
};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseQueryProjectionEvidence {
    projection_identity: String,
    owner_order: u64,
    posture: PlatformPulseQueryProjectionPosture,
    native_value: Option<String>,
    query_world: String,
    binding: String,
    source_generation: String,
    result_generation: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PlatformPulseQueryProjectionPosture {
    Pending,
    Current,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlatformPulseQueryWatcherShutdownEvidence {
    worker_joined: bool,
    pending_observation_count: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlatformPulseLiveQueryResidue {
    source_count: u64,
    attempt_count: u64,
    resource_count: u64,
    consumer_lease_count: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlatformPulseQueryProjectionResidue {
    retained_projection_count: u64,
    projection_receipt_count: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlatformPulseQueryShutdownEvidence {
    watcher: PlatformPulseQueryWatcherShutdownEvidence,
    owner_terminal: bool,
    live: PlatformPulseLiveQueryResidue,
    projection: PlatformPulseQueryProjectionResidue,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseQueryProjectionPublished {
    projection: PlatformPulseQueryProjectionEvidence,
    frame: super::lifecycle::PlatformPulseMountedFrameObservation,
}

impl PlatformPulseQueryWatcherShutdownEvidence {
    pub fn new(worker_joined: bool, pending_observation_count: u64) -> Self {
        Self {
            worker_joined,
            pending_observation_count,
        }
    }

    pub fn worker_joined(self) -> bool {
        self.worker_joined
    }

    pub fn pending_observation_count(self) -> u64 {
        self.pending_observation_count
    }
}

impl PlatformPulseLiveQueryResidue {
    pub fn new(
        source_count: u64,
        attempt_count: u64,
        resource_count: u64,
        consumer_lease_count: u64,
    ) -> Self {
        Self {
            source_count,
            attempt_count,
            resource_count,
            consumer_lease_count,
        }
    }

    pub fn source_count(self) -> u64 {
        self.source_count
    }

    pub fn attempt_count(self) -> u64 {
        self.attempt_count
    }

    pub fn resource_count(self) -> u64 {
        self.resource_count
    }

    pub fn consumer_lease_count(self) -> u64 {
        self.consumer_lease_count
    }
}

impl PlatformPulseQueryProjectionResidue {
    pub fn new(retained_projection_count: u64, projection_receipt_count: u64) -> Self {
        Self {
            retained_projection_count,
            projection_receipt_count,
        }
    }

    pub fn retained_projection_count(self) -> u64 {
        self.retained_projection_count
    }

    pub fn projection_receipt_count(self) -> u64 {
        self.projection_receipt_count
    }
}

impl PlatformPulseQueryShutdownEvidence {
    pub fn new(
        watcher: PlatformPulseQueryWatcherShutdownEvidence,
        owner_terminal: bool,
        live: PlatformPulseLiveQueryResidue,
        projection: PlatformPulseQueryProjectionResidue,
    ) -> Self {
        Self {
            watcher,
            owner_terminal,
            live,
            projection,
        }
    }

    pub fn watcher(self) -> PlatformPulseQueryWatcherShutdownEvidence {
        self.watcher
    }

    pub fn owner_terminal(self) -> bool {
        self.owner_terminal
    }

    pub fn live(self) -> PlatformPulseLiveQueryResidue {
        self.live
    }

    pub fn projection(self) -> PlatformPulseQueryProjectionResidue {
        self.projection
    }
}

impl PlatformPulseQueryProjectionEvidence {
    pub fn from_observation(
        observation: &UiProjectionObservation,
    ) -> Result<Self, super::projection::PlatformPulseLifecycleObservationProjectionDenial> {
        let UiProjectionObservation::Scalar(scalar) = observation else {
            return Err(
                super::projection::PlatformPulseLifecycleObservationProjectionDenial::
                    QueryProjectionUnsupported,
            );
        };
        let fact = scalar.fact();
        let (posture, native_value) = match fact.availability() {
            UiProjectionAvailability::Unavailable(unavailable)
                if unavailable.kind() == UiProjectionUnavailableKind::Pending =>
            {
                (PlatformPulseQueryProjectionPosture::Pending, None)
            }
            UiProjectionAvailability::Present(UiPresentProjection::Current(value)) => (
                PlatformPulseQueryProjectionPosture::Current,
                Some(value.as_str().to_owned()),
            ),
            _ => {
                return Err(
                    super::projection::PlatformPulseLifecycleObservationProjectionDenial::
                        QueryProjectionUnsupported,
                )
            }
        };
        let core = fact.core();
        Ok(Self {
            projection_identity: observation.projection_identity().as_str().to_owned(),
            owner_order: observation.owner_order(),
            posture,
            native_value,
            query_world: core.query_world_identity_for_reporting().to_owned(),
            binding: core.binding_identity_for_reporting().to_owned(),
            source_generation: core.source_generation_for_reporting().to_owned(),
            result_generation: core.result_generation_for_reporting().to_owned(),
        })
    }

    pub fn projection_identity(&self) -> &str {
        &self.projection_identity
    }

    pub fn owner_order(&self) -> u64 {
        self.owner_order
    }

    pub fn posture(&self) -> PlatformPulseQueryProjectionPosture {
        self.posture
    }

    pub fn native_value(&self) -> Option<&str> {
        self.native_value.as_deref()
    }

    pub fn query_world(&self) -> &str {
        &self.query_world
    }

    pub fn binding(&self) -> &str {
        &self.binding
    }

    pub fn source_generation(&self) -> &str {
        &self.source_generation
    }

    pub fn result_generation(&self) -> &str {
        &self.result_generation
    }
}

impl PlatformPulseQueryProjectionPublished {
    pub(super) fn new(
        projection: PlatformPulseQueryProjectionEvidence,
        frame: super::lifecycle::PlatformPulseMountedFrameObservation,
    ) -> Self {
        Self { projection, frame }
    }

    pub fn projection(&self) -> &PlatformPulseQueryProjectionEvidence {
        &self.projection
    }

    pub fn frame(&self) -> super::lifecycle::PlatformPulseMountedFrameObservation {
        self.frame
    }
}
