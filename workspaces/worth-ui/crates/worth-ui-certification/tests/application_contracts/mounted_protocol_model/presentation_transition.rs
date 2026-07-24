#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ModelSurfaceStart {
    Presented,
    RejectedBeforeEffects,
    InFlight,
    EffectStateUnknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ModelCompletion {
    Pending,
    Presented,
    RejectedBeforeEffects,
    EffectStateUnknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ModelCancellation {
    CancelledBeforeEffects,
    EffectsMayHaveBegun,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ModelFrameState {
    InFlight { pending_surfaces: usize },
    Presented,
    RejectedBeforeEffects,
    Indeterminate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ModelSurfaceState {
    Pending,
    Presented,
    RejectedBeforeEffects,
    Indeterminate,
}

#[derive(Debug)]
pub(crate) struct ModelPresentation {
    surfaces: Vec<ModelSurfaceState>,
}

#[derive(Debug, Default)]
pub(crate) struct ModelPublicationWorld {
    current_frame_ordinal: Option<u64>,
    presentation_in_flight: bool,
}

impl ModelPublicationWorld {
    pub(crate) fn begin_presentation(&mut self) {
        assert!(!self.presentation_in_flight);
        self.presentation_in_flight = true;
    }

    pub(crate) fn successor_mutation_allowed(&self) -> bool {
        !self.presentation_in_flight
    }

    pub(crate) fn complete_presentation(&mut self, frame_ordinal: u64) {
        assert!(self.presentation_in_flight);
        self.presentation_in_flight = false;
        self.current_frame_ordinal = Some(frame_ordinal);
    }

    pub(crate) fn current_frame_ordinal(&self) -> Option<u64> {
        self.current_frame_ordinal
    }
}

impl ModelPresentation {
    pub(crate) fn start(outcomes: &[ModelSurfaceStart]) -> Self {
        assert!(
            !outcomes.is_empty(),
            "a presentation has a surface manifest"
        );
        let surfaces = outcomes
            .iter()
            .map(|outcome| match outcome {
                ModelSurfaceStart::Presented => ModelSurfaceState::Presented,
                ModelSurfaceStart::RejectedBeforeEffects => {
                    ModelSurfaceState::RejectedBeforeEffects
                }
                ModelSurfaceStart::InFlight => ModelSurfaceState::Pending,
                ModelSurfaceStart::EffectStateUnknown => ModelSurfaceState::Indeterminate,
            })
            .collect();
        Self { surfaces }
    }

    pub(crate) fn complete(&mut self, surface: usize, completion: ModelCompletion) {
        let state = self
            .surfaces
            .get_mut(surface)
            .expect("authored completion names a manifest surface");
        assert_eq!(
            *state,
            ModelSurfaceState::Pending,
            "only an in-flight surface accepts completion"
        );
        *state = match completion {
            ModelCompletion::Pending => ModelSurfaceState::Pending,
            ModelCompletion::Presented => ModelSurfaceState::Presented,
            ModelCompletion::RejectedBeforeEffects => ModelSurfaceState::RejectedBeforeEffects,
            ModelCompletion::EffectStateUnknown => ModelSurfaceState::Indeterminate,
        };
    }

    pub(crate) fn cancel(&mut self, surface: usize, cancellation: ModelCancellation) {
        let state = self
            .surfaces
            .get_mut(surface)
            .expect("authored cancellation names a manifest surface");
        assert_eq!(
            *state,
            ModelSurfaceState::Pending,
            "only an in-flight surface accepts cancellation"
        );
        *state = match cancellation {
            ModelCancellation::CancelledBeforeEffects => ModelSurfaceState::RejectedBeforeEffects,
            ModelCancellation::EffectsMayHaveBegun => ModelSurfaceState::Indeterminate,
        };
    }

    pub(crate) fn frame_state(&self) -> ModelFrameState {
        let pending = self.count(ModelSurfaceState::Pending);
        let presented = self.count(ModelSurfaceState::Presented);
        let rejected = self.count(ModelSurfaceState::RejectedBeforeEffects);
        let indeterminate = self.count(ModelSurfaceState::Indeterminate);

        if indeterminate > 0 || (rejected > 0 && presented + pending > 0) {
            ModelFrameState::Indeterminate
        } else if pending > 0 {
            ModelFrameState::InFlight {
                pending_surfaces: pending,
            }
        } else if presented == self.surfaces.len() {
            ModelFrameState::Presented
        } else {
            debug_assert_eq!(rejected, self.surfaces.len());
            ModelFrameState::RejectedBeforeEffects
        }
    }

    pub(crate) fn publication_eligible(&self) -> bool {
        self.frame_state() == ModelFrameState::Presented
    }

    fn count(&self, expected: ModelSurfaceState) -> usize {
        self.surfaces
            .iter()
            .filter(|state| **state == expected)
            .count()
    }
}
