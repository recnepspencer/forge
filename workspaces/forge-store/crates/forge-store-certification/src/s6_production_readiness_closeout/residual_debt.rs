use forge_store_readiness::S6ReadinessResidualDebtEvidenceKind;

use crate::S6CertificationEvidenceAdoptionReceipt;

use super::denial::S6ProductionReadinessClosureDenial;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum S6ResidualDebtKind {
    UnsupportedBackendProfile,
    UnavailableEvidence,
    DegradedBackendPosture,
    DeniedClaim,
    StaleEvidence,
    RebindRequired,
    ResidualQualificationDebt,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct S6ResidualDebtRow {
    kind: S6ResidualDebtKind,
    observed_claims: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct S6ResidualDebtLedger {
    rows: Vec<S6ResidualDebtRow>,
}

impl S6ResidualDebtRow {
    const fn new(kind: S6ResidualDebtKind, observed_claims: usize) -> Self {
        Self {
            kind,
            observed_claims,
        }
    }

    pub const fn kind(&self) -> S6ResidualDebtKind {
        self.kind
    }

    pub const fn observed_claims(&self) -> usize {
        self.observed_claims
    }
}

impl S6ResidualDebtLedger {
    pub(super) fn from_phase13_adoption(
        adoption: &S6CertificationEvidenceAdoptionReceipt,
    ) -> Result<Self, S6ProductionReadinessClosureDenial> {
        if !has_exact_required_debt(adoption) {
            return Err(S6ProductionReadinessClosureDenial::Phase13EvidenceCannotSatisfyReadiness);
        }
        Ok(Self {
            rows: adoption
                .residual_debt_rows()
                .iter()
                .map(|row| S6ResidualDebtRow::new(row.kind().into(), row.observed_claims()))
                .collect(),
        })
    }

    pub fn rows(&self) -> &[S6ResidualDebtRow] {
        &self.rows
    }

    pub fn contains_non_platform_grade_posture(&self) -> bool {
        self.rows.iter().any(|row| row.observed_claims() > 0)
    }
}

impl From<S6ReadinessResidualDebtEvidenceKind> for S6ResidualDebtKind {
    fn from(kind: S6ReadinessResidualDebtEvidenceKind) -> Self {
        match kind {
            S6ReadinessResidualDebtEvidenceKind::UnsupportedBackendProfile => {
                Self::UnsupportedBackendProfile
            }
            S6ReadinessResidualDebtEvidenceKind::UnavailableEvidence => Self::UnavailableEvidence,
            S6ReadinessResidualDebtEvidenceKind::DegradedBackendPosture => {
                Self::DegradedBackendPosture
            }
            S6ReadinessResidualDebtEvidenceKind::DeniedClaim => Self::DeniedClaim,
            S6ReadinessResidualDebtEvidenceKind::StaleEvidence => Self::StaleEvidence,
            S6ReadinessResidualDebtEvidenceKind::RebindRequired => Self::RebindRequired,
            S6ReadinessResidualDebtEvidenceKind::ResidualQualificationDebt => {
                Self::ResidualQualificationDebt
            }
        }
    }
}

fn has_exact_required_debt(adoption: &S6CertificationEvidenceAdoptionReceipt) -> bool {
    use S6ReadinessResidualDebtEvidenceKind::{
        DegradedBackendPosture, DeniedClaim, RebindRequired, ResidualQualificationDebt,
        StaleEvidence, UnavailableEvidence, UnsupportedBackendProfile,
    };
    [
        UnsupportedBackendProfile,
        UnavailableEvidence,
        DegradedBackendPosture,
        DeniedClaim,
        StaleEvidence,
        RebindRequired,
        ResidualQualificationDebt,
    ]
    .into_iter()
    .all(|kind| {
        adoption
            .residual_debt_rows()
            .iter()
            .any(|row| row.kind() == kind && row.observed_claims() > 0)
    })
}
