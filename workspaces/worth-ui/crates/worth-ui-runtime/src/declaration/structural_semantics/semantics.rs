use crate::capability::MosaicSizingContractId;
use crate::declaration::{
    UiDeclarationContainmentIntent, UiDeclarationFamilyKind, UiDeclarationOrderingGuarantee,
    UiDeclarationPlanningOperatorKind, UiDeclarationRepetitionPosture,
    UiDeclarationSlotParticipationIntent, UiDeclarationStructuralRole,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiDeclarationStructuralSemantics {
    family_kind: UiDeclarationFamilyKind,
    role: UiDeclarationStructuralRole,
    operator_kind: UiDeclarationPlanningOperatorKind,
    mosaic_sizing_contract_id: Option<MosaicSizingContractId>,
    containment_intent: UiDeclarationContainmentIntent,
    slot_participation_intent: UiDeclarationSlotParticipationIntent,
    ordering_guarantee: UiDeclarationOrderingGuarantee,
    repetition_posture: UiDeclarationRepetitionPosture,
}

pub(crate) struct UiDeclarationStructuralSemanticsInput {
    pub(crate) family_kind: UiDeclarationFamilyKind,
    pub(crate) role: UiDeclarationStructuralRole,
    pub(crate) operator_kind: UiDeclarationPlanningOperatorKind,
    pub(crate) mosaic_sizing_contract_id: Option<MosaicSizingContractId>,
    pub(crate) containment_intent: UiDeclarationContainmentIntent,
    pub(crate) slot_participation_intent: UiDeclarationSlotParticipationIntent,
    pub(crate) ordering_guarantee: UiDeclarationOrderingGuarantee,
    pub(crate) repetition_posture: UiDeclarationRepetitionPosture,
}

impl UiDeclarationStructuralSemantics {
    pub(crate) fn new(input: UiDeclarationStructuralSemanticsInput) -> Self {
        let UiDeclarationStructuralSemanticsInput {
            family_kind,
            role,
            operator_kind,
            mosaic_sizing_contract_id,
            containment_intent,
            slot_participation_intent,
            ordering_guarantee,
            repetition_posture,
        } = input;
        Self {
            family_kind,
            role,
            operator_kind,
            mosaic_sizing_contract_id,
            containment_intent,
            slot_participation_intent,
            ordering_guarantee,
            repetition_posture,
        }
    }

    pub const fn family(&self) -> UiDeclarationFamilyKind {
        self.family_kind
    }

    pub const fn family_kind(&self) -> UiDeclarationFamilyKind {
        self.family_kind
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
