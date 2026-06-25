use worth_ui::facade::{WorthUiFontSizeValue, WorthUiPaddingValue};

use super::{
    ValidationWorkbenchAuthoredInputs, ValidationWorkbenchLaunch, ValidationWorkbenchLaunchError,
};
use crate::reload::{ValidationAppearanceSource, ValidationDensitySource};

#[test]
fn sample_startup_inputs_apply_appearance_and_density_through_launch_path() {
    let prepared = ValidationWorkbenchLaunch::new()
        .prepare_from_authored_inputs(ValidationWorkbenchAuthoredInputs::sample())
        .expect("sample validation workbench launch should remain valid");
    let receipt = prepared.header_appearance_plan().execute_frame();

    assert_eq!(
        receipt.font_size(),
        WorthUiFontSizeValue::from_px("13px").expect("valid font size")
    );
    assert_eq!(
        receipt.container_padding(),
        &WorthUiPaddingValue::from_shorthand_px("4px 8px").expect("valid container padding")
    );
}

#[test]
fn observed_startup_inputs_include_live_view_source() {
    let prepared = ValidationWorkbenchLaunch::new()
        .prepare()
        .expect("observed validation workbench launch should include live-view source");

    assert!(
        prepared.authored_inputs().live_view().is_some(),
        "normal launch must carry the observed live-view source into authored inputs"
    );
}

#[test]
fn invalid_startup_appearance_returns_typed_rejection_instead_of_panicking() {
    let result = ValidationWorkbenchLaunch::new().prepare_from_authored_inputs(
        ValidationWorkbenchAuthoredInputs::new(crate::reload::ValidationSourcePackage::sample())
            .with_appearance(ValidationAppearanceSource::new(
                "validation.appearance.header.font_size = nope",
            ))
            .with_density(ValidationDensitySource::new(
                "validation.density.header.container_padding = 4px 8px 4px 8px",
            )),
    );

    assert!(matches!(
        result,
        Err(ValidationWorkbenchLaunchError::AuthoredStartupRejected(
            "header appearance+density"
        ))
    ));
}
