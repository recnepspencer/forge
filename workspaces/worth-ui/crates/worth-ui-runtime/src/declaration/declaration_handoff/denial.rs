use crate::capability::MosaicSizingContractId;
use crate::declaration::{
    UiAspectContractAdmissionDenial, UiDeclarationFamilyAdmissionDenial,
    UiDeclarationStructuralSemanticsAdmissionDenial, UiDeclaredPostureAdmissionDenial,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiDeclarationGraphHandoffDenial {
    AspectContractNotAdmitted {
        denial: UiAspectContractAdmissionDenial,
    },
    FamilyNotAdmitted {
        denial: UiDeclarationFamilyAdmissionDenial,
    },
    StructuralSemanticsNotAdmitted {
        denial: UiDeclarationStructuralSemanticsAdmissionDenial,
    },
    DeclaredPostureNotAdmitted {
        denial: UiDeclaredPostureAdmissionDenial,
    },
    SourceBackedMosaicSizingContractProjectionDenied {
        observed: Vec<MosaicSizingContractId>,
    },
    SourceBackedMosaicSizingContractConflict {
        declared: MosaicSizingContractId,
        sourced: MosaicSizingContractId,
    },
}
