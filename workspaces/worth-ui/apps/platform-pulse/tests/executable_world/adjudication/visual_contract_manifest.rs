mod contrast;
mod control_point_validation;
mod interactive_content_validation;
mod model;
mod native_control_contract;
mod validation;

#[cfg(test)]
mod tests;

use model::PlatformPulseVisualContractManifest;

pub(super) use native_control_contract::{
    action_control, confirmation_control, portal_control, PlatformPulseNativeControlContract,
};
pub(super) use validation::PlatformPulseVisualContractFailure;

const SOURCE: &str = include_str!("platform_pulse_visual_contract.json");

fn checked_in(
) -> Result<PlatformPulseVisualContractManifest, validation::PlatformPulseVisualContractFailure> {
    let manifest = serde_json::from_str::<PlatformPulseVisualContractManifest>(SOURCE)
        .map_err(|_| validation::PlatformPulseVisualContractFailure::Decode)?;
    validation::validate(&manifest)?;
    Ok(manifest)
}
