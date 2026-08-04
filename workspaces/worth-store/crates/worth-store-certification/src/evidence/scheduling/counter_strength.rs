use worth_store_budgets::CounterEvidenceStrength;
use worth_store_io_scheduler::SecureIoCounterStrength;
use worth_store_physical_backend::AccessPolicyCounterStrength;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum S6MaterializedCounterStrength {
    Exact,
    Bounded,
    Sampled,
    Derived,
    CertificationOnly,
    Unavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum S6CounterStrengthFamily {
    ForegroundReservation,
    BackgroundPacing,
    QueueExecution,
    FlushDurability,
    LatencyInterference,
    SecureIoPreservation,
    AccessPolicy,
    PostAdmissionViolation,
    QualificationMatrix,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct S6CounterStrengthDeclaration {
    family: S6CounterStrengthFamily,
    strength: S6MaterializedCounterStrength,
    observed_rows: usize,
}

impl S6CounterStrengthDeclaration {
    pub const fn new(
        family: S6CounterStrengthFamily,
        strength: S6MaterializedCounterStrength,
        observed_rows: usize,
    ) -> Self {
        Self {
            family,
            strength,
            observed_rows,
        }
    }

    pub const fn family(self) -> S6CounterStrengthFamily {
        self.family
    }

    pub const fn strength(self) -> S6MaterializedCounterStrength {
        self.strength
    }

    pub const fn observed_rows(self) -> usize {
        self.observed_rows
    }
}

impl From<CounterEvidenceStrength> for S6MaterializedCounterStrength {
    fn from(strength: CounterEvidenceStrength) -> Self {
        match strength {
            CounterEvidenceStrength::Exact => Self::Exact,
            CounterEvidenceStrength::Bounded => Self::Bounded,
            CounterEvidenceStrength::Sampled => Self::Sampled,
            CounterEvidenceStrength::Derived => Self::Derived,
            CounterEvidenceStrength::CertificationOnly => Self::CertificationOnly,
            CounterEvidenceStrength::Unavailable => Self::Unavailable,
        }
    }
}

impl From<AccessPolicyCounterStrength> for S6MaterializedCounterStrength {
    fn from(strength: AccessPolicyCounterStrength) -> Self {
        match strength {
            AccessPolicyCounterStrength::Exact => Self::Exact,
            AccessPolicyCounterStrength::Bounded => Self::Bounded,
            AccessPolicyCounterStrength::Sampled => Self::Sampled,
            AccessPolicyCounterStrength::Derived => Self::Derived,
            AccessPolicyCounterStrength::CertificationOnly => Self::CertificationOnly,
            AccessPolicyCounterStrength::Unavailable => Self::Unavailable,
        }
    }
}

impl From<SecureIoCounterStrength> for S6MaterializedCounterStrength {
    fn from(strength: SecureIoCounterStrength) -> Self {
        match strength {
            SecureIoCounterStrength::Exact => Self::Exact,
            SecureIoCounterStrength::Derived => Self::Derived,
        }
    }
}
