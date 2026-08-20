use super::{
    routing::{
        UiNativePhysicalSignalExternalObservation, UiNativePhysicalSignalExternalStatus,
        UiNativePhysicalSignalWork,
    },
    UiNativePhysicalSignalObservation, UiNativePhysicalSignalSettlement,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativePhysicalSignalWorkClass {
    AtlasPlanning,
    AtlasUpload,
    Presentation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativePhysicalSignalExternalStatusClass {
    Pending,
    Completed,
    RejectedBeforeEffects,
    RejectedAfterRasterization,
    EffectsIndeterminate,
    CancellationEffectsMayHaveBegun,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativePhysicalSignalObservationOriginClass {
    NativeExternalPort,
    QualifiedExternalPort,
    PhysicalOwnerCancellation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativePhysicalSignalSettlementClass {
    Pending,
    Completed,
    Superseded,
    Rejected,
    Indeterminate,
    Stale,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiNativePhysicalSignalTransitionObservation {
    identity: [u64; 6],
    work: UiNativePhysicalSignalWorkClass,
    origin: UiNativePhysicalSignalObservationOriginClass,
    external_status: UiNativePhysicalSignalExternalStatusClass,
    settlement: UiNativePhysicalSignalSettlementClass,
    performed: [u64; 4],
}

impl UiNativePhysicalSignalTransitionObservation {
    pub(super) fn from_owner_reconciliation(
        observation: UiNativePhysicalSignalExternalObservation,
        settlement: UiNativePhysicalSignalSettlement,
        before: UiNativePhysicalSignalObservation,
        after: UiNativePhysicalSignalObservation,
    ) -> Self {
        Self::from_parts(
            observation.work(),
            origin_class(observation.origin()),
            status_class(observation.status()),
            settlement_class(settlement),
            before,
            after,
        )
    }

    pub(super) fn from_owner_cancellation(
        work: UiNativePhysicalSignalWork,
        before: UiNativePhysicalSignalObservation,
        after: UiNativePhysicalSignalObservation,
    ) -> Self {
        Self::from_parts(
            work,
            UiNativePhysicalSignalObservationOriginClass::PhysicalOwnerCancellation,
            UiNativePhysicalSignalExternalStatusClass::CancellationEffectsMayHaveBegun,
            UiNativePhysicalSignalSettlementClass::Indeterminate,
            before,
            after,
        )
    }

    fn from_parts(
        work: UiNativePhysicalSignalWork,
        origin: UiNativePhysicalSignalObservationOriginClass,
        external_status: UiNativePhysicalSignalExternalStatusClass,
        settlement: UiNativePhysicalSignalSettlementClass,
        before: UiNativePhysicalSignalObservation,
        after: UiNativePhysicalSignalObservation,
    ) -> Self {
        let request = work.request_identity();
        let basis = request.presentation_basis();
        Self {
            identity: [
                basis.host_session_identity(),
                basis.attempt().diagnostic_value(),
                basis.surface().diagnostic_value(),
                basis.host_surface().diagnostic_value(),
                basis.binding().diagnostic_value(),
                request.sequence(),
            ],
            work: work_class(work),
            origin,
            external_status,
            settlement,
            performed: [
                after
                    .signal_performed_transitions
                    .saturating_sub(before.signal_performed_transitions),
                after
                    .signal_performed_nodes
                    .saturating_sub(before.signal_performed_nodes),
                after.performed_fact_revision.unwrap_or(0),
                u64::from(after.performed_read_scopes),
            ],
        }
    }

    pub const fn host_session(self) -> u64 {
        self.identity[0]
    }
    pub const fn attempt(self) -> u64 {
        self.identity[1]
    }
    pub const fn surface(self) -> u64 {
        self.identity[2]
    }
    pub const fn host_surface(self) -> u64 {
        self.identity[3]
    }
    pub const fn binding(self) -> u64 {
        self.identity[4]
    }
    pub const fn request_sequence(self) -> u64 {
        self.identity[5]
    }
    pub const fn work(self) -> UiNativePhysicalSignalWorkClass {
        self.work
    }
    pub const fn external_status(self) -> UiNativePhysicalSignalExternalStatusClass {
        self.external_status
    }
    pub const fn origin(self) -> UiNativePhysicalSignalObservationOriginClass {
        self.origin
    }
    pub const fn settlement(self) -> UiNativePhysicalSignalSettlementClass {
        self.settlement
    }
    pub const fn performed_transitions(self) -> u64 {
        self.performed[0]
    }
    pub const fn performed_nodes(self) -> u64 {
        self.performed[1]
    }
    pub const fn fact_revision(self) -> u64 {
        self.performed[2]
    }
    pub const fn read_scopes(self) -> u64 {
        self.performed[3]
    }
}

const fn origin_class(
    origin: super::routing::UiNativePhysicalSignalExternalOrigin,
) -> UiNativePhysicalSignalObservationOriginClass {
    match origin {
        super::routing::UiNativePhysicalSignalExternalOrigin::NativeExternalPort => {
            UiNativePhysicalSignalObservationOriginClass::NativeExternalPort
        }
        super::routing::UiNativePhysicalSignalExternalOrigin::QualifiedExternalPort => {
            UiNativePhysicalSignalObservationOriginClass::QualifiedExternalPort
        }
    }
}

const fn work_class(work: UiNativePhysicalSignalWork) -> UiNativePhysicalSignalWorkClass {
    match work {
        UiNativePhysicalSignalWork::AtlasPlanning(_) => {
            UiNativePhysicalSignalWorkClass::AtlasPlanning
        }
        UiNativePhysicalSignalWork::AtlasUpload(_) => UiNativePhysicalSignalWorkClass::AtlasUpload,
        UiNativePhysicalSignalWork::Presentation(_) => {
            UiNativePhysicalSignalWorkClass::Presentation
        }
    }
}

const fn status_class(
    status: UiNativePhysicalSignalExternalStatus,
) -> UiNativePhysicalSignalExternalStatusClass {
    match status {
        UiNativePhysicalSignalExternalStatus::Pending => {
            UiNativePhysicalSignalExternalStatusClass::Pending
        }
        UiNativePhysicalSignalExternalStatus::Completed => {
            UiNativePhysicalSignalExternalStatusClass::Completed
        }
        UiNativePhysicalSignalExternalStatus::RejectedBeforeEffects => {
            UiNativePhysicalSignalExternalStatusClass::RejectedBeforeEffects
        }
        UiNativePhysicalSignalExternalStatus::RejectedAfterRasterization => {
            UiNativePhysicalSignalExternalStatusClass::RejectedAfterRasterization
        }
        UiNativePhysicalSignalExternalStatus::EffectsIndeterminate => {
            UiNativePhysicalSignalExternalStatusClass::EffectsIndeterminate
        }
    }
}

const fn settlement_class(
    settlement: UiNativePhysicalSignalSettlement,
) -> UiNativePhysicalSignalSettlementClass {
    match settlement {
        UiNativePhysicalSignalSettlement::Pending => UiNativePhysicalSignalSettlementClass::Pending,
        UiNativePhysicalSignalSettlement::Completed => {
            UiNativePhysicalSignalSettlementClass::Completed
        }
        UiNativePhysicalSignalSettlement::Superseded => {
            UiNativePhysicalSignalSettlementClass::Superseded
        }
        UiNativePhysicalSignalSettlement::Rejected => {
            UiNativePhysicalSignalSettlementClass::Rejected
        }
        UiNativePhysicalSignalSettlement::Indeterminate => {
            UiNativePhysicalSignalSettlementClass::Indeterminate
        }
        UiNativePhysicalSignalSettlement::Stale => UiNativePhysicalSignalSettlementClass::Stale,
    }
}
