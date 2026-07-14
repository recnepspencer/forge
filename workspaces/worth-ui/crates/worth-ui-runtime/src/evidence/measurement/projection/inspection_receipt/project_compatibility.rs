use worth_ui_inspection::UiInspectionMeasurementGenerationCompatibility;

use crate::evidence::measurement::UiMeasurementGenerationCompatibility;

pub(crate) fn project_generation_compatibility(
    compatibility: &UiMeasurementGenerationCompatibility,
) -> UiInspectionMeasurementGenerationCompatibility {
    match compatibility {
        UiMeasurementGenerationCompatibility::Compatible => {
            UiInspectionMeasurementGenerationCompatibility::Compatible
        }
        UiMeasurementGenerationCompatibility::StaleQueryFactReceipt { expected, observed } => {
            UiInspectionMeasurementGenerationCompatibility::StaleQueryFactReceipt {
                expected: expected.as_u64(),
                observed: observed.as_u64(),
            }
        }
        UiMeasurementGenerationCompatibility::StaleHostEvidence { expected, observed } => {
            UiInspectionMeasurementGenerationCompatibility::StaleHostEvidence {
                expected: expected.as_u64(),
                observed: observed.as_u64(),
            }
        }
        UiMeasurementGenerationCompatibility::StaleHostCapability { expected, observed } => {
            UiInspectionMeasurementGenerationCompatibility::StaleHostCapability {
                expected: expected.as_u64(),
                observed: observed.as_u64(),
            }
        }
        UiMeasurementGenerationCompatibility::IncompatibleWorld {
            expected_query_basis_digest,
            observed_world_basis_digest,
        } => UiInspectionMeasurementGenerationCompatibility::IncompatibleWorld {
            expected_query_basis_digest: expected_query_basis_digest.clone(),
            observed_world_basis_digest: observed_world_basis_digest.clone(),
        },
        UiMeasurementGenerationCompatibility::IncompatibleHostProfile {
            expected_profile_digest,
            observed_profile_digest,
        } => UiInspectionMeasurementGenerationCompatibility::IncompatibleHostProfile {
            expected_profile_digest: *expected_profile_digest,
            observed_profile_digest: *observed_profile_digest,
        },
    }
}
