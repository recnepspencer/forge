#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiDismissInteraction {
    cause: UiDismissInteractionCause,
    presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
    sequence: worth_ui_host_contract::UiHostObservationSequence,
    time_basis: worth_ui_host_contract::UiHostObservationTimeBasis,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiDismissInteractionCause {
    Escape,
    OutsidePress(worth_ui_host_contract::UiHostSurfacePosition),
}

impl UiDismissInteraction {
    pub(crate) const fn escape(
        presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
        sequence: worth_ui_host_contract::UiHostObservationSequence,
        time_basis: worth_ui_host_contract::UiHostObservationTimeBasis,
    ) -> Self {
        Self {
            cause: UiDismissInteractionCause::Escape,
            presentation,
            sequence,
            time_basis,
        }
    }

    pub(crate) const fn outside_press(
        presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
        sequence: worth_ui_host_contract::UiHostObservationSequence,
        time_basis: worth_ui_host_contract::UiHostObservationTimeBasis,
        position: worth_ui_host_contract::UiHostSurfacePosition,
    ) -> Self {
        Self {
            cause: UiDismissInteractionCause::OutsidePress(position),
            presentation,
            sequence,
            time_basis,
        }
    }

    pub const fn cause(self) -> UiDismissInteractionCause {
        self.cause
    }

    pub const fn presentation(self) -> worth_ui_host_contract::UiHostObservationPresentationBasis {
        self.presentation
    }

    pub const fn sequence(self) -> worth_ui_host_contract::UiHostObservationSequence {
        self.sequence
    }

    pub const fn time_basis(self) -> worth_ui_host_contract::UiHostObservationTimeBasis {
        self.time_basis
    }
}
