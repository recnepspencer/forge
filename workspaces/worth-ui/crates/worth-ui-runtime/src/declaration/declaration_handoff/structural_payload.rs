use crate::capability::MosaicSizingContractId;
use crate::declaration::{
    UiDeclarationContainmentIntent, UiDeclarationFamily, UiDeclarationFamilyKind,
    UiDeclarationOrderingGuarantee, UiDeclarationPlanningOperatorKind,
    UiDeclarationRepetitionPosture, UiDeclarationSlotParticipationIntent,
    UiDeclarationStructuralDigest, UiDeclarationStructuralRole, UiDeclarationStructuralSemantics,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiStructuralDeclarationPayload {
    family: UiDeclarationFamily,
    structural_digest: UiDeclarationStructuralDigest,
    semantics: UiDeclarationStructuralSemantics,
}

impl UiStructuralDeclarationPayload {
    pub(crate) fn new(
        family: UiDeclarationFamily,
        structural_digest: UiDeclarationStructuralDigest,
        semantics: UiDeclarationStructuralSemantics,
    ) -> Self {
        Self {
            family,
            structural_digest,
            semantics,
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
        self.semantics.role()
    }

    pub const fn operator_kind(&self) -> UiDeclarationPlanningOperatorKind {
        self.semantics.operator_kind()
    }

    pub fn mosaic_sizing_contract_id(&self) -> Option<&MosaicSizingContractId> {
        self.semantics.mosaic_sizing_contract_id()
    }

    pub fn containment_intent(&self) -> &UiDeclarationContainmentIntent {
        self.semantics.containment_intent()
    }

    pub fn slot_participation_intent(&self) -> &UiDeclarationSlotParticipationIntent {
        self.semantics.slot_participation_intent()
    }

    pub const fn ordering_guarantee(&self) -> UiDeclarationOrderingGuarantee {
        self.semantics.ordering_guarantee()
    }

    pub const fn repetition_posture(&self) -> UiDeclarationRepetitionPosture {
        self.semantics.repetition_posture()
    }

    pub const fn semantics(&self) -> &UiDeclarationStructuralSemantics {
        &self.semantics
    }
}
