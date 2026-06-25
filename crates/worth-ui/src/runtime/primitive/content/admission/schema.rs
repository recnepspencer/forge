use crate::runtime::{WorthUiRuntimeFactFamily, WorthUiSemanticSliceId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveContentValueKind {
    Kind,
    Order,
    Text,
    IconId,
    ImageAsset,
    MeasurementToken,
    AccessibilityName,
    ContentRole,
    ParticipationPosture,
    UnsupportedReference,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveContentValueDenialCode {
    InvalidKind,
    InvalidOrder,
    InvalidText,
    InvalidIconId,
    InvalidImageAsset,
    InvalidMeasurementToken,
    InvalidAccessibilityName,
    InvalidContentRole,
    InvalidParticipationPosture,
    UnsupportedImageReference,
    UnsupportedSlotDeclaration,
    UnknownContentProp,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiPrimitiveContentPropSchema {
    schema_id: &'static str,
    prop_key: &'static str,
    value_kind: WorthUiPrimitiveContentValueKind,
    default_value: Option<&'static str>,
    examples: &'static [&'static str],
}

impl WorthUiPrimitiveContentPropSchema {
    const fn new(
        schema_id: &'static str,
        prop_key: &'static str,
        value_kind: WorthUiPrimitiveContentValueKind,
        default_value: Option<&'static str>,
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

    pub fn value_kind(&self) -> WorthUiPrimitiveContentValueKind {
        self.value_kind
    }

    pub fn default_value(&self) -> Option<&'static str> {
        self.default_value
    }

    pub fn examples(&self) -> &'static [&'static str] {
        self.examples
    }

    pub fn expected_value_syntax(&self) -> &'static str {
        self.value_kind.expected_syntax()
    }

    pub fn denial_code(&self) -> WorthUiPrimitiveContentValueDenialCode {
        self.value_kind.denial_code(self.prop_key)
    }

    pub fn semantic_slice(&self) -> WorthUiSemanticSliceId {
        WorthUiSemanticSliceId::PrimitiveContent
    }

    pub fn fact_family(&self) -> WorthUiRuntimeFactFamily {
        WorthUiRuntimeFactFamily::PrimitiveContent
    }
}

impl WorthUiPrimitiveContentValueKind {
    pub fn expected_syntax(self) -> &'static str {
        match self {
            Self::Kind => "`inline`, `stack`, or `plain`",
            Self::Order => {
                "a comma-separated order using `icon`, `text`, `image`, `spacer`, `badge`, or `divider`"
            }
            Self::Text => "a non-empty quoted string or identifier text",
            Self::IconId => "a registered icon id like `worth.icon.action.plus`",
            Self::ImageAsset => "a registered local/static image id like `worth.image.logo`",
            Self::MeasurementToken => {
                "a named density token like `validation.density.primitive.content.icon.large`"
            }
            Self::AccessibilityName => "a quoted accessibility name",
            Self::ContentRole => {
                "`body`, `label`, `helper_text`, `error_text`, `prefix_adornment`, or `suffix_adornment`"
            }
            Self::ParticipationPosture => {
                "`present`, `absent`, `hidden_from_paint`, `hidden_from_accessibility`, `inert`, `loading`, `unsupported`, or `denied`"
            }
            Self::UnsupportedReference => {
                "a supported content reference for the current lowering capability"
            }
            Self::Unknown => "a declared content prop",
        }
    }

    pub fn denial_code(self, prop_key: &str) -> WorthUiPrimitiveContentValueDenialCode {
        match self {
            Self::Kind => WorthUiPrimitiveContentValueDenialCode::InvalidKind,
            Self::Order => WorthUiPrimitiveContentValueDenialCode::InvalidOrder,
            Self::Text => WorthUiPrimitiveContentValueDenialCode::InvalidText,
            Self::IconId => WorthUiPrimitiveContentValueDenialCode::InvalidIconId,
            Self::ImageAsset => WorthUiPrimitiveContentValueDenialCode::InvalidImageAsset,
            Self::MeasurementToken => {
                WorthUiPrimitiveContentValueDenialCode::InvalidMeasurementToken
            }
            Self::AccessibilityName => {
                WorthUiPrimitiveContentValueDenialCode::InvalidAccessibilityName
            }
            Self::ContentRole => WorthUiPrimitiveContentValueDenialCode::InvalidContentRole,
            Self::ParticipationPosture => {
                WorthUiPrimitiveContentValueDenialCode::InvalidParticipationPosture
            }
            Self::UnsupportedReference if prop_key == CONTENT_IMAGE_PROP => {
                WorthUiPrimitiveContentValueDenialCode::UnsupportedImageReference
            }
            Self::UnsupportedReference if prop_key == CONTENT_SLOT_PROP => {
                WorthUiPrimitiveContentValueDenialCode::UnsupportedSlotDeclaration
            }
            Self::UnsupportedReference => {
                WorthUiPrimitiveContentValueDenialCode::UnsupportedImageReference
            }
            Self::Unknown => WorthUiPrimitiveContentValueDenialCode::UnknownContentProp,
        }
    }
}

pub const CONTENT_KIND_PROP: &str = "content_kind";
pub const CONTENT_ORDER_PROP: &str = "content_order";
pub const CONTENT_TEXT_PROP: &str = "content_text";
pub const CONTENT_ICON_PROP: &str = "content_icon";
pub const CONTENT_IMAGE_PROP: &str = "content_image";
pub const CONTENT_TEXT_SIZE_PROP: &str = "content_text_size";
pub const CONTENT_ICON_SIZE_PROP: &str = "content_icon_size";
pub const CONTENT_ICON_STROKE_PROP: &str = "content_icon_stroke";
pub const CONTENT_SPACER_SIZE_PROP: &str = "content_spacer_size";
pub const CONTENT_BADGE_TEXT_PROP: &str = "content_badge_text";
pub const CONTENT_DIVIDER_THICKNESS_PROP: &str = "content_divider_thickness";
pub const CONTENT_ACCESSIBILITY_NAME_PROP: &str = "content_accessibility_name";
pub const CONTENT_ROLE_PROP: &str = "content_role";
pub const CONTENT_PRESENCE_PROP: &str = "content_presence";
pub const CONTENT_SLOT_PROP: &str = "content_slot";

const CONTENT_SCHEMAS: &[WorthUiPrimitiveContentPropSchema] = &[
    WorthUiPrimitiveContentPropSchema::new(
        "worth.primitive.content.prop.content_kind",
        CONTENT_KIND_PROP,
        WorthUiPrimitiveContentValueKind::Kind,
        Some("inline"),
        &["plain", "inline", "stack"],
    ),
    WorthUiPrimitiveContentPropSchema::new(
        "worth.primitive.content.prop.content_order",
        CONTENT_ORDER_PROP,
        WorthUiPrimitiveContentValueKind::Order,
        Some("icon,text"),
        &["text", "icon,text", "image,text", "text,spacer,badge"],
    ),
    WorthUiPrimitiveContentPropSchema::new(
        "worth.primitive.content.prop.content_text",
        CONTENT_TEXT_PROP,
        WorthUiPrimitiveContentValueKind::Text,
        Some("Submit"),
        &["Submit", "\"Save changes\""],
    ),
    WorthUiPrimitiveContentPropSchema::new(
        "worth.primitive.content.prop.content_icon",
        CONTENT_ICON_PROP,
        WorthUiPrimitiveContentValueKind::IconId,
        None,
        &["worth.icon.action.plus", "worth.icon.action.check"],
    ),
    WorthUiPrimitiveContentPropSchema::new(
        "worth.primitive.content.prop.content_image",
        CONTENT_IMAGE_PROP,
        WorthUiPrimitiveContentValueKind::ImageAsset,
        None,
        &["worth.image.logo"],
    ),
    WorthUiPrimitiveContentPropSchema::new(
        "worth.primitive.content.prop.content_text_size",
        CONTENT_TEXT_SIZE_PROP,
        WorthUiPrimitiveContentValueKind::MeasurementToken,
        Some("validation.density.primitive.content.text.default"),
        &[
            "validation.density.primitive.content.text.default",
            "validation.density.primitive.content.text.large",
        ],
    ),
    WorthUiPrimitiveContentPropSchema::new(
        "worth.primitive.content.prop.content_icon_size",
        CONTENT_ICON_SIZE_PROP,
        WorthUiPrimitiveContentValueKind::MeasurementToken,
        Some("validation.density.primitive.content.icon.default"),
        &[
            "validation.density.primitive.content.icon.default",
            "validation.density.primitive.content.icon.large",
        ],
    ),
    WorthUiPrimitiveContentPropSchema::new(
        "worth.primitive.content.prop.content_icon_stroke",
        CONTENT_ICON_STROKE_PROP,
        WorthUiPrimitiveContentValueKind::MeasurementToken,
        Some("validation.density.primitive.content.icon.stroke.default"),
        &[
            "validation.density.primitive.content.icon.stroke.thin",
            "validation.density.primitive.content.icon.stroke.default",
        ],
    ),
    WorthUiPrimitiveContentPropSchema::new(
        "worth.primitive.content.prop.content_spacer_size",
        CONTENT_SPACER_SIZE_PROP,
        WorthUiPrimitiveContentValueKind::MeasurementToken,
        Some("validation.density.primitive.content.spacer.default"),
        &["validation.density.primitive.content.spacer.default"],
    ),
    WorthUiPrimitiveContentPropSchema::new(
        "worth.primitive.content.prop.content_badge_text",
        CONTENT_BADGE_TEXT_PROP,
        WorthUiPrimitiveContentValueKind::Text,
        None,
        &["New", "\"Beta\""],
    ),
    WorthUiPrimitiveContentPropSchema::new(
        "worth.primitive.content.prop.content_divider_thickness",
        CONTENT_DIVIDER_THICKNESS_PROP,
        WorthUiPrimitiveContentValueKind::MeasurementToken,
        Some("validation.density.primitive.content.divider.default"),
        &["validation.density.primitive.content.divider.default"],
    ),
    WorthUiPrimitiveContentPropSchema::new(
        "worth.primitive.content.prop.content_accessibility_name",
        CONTENT_ACCESSIBILITY_NAME_PROP,
        WorthUiPrimitiveContentValueKind::AccessibilityName,
        None,
        &["\"Submit form\""],
    ),
    WorthUiPrimitiveContentPropSchema::new(
        "worth.primitive.content.prop.content_role",
        CONTENT_ROLE_PROP,
        WorthUiPrimitiveContentValueKind::ContentRole,
        Some("body"),
        &[
            "body",
            "label",
            "helper_text",
            "error_text",
            "prefix_adornment",
            "suffix_adornment",
        ],
    ),
    WorthUiPrimitiveContentPropSchema::new(
        "worth.primitive.content.prop.content_presence",
        CONTENT_PRESENCE_PROP,
        WorthUiPrimitiveContentValueKind::ParticipationPosture,
        Some("present"),
        &[
            "present",
            "absent",
            "hidden_from_paint",
            "hidden_from_accessibility",
            "inert",
            "loading",
            "unsupported",
            "denied",
        ],
    ),
    WorthUiPrimitiveContentPropSchema::new(
        "worth.primitive.content.prop.content_slot",
        CONTENT_SLOT_PROP,
        WorthUiPrimitiveContentValueKind::UnsupportedReference,
        None,
        &["leading", "trailing"],
    ),
];

pub fn primitive_content_prop_schema(
    prop_key: &str,
) -> Option<&'static WorthUiPrimitiveContentPropSchema> {
    CONTENT_SCHEMAS
        .iter()
        .find(|schema| schema.prop_key == prop_key)
}

pub fn primitive_content_prop_schemas() -> &'static [WorthUiPrimitiveContentPropSchema] {
    CONTENT_SCHEMAS
}
