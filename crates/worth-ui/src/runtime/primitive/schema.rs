use crate::runtime::{WorthUiRuntimeFactFamily, WorthUiSemanticSliceId};

#[cfg(test)]
#[path = "schema_tests.rs"]
mod schema_tests;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveAuthoredValueKind {
    Text,
    MeasurementToken,
    Color,
    Align,
    InteractionKind,
    Cursor,
    Focus,
    Boolean,
    MotionKind,
    MotionTarget,
    Easing,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveValueDenialCode {
    InvalidText,
    InvalidMeasurementToken,
    InvalidColorHex,
    InvalidAlignKeyword,
    InvalidInteractionKind,
    InvalidCursor,
    InvalidFocus,
    InvalidBoolean,
    InvalidMotionKind,
    InvalidMotionTarget,
    InvalidEasing,
    UnknownPrimitiveProp,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveDefaultPolicy {
    Defaulted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiPrimitiveAuthoredPropSchema {
    schema_id: &'static str,
    prop_key: &'static str,
    value_kind: WorthUiPrimitiveAuthoredValueKind,
    semantic_slice: WorthUiSemanticSliceId,
    fact_family: WorthUiRuntimeFactFamily,
    default_policy: WorthUiPrimitiveDefaultPolicy,
    default_value: &'static str,
    examples: &'static [&'static str],
}

impl WorthUiPrimitiveAuthoredPropSchema {
    const fn new(
        schema_id: &'static str,
        prop_key: &'static str,
        value_kind: WorthUiPrimitiveAuthoredValueKind,
        semantic_slice: WorthUiSemanticSliceId,
        fact_family: WorthUiRuntimeFactFamily,
        default_value: &'static str,
        examples: &'static [&'static str],
    ) -> Self {
        Self {
            schema_id,
            prop_key,
            value_kind,
            semantic_slice,
            fact_family,
            default_policy: WorthUiPrimitiveDefaultPolicy::Defaulted,
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

    pub fn value_kind(&self) -> WorthUiPrimitiveAuthoredValueKind {
        self.value_kind
    }

    pub fn semantic_slice(&self) -> WorthUiSemanticSliceId {
        self.semantic_slice
    }

    pub fn fact_family(&self) -> WorthUiRuntimeFactFamily {
        self.fact_family
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

    pub fn denial_code(&self) -> WorthUiPrimitiveValueDenialCode {
        self.value_kind.denial_code()
    }
}

impl WorthUiPrimitiveAuthoredValueKind {
    pub fn expected_syntax(self) -> &'static str {
        match self {
            Self::Text => "a text value",
            Self::MeasurementToken => {
                "a named density or measurement token like `validation.density.primitive.padding`"
            }
            Self::Color => "a hex color like `#2f7de1`",
            Self::Align => "`start`, `center`, or `end`",
            Self::InteractionKind => "`submit` or `none`",
            Self::Cursor => "`default` or `pointer`",
            Self::Focus => "`none` or `focusable`",
            Self::Boolean => "`true` or `false`",
            Self::MotionKind => "`none` or `transition`",
            Self::MotionTarget => {
                "`primitive_background`, `primitive_foreground`, or `primitive_radius`"
            }
            Self::Easing => "`linear`, `standard`, `ease_in`, or `ease_out`",
            Self::Unknown => "a declared primitive prop",
        }
    }

    pub fn denial_code(self) -> WorthUiPrimitiveValueDenialCode {
        match self {
            Self::Text => WorthUiPrimitiveValueDenialCode::InvalidText,
            Self::MeasurementToken => WorthUiPrimitiveValueDenialCode::InvalidMeasurementToken,
            Self::Color => WorthUiPrimitiveValueDenialCode::InvalidColorHex,
            Self::Align => WorthUiPrimitiveValueDenialCode::InvalidAlignKeyword,
            Self::InteractionKind => WorthUiPrimitiveValueDenialCode::InvalidInteractionKind,
            Self::Cursor => WorthUiPrimitiveValueDenialCode::InvalidCursor,
            Self::Focus => WorthUiPrimitiveValueDenialCode::InvalidFocus,
            Self::Boolean => WorthUiPrimitiveValueDenialCode::InvalidBoolean,
            Self::MotionKind => WorthUiPrimitiveValueDenialCode::InvalidMotionKind,
            Self::MotionTarget => WorthUiPrimitiveValueDenialCode::InvalidMotionTarget,
            Self::Easing => WorthUiPrimitiveValueDenialCode::InvalidEasing,
            Self::Unknown => WorthUiPrimitiveValueDenialCode::UnknownPrimitiveProp,
        }
    }
}

pub const PRIMITIVE_TEXT_PROP: &str = "primitive_text";
pub const PRIMITIVE_ALIGN_PROP: &str = "primitive_align";
pub const PRIMITIVE_PADDING_PROP: &str = "primitive_padding";
pub const PRIMITIVE_RADIUS_PROP: &str = "primitive_radius";
pub const PRIMITIVE_BACKGROUND_PROP: &str = "primitive_background";
pub const PRIMITIVE_FOREGROUND_PROP: &str = "primitive_foreground";
pub const PRIMITIVE_INTERACTION_PROP: &str = "primitive_interaction";
pub const PRIMITIVE_CURSOR_PROP: &str = "primitive_cursor";
pub const PRIMITIVE_FOCUS_PROP: &str = "primitive_focus";
pub const PRIMITIVE_DISABLED_PROP: &str = "primitive_disabled";
pub const PRIMITIVE_SELECTED_PROP: &str = "primitive_selected";
pub const PRIMITIVE_INTERACTION_ID_PROP: &str = "primitive_interaction_id";
pub const PRIMITIVE_SUBMIT_PAYLOAD_PROP: &str = "primitive_submit_payload";
pub const PRIMITIVE_MOTION_PROP: &str = "primitive_motion";
pub const PRIMITIVE_MOTION_TARGET_PROP: &str = "primitive_motion_target";
pub const PRIMITIVE_MOTION_DURATION_PROP: &str = "primitive_motion_duration";
pub const PRIMITIVE_MOTION_EASING_PROP: &str = "primitive_motion_easing";

const PRIMITIVE_PROP_SCHEMAS: &[WorthUiPrimitiveAuthoredPropSchema] = &[
    WorthUiPrimitiveAuthoredPropSchema::new(
        "worth.primitive.prop.primitive_text",
        PRIMITIVE_TEXT_PROP,
        WorthUiPrimitiveAuthoredValueKind::Text,
        WorthUiSemanticSliceId::PrimitiveContent,
        WorthUiRuntimeFactFamily::PrimitiveContent,
        "Worth primitive",
        &["\"Worth primitive\""],
    ),
    WorthUiPrimitiveAuthoredPropSchema::new(
        "worth.primitive.prop.primitive_align",
        PRIMITIVE_ALIGN_PROP,
        WorthUiPrimitiveAuthoredValueKind::Align,
        WorthUiSemanticSliceId::PrimitiveContainer,
        WorthUiRuntimeFactFamily::PrimitiveContainer,
        "center",
        &["start", "center", "end"],
    ),
    WorthUiPrimitiveAuthoredPropSchema::new(
        "worth.primitive.prop.primitive_padding",
        PRIMITIVE_PADDING_PROP,
        WorthUiPrimitiveAuthoredValueKind::MeasurementToken,
        WorthUiSemanticSliceId::PrimitiveMeasurement,
        WorthUiRuntimeFactFamily::PrimitiveMeasurement,
        "validation.density.primitive.padding",
        &[
            "validation.density.primitive.padding",
            "validation.density.primitive.padding.fat",
        ],
    ),
    WorthUiPrimitiveAuthoredPropSchema::new(
        "worth.primitive.prop.primitive_radius",
        PRIMITIVE_RADIUS_PROP,
        WorthUiPrimitiveAuthoredValueKind::MeasurementToken,
        WorthUiSemanticSliceId::PrimitiveMeasurement,
        WorthUiRuntimeFactFamily::PrimitiveMeasurement,
        "validation.density.primitive.radius",
        &[
            "validation.density.primitive.radius",
            "validation.density.primitive.radius.round",
        ],
    ),
    WorthUiPrimitiveAuthoredPropSchema::new(
        "worth.primitive.prop.primitive_background",
        PRIMITIVE_BACKGROUND_PROP,
        WorthUiPrimitiveAuthoredValueKind::Color,
        WorthUiSemanticSliceId::PrimitiveAppearance,
        WorthUiRuntimeFactFamily::PrimitiveAppearance,
        "#2f7de1",
        &["#2f7de1", "#b3261e"],
    ),
    WorthUiPrimitiveAuthoredPropSchema::new(
        "worth.primitive.prop.primitive_foreground",
        PRIMITIVE_FOREGROUND_PROP,
        WorthUiPrimitiveAuthoredValueKind::Color,
        WorthUiSemanticSliceId::PrimitiveAppearance,
        WorthUiRuntimeFactFamily::PrimitiveAppearance,
        "#f7f1e8",
        &["#f7f1e8", "#ffffff"],
    ),
    WorthUiPrimitiveAuthoredPropSchema::new(
        "worth.primitive.prop.primitive_interaction",
        PRIMITIVE_INTERACTION_PROP,
        WorthUiPrimitiveAuthoredValueKind::InteractionKind,
        WorthUiSemanticSliceId::PrimitiveInteraction,
        WorthUiRuntimeFactFamily::PrimitiveInteraction,
        "submit",
        &["submit", "none"],
    ),
    WorthUiPrimitiveAuthoredPropSchema::new(
        "worth.primitive.prop.primitive_cursor",
        PRIMITIVE_CURSOR_PROP,
        WorthUiPrimitiveAuthoredValueKind::Cursor,
        WorthUiSemanticSliceId::PrimitiveInteraction,
        WorthUiRuntimeFactFamily::PrimitiveInteraction,
        "pointer",
        &["default", "pointer"],
    ),
    WorthUiPrimitiveAuthoredPropSchema::new(
        "worth.primitive.prop.primitive_focus",
        PRIMITIVE_FOCUS_PROP,
        WorthUiPrimitiveAuthoredValueKind::Focus,
        WorthUiSemanticSliceId::PrimitiveInteraction,
        WorthUiRuntimeFactFamily::PrimitiveInteraction,
        "focusable",
        &["none", "focusable"],
    ),
    WorthUiPrimitiveAuthoredPropSchema::new(
        "worth.primitive.prop.primitive_disabled",
        PRIMITIVE_DISABLED_PROP,
        WorthUiPrimitiveAuthoredValueKind::Boolean,
        WorthUiSemanticSliceId::PrimitiveInteraction,
        WorthUiRuntimeFactFamily::PrimitiveInteraction,
        "false",
        &["false", "true"],
    ),
    WorthUiPrimitiveAuthoredPropSchema::new(
        "worth.primitive.prop.primitive_selected",
        PRIMITIVE_SELECTED_PROP,
        WorthUiPrimitiveAuthoredValueKind::Boolean,
        WorthUiSemanticSliceId::PrimitiveInteraction,
        WorthUiRuntimeFactFamily::PrimitiveInteraction,
        "false",
        &["false", "true"],
    ),
    WorthUiPrimitiveAuthoredPropSchema::new(
        "worth.primitive.prop.primitive_interaction_id",
        PRIMITIVE_INTERACTION_ID_PROP,
        WorthUiPrimitiveAuthoredValueKind::Text,
        WorthUiSemanticSliceId::PrimitiveInteraction,
        WorthUiRuntimeFactFamily::PrimitiveInteraction,
        "worth.interaction.primitive.submit",
        &["worth.interaction.primitive.submit"],
    ),
    WorthUiPrimitiveAuthoredPropSchema::new(
        "worth.primitive.prop.primitive_submit_payload",
        PRIMITIVE_SUBMIT_PAYLOAD_PROP,
        WorthUiPrimitiveAuthoredValueKind::Text,
        WorthUiSemanticSliceId::PrimitiveInteraction,
        WorthUiRuntimeFactFamily::PrimitiveInteraction,
        "submit.primary",
        &["submit.primary", "\"authored submit payload\""],
    ),
    WorthUiPrimitiveAuthoredPropSchema::new(
        "worth.primitive.prop.primitive_motion",
        PRIMITIVE_MOTION_PROP,
        WorthUiPrimitiveAuthoredValueKind::MotionKind,
        WorthUiSemanticSliceId::PrimitiveMotion,
        WorthUiRuntimeFactFamily::PrimitiveMotion,
        "transition",
        &["none", "transition"],
    ),
    WorthUiPrimitiveAuthoredPropSchema::new(
        "worth.primitive.prop.primitive_motion_target",
        PRIMITIVE_MOTION_TARGET_PROP,
        WorthUiPrimitiveAuthoredValueKind::MotionTarget,
        WorthUiSemanticSliceId::PrimitiveMotion,
        WorthUiRuntimeFactFamily::PrimitiveMotion,
        "primitive_background",
        &[
            "primitive_background",
            "primitive_foreground",
            "primitive_radius",
        ],
    ),
    WorthUiPrimitiveAuthoredPropSchema::new(
        "worth.primitive.prop.primitive_motion_duration",
        PRIMITIVE_MOTION_DURATION_PROP,
        WorthUiPrimitiveAuthoredValueKind::MeasurementToken,
        WorthUiSemanticSliceId::PrimitiveMotion,
        WorthUiRuntimeFactFamily::PrimitiveMotion,
        "validation.density.primitive.motion.fast",
        &[
            "validation.density.primitive.motion.fast",
            "validation.density.primitive.motion.slow",
        ],
    ),
    WorthUiPrimitiveAuthoredPropSchema::new(
        "worth.primitive.prop.primitive_motion_easing",
        PRIMITIVE_MOTION_EASING_PROP,
        WorthUiPrimitiveAuthoredValueKind::Easing,
        WorthUiSemanticSliceId::PrimitiveMotion,
        WorthUiRuntimeFactFamily::PrimitiveMotion,
        "standard",
        &["linear", "standard", "ease_in", "ease_out"],
    ),
];

pub fn primitive_authored_prop_schema(
    prop_key: &str,
) -> Option<&'static WorthUiPrimitiveAuthoredPropSchema> {
    PRIMITIVE_PROP_SCHEMAS
        .iter()
        .find(|schema| schema.prop_key == prop_key)
}

pub fn primitive_authored_prop_schemas() -> &'static [WorthUiPrimitiveAuthoredPropSchema] {
    PRIMITIVE_PROP_SCHEMAS
}
