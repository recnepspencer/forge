use sha2::{Digest, Sha256};

use crate::RecoveryAuthorityAdmissionPosture;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryAuthorityAdmissionPolicyKind {
    FullyTrustedOnly,
    ExactDeclaredResidualPosture,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RecoveryAuthorityRegionPosture;

    #[test]
    fn residual_posture_requires_an_exact_explicit_decision() {
        let residual = posture(0, 2);
        assert_eq!(
            RecoveryAuthorityAdmissionPolicy::fully_trusted_only().validate(residual),
            Err(RecoveryAuthorityAdmissionPolicyDenial::ResidualPostureNotPermitted)
        );
        let policy = RecoveryAuthorityAdmissionPolicy::admit_exact_declared_residual_posture(
            residual, [7; 32],
        )
        .unwrap();
        assert_eq!(policy.validate(residual), Ok(()));
        assert_eq!(
            policy.validate(posture(0, 3)),
            Err(RecoveryAuthorityAdmissionPolicyDenial::AdmittedPostureMismatch)
        );
    }

    #[test]
    fn fully_trusted_policy_rejects_residual_policy_construction() {
        let trusted = posture(3, 0);
        assert!(trusted.is_fully_trusted());
        assert_eq!(
            RecoveryAuthorityAdmissionPolicy::fully_trusted_only().validate(trusted),
            Ok(())
        );
        assert!(
            RecoveryAuthorityAdmissionPolicy::admit_exact_declared_residual_posture(
                trusted, [7; 32]
            )
            .is_none()
        );
    }

    fn posture(trusted_count: u64, unavailable_count: u64) -> RecoveryAuthorityAdmissionPosture {
        let region = |tag: u8, count: u64| {
            RecoveryAuthorityRegionPosture::observed(
                if count == 0 { [0; 32] } else { [tag; 32] },
                count,
            )
            .unwrap()
        };
        RecoveryAuthorityAdmissionPosture::from_independent_post_verification(
            [1; 32],
            [
                region(2, trusted_count),
                region(3, 0),
                region(4, 0),
                region(5, 0),
                region(6, unavailable_count),
            ],
        )
        .unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryAuthorityAdmissionPolicy {
    kind: RecoveryAuthorityAdmissionPolicyKind,
    admitted_posture_identity: [u8; 32],
    decision_basis: [u8; 32],
    identity: [u8; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryAuthorityAdmissionPolicyDenial {
    ResidualPostureNotPermitted,
    AdmittedPostureMismatch,
}

impl RecoveryAuthorityAdmissionPolicy {
    pub fn fully_trusted_only() -> Self {
        let decision_basis: [u8; 32] =
            Sha256::digest(b"worth-store-fully-trusted-recovery-admission-policy-v1").into();
        Self::new(
            RecoveryAuthorityAdmissionPolicyKind::FullyTrustedOnly,
            [0; 32],
            decision_basis,
        )
    }

    pub fn admit_exact_declared_residual_posture(
        posture: RecoveryAuthorityAdmissionPosture,
        decision_basis: [u8; 32],
    ) -> Option<Self> {
        if decision_basis == [0; 32] || posture.is_fully_trusted() {
            return None;
        }
        Some(Self::new(
            RecoveryAuthorityAdmissionPolicyKind::ExactDeclaredResidualPosture,
            posture.identity(),
            decision_basis,
        ))
    }

    pub fn validate(
        self,
        posture: RecoveryAuthorityAdmissionPosture,
    ) -> Result<(), RecoveryAuthorityAdmissionPolicyDenial> {
        match self.kind {
            RecoveryAuthorityAdmissionPolicyKind::FullyTrustedOnly => {
                if posture.is_fully_trusted() {
                    Ok(())
                } else {
                    Err(RecoveryAuthorityAdmissionPolicyDenial::ResidualPostureNotPermitted)
                }
            }
            RecoveryAuthorityAdmissionPolicyKind::ExactDeclaredResidualPosture => {
                if posture.identity() == self.admitted_posture_identity {
                    Ok(())
                } else {
                    Err(RecoveryAuthorityAdmissionPolicyDenial::AdmittedPostureMismatch)
                }
            }
        }
    }

    pub fn from_persisted(
        kind: RecoveryAuthorityAdmissionPolicyKind,
        admitted_posture_identity: [u8; 32],
        decision_basis: [u8; 32],
    ) -> Option<Self> {
        match kind {
            RecoveryAuthorityAdmissionPolicyKind::FullyTrustedOnly => {
                let canonical = Self::fully_trusted_only();
                (admitted_posture_identity == [0; 32] && decision_basis == canonical.decision_basis)
                    .then_some(canonical)
            }
            RecoveryAuthorityAdmissionPolicyKind::ExactDeclaredResidualPosture => {
                if admitted_posture_identity == [0; 32] || decision_basis == [0; 32] {
                    None
                } else {
                    Some(Self::new(kind, admitted_posture_identity, decision_basis))
                }
            }
        }
    }

    fn new(
        kind: RecoveryAuthorityAdmissionPolicyKind,
        admitted_posture_identity: [u8; 32],
        decision_basis: [u8; 32],
    ) -> Self {
        let mut digest = Sha256::new();
        digest.update(b"worth-store-recovery-authority-admission-policy-v1");
        digest.update([match kind {
            RecoveryAuthorityAdmissionPolicyKind::FullyTrustedOnly => 1,
            RecoveryAuthorityAdmissionPolicyKind::ExactDeclaredResidualPosture => 2,
        }]);
        digest.update(admitted_posture_identity);
        digest.update(decision_basis);
        Self {
            kind,
            admitted_posture_identity,
            decision_basis,
            identity: digest.finalize().into(),
        }
    }

    pub const fn kind(self) -> RecoveryAuthorityAdmissionPolicyKind {
        self.kind
    }

    pub const fn admitted_posture_identity(self) -> [u8; 32] {
        self.admitted_posture_identity
    }

    pub const fn decision_basis(self) -> [u8; 32] {
        self.decision_basis
    }

    pub const fn identity(self) -> [u8; 32] {
        self.identity
    }
}
