pub struct UiAppearanceOwnerSnapshot {
    turn: crate::runtime::observation::UiObservationTurnIdentity,
    session: crate::facade::WorthUiActiveApplicationSessionIdentity,
    source_basis: u64,
    generation: crate::runtime::WorthUiActiveApplicationGenerationIdentity,
    demand: super::UiAppearanceStateAxisDemand,
    focus: Option<crate::runtime::focus::UiFocusAppearancePosture>,
    selection: Option<crate::runtime::selection::UiSelectionAppearanceOwnerSnapshot>,
    operability: Option<crate::runtime::intent::UiIntentOperabilityStandingFactSnapshot>,
    validation: Option<crate::runtime::intent::UiValidationAppearanceFactSnapshot>,
    pointer_presence: Option<crate::runtime::interaction::UiPointerPresenceAppearanceOwnerSnapshot>,
    pressed: Option<crate::runtime::interaction::gesture::UiPressedAppearanceOwnerSnapshot>,
}

impl UiAppearanceOwnerSnapshot {
    #[allow(
        clippy::too_many_arguments,
        reason = "the close owner seals six independent axes plus exact bases"
    )]
    pub(crate) fn seal_at_turn_close(
        _authority: &crate::runtime::observation::UiObservationTurnCloseAuthority,
        turn: crate::runtime::observation::UiObservationTurnIdentity,
        session: crate::facade::WorthUiActiveApplicationSessionIdentity,
        source_basis: u64,
        generation: crate::runtime::WorthUiActiveApplicationGenerationIdentity,
        demand: super::UiAppearanceStateAxisDemand,
        focus: Option<crate::runtime::focus::UiFocusAppearancePosture>,
        selection: Option<crate::runtime::selection::UiSelectionAppearanceOwnerSnapshot>,
        operability: Option<crate::runtime::intent::UiIntentOperabilityStandingFactSnapshot>,
        validation: Option<crate::runtime::intent::UiValidationAppearanceFactSnapshot>,
        pointer_presence: Option<
            crate::runtime::interaction::UiPointerPresenceAppearanceOwnerSnapshot,
        >,
        pressed: Option<crate::runtime::interaction::gesture::UiPressedAppearanceOwnerSnapshot>,
    ) -> Self {
        Self {
            turn,
            session,
            source_basis,
            generation,
            demand,
            focus,
            selection,
            operability,
            validation,
            pointer_presence,
            pressed,
        }
    }

    pub(crate) const fn turn(&self) -> crate::runtime::observation::UiObservationTurnIdentity {
        self.turn
    }
    pub(crate) const fn session(&self) -> crate::facade::WorthUiActiveApplicationSessionIdentity {
        self.session
    }
    pub(crate) const fn source_basis(&self) -> u64 {
        self.source_basis
    }
    pub(crate) const fn generation(
        &self,
    ) -> &crate::runtime::WorthUiActiveApplicationGenerationIdentity {
        &self.generation
    }
    pub(crate) const fn demand(&self) -> super::UiAppearanceStateAxisDemand {
        self.demand
    }
    pub(crate) const fn focus(&self) -> Option<&crate::runtime::focus::UiFocusAppearancePosture> {
        self.focus.as_ref()
    }
    pub(crate) const fn selection(
        &self,
    ) -> Option<&crate::runtime::selection::UiSelectionAppearanceOwnerSnapshot> {
        self.selection.as_ref()
    }
    pub(crate) const fn operability(
        &self,
    ) -> Option<&crate::runtime::intent::UiIntentOperabilityStandingFactSnapshot> {
        self.operability.as_ref()
    }
    pub(crate) const fn validation(
        &self,
    ) -> Option<&crate::runtime::intent::UiValidationAppearanceFactSnapshot> {
        self.validation.as_ref()
    }
    pub(crate) const fn pointer_presence(
        &self,
    ) -> Option<&crate::runtime::interaction::UiPointerPresenceAppearanceOwnerSnapshot> {
        self.pointer_presence.as_ref()
    }
    pub(crate) const fn pressed(
        &self,
    ) -> Option<&crate::runtime::interaction::gesture::UiPressedAppearanceOwnerSnapshot> {
        self.pressed.as_ref()
    }
}
