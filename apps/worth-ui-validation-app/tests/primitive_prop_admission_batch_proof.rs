use worth_ui::facade::{
    SurfaceId, WorthUiPrimitivePropAdmissionStatus, WorthUiPrimitiveValueDenialCode,
};
use worth_ui_validation_app::reload::{ValidationAuthoredReloadEdit, ValidationSourcePackage};
use worth_ui_validation_app::{ValidationWorkbenchAuthoredInputs, ValidationWorkbenchLaunch};

const PRIMITIVE_SURFACE: &str = "worth.surface.preview.primitive.proof";

#[test]
fn primitive_prop_admission_accepts_valid_props_with_exact_counters() {
    let report = admission_report_for_source(stable_source_text());

    let WorthUiPrimitivePropAdmissionStatus::Accepted(receipt) = report.status() else {
        panic!("stable primitive source should admit");
    };
    let counters = report.counters();

    assert_eq!(receipt.surface_id(), PRIMITIVE_SURFACE);
    assert_eq!(receipt.prop_set().text(), "Worth primitive");
    assert_eq!(
        receipt.prop_set().background_color().hex_triplet(),
        "#2f7de1"
    );
    assert_eq!(
        receipt.prop_set().padding_token(),
        "validation.density.primitive.padding"
    );
    assert_eq!(
        receipt.prop_set().radius_token(),
        "validation.density.primitive.radius"
    );
    assert_eq!(receipt.prop_set().submit_payload(), "submit.primary");
    assert_eq!(
        receipt.prop_set().motion_duration_token(),
        "validation.density.primitive.motion.fast"
    );
    assert_eq!(counters.schema_count(), 17);
    assert_eq!(counters.authored_props_seen(), 51);
    assert_eq!(counters.defaults_applied(), 0);
    assert_eq!(counters.values_validated(), 17);
    assert_eq!(counters.denials_emitted(), 0);
    assert_ne!(report.schema_digest(), 0);
    assert_ne!(report.admission_digest(), 0);
}

#[test]
fn primitive_prop_defaults_are_admitted_through_the_same_batch_path() {
    for authored_line in [
        "    primitive_text \"Worth primitive\"\n",
        "    primitive_align center\n",
        "    primitive_padding validation.density.primitive.padding\n",
        "    primitive_radius validation.density.primitive.radius\n",
        "    primitive_background \"#2f7de1\"\n",
        "    primitive_foreground \"#f7f1e8\"\n",
        "    primitive_interaction submit\n",
        "    primitive_cursor pointer\n",
        "    primitive_focus focusable\n",
        "    primitive_disabled false\n",
        "    primitive_selected false\n",
        "    primitive_interaction_id worth.interaction.primitive.submit\n",
        "    primitive_submit_payload \"submit.primary\"\n",
        "    primitive_motion transition\n",
        "    primitive_motion_target primitive_background\n",
        "    primitive_motion_duration validation.density.primitive.motion.fast\n",
        "    primitive_motion_easing standard\n",
    ] {
        let report = admission_report_for_source(stable_source_text().replace(authored_line, ""));
        let WorthUiPrimitivePropAdmissionStatus::Accepted(receipt) = report.status() else {
            panic!("removed primitive prop should default through admission");
        };

        assert_eq!(
            receipt.prop_set().foreground_color().hex_triplet(),
            "#f7f1e8"
        );
        assert_eq!(report.counters().defaults_applied(), 1);
        assert_eq!(report.counters().values_validated(), 17);
    }
}

#[test]
fn primitive_prop_admission_reports_all_invalid_values_in_canonical_order() {
    let source = stable_source_text()
        .replace("primitive_align center", "primitive_align wide")
        .replace(
            "primitive_padding validation.density.primitive.padding",
            "primitive_padding 32",
        )
        .replace(
            "primitive_background \"#2f7de1\"",
            "primitive_background \"blue\"",
        );
    let report = admission_report_for_source(source);
    let denial_set = report
        .status()
        .denial_set()
        .expect("invalid source should reject with denial set");

    assert_eq!(report.counters().denials_emitted(), 3);
    assert_eq!(
        denial_set
            .denials()
            .iter()
            .map(|denial| denial.prop_key())
            .collect::<Vec<_>>(),
        vec![
            "primitive_align",
            "primitive_padding",
            "primitive_background"
        ]
    );
    assert_eq!(
        denial_set.denials()[0].denial_code(),
        WorthUiPrimitiveValueDenialCode::InvalidAlignKeyword
    );
    assert_eq!(
        denial_set.denials()[1].denial_code(),
        WorthUiPrimitiveValueDenialCode::InvalidMeasurementToken
    );
    assert_eq!(
        denial_set.denials()[2].denial_code(),
        WorthUiPrimitiveValueDenialCode::InvalidColorHex
    );
    assert_ne!(denial_set.denial_set_digest(), 0);
}

#[test]
fn denial_presentation_is_derived_from_the_receipt_not_the_proof_digest() {
    let report = admission_report_for_source(stable_source_text().replace(
        "primitive_background \"#2f7de1\"",
        "primitive_background \"blue\"",
    ));
    let denial = &report.status().denial_set().unwrap().denials()[0];
    let denial_digest = denial.denial_digest();
    let presentation = denial.presentation();

    assert!(denial.source_span().is_some());
    assert_eq!(presentation.title(), "Primitive value rejected");
    assert!(presentation
        .rows()
        .iter()
        .any(|row| row.label() == "expected"));
    assert!(presentation
        .rows()
        .iter()
        .any(|row| row.label() == "digest"));
    assert!(presentation
        .rows()
        .iter()
        .any(|row| row.label() == "source_span" && row.value() != "unavailable"));
    assert_eq!(denial.denial_digest(), denial_digest);
}

#[test]
fn unknown_primitive_prefixed_prop_is_rejected_as_schema_denial() {
    let source = stable_source_text().replace(
        "    primitive_foreground \"#f7f1e8\"\n",
        "    primitive_foreground \"#f7f1e8\"\n    primitive_backgrounnd \"#ffffff\"\n",
    );
    let report = admission_report_for_source(source);
    let denial = &report.status().denial_set().unwrap().denials()[0];

    assert_eq!(denial.prop_key(), "primitive_backgrounnd");
    assert_eq!(
        denial.denial_code(),
        WorthUiPrimitiveValueDenialCode::UnknownPrimitiveProp
    );
    assert_eq!(denial.expected_shape(), "a declared primitive prop");
}

#[test]
fn unknown_non_primitive_surface_props_are_ignored_by_primitive_admission() {
    let source = stable_source_text().replace(
        "    primitive_foreground \"#f7f1e8\"\n",
        "    primitive_foreground \"#f7f1e8\"\n    product_tracking_id \"hero\"\n",
    );
    let report = admission_report_for_source(source);

    assert!(report.status().accepted_receipt().is_some());
    assert_eq!(report.counters().authored_props_seen(), 52);
    assert_eq!(report.counters().denials_emitted(), 0);
}

#[test]
fn primitive_denial_set_digest_is_stable_and_changes_with_member_digest() {
    let blue = admission_report_for_source(stable_source_text().replace(
        "primitive_background \"#2f7de1\"",
        "primitive_background \"blue\"",
    ));
    let blue_again = admission_report_for_source(stable_source_text().replace(
        "primitive_background \"#2f7de1\"",
        "primitive_background \"blue\"",
    ));
    let purple = admission_report_for_source(stable_source_text().replace(
        "primitive_background \"#2f7de1\"",
        "primitive_background \"purple-ish\"",
    ));

    let blue_digest = blue.status().denial_set().unwrap().denial_set_digest();
    let blue_again_digest = blue_again
        .status()
        .denial_set()
        .unwrap()
        .denial_set_digest();
    let purple_digest = purple.status().denial_set().unwrap().denial_set_digest();

    assert_eq!(blue_digest, blue_again_digest);
    assert_ne!(blue_digest, purple_digest);
}

fn admission_report_for_source(
    source_text: String,
) -> worth_ui::facade::WorthUiPrimitivePropAdmissionReport {
    let prepared = ValidationWorkbenchLaunch::new()
        .prepare_from_authored_inputs(stable_inputs_with_source(source_text))
        .expect("validation workbench should prepare");
    prepared
        .runtime()
        .resolve_primitive_prop_admission_report(&primitive_surface_id())
}

fn primitive_surface_id() -> SurfaceId {
    SurfaceId::new(PRIMITIVE_SURFACE).expect("valid primitive surface id")
}

fn stable_inputs_with_source(source_text: String) -> ValidationWorkbenchAuthoredInputs {
    let inputs = ValidationWorkbenchAuthoredInputs::sample();
    let module_path = inputs.source().module_path().to_owned();
    inputs.with_source(ValidationSourcePackage::new(module_path, source_text))
}

fn stable_source_text() -> String {
    let mut source = ValidationWorkbenchAuthoredInputs::sample()
        .source()
        .source_text()
        .to_owned();
    for (key, value) in [
        ("primitive_text", "\"Worth primitive\""),
        ("primitive_align", "center"),
        ("primitive_padding", "validation.density.primitive.padding"),
        ("primitive_radius", "validation.density.primitive.radius"),
        ("primitive_background", "\"#2f7de1\""),
        ("primitive_foreground", "\"#f7f1e8\""),
        ("primitive_interaction", "submit"),
        ("primitive_cursor", "pointer"),
        ("primitive_focus", "focusable"),
        ("primitive_disabled", "false"),
        ("primitive_selected", "false"),
        (
            "primitive_interaction_id",
            "worth.interaction.primitive.submit",
        ),
        ("primitive_submit_payload", "\"submit.primary\""),
        ("primitive_motion", "transition"),
        ("primitive_motion_target", "primitive_background"),
        (
            "primitive_motion_duration",
            "validation.density.primitive.motion.fast",
        ),
        ("primitive_motion_easing", "standard"),
        ("flow_kind", "inline"),
        ("flow_gap", "validation.density.primitive.flow.gap.default"),
        (
            "flow_padding",
            "validation.density.primitive.flow.padding.default",
        ),
        ("flow_align", "center"),
        ("flow_cross_align", "center"),
        ("flow_fit", "hug"),
        ("flow_fill", "none"),
    ] {
        source = ValidationAuthoredReloadEdit::set_surface_prop(PRIMITIVE_SURFACE, key, value)
            .apply_to_source_text(&source)
            .expect("stable primitive prop source edit should apply");
    }
    ValidationAuthoredReloadEdit::remove_surface_prop(PRIMITIVE_SURFACE, "icon")
        .apply_to_source_text(&source)
        .unwrap_or(source)
}
