use std::sync::Arc;

use worth_ui::facade::intent::{
    UiIntent, UiIntentAcceptedInteractions, UiIntentBoolean, UiIntentId, UiIntentPayload,
    UiIntentPayloadField, UiIntentPayloadFieldDescriptor, UiIntentPayloadFieldSet,
    UiIntentPayloadProjection, UiIntentPayloadProjectionViolation, UiIntentProductOutcome,
    UiIntentSchema, UiIntentSelection, UiIntentSelectionValue, UiIntentText, UiIntentUnsigned64,
    UiSemanticInteractionFamily,
};

pub(super) const QUERY_TEXT_FIELD: UiIntentPayloadField<QueryTextPayload, UiIntentText> =
    UiIntentPayloadField::text(0, "status", 32);
pub(super) const APPLICATION_TEXT_FIELD: UiIntentPayloadField<ApplicationPayload, UiIntentText> =
    UiIntentPayloadField::text(0, "message", 32);
pub(super) const APPLICATION_BOOLEAN_FIELD: UiIntentPayloadField<
    ApplicationPayload,
    UiIntentBoolean,
> = UiIntentPayloadField::boolean(1, "allowed");
pub(super) const APPLICATION_UNSIGNED_FIELD: UiIntentPayloadField<
    ApplicationPayload,
    UiIntentUnsigned64,
> = UiIntentPayloadField::unsigned64(2, "revision");
pub(super) const DRAFT_FIELD: UiIntentPayloadField<DraftPayload, UiIntentText> =
    UiIntentPayloadField::text(0, "committed_text", 16);
pub(super) const SELECTION_FIELD: UiIntentPayloadField<SelectionPayload, UiIntentSelection> =
    UiIntentPayloadField::selection(0, "selected_status");

pub(super) struct QueryTextPayload;
pub(super) struct ApplicationPayload;
pub(super) struct DraftPayload;
pub(super) struct SelectionPayload {
    _selection: UiIntentSelectionValue,
}
pub(super) struct WidePayload;
pub(super) struct PayloadOutcome;

pub(super) struct QueryTextIntent;
pub(super) struct ApplicationIntent;
pub(super) struct DraftIntent;
pub(super) struct SelectionIntent;
pub(super) struct WideIntent;

macro_rules! intent {
    ($intent:ty, $payload:ty, $identity:literal, $family:ident) => {
        impl UiIntent for $intent {
            type Payload = $payload;
            type ProductOutcome = PayloadOutcome;

            const ID: UiIntentId = UiIntentId::stable($identity);
            const ACCEPTED_INTERACTIONS: UiIntentAcceptedInteractions =
                UiIntentAcceptedInteractions::new(&[UiSemanticInteractionFamily::$family]);
        }
    };
}

impl UiIntentPayload for QueryTextPayload {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("phase3.payload.query_text", 1);
    const FIELDS: UiIntentPayloadFieldSet =
        UiIntentPayloadFieldSet::new(&[QUERY_TEXT_FIELD.descriptor()]);

    fn project(
        fields: &mut UiIntentPayloadProjection<Self>,
    ) -> Result<Self, UiIntentPayloadProjectionViolation> {
        require_text(
            fields.take(QUERY_TEXT_FIELD)?,
            "query-current",
            QUERY_TEXT_FIELD,
        )?;
        Ok(Self)
    }
}

impl UiIntentPayload for ApplicationPayload {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("phase3.payload.application", 1);
    const FIELDS: UiIntentPayloadFieldSet = UiIntentPayloadFieldSet::new(&[
        APPLICATION_TEXT_FIELD.descriptor(),
        APPLICATION_BOOLEAN_FIELD.descriptor(),
        APPLICATION_UNSIGNED_FIELD.descriptor(),
    ]);

    fn project(
        fields: &mut UiIntentPayloadProjection<Self>,
    ) -> Result<Self, UiIntentPayloadProjectionViolation> {
        require_text(
            fields.take(APPLICATION_TEXT_FIELD)?,
            "application-current",
            APPLICATION_TEXT_FIELD,
        )?;
        if !fields.take(APPLICATION_BOOLEAN_FIELD)?
            || fields.take(APPLICATION_UNSIGNED_FIELD)? != 42
        {
            return Err(UiIntentPayloadProjection::malformed(
                APPLICATION_UNSIGNED_FIELD,
            ));
        }
        Ok(Self)
    }
}

impl UiIntentPayload for DraftPayload {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("phase3.payload.draft", 1);
    const FIELDS: UiIntentPayloadFieldSet =
        UiIntentPayloadFieldSet::new(&[DRAFT_FIELD.descriptor()]);

    fn project(
        fields: &mut UiIntentPayloadProjection<Self>,
    ) -> Result<Self, UiIntentPayloadProjectionViolation> {
        require_text(fields.take(DRAFT_FIELD)?, "é🦀done", DRAFT_FIELD)?;
        Ok(Self)
    }
}

impl UiIntentPayload for SelectionPayload {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("phase3.payload.selection", 1);
    const FIELDS: UiIntentPayloadFieldSet =
        UiIntentPayloadFieldSet::new(&[SELECTION_FIELD.descriptor()]);

    fn project(
        fields: &mut UiIntentPayloadProjection<Self>,
    ) -> Result<Self, UiIntentPayloadProjectionViolation> {
        let selection = fields.take(SELECTION_FIELD)?;
        Ok(Self {
            _selection: selection,
        })
    }
}

impl UiIntentPayload for WidePayload {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("phase3.payload.wide", 1);
    const FIELDS: UiIntentPayloadFieldSet = UiIntentPayloadFieldSet::new(&WIDE_DESCRIPTORS);

    fn project(
        fields: &mut UiIntentPayloadProjection<Self>,
    ) -> Result<Self, UiIntentPayloadProjectionViolation> {
        for (index, field) in WIDE_FIELDS.into_iter().enumerate() {
            if fields.take(field)? != index as u64 {
                return Err(UiIntentPayloadProjection::malformed(field));
            }
        }
        Ok(Self)
    }
}

impl UiIntentProductOutcome for PayloadOutcome {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("phase3.payload.outcome", 1);
}

intent!(
    QueryTextIntent,
    QueryTextPayload,
    "phase3.intent.query_text",
    Activate
);
intent!(
    ApplicationIntent,
    ApplicationPayload,
    "phase3.intent.application",
    Activate
);
intent!(DraftIntent, DraftPayload, "phase3.intent.draft", EditCommit);
intent!(
    SelectionIntent,
    SelectionPayload,
    "phase3.intent.selection",
    SelectionCommit
);
intent!(WideIntent, WidePayload, "phase3.intent.wide", Activate);

fn require_text<P: UiIntentPayload>(
    observed: Arc<str>,
    expected: &str,
    field: UiIntentPayloadField<P, UiIntentText>,
) -> Result<(), UiIntentPayloadProjectionViolation> {
    if observed.as_ref() == expected {
        Ok(())
    } else {
        Err(UiIntentPayloadProjection::malformed(field))
    }
}

const WIDE_NAMES: [&str; 64] = [
    "f00", "f01", "f02", "f03", "f04", "f05", "f06", "f07", "f08", "f09", "f10", "f11", "f12",
    "f13", "f14", "f15", "f16", "f17", "f18", "f19", "f20", "f21", "f22", "f23", "f24", "f25",
    "f26", "f27", "f28", "f29", "f30", "f31", "f32", "f33", "f34", "f35", "f36", "f37", "f38",
    "f39", "f40", "f41", "f42", "f43", "f44", "f45", "f46", "f47", "f48", "f49", "f50", "f51",
    "f52", "f53", "f54", "f55", "f56", "f57", "f58", "f59", "f60", "f61", "f62", "f63",
];

pub(super) const WIDE_FIELDS: [UiIntentPayloadField<WidePayload, UiIntentUnsigned64>; 64] =
    wide_fields();
const WIDE_DESCRIPTORS: [UiIntentPayloadFieldDescriptor; 64] = wide_descriptors();

const fn wide_fields() -> [UiIntentPayloadField<WidePayload, UiIntentUnsigned64>; 64] {
    let mut fields = [UiIntentPayloadField::unsigned64(0, WIDE_NAMES[0]); 64];
    let mut index = 0;
    while index < fields.len() {
        fields[index] = UiIntentPayloadField::unsigned64(index as u8, WIDE_NAMES[index]);
        index += 1;
    }
    fields
}

const fn wide_descriptors() -> [UiIntentPayloadFieldDescriptor; 64] {
    let mut fields = [WIDE_FIELDS[0].descriptor(); 64];
    let mut index = 0;
    while index < fields.len() {
        fields[index] = WIDE_FIELDS[index].descriptor();
        index += 1;
    }
    fields
}
