mod field;
mod projection;

pub use field::{
    UiIntentBoolean, UiIntentPayloadField, UiIntentPayloadFieldDescriptor,
    UiIntentPayloadFieldKind, UiIntentPayloadFieldSet, UiIntentPayloadSchemaViolation,
    UiIntentPayloadValueKind, UiIntentSelection, UiIntentSelectionValue, UiIntentText,
    UiIntentUnsigned64, UI_INTENT_PAYLOAD_FIELD_LIMIT, UI_INTENT_PAYLOAD_TEXT_BYTE_LIMIT,
};
pub use projection::{
    UiIntentPayloadProjection, UiIntentPayloadProjectionViolation, UiIntentProjectedValue,
};

use super::UiIntentSchema;

/// Typed payloads declare their stable schema and concrete field projection.
pub trait UiIntentPayload: Send + 'static + Sized {
    const SCHEMA: UiIntentSchema;
    const FIELDS: UiIntentPayloadFieldSet;

    fn project(
        fields: &mut UiIntentPayloadProjection<Self>,
    ) -> Result<Self, UiIntentPayloadProjectionViolation>;
}
