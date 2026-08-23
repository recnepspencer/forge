const MAXIMUM_PRESENTATION_POLL_ORDINAL: u64 = 4_096;

/// Certification-only control retained by the native physical-work owner.
///
/// The control selects one owner poll at which the native host reports that
/// physical effects cannot be determined. It carries no Signal identity,
/// request handle, completion envelope, or recovery authority.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiNativeQualificationPlan {
    deferred_presentations: [Option<u64>; 3],
    duplicate_completion_presentation: Option<u64>,
    effects_indeterminate_presentation: Option<u64>,
    derived_state_loss: Option<crate::UiNativeDerivedStateLossClass>,
    derived_state_loss_after_completed_presentation: Option<u64>,
    surface_basis_successor: Option<UiNativeQualificationSurfaceBasisSuccessor>,
    event_loop_thread_posture: crate::native::UiNativeEventLoopThreadPosture,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiNativeQualificationSurfaceBasisSuccessor {
    after_completed_presentation: u64,
    change: UiNativeQualificationSurfaceBasisChange,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiNativeQualificationSurfaceBasisChange {
    ClientPhysicalWidthDelta(i32),
    DpiScaleMultiplierMilli(u32),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativeQualificationPlanDenial {
    ZeroPresentationPollOrdinal,
    PresentationPollOrdinalCapacityExceeded,
    InvalidSurfaceBasis,
}

impl UiNativeQualificationPlan {
    pub const fn ordinary() -> Self {
        Self {
            deferred_presentations: [None, None, None],
            duplicate_completion_presentation: None,
            effects_indeterminate_presentation: None,
            derived_state_loss: None,
            derived_state_loss_after_completed_presentation: None,
            surface_basis_successor: None,
            event_loop_thread_posture:
                crate::native::UiNativeEventLoopThreadPosture::MainThreadRequired,
        }
    }

    /// Forces the selected presentation to cross the real asynchronous
    /// completion boundary without changing its eventual physical result.
    pub fn deferred_completion_on_presentation(
        one_based_presentation_ordinal: u64,
    ) -> Result<Self, UiNativeQualificationPlanDenial> {
        validate_presentation_ordinal(one_based_presentation_ordinal)?;
        Ok(Self {
            deferred_presentations: [Some(one_based_presentation_ordinal), None, None],
            duplicate_completion_presentation: None,
            effects_indeterminate_presentation: None,
            derived_state_loss: None,
            derived_state_loss_after_completed_presentation: None,
            surface_basis_successor: None,
            event_loop_thread_posture:
                crate::native::UiNativeEventLoopThreadPosture::MainThreadRequired,
        })
    }

    pub fn effects_indeterminate_on_presentation(
        one_based_presentation_ordinal: u64,
    ) -> Result<Self, UiNativeQualificationPlanDenial> {
        validate_presentation_ordinal(one_based_presentation_ordinal)?;
        Ok(Self {
            deferred_presentations: [Some(one_based_presentation_ordinal), None, None],
            duplicate_completion_presentation: None,
            effects_indeterminate_presentation: Some(one_based_presentation_ordinal),
            derived_state_loss: None,
            derived_state_loss_after_completed_presentation: None,
            surface_basis_successor: None,
            event_loop_thread_posture:
                crate::native::UiNativeEventLoopThreadPosture::MainThreadRequired,
        })
    }

    pub fn effects_indeterminate_with_derived_state_loss_on_presentation(
        one_based_presentation_ordinal: u64,
        derived_state_loss: crate::UiNativeDerivedStateLossClass,
    ) -> Result<Self, UiNativeQualificationPlanDenial> {
        validate_presentation_ordinal(one_based_presentation_ordinal)?;
        Ok(Self {
            deferred_presentations: [Some(one_based_presentation_ordinal), None, None],
            duplicate_completion_presentation: None,
            effects_indeterminate_presentation: Some(one_based_presentation_ordinal),
            derived_state_loss: Some(derived_state_loss),
            derived_state_loss_after_completed_presentation: None,
            surface_basis_successor: None,
            event_loop_thread_posture:
                crate::native::UiNativeEventLoopThreadPosture::MainThreadRequired,
        })
    }

    pub fn derived_state_loss_after_completed_presentation(
        one_based_presentation_ordinal: u64,
        derived_state_loss: crate::UiNativeDerivedStateLossClass,
    ) -> Result<Self, UiNativeQualificationPlanDenial> {
        validate_presentation_ordinal(one_based_presentation_ordinal)?;
        Ok(Self {
            deferred_presentations: [None, None, None],
            duplicate_completion_presentation: None,
            effects_indeterminate_presentation: None,
            derived_state_loss: Some(derived_state_loss),
            derived_state_loss_after_completed_presentation: Some(one_based_presentation_ordinal),
            surface_basis_successor: None,
            event_loop_thread_posture:
                crate::native::UiNativeEventLoopThreadPosture::MainThreadRequired,
        })
    }

    pub fn with_deferred_completion_on_presentation(
        mut self,
        one_based_presentation_ordinal: u64,
    ) -> Result<Self, UiNativeQualificationPlanDenial> {
        validate_presentation_ordinal(one_based_presentation_ordinal)?;
        if self
            .deferred_presentations
            .contains(&Some(one_based_presentation_ordinal))
        {
            return Ok(self);
        }
        let Some(slot) = self
            .deferred_presentations
            .iter_mut()
            .find(|slot| slot.is_none())
        else {
            return Err(UiNativeQualificationPlanDenial::InvalidSurfaceBasis);
        };
        *slot = Some(one_based_presentation_ordinal);
        Ok(self)
    }

    /// Causes the native external-observation owner to submit the selected
    /// completed observation a second time through the physical Signal.
    pub fn with_duplicate_completion_observation_on_presentation(
        mut self,
        one_based_presentation_ordinal: u64,
    ) -> Result<Self, UiNativeQualificationPlanDenial> {
        validate_presentation_ordinal(one_based_presentation_ordinal)?;
        self.duplicate_completion_presentation = Some(one_based_presentation_ordinal);
        Ok(self)
    }

    /// Schedules one native-owner surface-basis successor after the selected
    /// physical presentation. The application receives the change only
    /// through the ordinary readiness grant.
    pub fn with_client_width_delta_after_presentation(
        mut self,
        one_based_presentation_ordinal: u64,
        physical_width_delta: i32,
    ) -> Result<Self, UiNativeQualificationPlanDenial> {
        validate_presentation_ordinal(one_based_presentation_ordinal)?;
        if physical_width_delta == 0 || !(-4_096..=4_096).contains(&physical_width_delta) {
            return Err(UiNativeQualificationPlanDenial::InvalidSurfaceBasis);
        }
        self.surface_basis_successor = Some(UiNativeQualificationSurfaceBasisSuccessor {
            after_completed_presentation: one_based_presentation_ordinal,
            change: UiNativeQualificationSurfaceBasisChange::ClientPhysicalWidthDelta(
                physical_width_delta,
            ),
        });
        Ok(self)
    }

    pub fn with_dpi_scale_multiplier_after_presentation(
        mut self,
        one_based_presentation_ordinal: u64,
        scale_multiplier_milli: u32,
    ) -> Result<Self, UiNativeQualificationPlanDenial> {
        validate_presentation_ordinal(one_based_presentation_ordinal)?;
        if !(500..=2_000).contains(&scale_multiplier_milli) || scale_multiplier_milli == 1_000 {
            return Err(UiNativeQualificationPlanDenial::InvalidSurfaceBasis);
        }
        self.surface_basis_successor = Some(UiNativeQualificationSurfaceBasisSuccessor {
            after_completed_presentation: one_based_presentation_ordinal,
            change: UiNativeQualificationSurfaceBasisChange::DpiScaleMultiplierMilli(
                scale_multiplier_milli,
            ),
        });
        Ok(self)
    }

    pub fn with_certification_worker_event_loop(mut self) -> Self {
        self.event_loop_thread_posture =
            crate::native::UiNativeEventLoopThreadPosture::CertificationWorker;
        self
    }

    pub(crate) const fn deferred_presentations(self) -> [Option<u64>; 3] {
        self.deferred_presentations
    }

    pub(crate) const fn duplicate_completion_presentation(self) -> Option<u64> {
        self.duplicate_completion_presentation
    }

    pub(crate) const fn effects_indeterminate_presentation(self) -> Option<u64> {
        self.effects_indeterminate_presentation
    }

    pub(crate) const fn derived_state_loss(self) -> Option<crate::UiNativeDerivedStateLossClass> {
        self.derived_state_loss
    }

    pub(crate) const fn completed_derived_state_loss_ordinal(self) -> Option<u64> {
        self.derived_state_loss_after_completed_presentation
    }

    pub(crate) const fn surface_basis_successor(
        self,
    ) -> Option<UiNativeQualificationSurfaceBasisSuccessor> {
        self.surface_basis_successor
    }

    pub(crate) const fn event_loop_thread_posture(
        self,
    ) -> crate::native::UiNativeEventLoopThreadPosture {
        self.event_loop_thread_posture
    }
}

impl UiNativeQualificationSurfaceBasisSuccessor {
    pub(crate) const fn after_completed_presentation(self) -> u64 {
        self.after_completed_presentation
    }

    pub(crate) const fn change(self) -> UiNativeQualificationSurfaceBasisChange {
        self.change
    }
}

fn validate_presentation_ordinal(
    one_based_presentation_ordinal: u64,
) -> Result<(), UiNativeQualificationPlanDenial> {
    if one_based_presentation_ordinal == 0 {
        return Err(UiNativeQualificationPlanDenial::ZeroPresentationPollOrdinal);
    }
    if one_based_presentation_ordinal > MAXIMUM_PRESENTATION_POLL_ORDINAL {
        return Err(UiNativeQualificationPlanDenial::PresentationPollOrdinalCapacityExceeded);
    }
    Ok(())
}

impl Default for UiNativeQualificationPlanDenial {
    fn default() -> Self {
        Self::ZeroPresentationPollOrdinal
    }
}
