use crate::runtime::WorthUiPrimitiveContentValueDenialCode;

use super::support::{content_source, runtime_for_source, surface_id};

#[test]
fn interaction_shaped_content_props_reject_as_unknown_content_props() {
    let runtime = runtime_for_source(content_source(&[
        ("content_on_click", "submit"),
        ("content_interaction", "command"),
        ("content_payload", "\"payload\""),
    ]));

    let report = runtime.resolve_primitive_content_admission_report(&surface_id());
    let denial_set = report
        .status()
        .denial_set()
        .expect("content cannot own interaction props");

    assert_eq!(
        denial_set
            .denials()
            .iter()
            .map(|denial| denial.prop_key())
            .collect::<Vec<_>>(),
        vec!["content_on_click", "content_interaction", "content_payload"]
    );
    assert!(denial_set.denials().iter().all(|denial| {
        denial.denial_code() == WorthUiPrimitiveContentValueDenialCode::UnknownContentProp
    }));
}
