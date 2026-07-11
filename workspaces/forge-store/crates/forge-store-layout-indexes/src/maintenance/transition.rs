use super::failure::S8IndexMaintenanceFailureOutcome;
use super::lag::S8IndexLagWitness;
use super::mutation_plan::S8LayoutMutationPlan;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8LoweredMaintenanceProtocol {
    plan: S8LayoutMutationPlan,
}

impl S8LoweredMaintenanceProtocol {
    pub(crate) const fn new(plan: S8LayoutMutationPlan) -> Self {
        Self { plan }
    }

    pub const fn plan(self) -> S8LayoutMutationPlan {
        self.plan
    }
}

type LaggedMutationPlan = (S8LayoutMutationPlan, S8IndexLagWitness);

#[derive(Debug, PartialEq, Eq)]
enum S8LayoutMutationAdmissionCase {
    Ready(S8LayoutMutationPlan),
    Lagged(LaggedMutationPlan),
    Deferred(LaggedMutationPlan),
    Denied(S8IndexMaintenanceFailureOutcome),
}

#[derive(Debug, PartialEq, Eq)]
pub struct S8LayoutMutationAdmissionOutcome {
    case: S8LayoutMutationAdmissionCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8LayoutMutationAdmissionView<'a> {
    Ready(&'a S8LayoutMutationPlan),
    Lagged(&'a LaggedMutationPlan),
    Deferred(&'a LaggedMutationPlan),
    Denied(&'a S8IndexMaintenanceFailureOutcome),
}

impl S8LayoutMutationAdmissionOutcome {
    pub(crate) fn ready(value: S8LayoutMutationPlan) -> Self {
        Self::from_owner_payload(S8LayoutMutationAdmissionCase::Ready(value))
    }

    pub(crate) fn lagged(value: LaggedMutationPlan) -> Self {
        Self::from_owner_payload(S8LayoutMutationAdmissionCase::Lagged(value))
    }

    pub(crate) fn deferred(value: LaggedMutationPlan) -> Self {
        Self::from_owner_payload(S8LayoutMutationAdmissionCase::Deferred(value))
    }

    pub(crate) fn denied(value: S8IndexMaintenanceFailureOutcome) -> Self {
        Self::from_owner_payload(S8LayoutMutationAdmissionCase::Denied(value))
    }

    fn from_owner_payload(case: S8LayoutMutationAdmissionCase) -> Self {
        Self { case }
    }

    pub fn view(&self) -> S8LayoutMutationAdmissionView<'_> {
        match &self.case {
            S8LayoutMutationAdmissionCase::Ready(value) => {
                S8LayoutMutationAdmissionView::Ready(value)
            }
            S8LayoutMutationAdmissionCase::Lagged(value) => {
                S8LayoutMutationAdmissionView::Lagged(value)
            }
            S8LayoutMutationAdmissionCase::Deferred(value) => {
                S8LayoutMutationAdmissionView::Deferred(value)
            }
            S8LayoutMutationAdmissionCase::Denied(value) => {
                S8LayoutMutationAdmissionView::Denied(value)
            }
        }
    }

    fn into_owner_payload(self) -> S8LayoutMutationAdmissionCase {
        self.case
    }
}

impl S8LayoutMutationAdmissionOutcome {
    pub fn into_lagged(self) -> Result<LaggedMutationPlan, Self> {
        match self.into_owner_payload() {
            S8LayoutMutationAdmissionCase::Lagged(value) => Ok(value),
            case => Err(Self::from_owner_payload(case)),
        }
    }

    pub fn into_deferred(self) -> Result<LaggedMutationPlan, Self> {
        match self.into_owner_payload() {
            S8LayoutMutationAdmissionCase::Deferred(value) => Ok(value),
            case => Err(Self::from_owner_payload(case)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum S8IndexMaintenanceTransitionCase {
    ReadyExact(S8LoweredMaintenanceProtocol),
    Lagged(S8LoweredMaintenanceProtocol),
    RebuildOnly(S8LoweredMaintenanceProtocol),
    AdvisoryOnly(S8LoweredMaintenanceProtocol),
    VerifierOnly(S8LoweredMaintenanceProtocol),
    MigrationOnly(S8LoweredMaintenanceProtocol),
    Deferred(S8LoweredMaintenanceProtocol),
}

#[derive(Debug, PartialEq, Eq)]
pub struct S8IndexMaintenanceTransitionOutcome {
    case: S8IndexMaintenanceTransitionCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8IndexMaintenanceTransitionView<'a> {
    ReadyExact(&'a S8LoweredMaintenanceProtocol),
    Lagged(&'a S8LoweredMaintenanceProtocol),
    RebuildOnly(&'a S8LoweredMaintenanceProtocol),
    AdvisoryOnly(&'a S8LoweredMaintenanceProtocol),
    VerifierOnly(&'a S8LoweredMaintenanceProtocol),
    MigrationOnly(&'a S8LoweredMaintenanceProtocol),
    Deferred(&'a S8LoweredMaintenanceProtocol),
}

impl S8IndexMaintenanceTransitionOutcome {
    pub(crate) fn ready_exact(value: S8LoweredMaintenanceProtocol) -> Self {
        Self::from_owner_payload(S8IndexMaintenanceTransitionCase::ReadyExact(value))
    }

    pub(crate) fn lagged(value: S8LoweredMaintenanceProtocol) -> Self {
        Self::from_owner_payload(S8IndexMaintenanceTransitionCase::Lagged(value))
    }

    pub(crate) fn rebuild_only(value: S8LoweredMaintenanceProtocol) -> Self {
        Self::from_owner_payload(S8IndexMaintenanceTransitionCase::RebuildOnly(value))
    }

    pub(crate) fn advisory_only(value: S8LoweredMaintenanceProtocol) -> Self {
        Self::from_owner_payload(S8IndexMaintenanceTransitionCase::AdvisoryOnly(value))
    }

    pub(crate) fn verifier_only(value: S8LoweredMaintenanceProtocol) -> Self {
        Self::from_owner_payload(S8IndexMaintenanceTransitionCase::VerifierOnly(value))
    }

    pub(crate) fn migration_only(value: S8LoweredMaintenanceProtocol) -> Self {
        Self::from_owner_payload(S8IndexMaintenanceTransitionCase::MigrationOnly(value))
    }

    pub(crate) fn deferred(value: S8LoweredMaintenanceProtocol) -> Self {
        Self::from_owner_payload(S8IndexMaintenanceTransitionCase::Deferred(value))
    }

    fn from_owner_payload(case: S8IndexMaintenanceTransitionCase) -> Self {
        Self { case }
    }

    pub fn view(&self) -> S8IndexMaintenanceTransitionView<'_> {
        match &self.case {
            S8IndexMaintenanceTransitionCase::ReadyExact(value) => {
                S8IndexMaintenanceTransitionView::ReadyExact(value)
            }
            S8IndexMaintenanceTransitionCase::Lagged(value) => {
                S8IndexMaintenanceTransitionView::Lagged(value)
            }
            S8IndexMaintenanceTransitionCase::RebuildOnly(value) => {
                S8IndexMaintenanceTransitionView::RebuildOnly(value)
            }
            S8IndexMaintenanceTransitionCase::AdvisoryOnly(value) => {
                S8IndexMaintenanceTransitionView::AdvisoryOnly(value)
            }
            S8IndexMaintenanceTransitionCase::VerifierOnly(value) => {
                S8IndexMaintenanceTransitionView::VerifierOnly(value)
            }
            S8IndexMaintenanceTransitionCase::MigrationOnly(value) => {
                S8IndexMaintenanceTransitionView::MigrationOnly(value)
            }
            S8IndexMaintenanceTransitionCase::Deferred(value) => {
                S8IndexMaintenanceTransitionView::Deferred(value)
            }
        }
    }

    fn into_owner_payload(self) -> S8IndexMaintenanceTransitionCase {
        self.case
    }
}

impl S8IndexMaintenanceTransitionOutcome {
    pub fn into_lagged(self) -> Result<S8LoweredMaintenanceProtocol, Self> {
        match self.into_owner_payload() {
            S8IndexMaintenanceTransitionCase::Lagged(value) => Ok(value),
            case => Err(Self::from_owner_payload(case)),
        }
    }

    pub fn into_verifier_only(self) -> Result<S8LoweredMaintenanceProtocol, Self> {
        match self.into_owner_payload() {
            S8IndexMaintenanceTransitionCase::VerifierOnly(value) => Ok(value),
            case => Err(Self::from_owner_payload(case)),
        }
    }
}
