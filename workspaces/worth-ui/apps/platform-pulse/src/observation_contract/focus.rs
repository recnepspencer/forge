use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseSemanticFocusParticipant {
    mounted_instance: u64,
    incarnation: u64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PlatformPulseSemanticFocusCause {
    Direct,
    KeyboardTraversal,
    PortalInitial,
    PortalRestoration,
    RebindPreserved,
    RebindFallback,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PlatformPulseSemanticFocusOutcome {
    Moved,
    Unchanged,
    Cleared,
    NoEligibleParticipant,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PlatformPulseSemanticFocusPhysicalOutcome {
    Cleared,
    Applied,
    RejectedBeforeEffect,
    Indeterminate,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseSemanticFocusPublished {
    frame: u64,
    previous: Option<PlatformPulseSemanticFocusParticipant>,
    current: Option<PlatformPulseSemanticFocusParticipant>,
    cause: PlatformPulseSemanticFocusCause,
    outcome: PlatformPulseSemanticFocusOutcome,
    physical_outcome: PlatformPulseSemanticFocusPhysicalOutcome,
    host_request: Option<u64>,
    participants_visited: u32,
    revision: u64,
}

impl PlatformPulseSemanticFocusParticipant {
    fn from_runtime(
        observation: worth_ui::facade::app::UiSemanticFocusParticipantObservation,
    ) -> Self {
        Self {
            mounted_instance: observation.mounted_instance().diagnostic_value(),
            incarnation: observation.incarnation().diagnostic_value(),
        }
    }

    pub const fn mounted_instance(self) -> u64 {
        self.mounted_instance
    }

    pub const fn incarnation(self) -> u64 {
        self.incarnation
    }
}

impl PlatformPulseSemanticFocusPublished {
    pub(super) fn from_runtime(
        receipt: worth_ui::facade::app::UiSemanticFocusPublicationReceipt,
    ) -> Result<Self, super::PlatformPulseLifecycleObservationProjectionDenial> {
        let previous = receipt
            .previous()
            .map(PlatformPulseSemanticFocusParticipant::from_runtime);
        let current = receipt
            .current()
            .map(PlatformPulseSemanticFocusParticipant::from_runtime);
        let host_request = receipt
            .host_placement()
            .map(|acknowledgement| acknowledgement.request().identity().diagnostic_value());
        match (receipt.current(), receipt.host_placement()) {
            (Some(current), Some(acknowledgement))
                if acknowledgement.request().presentation().frame() == receipt.frame()
                    && acknowledgement.request().target().mounted_instance()
                        == current.mounted_instance() => {}
            (None, None) => {}
            _ => {
                return Err(
                    super::PlatformPulseLifecycleObservationProjectionDenial::
                        SemanticFocusPublicationMismatch,
                )
            }
        }
        Ok(Self {
            frame: receipt.frame().diagnostic_value(),
            previous,
            current,
            cause: map_cause(receipt.cause()),
            outcome: map_outcome(receipt.outcome()),
            physical_outcome: map_physical_outcome(receipt.physical_outcome()),
            host_request,
            participants_visited: receipt.participants_visited(),
            revision: receipt.revision(),
        })
    }

    pub const fn frame(self) -> u64 {
        self.frame
    }

    pub const fn previous(self) -> Option<PlatformPulseSemanticFocusParticipant> {
        self.previous
    }

    pub const fn current(self) -> Option<PlatformPulseSemanticFocusParticipant> {
        self.current
    }

    pub const fn cause(self) -> PlatformPulseSemanticFocusCause {
        self.cause
    }

    pub const fn outcome(self) -> PlatformPulseSemanticFocusOutcome {
        self.outcome
    }

    pub const fn physical_outcome(self) -> PlatformPulseSemanticFocusPhysicalOutcome {
        self.physical_outcome
    }

    pub const fn host_request(self) -> Option<u64> {
        self.host_request
    }

    pub const fn participants_visited(self) -> u32 {
        self.participants_visited
    }

    pub const fn revision(self) -> u64 {
        self.revision
    }
}

const fn map_cause(
    cause: worth_ui::facade::app::UiSemanticFocusPublicationCause,
) -> PlatformPulseSemanticFocusCause {
    match cause {
        worth_ui::facade::app::UiSemanticFocusPublicationCause::Direct => {
            PlatformPulseSemanticFocusCause::Direct
        }
        worth_ui::facade::app::UiSemanticFocusPublicationCause::KeyboardTraversal => {
            PlatformPulseSemanticFocusCause::KeyboardTraversal
        }
        worth_ui::facade::app::UiSemanticFocusPublicationCause::PortalInitial => {
            PlatformPulseSemanticFocusCause::PortalInitial
        }
        worth_ui::facade::app::UiSemanticFocusPublicationCause::PortalRestoration => {
            PlatformPulseSemanticFocusCause::PortalRestoration
        }
        worth_ui::facade::app::UiSemanticFocusPublicationCause::RebindPreserved => {
            PlatformPulseSemanticFocusCause::RebindPreserved
        }
        worth_ui::facade::app::UiSemanticFocusPublicationCause::RebindFallback => {
            PlatformPulseSemanticFocusCause::RebindFallback
        }
    }
}

const fn map_outcome(
    outcome: worth_ui::facade::app::UiSemanticFocusPublicationOutcome,
) -> PlatformPulseSemanticFocusOutcome {
    match outcome {
        worth_ui::facade::app::UiSemanticFocusPublicationOutcome::Moved => {
            PlatformPulseSemanticFocusOutcome::Moved
        }
        worth_ui::facade::app::UiSemanticFocusPublicationOutcome::Unchanged => {
            PlatformPulseSemanticFocusOutcome::Unchanged
        }
        worth_ui::facade::app::UiSemanticFocusPublicationOutcome::Cleared => {
            PlatformPulseSemanticFocusOutcome::Cleared
        }
        worth_ui::facade::app::UiSemanticFocusPublicationOutcome::NoEligibleParticipant => {
            PlatformPulseSemanticFocusOutcome::NoEligibleParticipant
        }
    }
}

const fn map_physical_outcome(
    outcome: worth_ui::facade::app::UiSemanticFocusPhysicalPlacementOutcome,
) -> PlatformPulseSemanticFocusPhysicalOutcome {
    match outcome {
        worth_ui::facade::app::UiSemanticFocusPhysicalPlacementOutcome::Cleared => {
            PlatformPulseSemanticFocusPhysicalOutcome::Cleared
        }
        worth_ui::facade::app::UiSemanticFocusPhysicalPlacementOutcome::Applied => {
            PlatformPulseSemanticFocusPhysicalOutcome::Applied
        }
        worth_ui::facade::app::UiSemanticFocusPhysicalPlacementOutcome::RejectedBeforeEffect => {
            PlatformPulseSemanticFocusPhysicalOutcome::RejectedBeforeEffect
        }
        worth_ui::facade::app::UiSemanticFocusPhysicalPlacementOutcome::Indeterminate => {
            PlatformPulseSemanticFocusPhysicalOutcome::Indeterminate
        }
    }
}
