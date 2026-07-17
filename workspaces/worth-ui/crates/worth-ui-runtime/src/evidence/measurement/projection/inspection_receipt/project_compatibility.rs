use worth_ui_inspection::{
    UiInspectionMeasurementGenerationCompatibility, UiInspectionQueryWorldCompatibilityFailure,
};

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
        UiMeasurementGenerationCompatibility::IncompatibleWorld { reason } => {
            UiInspectionMeasurementGenerationCompatibility::IncompatibleWorld {
                reason: match reason {
                    crate::evidence::UiQueryWorldCompatibilityFailure::InstalledAuthorityMismatch => UiInspectionQueryWorldCompatibilityFailure::InstalledAuthorityMismatch,
                    crate::evidence::UiQueryWorldCompatibilityFailure::SnapshotBasisMismatch => UiInspectionQueryWorldCompatibilityFailure::SnapshotBasisMismatch,
                    crate::evidence::UiQueryWorldCompatibilityFailure::QueryAuthorityUnavailable => UiInspectionQueryWorldCompatibilityFailure::QueryAuthorityUnavailable,
                },
            }
        }
        UiMeasurementGenerationCompatibility::IncompatibleHostProfile {
            expected_profile_digest,
            observed_profile_digest,
        } => UiInspectionMeasurementGenerationCompatibility::IncompatibleHostProfile {
            expected_profile_digest: *expected_profile_digest,
            observed_profile_digest: *observed_profile_digest,
        },
    }
}
