#[derive(Debug, Eq, PartialEq)]
pub enum UiIntentAdmissionStopReason {
    DefinitionContractMismatch {
        candidate: crate::capability::UiIntentId,
        requested: crate::capability::UiIntentId,
    },
    Inoperable(Box<super::super::operability::UiIntentOperabilityDecision>),
    Confirmation(Box<super::super::UiIntentConfirmationStop>),
    ApplicationWorldChanged,
    ApplicationGenerationChanged,
    PresentationInFlight,
    TargetChanged(crate::runtime::interaction::UiInteractionTargetingDenial),
    ProductRouteChanged,
    PayloadInputChanged,
    OperabilityDependencyChanged,
    PolicyChanged,
    ConfirmationPolicyChanged,
    OccupancyChanged,
    OccupancyCapacityExceeded {
        maximum: usize,
    },
    ExecutionReservation(crate::runtime::intent_execution::UiIntentExecutionReservationDenial),
    AttemptLineageExhausted,
    ReservationIdentityExhausted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiIntentAdmissionCost {
    route_resolution: crate::declaration::UiIntentRouteResolutionCost,
    payload_projection: super::super::payload::UiIntentPayloadProjectionCost,
    operability_dependencies_visited: usize,
    currentness_checks: usize,
    occupancy_slots_inspected: usize,
    slots_inspected: usize,
}

#[must_use]
#[derive(Debug, Eq, PartialEq)]
pub struct UiIntentAdmissionStop {
    reason: UiIntentAdmissionStopReason,
    cost: UiIntentAdmissionCost,
}

impl UiIntentAdmissionCost {
    pub(super) const fn prepared(
        route_resolution: crate::declaration::UiIntentRouteResolutionCost,
        payload_projection: super::super::payload::UiIntentPayloadProjectionCost,
        operability_dependencies_visited: usize,
        currentness_checks: usize,
    ) -> Self {
        Self {
            route_resolution,
            payload_projection,
            operability_dependencies_visited,
            currentness_checks,
            occupancy_slots_inspected: 0,
            slots_inspected: 0,
        }
    }

    pub(super) const fn with_slots_inspected(self, slots_inspected: usize) -> Self {
        Self {
            slots_inspected,
            ..self
        }
    }

    pub(super) const fn with_occupancy_slots_inspected(
        self,
        occupancy_slots_inspected: usize,
    ) -> Self {
        Self {
            occupancy_slots_inspected,
            ..self
        }
    }

    pub const fn operability_dependencies_visited(self) -> usize {
        self.operability_dependencies_visited
    }

    pub const fn route_resolution(self) -> crate::declaration::UiIntentRouteResolutionCost {
        self.route_resolution
    }

    pub const fn payload_projection(self) -> super::super::payload::UiIntentPayloadProjectionCost {
        self.payload_projection
    }

    pub const fn currentness_checks(self) -> usize {
        self.currentness_checks
    }

    pub const fn occupancy_slots_inspected(self) -> usize {
        self.occupancy_slots_inspected
    }

    pub const fn slots_inspected(self) -> usize {
        self.slots_inspected
    }
}

impl UiIntentAdmissionStop {
    pub(super) const fn new(
        reason: UiIntentAdmissionStopReason,
        cost: UiIntentAdmissionCost,
    ) -> Self {
        Self { reason, cost }
    }

    pub const fn reason(&self) -> &UiIntentAdmissionStopReason {
        &self.reason
    }

    pub const fn cost(&self) -> UiIntentAdmissionCost {
        self.cost
    }
}
