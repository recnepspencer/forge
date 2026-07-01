use crate::declaration::{
    UiDeclarationContainmentIntent, UiDeclarationFamily, UiDeclarationFamilyKind,
    UiDeclarationOrderingGuarantee, UiDeclarationRepetitionPosture,
    UiDeclarationSlotParticipationIntent, UiDeclarationStructuralRole,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiStructuralDeclarationPayload {
    family: UiDeclarationFamily,
    role: UiDeclarationStructuralRole,
    containment_intent: UiDeclarationContainmentIntent,
    slot_participation_intent: UiDeclarationSlotParticipationIntent,
    ordering_guarantee: UiDeclarationOrderingGuarantee,
    repetition_posture: UiDeclarationRepetitionPosture,
}

impl UiStructuralDeclarationPayload {
    pub(crate) fn new(
        family: UiDeclarationFamily,
        role: UiDeclarationStructuralRole,
        containment_intent: UiDeclarationContainmentIntent,
        slot_participation_intent: UiDeclarationSlotParticipationIntent,
        ordering_guarantee: UiDeclarationOrderingGuarantee,
        repetition_posture: UiDeclarationRepetitionPosture,
    ) -> Self {
        Self {
            family,
            role,
            containment_intent,
            slot_participation_intent,
            ordering_guarantee,
            repetition_posture,
        }
    }

    pub fn family(&self) -> &UiDeclarationFamily {
        &self.family
    }

    pub const fn family_kind(&self) -> UiDeclarationFamilyKind {
        self.family.kind()
    }

    pub const fn role(&self) -> UiDeclarationStructuralRole {
        self.role
    }

    pub fn containment_intent(&self) -> &UiDeclarationContainmentIntent {
        &self.containment_intent
    }

    pub fn slot_participation_intent(&self) -> &UiDeclarationSlotParticipationIntent {
        &self.slot_participation_intent
    }

    pub const fn ordering_guarantee(&self) -> UiDeclarationOrderingGuarantee {
        self.ordering_guarantee
    }

    pub const fn repetition_posture(&self) -> UiDeclarationRepetitionPosture {
        self.repetition_posture
    }
}
