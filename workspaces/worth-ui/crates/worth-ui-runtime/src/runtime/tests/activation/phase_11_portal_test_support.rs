use worth_ui_host_contract::{
    UiHostObservationValue, UiMeasurementEvidenceFamily, UiMeasurementRequest,
    UiMeasurementRequestIdentity, UiPortalAnchorRectObservation, UiPortalAnchorRectRequest,
    WorthUiMeasurementHostAdapter,
};

struct PortalAnchorAdapter(UiPortalAnchorRectObservation);

impl WorthUiMeasurementHostAdapter for PortalAnchorAdapter {
    fn observe_measurement(&self, _request: &UiMeasurementRequest) -> UiHostObservationValue {
        UiHostObservationValue::PortalAnchorRect(self.0)
    }
}

pub(super) fn submit_portal_observation(
    runtime: &mut crate::runtime::WorthUiRuntimeFrameworkLoop,
    target: u64,
    rect: UiPortalAnchorRectObservation,
) -> crate::runtime::WorthUiFrameworkTurnCompletion<'_> {
    submit_portal_observation_in(
        runtime,
        target,
        rect,
        worth_ui_inspection::UiEvidenceAuthorityGeneration::new(18),
        crate::host::UiPortalAnchorCoordinateSpacePosture::PortalLayer,
    )
}

pub(super) fn submit_portal_observation_in(
    runtime: &mut crate::runtime::WorthUiRuntimeFrameworkLoop,
    target: u64,
    rect: UiPortalAnchorRectObservation,
    generation: worth_ui_inspection::UiEvidenceAuthorityGeneration,
    posture: crate::host::UiPortalAnchorCoordinateSpacePosture,
) -> crate::runtime::WorthUiFrameworkTurnCompletion<'_> {
    let report = crate::evidence::measurement::projection::fact_test_support::capability_report(77);
    let profile = crate::host::UiHostMeasurementAssumptionProfile::from_capability_report(
        &report, 11, 22, 33, 44,
    );
    runtime.execute_framework_turn(|turn| {
        turn.host_measurement(|source| {
            source
                .collect_and_submit(
                    &PortalAnchorAdapter(rect),
                    UiMeasurementRequestIdentity::new(981),
                    UiMeasurementEvidenceFamily::PortalAnchorRect,
                    crate::host::UiHostMeasurementNeed::PortalAnchorRect(
                        UiPortalAnchorRectRequest::new(target),
                    ),
                    &report,
                    generation,
                    crate::host::UiHostMeasurementNormalizationContext::portal_anchor_logical_exact_in(
                        posture, profile,
                    ),
                )
                .expect("ordinary portal observation enters the allocation stream");
        });
    })
}

pub(super) fn committed(
    outcome: crate::runtime::UiAllocationReplanTransactionOutcome,
) -> crate::runtime::UiCommittedAllocationReplan {
    match outcome {
        crate::runtime::UiAllocationReplanTransactionOutcome::Committed(value)
        | crate::runtime::UiAllocationReplanTransactionOutcome::Replayed(value) => value,
        denied => panic!("portal locality must commit atomically: {denied:?}"),
    }
}
