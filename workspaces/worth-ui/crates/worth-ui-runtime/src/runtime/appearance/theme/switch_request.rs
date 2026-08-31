#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiThemeSwitchOriginFamily {
    SourceEditObservation,
    ProgrammaticObservation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiThemeSwitchOriginAdmissionDenial {
    MissingRequiredObservationFamily,
    ForeignSession,
    StaleSourceBasis,
    MissingAppearanceGeneration,
    StaleApplicationGeneration,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiThemeSwitchOrigin {
    family: UiThemeSwitchOriginFamily,
    session: crate::facade::WorthUiActiveApplicationSessionIdentity,
    turn: crate::runtime::observation::UiObservationTurnIdentity,
    source_basis: u64,
    generation: crate::runtime::WorthUiActiveApplicationGenerationIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiThemeSwitchRequest {
    pub(super) origin: UiThemeSwitchOrigin,
    pub(super) surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
    pub(super) expected_binding_generation: u64,
    pub(super) capability: super::UiThemeCapabilityReceipt,
}

impl UiThemeSwitchOrigin {
    pub(crate) fn admit_current(
        owner: &crate::facade::WorthUiActiveApplicationSession,
        admitted: &crate::runtime::observation::UiAdmittedObservationSet,
        family: UiThemeSwitchOriginFamily,
    ) -> Result<Self, UiThemeSwitchOriginAdmissionDenial> {
        if admitted.session() != owner.session_identity() {
            return Err(UiThemeSwitchOriginAdmissionDenial::ForeignSession);
        }
        if admitted.source_basis() != owner.capabilities().digest().as_u64() {
            return Err(UiThemeSwitchOriginAdmissionDenial::StaleSourceBasis);
        }
        let carried_generation = admitted
            .appearance_owner_snapshot()
            .map(crate::runtime::appearance::UiAppearanceOwnerSnapshot::generation)
            .ok_or(UiThemeSwitchOriginAdmissionDenial::MissingAppearanceGeneration)?;
        if carried_generation != &owner.active_generation_identity() {
            return Err(UiThemeSwitchOriginAdmissionDenial::StaleApplicationGeneration);
        }
        let required = match family {
            UiThemeSwitchOriginFamily::SourceEditObservation => {
                crate::runtime::observation::UiObservationFamily::AuthoredSource
            }
            UiThemeSwitchOriginFamily::ProgrammaticObservation => {
                crate::runtime::observation::UiObservationFamily::IntentPosture
            }
        };
        if !admitted
            .observations()
            .iter()
            .any(|observation| observation.family() == required)
        {
            return Err(UiThemeSwitchOriginAdmissionDenial::MissingRequiredObservationFamily);
        }
        Ok(Self {
            family,
            session: admitted.session(),
            turn: admitted.turn(),
            source_basis: admitted.source_basis(),
            generation: carried_generation.clone(),
        })
    }

    pub(crate) const fn family(&self) -> UiThemeSwitchOriginFamily {
        self.family
    }
    pub(crate) const fn session(&self) -> crate::facade::WorthUiActiveApplicationSessionIdentity {
        self.session
    }
    pub(crate) const fn turn(&self) -> crate::runtime::observation::UiObservationTurnIdentity {
        self.turn
    }
    pub(crate) const fn source_basis(&self) -> u64 {
        self.source_basis
    }
    pub(crate) const fn generation(
        &self,
    ) -> &crate::runtime::WorthUiActiveApplicationGenerationIdentity {
        &self.generation
    }
}

impl UiThemeSwitchRequest {
    pub(crate) fn new(
        origin: UiThemeSwitchOrigin,
        surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
        expected_binding_generation: u64,
        capability: super::UiThemeCapabilityReceipt,
    ) -> Self {
        Self {
            origin,
            surface,
            expected_binding_generation,
            capability,
        }
    }
}
