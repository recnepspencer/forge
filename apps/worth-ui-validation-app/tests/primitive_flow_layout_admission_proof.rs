use worth_ui::facade::{
    SurfaceId, WorthUiFlowLayoutAdmissionStatus, WorthUiFlowLayoutCrossAlign,
    WorthUiFlowLayoutKind, WorthUiFlowLayoutValueDenialCode,
};
use worth_ui_validation_app::reload::ValidationAuthoredReloadEdit;
use worth_ui_validation_app::reload::ValidationSourcePackage;
use worth_ui_validation_app::{ValidationWorkbenchAuthoredInputs, ValidationWorkbenchLaunch};

const PRIMITIVE_SURFACE: &str = "worth.surface.preview.primitive.proof";
const STABLE_AUTHORED_PRIMITIVE_SURFACE_PROP_COUNT: usize = 52;

#[test]
fn flow_layout_admission_accepts_valid_props_with_exact_counters() {
    let report = admission_report_for_source(stable_source_text());
    let WorthUiFlowLayoutAdmissionStatus::Accepted(receipt) = report.status() else {
        panic!("stable flow layout source should admit");
    };
    let counters = report.counters();

    assert_eq!(receipt.surface_id(), PRIMITIVE_SURFACE);
    assert_eq!(receipt.prop_set().kind(), WorthUiFlowLayoutKind::Inline);
    assert_eq!(
        receipt.prop_set().gap_token(),
        "validation.density.primitive.flow.gap.default"
    );
    assert_eq!(receipt.prop_set().gap_points(), 8.0);
    assert_eq!(
        receipt.prop_set().padding_token(),
        "validation.density.primitive.flow.padding.default"
    );
    assert_eq!(receipt.prop_set().padding_points(), 32.0);
    assert_eq!(
        receipt.prop_set().cross_align(),
        WorthUiFlowLayoutCrossAlign::Center
    );
    assert_eq!(counters.schema_count(), 7);
    assert_eq!(
        counters.authored_props_seen(),
        STABLE_AUTHORED_PRIMITIVE_SURFACE_PROP_COUNT
    );
    assert_eq!(counters.defaults_applied(), 0);
    assert_eq!(counters.values_validated(), 7);
    assert_eq!(counters.denials_emitted(), 0);
    assert_ne!(report.schema_digest(), 0);
    assert_ne!(report.admission_digest(), 0);
}

#[test]
fn flow_layout_defaults_are_admitted_through_the_same_batch_path() {
    for authored_line in [
        "    flow_kind inline\n",
        "    flow_gap validation.density.primitive.flow.gap.default\n",
        "    flow_padding validation.density.primitive.flow.padding.default\n",
        "    flow_align center\n",
        "    flow_cross_align center\n",
        "    flow_fit hug\n",
        "    flow_fill none\n",
    ] {
        let report = admission_report_for_source(stable_source_text().replace(authored_line, ""));
        let WorthUiFlowLayoutAdmissionStatus::Accepted(receipt) = report.status() else {
            panic!("removed flow layout prop should default through admission");
        };

        assert_eq!(receipt.prop_set().kind(), WorthUiFlowLayoutKind::Inline);
        assert_eq!(report.counters().defaults_applied(), 1);
        assert_eq!(report.counters().values_validated(), 7);
    }
}

#[test]
fn every_declared_flow_kind_admits_through_the_same_schema() {
    for (authored_kind, expected_kind) in [
        ("row", WorthUiFlowLayoutKind::Row),
        ("column", WorthUiFlowLayoutKind::Column),
        ("inline", WorthUiFlowLayoutKind::Inline),
        ("stack", WorthUiFlowLayoutKind::Stack),
        ("grid", WorthUiFlowLayoutKind::Grid),
        ("spacer", WorthUiFlowLayoutKind::Spacer),
    ] {
        let report = admission_report_for_source(
            stable_source_text().replace("flow_kind inline", &format!("flow_kind {authored_kind}")),
        );
        let WorthUiFlowLayoutAdmissionStatus::Accepted(receipt) = report.status() else {
            panic!("declared flow kind {authored_kind} should admit");
        };

        assert_eq!(receipt.prop_set().kind(), expected_kind);
    }
}

#[test]
fn flow_layout_admission_reports_all_invalid_values_in_canonical_order() {
    let source = stable_source_text_with_edits(&[
        ("flow_kind", "diagonal"),
        ("flow_gap", "fat"),
        ("flow_padding", "32"),
        ("flow_align", "wide"),
        ("flow_cross_align", "stretch"),
        ("flow_fit", "maybe"),
        ("flow_fill", "sideways"),
    ]);
    let report = admission_report_for_source(source);
    let denial_set = report
        .status()
        .denial_set()
        .expect("invalid flow source should reject with denial set");

    assert_eq!(report.counters().denials_emitted(), 7);
    assert_eq!(
        denial_set
            .denials()
            .iter()
            .map(|denial| denial.prop_key())
            .collect::<Vec<_>>(),
        vec![
            "flow_kind",
            "flow_gap",
            "flow_padding",
            "flow_align",
            "flow_cross_align",
            "flow_fit",
            "flow_fill"
        ]
    );
    assert_eq!(
        denial_set.denials()[0].denial_code(),
        WorthUiFlowLayoutValueDenialCode::InvalidKind
    );
    assert_eq!(
        denial_set.denials()[1].denial_code(),
        WorthUiFlowLayoutValueDenialCode::InvalidMeasurementToken
    );
    assert_eq!(
        denial_set.denials()[4].denial_code(),
        WorthUiFlowLayoutValueDenialCode::InvalidCrossAlign
    );
    assert!(denial_set.denials()[1].source_span().is_some());
    assert_ne!(denial_set.denial_set_digest(), 0);
}

#[test]
fn flow_layout_measurements_reject_raw_numbers_and_pixels() {
    for invalid_value in ["6", "\"8px\"", "NaN", "inf"] {
        let report = admission_report_for_source(stable_source_text_with_edits(&[(
            "flow_gap",
            invalid_value,
        )]));
        let denial = &report.status().denial_set().unwrap().denials()[0];

        assert_eq!(denial.prop_key(), "flow_gap");
        assert_eq!(
            denial.denial_code(),
            WorthUiFlowLayoutValueDenialCode::InvalidMeasurementToken
        );
        assert!(denial.source_span().is_some());
    }
}

#[test]
fn equivalent_flow_measurements_lower_to_equivalent_receipts() {
    let default = admission_report_for_source(stable_source_text_with_edits(&[(
        "flow_gap",
        "validation.density.primitive.flow.gap.default",
    )]));
    let alias = admission_report_for_source(stable_source_text_with_edits(&[(
        "flow_gap",
        "validation.density.primitive.flow.gap.alias",
    )]));
    let default = default.status().accepted_receipt().unwrap();
    let alias = alias.status().accepted_receipt().unwrap();

    assert_eq!(
        default.prop_set().gap_points(),
        alias.prop_set().gap_points()
    );
}

#[test]
fn flow_layout_unknown_policy_is_namespace_scoped() {
    let accepted = admission_report_for_source(stable_source_text().replace(
        "    flow_fill none\n",
        "    flow_fill none\n    product_tracking_id \"hero\"\n",
    ));
    assert!(accepted.status().accepted_receipt().is_some());
    assert_eq!(
        accepted.counters().authored_props_seen(),
        STABLE_AUTHORED_PRIMITIVE_SURFACE_PROP_COUNT + 1
    );

    let rejected = admission_report_for_source(stable_source_text().replace(
        "    flow_fill none\n",
        "    flow_fill none\n    flow_gapp 4\n",
    ));
    let denial = &rejected.status().denial_set().unwrap().denials()[0];
    assert_eq!(denial.prop_key(), "flow_gapp");
    assert_eq!(
        denial.denial_code(),
        WorthUiFlowLayoutValueDenialCode::UnknownFlowLayoutProp
    );
}

#[test]
fn flow_padding_tokens_resolve_to_edge_receipts() {
    let report = admission_report_for_source(stable_source_text_with_edits(&[(
        "flow_padding",
        "validation.density.primitive.flow.padding.wide_shallow",
    )]));
    let receipt = report
        .status()
        .accepted_receipt()
        .expect("wide shallow padding token should admit");
    let padding = receipt.prop_set().padding_edges();

    assert_eq!(padding.top(), 8.0);
    assert_eq!(padding.right(), 64.0);
    assert_eq!(padding.bottom(), 8.0);
    assert_eq!(padding.left(), 64.0);
}

#[test]
fn flow_layout_denial_presentation_and_digest_are_receipt_derived() {
    let report = admission_report_for_source(stable_source_text_with_edits(&[("flow_gap", "fat")]));
    let denial = &report.status().denial_set().unwrap().denials()[0];
    let denial_digest = denial.denial_digest();
    let presentation = denial.presentation();

    assert_eq!(presentation.title(), "Flow layout value rejected");
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

fn admission_report_for_source(
    source_text: String,
) -> worth_ui::facade::WorthUiFlowLayoutAdmissionReport {
    let prepared = ValidationWorkbenchLaunch::new()
        .prepare_from_authored_inputs(stable_inputs_with_source(source_text))
        .expect("validation workbench should prepare");
    prepared
        .runtime()
        .resolve_flow_layout_admission_report(&primitive_surface_id())
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
    stable_source_text_with_edits(&[])
}

fn stable_source_text_with_edits(edits: &[(&str, &str)]) -> String {
    let mut source = ValidationWorkbenchAuthoredInputs::sample()
        .source()
        .source_text()
        .to_owned();
    for (key, value) in [
        ("primitive_align", "center"),
        ("primitive_padding", "validation.density.primitive.padding"),
        ("primitive_radius", "validation.density.primitive.radius"),
        ("primitive_background", "\"#2f7de1\""),
        ("primitive_foreground", "\"#f7f1e8\""),
        ("primitive_interaction", "submit"),
        ("primitive_cursor", "pointer"),
        ("primitive_focus", "focusable"),
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
    ]
    .into_iter()
    .chain(edits.iter().copied())
    {
        source = ValidationAuthoredReloadEdit::set_surface_prop(PRIMITIVE_SURFACE, key, value)
            .apply_to_source_text(&source)
            .expect("stable flow source edit should apply");
    }
    source
}
