use crate::runtime::{WorthUiRuntimeFactFamily, WorthUiSemanticSliceId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiAppearanceStateValueKind {
    Color,
    MeasurementToken,
    Opacity,
    TypographyToken,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiAppearanceStateValueDenialCode {
    InvalidColor,
    InvalidMeasurementToken,
    InvalidOpacity,
    InvalidTypographyToken,
    UnknownAppearanceStateProp,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAppearanceStatePropSchema {
    schema_id: String,
    prop_key: String,
    state: &'static str,
    field: &'static str,
    value_kind: WorthUiAppearanceStateValueKind,
    default_value: Option<&'static str>,
    examples: &'static [&'static str],
}

impl WorthUiAppearanceStatePropSchema {
    fn new(
        state: &'static str,
        field: &'static str,
        value_kind: WorthUiAppearanceStateValueKind,
        default_value: Option<&'static str>,
        examples: &'static [&'static str],
    ) -> Self {
        Self {
            schema_id: format!("worth.primitive.appearance_state.prop.appearance_{state}_{field}"),
            prop_key: format!("appearance_{state}_{field}"),
            state,
            field,
            value_kind,
            default_value,
            examples,
        }
    }

    pub fn schema_id(&self) -> &str {
        &self.schema_id
    }

    pub fn prop_key(&self) -> &str {
        &self.prop_key
    }

    pub fn state(&self) -> &'static str {
        self.state
    }

    pub fn field(&self) -> &'static str {
        self.field
    }

    pub fn value_kind(&self) -> WorthUiAppearanceStateValueKind {
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

    pub fn denial_code(&self) -> WorthUiAppearanceStateValueDenialCode {
        self.value_kind.denial_code()
    }

    pub fn semantic_slice(&self) -> WorthUiSemanticSliceId {
        WorthUiSemanticSliceId::PrimitiveAppearanceState
    }

    pub fn fact_family(&self) -> WorthUiRuntimeFactFamily {
        WorthUiRuntimeFactFamily::PrimitiveAppearanceState
    }
}

impl WorthUiAppearanceStateValueKind {
    pub fn expected_syntax(self) -> &'static str {
        match self {
            Self::Color => {
                "a hex color, `transparent`, or a theme color token like `validation.theme.header.text`"
            }
            Self::MeasurementToken => {
                "a named density token like `validation.density.primitive.border.width`"
            }
            Self::Opacity => "a number from `0` through `1`",
            Self::TypographyToken => {
                "an appearance font-size token like `validation.appearance.header.font_size`"
            }
            Self::Unknown => "a declared appearance state prop",
        }
    }

    pub fn denial_code(self) -> WorthUiAppearanceStateValueDenialCode {
        match self {
            Self::Color => WorthUiAppearanceStateValueDenialCode::InvalidColor,
            Self::MeasurementToken => {
                WorthUiAppearanceStateValueDenialCode::InvalidMeasurementToken
            }
            Self::Opacity => WorthUiAppearanceStateValueDenialCode::InvalidOpacity,
            Self::TypographyToken => WorthUiAppearanceStateValueDenialCode::InvalidTypographyToken,
            Self::Unknown => WorthUiAppearanceStateValueDenialCode::UnknownAppearanceStateProp,
        }
    }
}

const STATES: &[&str] = &["rest", "hover", "pressed", "focus", "disabled", "selected"];
const COLOR_FIELDS: &[&str] = &[
    "background",
    "foreground",
    "border_color",
    "focus_ring_color",
    "icon_color",
    "text_color",
];
const MEASUREMENT_FIELDS: &[&str] = &["border_width", "radius", "focus_ring_width"];

pub fn appearance_state_prop_schemas() -> Vec<WorthUiAppearanceStatePropSchema> {
    let mut schemas = Vec::new();
    for state in STATES {
        push_color_schemas(&mut schemas, state);
        push_measurement_schemas(&mut schemas, state);
        schemas.push(WorthUiAppearanceStatePropSchema::new(
            state,
            "opacity",
            WorthUiAppearanceStateValueKind::Opacity,
            rest_default(state, "opacity"),
            &["1", "0.65"],
        ));
        schemas.push(WorthUiAppearanceStatePropSchema::new(
            state,
            "typography",
            WorthUiAppearanceStateValueKind::TypographyToken,
            rest_default(state, "typography"),
            examples_for(WorthUiAppearanceStateValueKind::TypographyToken),
        ));
    }
    schemas
}

fn push_color_schemas(schemas: &mut Vec<WorthUiAppearanceStatePropSchema>, state: &'static str) {
    for field in COLOR_FIELDS {
        schemas.push(WorthUiAppearanceStatePropSchema::new(
            state,
            field,
            WorthUiAppearanceStateValueKind::Color,
            rest_default(state, field),
            &["#2f7de1", "validation.theme.header.text", "transparent"],
        ));
    }
}

fn push_measurement_schemas(
    schemas: &mut Vec<WorthUiAppearanceStatePropSchema>,
    state: &'static str,
) {
    for field in MEASUREMENT_FIELDS {
        schemas.push(WorthUiAppearanceStatePropSchema::new(
            state,
            field,
            WorthUiAppearanceStateValueKind::MeasurementToken,
            rest_default(state, field),
            &[
                "validation.density.primitive.border.none",
                "validation.density.primitive.radius",
            ],
        ));
    }
}

fn rest_default(state: &str, field: &str) -> Option<&'static str> {
    if state != "rest" {
        return None;
    }
    match field {
        "background" => Some("#2f7de1"),
        "foreground" => Some("#ffffff"),
        "border_color" => Some("transparent"),
        "border_width" => None,
        "radius" => None,
        "opacity" => Some("1"),
        "focus_ring_color" => Some("#ffffff"),
        "focus_ring_width" => None,
        "icon_color" => Some("#ffffff"),
        "text_color" => Some("#ffffff"),
        "typography" => None,
        _ => None,
    }
}

pub fn appearance_state_prop_schema(prop_key: &str) -> Option<WorthUiAppearanceStatePropSchema> {
    parse_appearance_prop_key(prop_key).and_then(|(state, field, kind)| {
        STATES.contains(&state).then(|| {
            WorthUiAppearanceStatePropSchema::new(
                state,
                field,
                kind,
                rest_default(state, field),
                examples_for(kind),
            )
        })
    })
}

fn parse_appearance_prop_key(
    prop_key: &str,
) -> Option<(&'static str, &'static str, WorthUiAppearanceStateValueKind)> {
    let tail = prop_key.strip_prefix("appearance_")?;
    for state in STATES {
        let Some(field) = tail.strip_prefix(&format!("{state}_")) else {
            continue;
        };
        if COLOR_FIELDS.contains(&field) {
            return Some((
                state,
                canonical_field(field)?,
                WorthUiAppearanceStateValueKind::Color,
            ));
        }
        if MEASUREMENT_FIELDS.contains(&field) {
            return Some((
                state,
                canonical_field(field)?,
                WorthUiAppearanceStateValueKind::MeasurementToken,
            ));
        }
        if field == "opacity" {
            return Some((state, "opacity", WorthUiAppearanceStateValueKind::Opacity));
        }
        if field == "typography" {
            return Some((
                state,
                "typography",
                WorthUiAppearanceStateValueKind::TypographyToken,
            ));
        }
    }
    None
}

fn canonical_field(field: &str) -> Option<&'static str> {
    COLOR_FIELDS
        .iter()
        .chain(MEASUREMENT_FIELDS.iter())
        .copied()
        .find(|candidate| *candidate == field)
}

fn examples_for(kind: WorthUiAppearanceStateValueKind) -> &'static [&'static str] {
    match kind {
        WorthUiAppearanceStateValueKind::Color => {
            &["#2f7de1", "validation.theme.header.text", "transparent"]
        }
        WorthUiAppearanceStateValueKind::MeasurementToken => &[
            "validation.density.primitive.border.none",
            "validation.density.primitive.radius",
        ],
        WorthUiAppearanceStateValueKind::Opacity => &["1", "0.65"],
        WorthUiAppearanceStateValueKind::TypographyToken => {
            &["validation.appearance.header.font_size"]
        }
        WorthUiAppearanceStateValueKind::Unknown => &["appearance_rest_background"],
    }
}

#[cfg(test)]
mod tests {
    use super::appearance_state_prop_schemas;

    #[test]
    fn appearance_state_defaults_do_not_depend_on_validation_app_tokens() {
        for schema in appearance_state_prop_schemas() {
            let Some(default_value) = schema.default_value() else {
                continue;
            };
            assert!(
                !default_value.starts_with("validation."),
                "schema {} default leaked validation-app token {}",
                schema.schema_id(),
                default_value
            );
        }
    }
}
