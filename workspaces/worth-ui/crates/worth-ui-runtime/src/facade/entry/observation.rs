impl super::WorthUiActiveApplicationSession {
    #[allow(
        dead_code,
        reason = "Gate 0 proves origin admission without enabling live theme switching"
    )]
    pub(crate) fn issue_theme_switch_origin(
        &self,
        admitted: &crate::runtime::observation::UiAdmittedObservationSet,
        family: crate::runtime::appearance::UiThemeSwitchOriginFamily,
    ) -> Result<
        crate::runtime::appearance::UiThemeSwitchOrigin,
        crate::runtime::appearance::UiThemeSwitchOriginAdmissionDenial,
    > {
        crate::runtime::appearance::UiThemeSwitchOrigin::admit_current(self, admitted, family)
    }

    pub fn begin_observation_turn(
        &mut self,
    ) -> Result<
        crate::facade::observation::UiObservationTurn<'_>,
        crate::facade::observation::UiObservationTurnDenial,
    > {
        let session = self.identity;
        let consumed_facts = self.application.prepared_authority().consumed_fact_index();
        let appearance_axis_demand = consumed_facts.appearance_axis_demand();
        let appearance_close = consumed_facts.has_appearance_consumers().then(|| {
            crate::runtime::observation::UiAppearanceObservationCloseInput::new(
                appearance_axis_demand,
                crate::runtime::WorthUiActiveApplicationGenerationIdentity::current(
                    session,
                    self.application.generation_identity(),
                ),
                self.focus.as_ref(),
                self.selection.as_ref(),
                &self.intent_admission,
                &self.intent_application_facts,
                &self.interaction,
            )
        });
        self.application
            .begin_observation_turn(session, appearance_close)
    }
}
