use worth_ui::facade::intent::{
    UiIntent, UiIntentAcceptedInteractions, UiIntentId, UiIntentPayload, UiIntentPayloadField,
    UiIntentPayloadFieldSet, UiIntentPayloadProjection, UiIntentPayloadProjectionViolation,
    UiIntentProductOutcome, UiIntentSchema, UiIntentText, UiIntentTransitionDestination,
    UiIntentTransitionOutcome, UiSemanticInteractionFamily,
};

pub(super) const EDIT_FIELD: UiIntentPayloadField<EditPayload, UiIntentText> =
    UiIntentPayloadField::text(0, "committed_text", 32);

pub(in crate::intent) struct EmptyPayload;
pub(in crate::intent) struct EmptyOutcome;
pub(in crate::intent) struct ConsequenceOutcome {
    query: Option<worth_ui_query_binding::WorthUiCollectionChangeConsequence>,
}
pub(super) struct EditPayload;
pub(in crate::intent) struct PrimaryIntent;
pub(in crate::intent) struct ConsequenceIntent;
pub(super) struct SecondaryIntent;
pub(super) struct ProjectionIntent;
pub(super) struct EditIntent;
pub(super) struct UnsupportedIntent;

macro_rules! activation_intent {
    ($intent:ty, $identity:literal) => {
        impl UiIntent for $intent {
            type Payload = EmptyPayload;
            type ProductOutcome = EmptyOutcome;

            const ID: UiIntentId = UiIntentId::stable($identity);
            const ACCEPTED_INTERACTIONS: UiIntentAcceptedInteractions =
                UiIntentAcceptedInteractions::new(&[UiSemanticInteractionFamily::Activate]);
        }
    };
}

impl UiIntentPayload for EmptyPayload {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("phase3.operability.payload", 1);
    const FIELDS: UiIntentPayloadFieldSet = UiIntentPayloadFieldSet::EMPTY;

    fn project(
        _fields: &mut UiIntentPayloadProjection<Self>,
    ) -> Result<Self, UiIntentPayloadProjectionViolation> {
        Ok(Self)
    }
}

impl UiIntentPayload for EditPayload {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("phase3.operability.edit_payload", 1);
    const FIELDS: UiIntentPayloadFieldSet =
        UiIntentPayloadFieldSet::new(&[EDIT_FIELD.descriptor()]);

    fn project(
        fields: &mut UiIntentPayloadProjection<Self>,
    ) -> Result<Self, UiIntentPayloadProjectionViolation> {
        let _ = fields.take(EDIT_FIELD)?;
        Ok(Self)
    }
}

impl UiIntentProductOutcome for EmptyOutcome {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("phase3.operability.outcome", 1);
    const CONSEQUENCE_FAMILIES: worth_ui::facade::intent::UiIntentProductConsequenceFamilies =
        worth_ui::facade::intent::UiIntentProductConsequenceFamilies::NONE;

    fn into_consequences(self) -> worth_ui::facade::intent::UiIntentProductConsequences {
        worth_ui::facade::intent::UiIntentProductConsequences::none()
    }
}

impl UiIntentTransitionOutcome for EmptyOutcome {
    fn from_completed_transition(_destination: UiIntentTransitionDestination) -> Self {
        Self
    }
}

impl ConsequenceOutcome {
    pub(in crate::intent) fn query(
        query: worth_ui_query_binding::WorthUiCollectionChangeConsequence,
    ) -> Self {
        Self { query: Some(query) }
    }

    pub(in crate::intent) const fn none() -> Self {
        Self { query: None }
    }
}

impl UiIntentProductOutcome for ConsequenceOutcome {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("phase4.consequence.outcome", 1);
    const CONSEQUENCE_FAMILIES: worth_ui::facade::intent::UiIntentProductConsequenceFamilies =
        worth_ui::facade::intent::UiIntentProductConsequenceFamilies::QUERY_COLLECTION_CHANGE;

    fn into_consequences(self) -> worth_ui::facade::intent::UiIntentProductConsequences {
        match self.query {
            Some(query) => {
                worth_ui::facade::intent::UiIntentProductConsequences::query_collection_change(
                    query,
                )
            }
            None => worth_ui::facade::intent::UiIntentProductConsequences::none(),
        }
    }
}

activation_intent!(PrimaryIntent, "phase3.operability.intent.primary");
activation_intent!(SecondaryIntent, "phase3.operability.intent.secondary");
activation_intent!(ProjectionIntent, "phase3.operability.intent.projection");
activation_intent!(UnsupportedIntent, "phase3.operability.intent.unsupported");

impl UiIntent for ConsequenceIntent {
    type Payload = EmptyPayload;
    type ProductOutcome = ConsequenceOutcome;

    const ID: UiIntentId = UiIntentId::stable("phase4.consequence.intent");
    const ACCEPTED_INTERACTIONS: UiIntentAcceptedInteractions =
        UiIntentAcceptedInteractions::new(&[UiSemanticInteractionFamily::Activate]);
}

impl UiIntent for EditIntent {
    type Payload = EditPayload;
    type ProductOutcome = EmptyOutcome;

    const ID: UiIntentId = UiIntentId::stable("phase3.operability.intent.edit");
    const ACCEPTED_INTERACTIONS: UiIntentAcceptedInteractions =
        UiIntentAcceptedInteractions::new(&[UiSemanticInteractionFamily::EditCommit]);
}
