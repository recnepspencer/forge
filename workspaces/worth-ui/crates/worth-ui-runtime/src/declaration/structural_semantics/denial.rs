use crate::declaration::{UiDeclarationFamilyAdmissionDenial, UiDeclarationFamilyKind};

use super::UiDeclarationStructuralSemantics;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiDeclarationStructuralSemanticsAdmission {
    Admitted(UiDeclarationStructuralSemantics),
    Denied(UiDeclarationStructuralSemanticsAdmissionDenial),
}

impl UiDeclarationStructuralSemanticsAdmission {
    pub const fn admitted_structural_semantics(
        &self,
    ) -> Result<&UiDeclarationStructuralSemantics, &UiDeclarationStructuralSemanticsAdmissionDenial>
    {
        match self {
            Self::Admitted(semantics) => Ok(semantics),
            Self::Denied(denial) => Err(denial),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiDeclarationStructuralSemanticsAdmissionDenial {
    FamilyNotAdmitted {
        denial: UiDeclarationFamilyAdmissionDenial,
    },
    FamilyDoesNotProjectStructuralSemantics {
        family: UiDeclarationFamilyKind,
    },
    ContradictorySlotParticipationClaims {
        family: UiDeclarationFamilyKind,
        observed: Vec<String>,
    },
    InvalidSlotParticipationClaim {
        family: UiDeclarationFamilyKind,
        observed: Vec<String>,
    },
    SlotParticipationNotAdmittedForFamily {
        family: UiDeclarationFamilyKind,
        observed: Vec<String>,
    },
    ContradictoryPlanningOperatorClaims {
        family: UiDeclarationFamilyKind,
        observed: Vec<String>,
    },
    InvalidPlanningOperatorClaim {
        family: UiDeclarationFamilyKind,
        observed: Vec<String>,
    },
    ContradictoryMosaicSizingContractClaims {
        family: UiDeclarationFamilyKind,
        observed: Vec<String>,
    },
    InvalidMosaicSizingContractClaim {
        family: UiDeclarationFamilyKind,
        observed: Vec<String>,
    },
    PlanningOperatorNotAdmittedForFamily {
        family: UiDeclarationFamilyKind,
        observed: Vec<String>,
    },
    InvalidStructuralMembershipClaim {
        family: UiDeclarationFamilyKind,
        observed: Vec<String>,
    },
    UnsupportedStructuralTokens {
        family: UiDeclarationFamilyKind,
        observed: Vec<String>,
    },
}
