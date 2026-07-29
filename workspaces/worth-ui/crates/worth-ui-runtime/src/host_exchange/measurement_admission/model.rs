use worth_ui_host_contract::{
    UiHostMeasurementDeadline, UiHostMeasurementEnvironmentReport, UiHostMeasurementObservation,
    UiHostMeasurementRequest, UiHostMeasurementRequestIntent, UiMeasurementRequestFamily,
    UiMeasurementRequestIdentity, UiSurfaceBindingGeneration,
    WorthUiHostCapabilityObservationGeneration,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct UiHostMeasurementDependencyBasis {
    host_session: u64,
    binding: Option<UiSurfaceBindingGeneration>,
    allocation_revision: Option<crate::runtime::UiAllocationTruthRevision>,
    environment_generation: u64,
    capability_generation: WorthUiHostCapabilityObservationGeneration,
}

impl UiHostMeasurementDependencyBasis {
    pub(super) fn binding(self) -> Option<UiSurfaceBindingGeneration> {
        self.binding
    }
}

pub struct UiHostMeasurementIntent {
    binding: Option<UiSurfaceBindingGeneration>,
    request: UiHostMeasurementRequestIntent,
    deadline: UiHostMeasurementDeadline,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiRequestedHostMeasurement {
    request: Rc<UiHostMeasurementRequest>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UiSolicitedHostMeasurementResult {
    observation: UiHostMeasurementObservation,
    source_identity: u64,
    source_generation: WorthUiHostCapabilityObservationGeneration,
    source_order: u64,
}

#[derive(Clone, Copy)]
pub(crate) struct UiHostMeasurementCurrentTruth {
    host_session: u64,
    allocation_revision: crate::runtime::UiAllocationTruthRevision,
    environment: UiHostMeasurementEnvironmentReport,
    capability_generation: WorthUiHostCapabilityObservationGeneration,
    pending_binding_is_live: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiHostMeasurementDenial {
    Shutdown,
    ForeignHostSession,
    UnknownSurfaceBinding,
    BindingRequired,
    UnsupportedEnvironment,
    DeadlineExpired,
    DuplicateRequest,
    UnknownRequest,
    StaleBasis,
    CapacityExceeded,
    ByteCapacityExceeded,
    IdentityExhausted,
    RequestDenied(worth_ui_host_contract::UiMeasurementRequestDenial),
}

#[derive(Clone, Debug, PartialEq)]
pub enum UiHostMeasurementOutcome {
    Admitted(UiRequestedHostMeasurement),
    Completed(UiSolicitedHostMeasurementResult),
    Cancelled(UiMeasurementRequestIdentity),
    Expired(UiMeasurementRequestIdentity),
    DuplicateSuppressed(UiMeasurementRequestIdentity),
    Denied(UiHostMeasurementDenial),
}

impl UiHostMeasurementIntent {
    pub fn new(
        binding: Option<UiSurfaceBindingGeneration>,
        request: UiHostMeasurementRequestIntent,
        deadline: UiHostMeasurementDeadline,
    ) -> Self {
        Self {
            binding,
            request,
            deadline,
        }
    }

    pub fn binding(&self) -> Option<UiSurfaceBindingGeneration> {
        self.binding
    }

    pub fn family(&self) -> UiMeasurementRequestFamily {
        self.request.family()
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        Option<UiSurfaceBindingGeneration>,
        UiHostMeasurementRequestIntent,
        UiHostMeasurementDeadline,
    ) {
        (self.binding, self.request, self.deadline)
    }
}

impl UiRequestedHostMeasurement {
    pub(super) fn new(request: Rc<UiHostMeasurementRequest>) -> Self {
        Self { request }
    }

    pub fn request(&self) -> &UiHostMeasurementRequest {
        self.request.as_ref()
    }

    pub fn identity(&self) -> UiMeasurementRequestIdentity {
        self.request.identity()
    }
}

impl UiSolicitedHostMeasurementResult {
    pub(super) fn new(
        observation: UiHostMeasurementObservation,
        current: UiHostMeasurementCurrentTruth,
    ) -> Self {
        Self {
            source_identity: current.host_session,
            source_generation: current.capability_generation,
            source_order: observation.request_identity().as_u64(),
            observation,
        }
    }

    pub fn observation(&self) -> &UiHostMeasurementObservation {
        &self.observation
    }

    pub const fn source_identity(&self) -> u64 {
        self.source_identity
    }

    pub const fn source_generation(&self) -> WorthUiHostCapabilityObservationGeneration {
        self.source_generation
    }

    pub const fn source_order(&self) -> u64 {
        self.source_order
    }

    pub(crate) fn retained_bytes(&self) -> usize {
        std::mem::size_of::<Self>().saturating_add(self.observation.request().encoded_len())
    }
}

impl UiHostMeasurementCurrentTruth {
    pub(crate) fn new(
        host_session: u64,
        allocation_revision: crate::runtime::UiAllocationTruthRevision,
        environment: UiHostMeasurementEnvironmentReport,
        capability_generation: WorthUiHostCapabilityObservationGeneration,
        pending_binding_is_live: bool,
    ) -> Self {
        Self {
            host_session,
            allocation_revision,
            environment,
            capability_generation,
            pending_binding_is_live,
        }
    }

    pub(super) fn basis_for(
        self,
        family: UiMeasurementRequestFamily,
        binding: Option<UiSurfaceBindingGeneration>,
    ) -> Result<UiHostMeasurementDependencyBasis, UiHostMeasurementDenial> {
        if request_requires_binding(family) && binding.is_none() {
            return Err(UiHostMeasurementDenial::BindingRequired);
        }
        let environment_generation = self
            .environment
            .generation_for(family)
            .ok_or(UiHostMeasurementDenial::UnsupportedEnvironment)?;
        Ok(UiHostMeasurementDependencyBasis {
            host_session: self.host_session,
            binding,
            allocation_revision: request_uses_allocation(family)
                .then_some(self.allocation_revision),
            environment_generation,
            capability_generation: self.capability_generation,
        })
    }

    pub(super) fn still_satisfies(
        self,
        family: UiMeasurementRequestFamily,
        basis: UiHostMeasurementDependencyBasis,
    ) -> bool {
        basis.host_session == self.host_session
            && basis.capability_generation == self.capability_generation
            && (basis.binding.is_none() || self.pending_binding_is_live)
            && basis.allocation_revision
                == request_uses_allocation(family).then_some(self.allocation_revision)
            && self.environment.generation_for(family) == Some(basis.environment_generation)
    }
}

fn request_requires_binding(family: UiMeasurementRequestFamily) -> bool {
    matches!(
        family,
        UiMeasurementRequestFamily::PortalAnchorRect
            | UiMeasurementRequestFamily::ScrollContainerViewport
    )
}

fn request_uses_allocation(family: UiMeasurementRequestFamily) -> bool {
    request_requires_binding(family)
}
use std::rc::Rc;
