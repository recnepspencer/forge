use super::super::{WorthQueryRuntime, WorthQueryRuntimeError};
use super::conditional_delivery::{
    WorthQueryAdmittedStagedOwnerDelivery, WorthQueryClassifiedOwnerDeliveryEmissionError,
};

pub(super) fn deny_injected_emission(
    runtime: &mut WorthQueryRuntime,
    admitted: &WorthQueryAdmittedStagedOwnerDelivery,
) -> Result<(), WorthQueryClassifiedOwnerDeliveryEmissionError> {
    if runtime
        .installed_live_routes
        .injected_classified_emission_failures
        == 0
    {
        return Ok(());
    }
    runtime
        .installed_live_routes
        .injected_classified_emission_failures -= 1;
    Err(WorthQueryClassifiedOwnerDeliveryEmissionError::Runtime(
        WorthQueryRuntimeError::LiveSubscriptionInstallation {
            view_name: admitted.target.view_name().to_string(),
            stage: "classified-owner-delivery-injection",
            message: "injected classified live emission failure".into(),
        },
    ))
}

impl WorthQueryRuntime {
    pub(crate) fn inject_classified_live_emission_failures(&mut self, count: usize) {
        self.installed_live_routes
            .injected_classified_emission_failures = count;
    }
}
