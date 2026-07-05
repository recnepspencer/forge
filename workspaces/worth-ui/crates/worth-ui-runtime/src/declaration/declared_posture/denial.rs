use crate::declaration::UiDeclarationFamilyAdmissionDenial;
use crate::declaration::UiDeclarationFamilyKind;

use super::UiDeclaredPostureContract;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiDeclaredPostureLaneKind {
    QueryBinding,
    ServiceUsage,
    TouchMeaning,
    MeasurementPolicy,
    HostCapability,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiDeclaredPostureAdmission {
    Admitted(UiDeclaredPostureContract),
    Denied(UiDeclaredPostureAdmissionDenial),
}

impl UiDeclaredPostureAdmission {
    pub const fn admitted_contract(
        &self,
    ) -> Result<&UiDeclaredPostureContract, &UiDeclaredPostureAdmissionDenial> {
        match self {
            Self::Admitted(contract) => Ok(contract),
            Self::Denied(denial) => Err(denial),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiDeclaredPostureAdmissionDenial {
    FamilyNotAdmitted {
        denial: UiDeclarationFamilyAdmissionDenial,
    },
    LaneNotApplicableForFamily {
        family: UiDeclarationFamilyKind,
        lane: UiDeclaredPostureLaneKind,
        observed: Vec<String>,
    },
    LaneArchitecturallyOwnedButNotYetAdmitted {
        family: UiDeclarationFamilyKind,
        lane: UiDeclaredPostureLaneKind,
        observed: Vec<String>,
    },
    ContradictoryLaneClaims {
        family: UiDeclarationFamilyKind,
        lane: UiDeclaredPostureLaneKind,
        observed: Vec<String>,
    },
    ImpossibleLaneCombination {
        family: UiDeclarationFamilyKind,
        lane: UiDeclaredPostureLaneKind,
        observed: Vec<String>,
        reason: &'static str,
    },
    InvalidLaneClaim {
        family: UiDeclarationFamilyKind,
        lane: UiDeclaredPostureLaneKind,
        observed: Vec<String>,
    },
}
