use crate::runtime::{WorthUiRuntimeFactFamily, WorthUiSemanticSliceId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiEventGeometryValueKind {
    Cursor,
    HitArea,
    MeasurementToken,
    Containment,
    Capture,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiEventGeometryValueDenialCode {
    InvalidCursor,
    InvalidHitArea,
    InvalidMeasurementToken,
    InvalidContainment,
    InvalidCapture,
    UnknownEventGeometryProp,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiEventGeometryPropSchema {
    schema_id: &'static str,
    prop_key: &'static str,
    value_kind: WorthUiEventGeometryValueKind,
    default_value: &'static str,
    examples: &'static [&'static str],
}

impl WorthUiEventGeometryPropSchema {
    const fn new(
        schema_id: &'static str,
        prop_key: &'static str,
        value_kind: WorthUiEventGeometryValueKind,
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

    pub fn value_kind(&self) -> WorthUiEventGeometryValueKind {
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

    pub fn denial_code(&self) -> WorthUiEventGeometryValueDenialCode {
        self.value_kind.denial_code()
    }

    pub fn semantic_slice(&self) -> WorthUiSemanticSliceId {
        WorthUiSemanticSliceId::PrimitiveEventGeometry
    }

    pub fn fact_family(&self) -> WorthUiRuntimeFactFamily {
        WorthUiRuntimeFactFamily::PrimitiveEventGeometry
    }
}

impl WorthUiEventGeometryValueKind {
    pub fn expected_syntax(self) -> &'static str {
        match self {
            Self::Cursor => "`default`, `pointer`, `text`, `grab`, `grabbing`, or `resize`",
            Self::HitArea => "`visual_bounds`, `padded_bounds`, `explicit_hit_slop`, or `disabled_none`",
            Self::MeasurementToken => {
                "a named density or measurement token like `validation.density.primitive.event.hit_slop.default`"
            }
            Self::Containment => "`contain` or `bubble`",
            Self::Capture => "`none` or `press_drag`",
            Self::Unknown => "a declared event geometry prop",
        }
    }

    pub fn denial_code(self) -> WorthUiEventGeometryValueDenialCode {
        match self {
            Self::Cursor => WorthUiEventGeometryValueDenialCode::InvalidCursor,
            Self::HitArea => WorthUiEventGeometryValueDenialCode::InvalidHitArea,
            Self::MeasurementToken => WorthUiEventGeometryValueDenialCode::InvalidMeasurementToken,
            Self::Containment => WorthUiEventGeometryValueDenialCode::InvalidContainment,
            Self::Capture => WorthUiEventGeometryValueDenialCode::InvalidCapture,
            Self::Unknown => WorthUiEventGeometryValueDenialCode::UnknownEventGeometryProp,
        }
    }
}

pub const EVENT_CURSOR_PROP: &str = "event_cursor";
pub const EVENT_HIT_AREA_PROP: &str = "event_hit_area";
pub const EVENT_HIT_SLOP_PROP: &str = "event_hit_slop";
pub const EVENT_CONTAINMENT_PROP: &str = "event_containment";
pub const EVENT_CAPTURE_PROP: &str = "event_capture";

const EVENT_GEOMETRY_SCHEMAS: &[WorthUiEventGeometryPropSchema] = &[
    WorthUiEventGeometryPropSchema::new(
        "worth.primitive.event_geometry.prop.event_cursor",
        EVENT_CURSOR_PROP,
        WorthUiEventGeometryValueKind::Cursor,
        "pointer",
        &["default", "pointer", "text", "grab", "grabbing", "resize"],
    ),
    WorthUiEventGeometryPropSchema::new(
        "worth.primitive.event_geometry.prop.event_hit_area",
        EVENT_HIT_AREA_PROP,
        WorthUiEventGeometryValueKind::HitArea,
        "padded_bounds",
        &[
            "visual_bounds",
            "padded_bounds",
            "explicit_hit_slop",
            "disabled_none",
        ],
    ),
    WorthUiEventGeometryPropSchema::new(
        "worth.primitive.event_geometry.prop.event_hit_slop",
        EVENT_HIT_SLOP_PROP,
        WorthUiEventGeometryValueKind::MeasurementToken,
        "validation.density.primitive.event.hit_slop.default",
        &[
            "validation.density.primitive.event.hit_slop.compact",
            "validation.density.primitive.event.hit_slop.default",
        ],
    ),
    WorthUiEventGeometryPropSchema::new(
        "worth.primitive.event_geometry.prop.event_containment",
        EVENT_CONTAINMENT_PROP,
        WorthUiEventGeometryValueKind::Containment,
        "contain",
        &["contain", "bubble"],
    ),
    WorthUiEventGeometryPropSchema::new(
        "worth.primitive.event_geometry.prop.event_capture",
        EVENT_CAPTURE_PROP,
        WorthUiEventGeometryValueKind::Capture,
        "none",
        &["none", "press_drag"],
    ),
];

pub fn event_geometry_prop_schema(
    prop_key: &str,
) -> Option<&'static WorthUiEventGeometryPropSchema> {
    EVENT_GEOMETRY_SCHEMAS
        .iter()
        .find(|schema| schema.prop_key == prop_key)
}

pub fn event_geometry_prop_schemas() -> &'static [WorthUiEventGeometryPropSchema] {
    EVENT_GEOMETRY_SCHEMAS
}
