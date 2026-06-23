use super::support::{content_source, runtime_for_source, surface_id};

#[test]
fn accessibility_name_reaches_host_facing_draw_plan_receipt() {
    let runtime = runtime_for_source(content_source(&[
        ("content_text", "\"Submit\""),
        ("content_accessibility_name", "\"Submit confirmation\""),
    ]));

    let primitive = runtime
        .resolve_primitive_proof(&surface_id())
        .expect("primitive proof resolves");
    let draw_plan = primitive.draw_plan(400.0, 240.0);

    assert_eq!(
        draw_plan.receipt().content().accessibility_name(),
        Some("Submit confirmation")
    );
}

#[test]
fn empty_accessibility_name_rejects_through_content_schema() {
    let runtime = runtime_for_source(content_source(&[
        ("content_text", "\"Submit\""),
        ("content_accessibility_name", "\"\""),
    ]));

    let denial = runtime
        .resolve_primitive_content_admission_report(&surface_id())
        .status()
        .denial_set()
        .expect("empty accessibility names reject")
        .denials()[0]
        .clone();

    assert_eq!(denial.prop_key(), "content_accessibility_name");
    assert_eq!(
        denial.schema_id(),
        "worth.primitive.content.prop.content_accessibility_name"
    );
}
