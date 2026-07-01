use crate::declaration::UiDeclaredPostureAdmissionDenial;

use super::UiDeclarationSupportSnapshot;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiDeclarationSupportSnapshotAdmission {
    Admitted(UiDeclarationSupportSnapshot),
    Denied(UiDeclarationSupportSnapshotAdmissionDenial),
}

impl UiDeclarationSupportSnapshotAdmission {
    pub const fn admitted_snapshot(
        &self,
    ) -> Result<&UiDeclarationSupportSnapshot, &UiDeclarationSupportSnapshotAdmissionDenial> {
        match self {
            Self::Admitted(snapshot) => Ok(snapshot),
            Self::Denied(denial) => Err(denial),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiDeclarationSupportSnapshotAdmissionDenial {
    DeclaredPostureNotAdmitted {
        denial: UiDeclaredPostureAdmissionDenial,
    },
}
