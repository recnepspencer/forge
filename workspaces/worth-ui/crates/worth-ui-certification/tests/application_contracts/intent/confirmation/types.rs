use worth_ui::facade::intent::{
    UiIntent, UiIntentAcceptedInteractions, UiIntentId, UiIntentPayload, UiIntentPayloadField,
    UiIntentPayloadFieldSet, UiIntentPayloadProjection, UiIntentPayloadProjectionViolation,
    UiIntentProductOutcome, UiIntentSchema, UiIntentUnsigned64, UiSemanticInteractionFamily,
};

pub(super) const REVISION_FIELD: UiIntentPayloadField<ConfirmationPayload, UiIntentUnsigned64> =
    UiIntentPayloadField::unsigned64(0, "revision");

pub(in crate::intent) struct ConfirmationIntent;
pub(in crate::intent) struct ConfirmationPayload {
    _revision: u64,
}
pub(in crate::intent) struct ConfirmationOutcome;

impl UiIntent for ConfirmationIntent {
    type Payload = ConfirmationPayload;
    type ProductOutcome = ConfirmationOutcome;

    const ID: UiIntentId = UiIntentId::stable("phase3.confirmation.intent");
    const ACCEPTED_INTERACTIONS: UiIntentAcceptedInteractions =
        UiIntentAcceptedInteractions::new(&[UiSemanticInteractionFamily::Activate]);
}

impl UiIntentPayload for ConfirmationPayload {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("phase3.confirmation.payload", 1);
    const FIELDS: UiIntentPayloadFieldSet =
        UiIntentPayloadFieldSet::new(&[REVISION_FIELD.descriptor()]);

    fn project(
        fields: &mut UiIntentPayloadProjection<Self>,
    ) -> Result<Self, UiIntentPayloadProjectionViolation> {
        Ok(Self {
            _revision: fields.take(REVISION_FIELD)?,
        })
    }
}

impl UiIntentProductOutcome for ConfirmationOutcome {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("phase3.confirmation.outcome", 1);
    const CONSEQUENCE_FAMILIES: worth_ui::facade::intent::UiIntentProductConsequenceFamilies =
        worth_ui::facade::intent::UiIntentProductConsequenceFamilies::NONE;

    fn into_consequences(self) -> worth_ui::facade::intent::UiIntentProductConsequences {
        worth_ui::facade::intent::UiIntentProductConsequences::none()
    }
}
