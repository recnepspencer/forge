use forge_store_physical_backend::{BackendCapabilityKind, BackendTargetProfile};

use super::{
    BackendQualificationMatrixDenial, BackendQualificationRow, PublishedQualificationPosture,
    QualificationResidualDebt,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackendQualificationParityComparison {
    left_profile: BackendTargetProfile,
    right_profile: BackendTargetProfile,
    capability: BackendCapabilityKind,
    left_posture: PublishedQualificationPosture,
    right_posture: PublishedQualificationPosture,
    left_residual_debt: QualificationResidualDebt,
    right_residual_debt: QualificationResidualDebt,
}

impl BackendQualificationParityComparison {
    pub fn compare(
        left: &BackendQualificationRow,
        right: &BackendQualificationRow,
    ) -> Result<Self, BackendQualificationMatrixDenial> {
        if left.capability() != right.capability() {
            return Err(BackendQualificationMatrixDenial::UnsupportedCapability {
                capability: right.capability(),
                posture: right.support_posture(),
            });
        }
        Ok(Self {
            left_profile: left.profile(),
            right_profile: right.profile(),
            capability: left.capability(),
            left_posture: left.published_posture(),
            right_posture: right.published_posture(),
            left_residual_debt: left.residual_debt(),
            right_residual_debt: right.residual_debt(),
        })
    }

    pub fn policy_equivalent(self) -> bool {
        self.left_posture == self.right_posture
    }

    pub const fn left_profile(self) -> BackendTargetProfile {
        self.left_profile
    }

    pub const fn right_profile(self) -> BackendTargetProfile {
        self.right_profile
    }

    pub const fn capability(self) -> BackendCapabilityKind {
        self.capability
    }

    pub const fn left_posture(self) -> PublishedQualificationPosture {
        self.left_posture
    }

    pub const fn right_posture(self) -> PublishedQualificationPosture {
        self.right_posture
    }

    pub const fn left_residual_debt(self) -> QualificationResidualDebt {
        self.left_residual_debt
    }

    pub const fn right_residual_debt(self) -> QualificationResidualDebt {
        self.right_residual_debt
    }
}

pub fn require_profile_local_row(
    expected: BackendTargetProfile,
    row: &BackendQualificationRow,
) -> Result<(), BackendQualificationMatrixDenial> {
    if row.profile() == expected {
        Ok(())
    } else {
        Err(
            BackendQualificationMatrixDenial::CrossBackendEvidenceSubstitution {
                expected,
                actual: row.profile(),
            },
        )
    }
}
