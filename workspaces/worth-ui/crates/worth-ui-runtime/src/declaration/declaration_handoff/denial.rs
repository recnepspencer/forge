use crate::declaration::{
    UiDeclarationFamilyAdmissionDenial, UiDeclarationStructuralSemanticsAdmissionDenial,
    UiDeclaredPostureAdmissionDenial,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiDeclarationGraphHandoffDenial {
    FamilyNotAdmitted {
        denial: UiDeclarationFamilyAdmissionDenial,
    },
    StructuralSemanticsNotAdmitted {
        denial: UiDeclarationStructuralSemanticsAdmissionDenial,
    },
    DeclaredPostureNotAdmitted {
        denial: UiDeclaredPostureAdmissionDenial,
    },
}
