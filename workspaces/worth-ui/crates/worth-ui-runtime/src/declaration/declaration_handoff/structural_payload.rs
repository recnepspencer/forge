use crate::capability::MosaicSizingContractId;
use crate::declaration::{
    UiDeclarationContainmentIntent, UiDeclarationFamily, UiDeclarationFamilyKind,
    UiDeclarationOrderingGuarantee, UiDeclarationPlanningOperatorKind,
    UiDeclarationRepetitionPosture, UiDeclarationSlotParticipationIntent,
    UiDeclarationStructuralDigest, UiDeclarationStructuralRole,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiStructuralDeclarationPayload {
    family: UiDeclarationFamily,
    structural_digest: UiDeclarationStructuralDigest,
    role: UiDeclarationStructuralRole,
    operator_kind: UiDeclarationPlanningOperatorKind,
    mosaic_sizing_contract_id: Option<MosaicSizingContractId>,
    containment_intent: UiDeclarationContainmentIntent,
    slot_participation_intent: UiDeclarationSlotParticipationIntent,
    ordering_guarantee: UiDeclarationOrderingGuarantee,
    repetition_posture: UiDeclarationRepetitionPosture,
}

impl UiStructuralDeclarationPayload {
    pub(crate) fn new(
        family: UiDeclarationFamily,
        structural_digest: UiDeclarationStructuralDigest,
        role: UiDeclarationStructuralRole,
        operator_kind: UiDeclarationPlanningOperatorKind,
        mosaic_sizing_contract_id: Option<MosaicSizingContractId>,
        containment_intent: UiDeclarationContainmentIntent,
        slot_participation_intent: UiDeclarationSlotParticipationIntent,
        ordering_guarantee: UiDeclarationOrderingGuarantee,
        repetition_posture: UiDeclarationRepetitionPosture,
    ) -> Self {
        Self {
            family,
            structural_digest,
            role,
            operator_kind,
            mosaic_sizing_contract_id,
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

    pub const fn structural_digest(&self) -> UiDeclarationStructuralDigest {
        self.structural_digest
    }

    pub const fn role(&self) -> UiDeclarationStructuralRole {
        self.role
    }

    pub const fn operator_kind(&self) -> UiDeclarationPlanningOperatorKind {
        self.operator_kind
    }

    pub fn mosaic_sizing_contract_id(&self) -> Option<&MosaicSizingContractId> {
        self.mosaic_sizing_contract_id.as_ref()
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
