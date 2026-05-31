use forge_foundational::facade::AspectValue;

use super::*;

#[test]
fn terminal_harness_projection_keeps_aspect_value_typed_until_external_json() {
    let typed_diagnostic = RelationalDiagnosticValue::object([(
        "typed_aspect",
        RelationalDiagnosticValue::AspectValue(AspectValue::UInt64(7)),
    )]);

    let harness_projection =
        project_diagnostic_fields_for_terminal_harness_summary(typed_diagnostic);
    let TerminalHarnessSummaryProjection::Object(projected_fields) = &harness_projection else {
        panic!("expected typed harness diagnostic object projection");
    };
    let Some(TerminalHarnessSummaryProjection::Object(typed_aspect_terms)) =
        projected_fields.get("typed_aspect")
    else {
        panic!("expected aspect diagnostic terms before terminal JSON egress");
    };

    assert!(matches!(
        typed_aspect_terms.get("value_family"),
        Some(TerminalHarnessSummaryProjection::String(value)) if value == "UInt64"
    ));

    let external_json = harness_projection.into_external_harness_json();
    assert_eq!(external_json["typed_aspect"]["value_family"], "UInt64");
    assert!(
        external_json["typed_aspect"]["canonical_value_bytes"].is_array(),
        "terminal harness JSON should be the first JSON materialization"
    );
}
