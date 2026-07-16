#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum QuarantineReadmissionState {
    Proposed,
    Sealed,
    RecoveryVerificationPending,
    Readmitted,
    RetainedForAudit,
    Denied,
}

impl QuarantineReadmissionState {
    pub const fn all() -> [Self; 6] {
        [
            Self::Proposed,
            Self::Sealed,
            Self::RecoveryVerificationPending,
            Self::Readmitted,
            Self::RetainedForAudit,
            Self::Denied,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuarantineReadmissionDenial {
    QuarantineReceiptRequired,
    VerificationFrontierIncomplete,
    CurrentAuthorityRequired,
    ScopeMismatch,
    ObservationIsNotRepairAuthority,
    OperatorIntentIsNotRepairAuthority,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuarantineReadmissionModel {
    state: QuarantineReadmissionState,
    quarantined_scope: String,
}

impl QuarantineReadmissionModel {
    pub fn sealed(scope: impl Into<String>) -> Self {
        Self {
            state: QuarantineReadmissionState::Sealed,
            quarantined_scope: scope.into(),
        }
    }

    pub fn begin_verification(&mut self) {
        self.state = QuarantineReadmissionState::RecoveryVerificationPending;
    }

    pub fn readmit(
        &mut self,
        observed_scope: &str,
        verification_complete: bool,
        current_authority: bool,
    ) -> Result<(), QuarantineReadmissionDenial> {
        if self.state != QuarantineReadmissionState::RecoveryVerificationPending {
            return Err(QuarantineReadmissionDenial::QuarantineReceiptRequired);
        }
        if observed_scope != self.quarantined_scope {
            return Err(QuarantineReadmissionDenial::ScopeMismatch);
        }
        if !verification_complete {
            return Err(QuarantineReadmissionDenial::VerificationFrontierIncomplete);
        }
        if !current_authority {
            return Err(QuarantineReadmissionDenial::CurrentAuthorityRequired);
        }
        self.state = QuarantineReadmissionState::Readmitted;
        Ok(())
    }

    pub const fn reject_operator_repair() -> QuarantineReadmissionDenial {
        QuarantineReadmissionDenial::OperatorIntentIsNotRepairAuthority
    }

    pub const fn reject_offline_observation() -> QuarantineReadmissionDenial {
        QuarantineReadmissionDenial::ObservationIsNotRepairAuthority
    }

    pub const fn state(&self) -> QuarantineReadmissionState {
        self.state
    }
}
