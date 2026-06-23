use crate::runtime::WorthUiSemanticSliceId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiInteractionValueKind {
    Kind,
    Identifier,
    Payload,
    Readiness,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiInteractionValueDenialCode {
    InvalidKind,
    InvalidIdentifier,
    InvalidPayload,
    InvalidReadiness,
    InvalidTargetReference,
    MissingRequiredValue,
    UnknownInteractionProp,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiInteractionPropSchema {
    schema_id: &'static str,
    prop_key: &'static str,
    value_kind: WorthUiInteractionValueKind,
    default_value: Option<&'static str>,
}

impl WorthUiInteractionPropSchema {
    const fn new(
        schema_id: &'static str,
        prop_key: &'static str,
        value_kind: WorthUiInteractionValueKind,
        default_value: Option<&'static str>,
    ) -> Self {
        Self {
            schema_id,
            prop_key,
            value_kind,
            default_value,
        }
    }

    pub fn schema_id(&self) -> &'static str {
        self.schema_id
    }

    pub fn prop_key(&self) -> &'static str {
        self.prop_key
    }

    pub fn value_kind(&self) -> WorthUiInteractionValueKind {
        self.value_kind
    }

    pub fn default_value(&self) -> Option<&'static str> {
        self.default_value
    }

    pub fn semantic_slice(&self) -> WorthUiSemanticSliceId {
        WorthUiSemanticSliceId::PrimitiveInteraction
    }

    pub fn expected_value_syntax(&self) -> &'static str {
        self.value_kind.expected_syntax()
    }

    pub fn denial_code(&self) -> WorthUiInteractionValueDenialCode {
        self.value_kind.denial_code()
    }
}

impl WorthUiInteractionValueKind {
    pub fn expected_syntax(self) -> &'static str {
        match self {
            Self::Kind => "`click`, `submit`, `command`, `toggle`, `open`, or `focus`",
            Self::Identifier => "a namespaced identifier like `worth.interaction.submit`",
            Self::Payload => "a text, number, or identifier payload value",
            Self::Readiness => "`enabled` or `disabled`",
            Self::Unknown => "a declared interaction prop",
        }
    }

    pub fn denial_code(self) -> WorthUiInteractionValueDenialCode {
        match self {
            Self::Kind => WorthUiInteractionValueDenialCode::InvalidKind,
            Self::Identifier => WorthUiInteractionValueDenialCode::InvalidIdentifier,
            Self::Payload => WorthUiInteractionValueDenialCode::InvalidPayload,
            Self::Readiness => WorthUiInteractionValueDenialCode::InvalidReadiness,
            Self::Unknown => WorthUiInteractionValueDenialCode::UnknownInteractionProp,
        }
    }
}

pub const INTERACTION_KIND_PROP: &str = "interaction_kind";
pub const INTERACTION_ID_PROP: &str = "interaction_id";
pub const INTERACTION_PAYLOAD_PROP: &str = "interaction_payload";
pub const INTERACTION_TARGET_PROP: &str = "interaction_target";
pub const INTERACTION_COMMAND_PROP: &str = "interaction_command";
pub const INTERACTION_TOGGLE_VALUE_PROP: &str = "interaction_toggle_value";
pub const INTERACTION_OPEN_TARGET_PROP: &str = "interaction_open_target";
pub const INTERACTION_FOCUS_TARGET_PROP: &str = "interaction_focus_target";
pub const INTERACTION_READINESS_PROP: &str = "interaction_readiness";

const INTERACTION_PROP_SCHEMAS: &[WorthUiInteractionPropSchema] = &[
    WorthUiInteractionPropSchema::new(
        "worth.interaction.prop.kind",
        INTERACTION_KIND_PROP,
        WorthUiInteractionValueKind::Kind,
        Some("submit"),
    ),
    WorthUiInteractionPropSchema::new(
        "worth.interaction.prop.id",
        INTERACTION_ID_PROP,
        WorthUiInteractionValueKind::Identifier,
        Some("worth.interaction.primitive.submit"),
    ),
    WorthUiInteractionPropSchema::new(
        "worth.interaction.prop.payload",
        INTERACTION_PAYLOAD_PROP,
        WorthUiInteractionValueKind::Payload,
        Some("submit.primary"),
    ),
    WorthUiInteractionPropSchema::new(
        "worth.interaction.prop.target",
        INTERACTION_TARGET_PROP,
        WorthUiInteractionValueKind::Identifier,
        None,
    ),
    WorthUiInteractionPropSchema::new(
        "worth.interaction.prop.command",
        INTERACTION_COMMAND_PROP,
        WorthUiInteractionValueKind::Identifier,
        None,
    ),
    WorthUiInteractionPropSchema::new(
        "worth.interaction.prop.toggle_value",
        INTERACTION_TOGGLE_VALUE_PROP,
        WorthUiInteractionValueKind::Identifier,
        None,
    ),
    WorthUiInteractionPropSchema::new(
        "worth.interaction.prop.open_target",
        INTERACTION_OPEN_TARGET_PROP,
        WorthUiInteractionValueKind::Identifier,
        None,
    ),
    WorthUiInteractionPropSchema::new(
        "worth.interaction.prop.focus_target",
        INTERACTION_FOCUS_TARGET_PROP,
        WorthUiInteractionValueKind::Identifier,
        None,
    ),
    WorthUiInteractionPropSchema::new(
        "worth.interaction.prop.readiness",
        INTERACTION_READINESS_PROP,
        WorthUiInteractionValueKind::Readiness,
        Some("enabled"),
    ),
];

pub fn interaction_prop_schemas() -> &'static [WorthUiInteractionPropSchema] {
    INTERACTION_PROP_SCHEMAS
}

pub fn interaction_prop_schema(prop_key: &str) -> Option<&'static WorthUiInteractionPropSchema> {
    interaction_prop_schemas()
        .iter()
        .find(|schema| schema.prop_key() == prop_key)
}
