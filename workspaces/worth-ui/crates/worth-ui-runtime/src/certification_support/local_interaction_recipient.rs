use crate::runtime::interaction::UiLocalInputRecipientContract;

struct CertificationDraftPayload;

impl crate::capability::UiIntentPayload for CertificationDraftPayload {
    const SCHEMA: crate::capability::UiIntentSchema =
        crate::capability::UiIntentSchema::stable("worth-ui.certification.draft", 1);
    const FIELDS: crate::capability::UiIntentPayloadFieldSet =
        crate::capability::UiIntentPayloadFieldSet::new(&[CERTIFICATION_DRAFT_FIELD.descriptor()]);

    fn project(
        _fields: &mut crate::capability::UiIntentPayloadProjection<Self>,
    ) -> Result<Self, crate::capability::UiIntentPayloadProjectionViolation> {
        Ok(Self)
    }
}

const CERTIFICATION_DRAFT_FIELD: crate::capability::UiIntentPayloadField<
    CertificationDraftPayload,
    crate::capability::UiIntentText,
> = crate::capability::UiIntentPayloadField::text(0, "certification.draft", 32);

/// Privileged world-compiler input for exercising the production draft owner
/// before Phase 3's declaration compiler becomes the ordinary minter.
pub fn draft_recipient_contract_for_certification() -> UiLocalInputRecipientContract {
    UiLocalInputRecipientContract::draft(CERTIFICATION_DRAFT_FIELD)
        .expect("the certification compiler accepts only prevalidated draft budgets")
}
