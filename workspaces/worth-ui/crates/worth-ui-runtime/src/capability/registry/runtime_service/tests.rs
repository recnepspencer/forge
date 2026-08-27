use super::{
    support::{UiRuntimeServiceSupport, UiRuntimeServiceSupportPosture},
    UiRuntimeServiceFamily,
};

#[test]
fn service_family_catalog_is_closed_and_stably_named() {
    assert_eq!(
        UiRuntimeServiceFamily::ALL.map(UiRuntimeServiceFamily::stable_name),
        [
            "portal",
            "focus",
            "motion",
            "command-routing",
            "scroll",
            "selection",
        ]
    );
}

#[test]
fn support_is_explicit_per_family_without_installing_unrequested_families() {
    let support = UiRuntimeServiceSupport::none_installed()
        .with_installed(UiRuntimeServiceFamily::Portal)
        .with_installed(UiRuntimeServiceFamily::Scroll);

    for family in UiRuntimeServiceFamily::ALL {
        let expected = if matches!(
            family,
            UiRuntimeServiceFamily::Portal | UiRuntimeServiceFamily::Scroll
        ) {
            UiRuntimeServiceSupportPosture::Installed
        } else {
            UiRuntimeServiceSupportPosture::Unsupported
        };
        assert_eq!(support.posture(family), expected);
    }
}
