use crate::declaration::family::UiDeclarationFamilyKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiDeclarationFamilyAdmissionDenial {
    MissingStructuralClaim {
        family: UiDeclarationFamilyKind,
        expected_prefix: &'static str,
    },
    ContradictoryStructuralClaims {
        family: UiDeclarationFamilyKind,
        observed: Vec<String>,
    },
    StandaloneFamilyRequiresStandalonePosture {
        family: UiDeclarationFamilyKind,
        expected_token: &'static str,
    },
    StandaloneFamilyCannotCarryStructuralClaims {
        family: UiDeclarationFamilyKind,
        observed: Vec<String>,
    },
    StandaloneFamilyCannotProjectAttachedRole {
        family: UiDeclarationFamilyKind,
        observed: Vec<String>,
    },
    StructuralFamilyCannotClaimStandaloneRole {
        family: UiDeclarationFamilyKind,
        observed: Vec<String>,
    },
    ContradictoryAttachedRoleClaims {
        family: UiDeclarationFamilyKind,
        observed: Vec<String>,
    },
    InvalidAttachedRoleClaim {
        family: UiDeclarationFamilyKind,
        expected_prefix: &'static str,
        observed: Vec<String>,
    },
}
