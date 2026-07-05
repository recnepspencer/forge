use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::evidence::UiMeasurementResult;

use super::{UiHostMeasurementAssumptionProfile, UiHostMeasurementInvalidationReason};

pub(crate) fn invalidate_stale_host_measurement_evidence(
    result: &UiMeasurementResult,
    current_generation: UiEvidenceAuthorityGeneration,
    current_profile: UiHostMeasurementAssumptionProfile,
) -> Result<(), UiHostMeasurementInvalidationReason> {
    if result.evidence_generation() != current_generation {
        return Err(
            UiHostMeasurementInvalidationReason::EvidenceGenerationDrift {
                recorded: result.evidence_generation(),
                current: current_generation,
            },
        );
    }

    let recorded = result.assumption_profile();
    if recorded.capability_observation_generation()
        != current_profile.capability_observation_generation()
    {
        return Err(
            UiHostMeasurementInvalidationReason::CapabilityObservationGenerationDrift {
                recorded: recorded.capability_observation_generation(),
                current: current_profile.capability_observation_generation(),
            },
        );
    }
    if recorded.capability_profile_digest() != current_profile.capability_profile_digest() {
        return Err(
            UiHostMeasurementInvalidationReason::CapabilityProfileDrift {
                recorded: recorded.capability_profile_digest(),
                current: current_profile.capability_profile_digest(),
            },
        );
    }
    if recorded.viewport_assumption_digest() != current_profile.viewport_assumption_digest() {
        return Err(
            UiHostMeasurementInvalidationReason::ViewportAssumptionDrift {
                recorded: recorded.viewport_assumption_digest(),
                current: current_profile.viewport_assumption_digest(),
            },
        );
    }
    if recorded.dpi_assumption_digest() != current_profile.dpi_assumption_digest() {
        return Err(UiHostMeasurementInvalidationReason::DpiAssumptionDrift {
            recorded: recorded.dpi_assumption_digest(),
            current: current_profile.dpi_assumption_digest(),
        });
    }
    if recorded.font_assumption_digest() != current_profile.font_assumption_digest() {
        return Err(UiHostMeasurementInvalidationReason::FontAssumptionDrift {
            recorded: recorded.font_assumption_digest(),
            current: current_profile.font_assumption_digest(),
        });
    }
    if recorded.adapter_profile_digest() != current_profile.adapter_profile_digest() {
        return Err(UiHostMeasurementInvalidationReason::AdapterProfileDrift {
            recorded: recorded.adapter_profile_digest(),
            current: current_profile.adapter_profile_digest(),
        });
    }

    Ok(())
}
