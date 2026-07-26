use worth_ui::facade::measurement_exchange::{
    UiHostMeasurementObservationValue, UiHostMeasurementRequest, UiMeasurementEvidenceFamily,
    UiMeasurementRequestFamily, UiMeasurementRequestIdentity, UiViewportExtentObservation,
    UiViewportExtentRequest, WorthUiMeasurementHostAdapter,
};
use worth_ui_runtime::facade::host::{
    UiHeadlessRecorderCapacity, WorthUiHeadlessRecorder, WorthUiHostCapability,
    WorthUiOperationalHostAdapter,
};

#[test]
fn default_recorder_remains_measurement_free() {
    let recorder = WorthUiHeadlessRecorder::default();

    assert_eq!(
        recorder
            .operational_capability_report()
            .observed_capabilities(),
        &[WorthUiHostCapability::MountedFrameRecording]
    );
    assert_eq!(
        recorder
            .measurement_environment_report()
            .generation_for(UiMeasurementRequestFamily::ViewportExtent),
        None
    );
}

#[test]
fn configured_recorder_reports_and_observes_one_fixed_viewport() {
    let expected = UiViewportExtentObservation {
        width: 1024.0,
        height: 768.0,
    };
    let recorder = WorthUiHeadlessRecorder::with_viewport_extent(
        UiHeadlessRecorderCapacity::production_default(),
        expected,
    );
    let capability_report = recorder.operational_capability_report();
    assert_eq!(
        capability_report.observed_capabilities(),
        &[
            WorthUiHostCapability::MountedFrameRecording,
            WorthUiHostCapability::ViewportObservation,
        ]
    );
    assert_eq!(
        recorder
            .measurement_environment_report()
            .generation_for(UiMeasurementRequestFamily::ViewportExtent),
        Some(1)
    );
    assert_eq!(
        recorder
            .measurement_environment_report()
            .generation_for(UiMeasurementRequestFamily::ViewportExtent),
        Some(1),
        "fixed headless mechanics must not invent environment churn"
    );

    let request = UiHostMeasurementRequest::viewport_extent(
        UiMeasurementRequestIdentity::new(1),
        UiMeasurementEvidenceFamily::ViewportExtent,
        UiViewportExtentRequest,
        &capability_report,
    )
    .expect("configured viewport capability admits the request");
    assert_eq!(
        recorder.observe_measurement(&request),
        UiHostMeasurementObservationValue::ViewportExtent(expected)
    );
}
