impl super::WorthUiHostEgui {
    #[cfg(test)]
    pub(crate) fn without_input_translator_for_testing(
        mut self,
        family: super::super::UiEguiInputTranslatorFamily,
    ) -> Self {
        self.input_translators = self.input_translators.without(family);
        self
    }

    pub fn retain_host_observation(
        &self,
        batch: worth_ui_host_contract::UiHostObservationBatch,
    ) -> Result<(), worth_ui_host_contract::UiHostObservationRetentionDenial> {
        self.observation_retention.retain(batch)
    }

    pub fn observe_native_input(
        &self,
        raw_input: &egui::RawInput,
    ) -> super::super::UiEguiRawInputIngressOutcome {
        super::super::input_observation::observe_raw_input(
            self.input_translators,
            &self.input_observation,
            &self.observation_retention,
            raw_input,
        )
    }
}
