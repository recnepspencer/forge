#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiIntentExecutionBindingRegistrationMetrics {
    definitions: usize,
    bindings: usize,
}

pub trait WorthUiIntentExecutionBindingCertificationExt {
    fn intent_execution_binding_registration_metrics_for_certification(
        &self,
    ) -> UiIntentExecutionBindingRegistrationMetrics;
}

impl UiIntentExecutionBindingRegistrationMetrics {
    pub const fn definitions(self) -> usize {
        self.definitions
    }

    pub const fn bindings(self) -> usize {
        self.bindings
    }
}

impl WorthUiIntentExecutionBindingCertificationExt for crate::facade::WorthUiApp {
    fn intent_execution_binding_registration_metrics_for_certification(
        &self,
    ) -> UiIntentExecutionBindingRegistrationMetrics {
        UiIntentExecutionBindingRegistrationMetrics {
            definitions: self.capabilities().intent_definitions().len(),
            bindings: self.prepared_authority().intent_execution_bindings().len(),
        }
    }
}
