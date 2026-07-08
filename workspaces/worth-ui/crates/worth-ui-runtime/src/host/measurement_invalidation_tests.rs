use worth_ui_host_contract::{
    UiMeasurementRequestIdentity, WorthUiHostCapability,
    WorthUiHostCapabilityObservationGeneration, WorthUiHostCapabilityReport,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use super::tests::measurement_result_test_support::normalized_viewport_result;
use super::{
    admit_current_host_measurement_evidence, UiHostMeasurementAssumptionProfile,
    UiHostMeasurementEvidenceDenial, UiHostMeasurementFreshnessWitness,
    UiHostMeasurementInvalidationReason,
};

#[test]
fn stale_host_measurement_evidence_invalidates_when_generation_or_profile_drift() {
    let current_report =
        WorthUiHostCapabilityReport::available(vec![WorthUiHostCapability::ViewportObservation])
            .with_observation_generation(WorthUiHostCapabilityObservationGeneration::new(5));
    let current_profile =
        UiHostMeasurementAssumptionProfile::from_capability_report(&current_report, 10, 20, 30, 40);
    let result = normalized_viewport_result(
        UiMeasurementRequestIdentity::new(91),
        &current_report,
        UiEvidenceAuthorityGeneration::new(100),
        current_profile,
    );

    let capability_drift_profile = UiHostMeasurementAssumptionProfile::from_capability_report(
        &WorthUiHostCapabilityReport::available(vec![
            WorthUiHostCapability::ViewportObservation,
            WorthUiHostCapability::DpiObservation,
        ])
        .with_observation_generation(WorthUiHostCapabilityObservationGeneration::new(5)),
        10,
        20,
        30,
        40,
    );

    let cases = [
        (
            UiEvidenceAuthorityGeneration::new(101),
            current_profile,
            UiHostMeasurementInvalidationReason::EvidenceGenerationDrift {
                recorded: UiEvidenceAuthorityGeneration::new(100),
                current: UiEvidenceAuthorityGeneration::new(101),
            },
        ),
        (
            UiEvidenceAuthorityGeneration::new(100),
            UiHostMeasurementAssumptionProfile::from_capability_report(
                &current_report.clone().with_observation_generation(
                    WorthUiHostCapabilityObservationGeneration::new(6),
                ),
                10,
                20,
                30,
                40,
            ),
            UiHostMeasurementInvalidationReason::CapabilityObservationGenerationDrift {
                recorded: WorthUiHostCapabilityObservationGeneration::new(5),
                current: WorthUiHostCapabilityObservationGeneration::new(6),
            },
        ),
        (
            UiEvidenceAuthorityGeneration::new(100),
            capability_drift_profile,
            UiHostMeasurementInvalidationReason::CapabilityProfileDrift {
                recorded: current_profile.capability_profile_digest(),
                current: capability_drift_profile.capability_profile_digest(),
            },
        ),
        (
            UiEvidenceAuthorityGeneration::new(100),
            UiHostMeasurementAssumptionProfile::from_capability_report(
                &current_report,
                11,
                20,
                30,
                40,
            ),
            UiHostMeasurementInvalidationReason::ViewportAssumptionDrift {
                recorded: 10,
                current: 11,
            },
        ),
        (
            UiEvidenceAuthorityGeneration::new(100),
            UiHostMeasurementAssumptionProfile::from_capability_report(
                &current_report,
                10,
                21,
                30,
                40,
            ),
            UiHostMeasurementInvalidationReason::DpiAssumptionDrift {
                recorded: 20,
                current: 21,
            },
        ),
        (
            UiEvidenceAuthorityGeneration::new(100),
            UiHostMeasurementAssumptionProfile::from_capability_report(
                &current_report,
                10,
                20,
                31,
                40,
            ),
            UiHostMeasurementInvalidationReason::FontAssumptionDrift {
                recorded: 30,
                current: 31,
            },
        ),
        (
            UiEvidenceAuthorityGeneration::new(100),
            UiHostMeasurementAssumptionProfile::from_capability_report(
                &current_report,
                10,
                20,
                30,
                41,
            ),
            UiHostMeasurementInvalidationReason::AdapterProfileDrift {
                recorded: 40,
                current: 41,
            },
        ),
    ];

    for (generation, profile, expected) in cases {
        let denial = admit_current_host_measurement_evidence(
            &result,
            UiHostMeasurementFreshnessWitness::new(generation, profile),
        )
        .unwrap_err();
        assert_eq!(denial, UiHostMeasurementEvidenceDenial::Stale(expected));
    }
}
