use std::path::PathBuf;

use worth_store_offline_integrity_observer::{
    OfflineIntegrityObservationLimits, OfflineIntegrityObservationLimitsDenial,
    OfflineIntegrityObservationRequest, OfflineIntegrityObservationRequestDenial,
    OfflineIntegrityReportBoundaryDenial, OfflineIntegrityReportDestination,
    OfflineIntegrityReportDestinationDenial, OFFLINE_INTEGRITY_ROOT_PROTOCOL_DECLARATIONS,
    PHYSICAL_INTEGRITY_OBSERVATION_PROTOCOL_IDENTITY,
    PHYSICAL_INTEGRITY_OBSERVATION_PROTOCOL_VERSION,
};
use worth_store_physical_format::integrity_declarations::PhysicalIntegrityArtifactFamily;

fn admitted_limits() -> OfflineIntegrityObservationLimits {
    OfflineIntegrityObservationLimits::new(100, 4096, 4, 8, 0, 250, 2048).unwrap()
}

#[test]
fn request_preserves_every_declared_bound_and_report_boundary() {
    let request = OfflineIntegrityObservationRequest::new(
        PathBuf::from("fixture/store"),
        admitted_limits(),
        OfflineIntegrityReportDestination::file(PathBuf::from("fixture/reports/result.json"))
            .unwrap(),
    )
    .unwrap();

    let limits = request.limits();
    assert_eq!(limits.maximum_entries(), 100);
    assert_eq!(limits.maximum_bytes(), 4096);
    assert_eq!(limits.maximum_open_files(), 4);
    assert_eq!(limits.maximum_depth(), 8);
    assert_eq!(limits.maximum_symlinks(), 0);
    assert_eq!(limits.maximum_elapsed_milliseconds(), 250);
    assert_eq!(limits.maximum_report_bytes(), 2048);
    assert_eq!(request.report().maximum_bytes(), 2048);
    assert_eq!(
        request.report().protocol_identity(),
        &PHYSICAL_INTEGRITY_OBSERVATION_PROTOCOL_IDENTITY
    );
    assert_eq!(
        request.report().protocol_version(),
        PHYSICAL_INTEGRITY_OBSERVATION_PROTOCOL_VERSION
    );
    assert_eq!(request.report().compatibility_window().earliest().get(), 1);
    assert_eq!(request.report().compatibility_window().latest().get(), 1);
}

#[test]
fn each_required_positive_limit_has_a_distinct_denial() {
    let cases = [
        (
            OfflineIntegrityObservationLimits::new(0, 1, 1, 1, 0, 1, 1),
            OfflineIntegrityObservationLimitsDenial::ZeroEntries,
        ),
        (
            OfflineIntegrityObservationLimits::new(1, 0, 1, 1, 0, 1, 1),
            OfflineIntegrityObservationLimitsDenial::ZeroBytes,
        ),
        (
            OfflineIntegrityObservationLimits::new(1, 1, 0, 1, 0, 1, 1),
            OfflineIntegrityObservationLimitsDenial::ZeroOpenFiles,
        ),
        (
            OfflineIntegrityObservationLimits::new(1, 1, 1, 0, 0, 1, 1),
            OfflineIntegrityObservationLimitsDenial::ZeroDepth,
        ),
        (
            OfflineIntegrityObservationLimits::new(1, 1, 1, 1, 0, 0, 1),
            OfflineIntegrityObservationLimitsDenial::ZeroElapsedMilliseconds,
        ),
        (
            OfflineIntegrityObservationLimits::new(1, 1, 1, 1, 0, 1, 0),
            OfflineIntegrityObservationLimitsDenial::ZeroReportBytes,
        ),
    ];

    for (result, expected) in cases {
        assert_eq!(result.unwrap_err(), expected);
    }
}

#[test]
fn report_destination_cannot_be_declared_inside_the_store_root() {
    assert_eq!(
        OfflineIntegrityReportDestination::file(PathBuf::new()),
        Err(OfflineIntegrityReportDestinationDenial::EmptyFilePath)
    );

    let denial = OfflineIntegrityObservationRequest::new(
        PathBuf::from("fixture/store"),
        admitted_limits(),
        OfflineIntegrityReportDestination::file(PathBuf::from("fixture/store/report.json"))
            .unwrap(),
    )
    .unwrap_err();

    assert_eq!(
        denial,
        OfflineIntegrityObservationRequestDenial::ReportBoundary(
            OfflineIntegrityReportBoundaryDenial::DestinationInsideDeclaredStoreRoot,
        )
    );

    let standard_output = OfflineIntegrityObservationRequest::new(
        PathBuf::from("fixture/store"),
        admitted_limits(),
        OfflineIntegrityReportDestination::standard_output(),
    )
    .unwrap();
    assert!(standard_output.report().destination().is_standard_output());
}

#[test]
fn phase_three_root_slice_uses_only_declaration_facade_facts() {
    let declarations = OFFLINE_INTEGRITY_ROOT_PROTOCOL_DECLARATIONS;
    assert_eq!(
        declarations.current_selector().family(),
        PhysicalIntegrityArtifactFamily::CurrentRootSelector
    );
    assert_eq!(
        declarations.previous_selector().family(),
        PhysicalIntegrityArtifactFamily::PreviousRootSelector
    );
    assert_eq!(
        declarations.root_manifest().family(),
        PhysicalIntegrityArtifactFamily::RootManifest
    );
    for declaration in [
        declarations.current_selector(),
        declarations.previous_selector(),
        declarations.root_manifest(),
    ] {
        assert_eq!(declaration.version().format_version(), 1);
        assert_eq!(declaration.version().envelope_schema(), Some(2));
        assert_eq!(declaration.checksums().len(), 1);
    }
}
