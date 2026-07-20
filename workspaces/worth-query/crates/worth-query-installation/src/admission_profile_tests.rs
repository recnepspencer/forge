use crate::facade::{
    WorthQueryInstallationAdmissionDenialKind, WorthQueryInstallationAdmissionProfile,
    WorthQueryInstallationSupportStatus, WorthQueryPortableDefinition,
    WorthQueryPortableDomainIdentity, WorthQueryPortableDomainPackage,
};

fn package() -> WorthQueryPortableDomainPackage {
    WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new(
        "worth.geometry",
        1,
        0,
    ))
    .requires_capability("query-read")
    .requires_configuration("query")
    .definition(WorthQueryPortableDefinition::graph_read_operation(
        "geometry.read",
        "direct-edge:relation-2",
    ))
}

#[test]
fn configuration_drift_has_an_exact_admission_denial() {
    let denial = WorthQueryInstallationAdmissionProfile::new("support-v1", "config-v1")
        .capability("query-read", WorthQueryInstallationSupportStatus::Admitted)
        .configuration("query", false)
        .admit(package().validate().unwrap())
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        WorthQueryInstallationAdmissionDenialKind::DisabledConfiguration
    );
    assert_eq!(denial.subject(), "query");
}

#[test]
fn operating_posture_drift_has_an_exact_admission_denial() {
    let package = package()
        .requires_operating_posture("snapshot-read")
        .validate()
        .unwrap();
    let denial = WorthQueryInstallationAdmissionProfile::new("support-v1", "config-v1")
        .capability("query-read", WorthQueryInstallationSupportStatus::Admitted)
        .configuration("query", true)
        .operating_requirement(
            "snapshot-read",
            WorthQueryInstallationSupportStatus::Deferred,
        )
        .admit(package)
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        WorthQueryInstallationAdmissionDenialKind::DeferredOperatingRequirement
    );
    assert_eq!(denial.subject(), "snapshot-read");
}

#[test]
fn irrelevant_profile_rows_do_not_fragment_admitted_package_identity() {
    let profile = |status| {
        WorthQueryInstallationAdmissionProfile::new("support-v1", "config-v1")
            .capability("query-read", WorthQueryInstallationSupportStatus::Admitted)
            .configuration("query", true)
            .operating_requirement("unused-posture", status)
    };
    let admitted = profile(WorthQueryInstallationSupportStatus::Admitted)
        .admit(package().validate().unwrap())
        .unwrap();
    let drifted = profile(WorthQueryInstallationSupportStatus::Unsupported)
        .admit(package().validate().unwrap())
        .unwrap();

    assert_eq!(admitted.admission_identity(), drifted.admission_identity());
}

#[test]
fn delimiter_like_profile_text_cannot_alias_admission_identity_fields() {
    let admit = |support: &str, configuration: &str| {
        WorthQueryInstallationAdmissionProfile::new(support, configuration)
            .capability("query-read", WorthQueryInstallationSupportStatus::Admitted)
            .configuration("query", true)
            .admit(package().validate().unwrap())
            .unwrap()
    };
    let left = admit("support\u{1f}configuration", "profile");
    let right = admit("support", "configuration\u{1f}profile");

    assert_ne!(left.admission_identity(), right.admission_identity());
}

#[test]
fn contradictory_profile_rows_are_denied_independent_of_declaration_order() {
    let admit = |first, second| {
        WorthQueryInstallationAdmissionProfile::new("support-v1", "config-v1")
            .capability("query-read", first)
            .capability("query-read", second)
            .configuration("query", true)
            .admit(package().validate().unwrap())
            .unwrap_err()
    };
    let left = admit(
        WorthQueryInstallationSupportStatus::Admitted,
        WorthQueryInstallationSupportStatus::Unsupported,
    );
    let right = admit(
        WorthQueryInstallationSupportStatus::Unsupported,
        WorthQueryInstallationSupportStatus::Admitted,
    );

    assert_eq!(
        left.kind(),
        WorthQueryInstallationAdmissionDenialKind::ConflictingProfileRow
    );
    assert_eq!(left, right);
    assert_eq!(left.subject(), "capability:query-read");
}

#[test]
fn empty_profile_identity_is_denied_before_requirement_admission() {
    let denial = WorthQueryInstallationAdmissionProfile::new("", "config-v1")
        .admit(package().validate().unwrap())
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        WorthQueryInstallationAdmissionDenialKind::InvalidSupportProfileIdentity
    );
}
