use crate::runtime::{WorthUiRuntimeFactFamily, WorthUiSemanticSliceId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiFlowLayoutValueKind {
    Kind,
    MeasurementToken,
    Align,
    CrossAlign,
    Fit,
    Fill,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiFlowLayoutValueDenialCode {
    InvalidKind,
    InvalidMeasurementToken,
    InvalidAlign,
    InvalidCrossAlign,
    InvalidFit,
    InvalidFill,
    UnknownFlowLayoutProp,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiFlowLayoutPropSchema {
    schema_id: &'static str,
    prop_key: &'static str,
    value_kind: WorthUiFlowLayoutValueKind,
    default_value: &'static str,
    examples: &'static [&'static str],
}

impl WorthUiFlowLayoutPropSchema {
    const fn new(
        schema_id: &'static str,
        prop_key: &'static str,
        value_kind: WorthUiFlowLayoutValueKind,
        default_value: &'static str,
        examples: &'static [&'static str],
    ) -> Self {
        Self {
            schema_id,
            prop_key,
            value_kind,
            default_value,
            examples,
        }
    }

    pub fn schema_id(&self) -> &'static str {
        self.schema_id
    }

    pub fn prop_key(&self) -> &'static str {
        self.prop_key
    }

    pub fn value_kind(&self) -> WorthUiFlowLayoutValueKind {
        self.value_kind
    }

    pub fn default_value(&self) -> &'static str {
        self.default_value
    }

    pub fn examples(&self) -> &'static [&'static str] {
        self.examples
    }

    pub fn expected_value_syntax(&self) -> &'static str {
        self.value_kind.expected_syntax()
    }

    pub fn denial_code(&self) -> WorthUiFlowLayoutValueDenialCode {
        self.value_kind.denial_code()
    }

    pub fn semantic_slice(&self) -> WorthUiSemanticSliceId {
        WorthUiSemanticSliceId::PrimitiveFlowLayout
    }

    pub fn fact_family(&self) -> WorthUiRuntimeFactFamily {
        WorthUiRuntimeFactFamily::PrimitiveFlowLayout
    }
}

impl WorthUiFlowLayoutValueKind {
    pub fn expected_syntax(self) -> &'static str {
        match self {
            Self::Kind => "`row`, `column`, `inline`, `stack`, `grid`, or `spacer`",
            Self::MeasurementToken => {
                "a named density or measurement token like `validation.density.primitive.flow.gap.default`"
            }
            Self::Align => "`start`, `center`, or `end`",
            Self::CrossAlign => "`start`, `center`, `end`, or `baseline`",
            Self::Fit => "`hug` or `fill`",
            Self::Fill => "`none`, `width`, `height`, or `both`",
            Self::Unknown => "a declared flow layout prop",
        }
    }

    pub fn denial_code(self) -> WorthUiFlowLayoutValueDenialCode {
        match self {
            Self::Kind => WorthUiFlowLayoutValueDenialCode::InvalidKind,
            Self::MeasurementToken => WorthUiFlowLayoutValueDenialCode::InvalidMeasurementToken,
            Self::Align => WorthUiFlowLayoutValueDenialCode::InvalidAlign,
            Self::CrossAlign => WorthUiFlowLayoutValueDenialCode::InvalidCrossAlign,
            Self::Fit => WorthUiFlowLayoutValueDenialCode::InvalidFit,
            Self::Fill => WorthUiFlowLayoutValueDenialCode::InvalidFill,
            Self::Unknown => WorthUiFlowLayoutValueDenialCode::UnknownFlowLayoutProp,
        }
    }
}

pub const FLOW_KIND_PROP: &str = "flow_kind";
pub const FLOW_GAP_PROP: &str = "flow_gap";
pub const FLOW_PADDING_PROP: &str = "flow_padding";
pub const FLOW_ALIGN_PROP: &str = "flow_align";
pub const FLOW_CROSS_ALIGN_PROP: &str = "flow_cross_align";
pub const FLOW_FIT_PROP: &str = "flow_fit";
pub const FLOW_FILL_PROP: &str = "flow_fill";

const FLOW_LAYOUT_SCHEMAS: &[WorthUiFlowLayoutPropSchema] = &[
    WorthUiFlowLayoutPropSchema::new(
        "worth.primitive.flow.prop.flow_kind",
        FLOW_KIND_PROP,
        WorthUiFlowLayoutValueKind::Kind,
        "inline",
        &["row", "column", "inline", "stack", "grid", "spacer"],
    ),
    WorthUiFlowLayoutPropSchema::new(
        "worth.primitive.flow.prop.flow_gap",
        FLOW_GAP_PROP,
        WorthUiFlowLayoutValueKind::MeasurementToken,
        "validation.density.primitive.flow.gap.default",
        &[
            "validation.density.primitive.flow.gap.compact",
            "validation.density.primitive.flow.gap.default",
        ],
    ),
    WorthUiFlowLayoutPropSchema::new(
        "worth.primitive.flow.prop.flow_padding",
        FLOW_PADDING_PROP,
        WorthUiFlowLayoutValueKind::MeasurementToken,
        "validation.density.primitive.flow.padding.default",
        &[
            "validation.density.primitive.flow.padding.default",
            "validation.density.primitive.flow.padding.fat",
        ],
    ),
    WorthUiFlowLayoutPropSchema::new(
        "worth.primitive.flow.prop.flow_align",
        FLOW_ALIGN_PROP,
        WorthUiFlowLayoutValueKind::Align,
        "center",
        &["start", "center", "end"],
    ),
    WorthUiFlowLayoutPropSchema::new(
        "worth.primitive.flow.prop.flow_cross_align",
        FLOW_CROSS_ALIGN_PROP,
        WorthUiFlowLayoutValueKind::CrossAlign,
        "center",
        &["start", "center", "end", "baseline"],
    ),
    WorthUiFlowLayoutPropSchema::new(
        "worth.primitive.flow.prop.flow_fit",
        FLOW_FIT_PROP,
        WorthUiFlowLayoutValueKind::Fit,
        "hug",
        &["hug", "fill"],
    ),
    WorthUiFlowLayoutPropSchema::new(
        "worth.primitive.flow.prop.flow_fill",
        FLOW_FILL_PROP,
        WorthUiFlowLayoutValueKind::Fill,
        "none",
        &["none", "width", "height", "both"],
    ),
];

pub fn flow_layout_prop_schema(prop_key: &str) -> Option<&'static WorthUiFlowLayoutPropSchema> {
    FLOW_LAYOUT_SCHEMAS
        .iter()
        .find(|schema| schema.prop_key == prop_key)
}

pub fn flow_layout_prop_schemas() -> &'static [WorthUiFlowLayoutPropSchema] {
    FLOW_LAYOUT_SCHEMAS
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn flow_layout_schema_declarations_are_complete_unique_and_self_certifying() {
        let mut schema_ids = BTreeSet::new();
        let mut prop_keys = BTreeSet::new();

        for schema in flow_layout_prop_schemas() {
            assert!(!schema.schema_id().is_empty());
            assert!(schema.schema_id().starts_with("worth.primitive.flow.prop."));
            assert!(!schema.prop_key().is_empty());
            assert!(schema.prop_key().starts_with("flow_"));
            assert!(!schema.default_value().is_empty());
            assert!(!schema.expected_value_syntax().is_empty());
            assert!(!schema.examples().is_empty());
            assert_eq!(schema.denial_code(), schema.value_kind().denial_code());
            assert!(schema_ids.insert(schema.schema_id()));
            assert!(prop_keys.insert(schema.prop_key()));
            assert_default_value_matches_schema_kind(schema);
        }
    }

    fn assert_default_value_matches_schema_kind(schema: &WorthUiFlowLayoutPropSchema) {
        match schema.value_kind() {
            WorthUiFlowLayoutValueKind::Kind => {
                assert!(matches!(
                    schema.default_value(),
                    "row" | "column" | "inline" | "stack" | "grid" | "spacer"
                ));
            }
            WorthUiFlowLayoutValueKind::MeasurementToken => {
                assert!(schema.default_value().starts_with("validation.density."));
            }
            WorthUiFlowLayoutValueKind::Align => {
                assert!(matches!(schema.default_value(), "start" | "center" | "end"));
            }
            WorthUiFlowLayoutValueKind::CrossAlign => {
                assert!(matches!(
                    schema.default_value(),
                    "start" | "center" | "end" | "baseline"
                ));
            }
            WorthUiFlowLayoutValueKind::Fit => {
                assert!(matches!(schema.default_value(), "hug" | "fill"));
            }
            WorthUiFlowLayoutValueKind::Fill => {
                assert!(matches!(
                    schema.default_value(),
                    "none" | "width" | "height" | "both"
                ));
            }
            WorthUiFlowLayoutValueKind::Unknown => {
                panic!("unknown flow value kind must not be declared in schemas");
            }
        }
    }
}
