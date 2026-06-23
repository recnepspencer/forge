use std::collections::BTreeSet;

use super::{
    primitive_authored_prop_schemas, WorthUiPrimitiveAuthoredPropSchema,
    WorthUiPrimitiveAuthoredValueKind, WorthUiPrimitiveDefaultPolicy,
};

#[test]
fn primitive_schema_declarations_are_complete_unique_and_self_certifying() {
    let mut schema_ids = BTreeSet::new();
    let mut prop_keys = BTreeSet::new();

    for schema in primitive_authored_prop_schemas() {
        assert!(!schema.schema_id().is_empty());
        assert!(schema.schema_id().starts_with("worth.primitive.prop."));
        assert!(!schema.prop_key().is_empty());
        assert!(schema.prop_key().starts_with("primitive_"));
        assert!(!schema.default_value().is_empty());
        assert!(!schema.expected_value_syntax().is_empty());
        assert!(!schema.examples().is_empty());
        assert_eq!(
            schema.default_policy,
            WorthUiPrimitiveDefaultPolicy::Defaulted
        );
        assert_eq!(schema.denial_code(), schema.value_kind().denial_code());
        assert!(schema_ids.insert(schema.schema_id()));
        assert!(prop_keys.insert(schema.prop_key()));
        assert_default_value_matches_schema_kind(schema);
    }
}

fn assert_default_value_matches_schema_kind(schema: &WorthUiPrimitiveAuthoredPropSchema) {
    match schema.value_kind() {
        WorthUiPrimitiveAuthoredValueKind::Text => {}
        WorthUiPrimitiveAuthoredValueKind::MeasurementToken => {
            assert!(schema.default_value().contains('.'));
        }
        WorthUiPrimitiveAuthoredValueKind::Color => assert_hex_triplet(schema.default_value()),
        WorthUiPrimitiveAuthoredValueKind::Align => {
            assert!(matches!(schema.default_value(), "start" | "center" | "end"));
        }
        WorthUiPrimitiveAuthoredValueKind::InteractionKind => {
            assert!(matches!(schema.default_value(), "submit" | "none"));
        }
        WorthUiPrimitiveAuthoredValueKind::Cursor => {
            assert!(matches!(schema.default_value(), "default" | "pointer"));
        }
        WorthUiPrimitiveAuthoredValueKind::Focus => {
            assert!(matches!(schema.default_value(), "none" | "focusable"));
        }
        WorthUiPrimitiveAuthoredValueKind::Boolean => {
            assert!(matches!(schema.default_value(), "true" | "false"));
        }
        WorthUiPrimitiveAuthoredValueKind::MotionKind => {
            assert!(matches!(schema.default_value(), "none" | "transition"));
        }
        WorthUiPrimitiveAuthoredValueKind::MotionTarget => {
            assert!(schema.default_value().starts_with("primitive_"));
        }
        WorthUiPrimitiveAuthoredValueKind::Easing => {
            assert!(matches!(
                schema.default_value(),
                "linear" | "standard" | "ease_in" | "ease_out"
            ));
        }
        WorthUiPrimitiveAuthoredValueKind::Unknown => {
            panic!("unknown primitive value kind must not be declared in schemas");
        }
    }
}

fn assert_hex_triplet(value: &str) {
    let hex = value
        .strip_prefix('#')
        .expect("color default must start with #");
    assert_eq!(hex.len(), 6);
    assert!(hex.bytes().all(|byte| byte.is_ascii_hexdigit()));
}
