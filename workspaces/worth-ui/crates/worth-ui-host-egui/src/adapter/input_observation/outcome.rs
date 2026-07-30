use worth_ui_host_contract::{UiHostObservationPresentationBasis, UiHostObservationSequenceRange};

use super::UiEguiRawInputReachability;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiEguiRetainedRawInput {
    reachability: UiEguiRawInputReachability,
    presentation: UiHostObservationPresentationBasis,
    sequences: UiHostObservationSequenceRange,
    report_count: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiEguiRawInputIngressStop {
    reachability: UiEguiRawInputReachability,
    reason: UiEguiRawInputIngressStopReason,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiEguiRawInputIngressStopReason {
    NoPresentedSurface,
    AmbiguousPresentedSurfaces {
        count: usize,
    },
    UnsupportedEvent {
        index: usize,
        family: UiEguiUnsupportedEventFamily,
    },
    Coordinate {
        index: usize,
        denial: UiEguiCoordinateConversionDenial,
    },
    ImePreedit {
        index: usize,
        denial: worth_ui_host_contract::UiHostImePreeditConstructionDenial,
    },
    ReportLimitExceeded,
    ByteLimitExceeded,
    SequenceExhausted,
    TextRevisionExhausted,
    PointerCaptureEpochExhausted,
    BatchConstruction(worth_ui_host_contract::UiHostObservationBatchConstructionDenial),
    Retention(worth_ui_host_contract::UiHostObservationRetentionDenial),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiEguiUnsupportedEventFamily {
    Copy,
    Cut,
    Zoom,
    Rotate,
    LineScroll,
    PageScroll,
    ImeLifecycle,
    AccessKitAction,
    Screenshot,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiEguiCoordinateConversionDenial {
    NotFinite,
    OutsideCanonicalRange,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use]
pub enum UiEguiRawInputIngressOutcome {
    Retained(UiEguiRetainedRawInput),
    NoMechanicalObservations(UiEguiRawInputReachability),
    Stopped(UiEguiRawInputIngressStop),
}

impl UiEguiRetainedRawInput {
    pub(super) const fn new(
        reachability: UiEguiRawInputReachability,
        presentation: UiHostObservationPresentationBasis,
        sequences: UiHostObservationSequenceRange,
        report_count: usize,
    ) -> Self {
        Self {
            reachability,
            presentation,
            sequences,
            report_count,
        }
    }

    pub const fn reachability(self) -> UiEguiRawInputReachability {
        self.reachability
    }

    pub const fn presentation(self) -> UiHostObservationPresentationBasis {
        self.presentation
    }

    pub const fn sequences(self) -> UiHostObservationSequenceRange {
        self.sequences
    }

    pub const fn report_count(self) -> usize {
        self.report_count
    }
}

impl UiEguiRawInputIngressStop {
    pub(super) const fn new(
        reachability: UiEguiRawInputReachability,
        reason: UiEguiRawInputIngressStopReason,
    ) -> Self {
        Self {
            reachability,
            reason,
        }
    }

    pub const fn reachability(self) -> UiEguiRawInputReachability {
        self.reachability
    }

    pub const fn reason(self) -> UiEguiRawInputIngressStopReason {
        self.reason
    }
}

impl UiEguiRawInputIngressOutcome {
    pub const fn reachability(self) -> UiEguiRawInputReachability {
        match self {
            Self::Retained(retained) => retained.reachability(),
            Self::NoMechanicalObservations(reachability) => reachability,
            Self::Stopped(stop) => stop.reachability(),
        }
    }
}
