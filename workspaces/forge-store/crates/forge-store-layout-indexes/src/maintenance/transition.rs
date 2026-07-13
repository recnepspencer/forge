use super::failure::IndexMaintenanceFailureOutcome;
use super::lag::IndexLagWitness;
use super::mutation_plan::LayoutMutationPlan;

macro_rules! maintenance_plan {
    ($plan:ident) => {
        #[derive(Debug, PartialEq, Eq)]
        pub struct $plan {
            plan: LayoutMutationPlan,
        }

        impl $plan {
            pub(crate) const fn issue(plan: LayoutMutationPlan) -> Self {
                Self { plan }
            }

            pub const fn observation(&self) -> &LayoutMutationPlan {
                &self.plan
            }
        }
    };
}

macro_rules! maintenance_protocol {
    ($protocol:ident) => {
        #[derive(Debug, PartialEq, Eq)]
        pub struct $protocol {
            plan: LayoutMutationPlan,
        }

        impl $protocol {
            pub(crate) const fn issue(plan: LayoutMutationPlan) -> Self {
                Self { plan }
            }

            pub(crate) const fn plan(&self) -> &LayoutMutationPlan {
                &self.plan
            }
        }
    };
}

maintenance_plan!(ExactMaintenancePlan);
maintenance_plan!(LaggedMaintenancePlan);
maintenance_plan!(RebuildMaintenancePlan);
maintenance_plan!(LazyMaintenancePlan);
maintenance_plan!(AdvisoryMaintenancePlan);
maintenance_plan!(VerifierMaintenancePlan);
maintenance_plan!(MigrationMaintenancePlan);

maintenance_protocol!(ExactMaintenanceProtocol);
maintenance_protocol!(LaggedMaintenanceProtocol);
maintenance_protocol!(VerifierMaintenanceProtocol);

#[derive(Debug, PartialEq, Eq)]
enum LayoutMutationAdmissionCase {
    Exact(ExactMaintenancePlan),
    Lagged(LaggedMaintenancePlan, IndexLagWitness),
    Rebuild(RebuildMaintenancePlan, IndexLagWitness),
    Lazy(LazyMaintenancePlan, IndexLagWitness),
    Advisory(AdvisoryMaintenancePlan, IndexLagWitness),
    Verifier(VerifierMaintenancePlan, IndexLagWitness),
    Migration(MigrationMaintenancePlan, IndexLagWitness),
    Denied(IndexMaintenanceFailureOutcome),
}

#[derive(Debug, PartialEq, Eq)]
pub struct LayoutMutationAdmissionOutcome {
    case: LayoutMutationAdmissionCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutMutationAdmissionView<'a> {
    Exact(&'a ExactMaintenancePlan),
    Lagged(&'a LaggedMaintenancePlan, &'a IndexLagWitness),
    Rebuild(&'a RebuildMaintenancePlan, &'a IndexLagWitness),
    Lazy(&'a LazyMaintenancePlan, &'a IndexLagWitness),
    Advisory(&'a AdvisoryMaintenancePlan, &'a IndexLagWitness),
    Verifier(&'a VerifierMaintenancePlan, &'a IndexLagWitness),
    Migration(&'a MigrationMaintenancePlan, &'a IndexLagWitness),
    Denied(&'a IndexMaintenanceFailureOutcome),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MaintenanceAdmissionCaseId(&'static str);

impl MaintenanceAdmissionCaseId {
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

const SUCCESS_CASES: [MaintenanceAdmissionCaseId; 7] = [
    MaintenanceAdmissionCaseId("maintenance.admission.exact"),
    MaintenanceAdmissionCaseId("maintenance.admission.lagged"),
    MaintenanceAdmissionCaseId("maintenance.admission.rebuild"),
    MaintenanceAdmissionCaseId("maintenance.admission.lazy"),
    MaintenanceAdmissionCaseId("maintenance.admission.advisory"),
    MaintenanceAdmissionCaseId("maintenance.admission.verifier"),
    MaintenanceAdmissionCaseId("maintenance.admission.migration"),
];

const DENIAL_CASES: [MaintenanceAdmissionCaseId; 14] = [
    MaintenanceAdmissionCaseId("maintenance.admission.denied.strategy"),
    MaintenanceAdmissionCaseId("maintenance.admission.denied.lane"),
    MaintenanceAdmissionCaseId("maintenance.admission.denied.mutation_shape"),
    MaintenanceAdmissionCaseId("maintenance.admission.denied.publication_protocol"),
    MaintenanceAdmissionCaseId("maintenance.admission.denied.exact_publication_authority"),
    MaintenanceAdmissionCaseId("maintenance.admission.denied.publication_coverage_binding"),
    MaintenanceAdmissionCaseId("maintenance.admission.denied.exact_coverage"),
    MaintenanceAdmissionCaseId("maintenance.admission.denied.coverage_family"),
    MaintenanceAdmissionCaseId("maintenance.admission.denied.lag_witness_missing"),
    MaintenanceAdmissionCaseId("maintenance.admission.denied.lag_witness_unexpected"),
    MaintenanceAdmissionCaseId("maintenance.admission.denied.migration_posture"),
    MaintenanceAdmissionCaseId("maintenance.admission.denied.lower_mutation_capability"),
    MaintenanceAdmissionCaseId("maintenance.admission.denied.lower_publication_capability"),
    MaintenanceAdmissionCaseId("maintenance.admission.denied.lag_coverage_binding"),
];

pub fn maintenance_admission_cases() -> impl Iterator<Item = MaintenanceAdmissionCaseId> {
    SUCCESS_CASES.into_iter().chain(DENIAL_CASES)
}

macro_rules! admission_case {
    ($issue:ident, $take:ident, $variant:ident, $plan:ident) => {
        pub(crate) fn $issue(plan: LayoutMutationPlan, lag: IndexLagWitness) -> Self {
            Self {
                case: LayoutMutationAdmissionCase::$variant($plan::issue(plan), lag),
            }
        }

        pub fn $take(self) -> Result<($plan, IndexLagWitness), Self> {
            match self.case {
                LayoutMutationAdmissionCase::$variant(plan, lag) => Ok((plan, lag)),
                case => Err(Self { case }),
            }
        }
    };
}

impl LayoutMutationAdmissionOutcome {
    pub(crate) fn exact(plan: LayoutMutationPlan) -> Self {
        Self {
            case: LayoutMutationAdmissionCase::Exact(ExactMaintenancePlan::issue(plan)),
        }
    }

    admission_case!(lagged, into_lagged, Lagged, LaggedMaintenancePlan);
    admission_case!(rebuild, into_rebuild, Rebuild, RebuildMaintenancePlan);
    admission_case!(lazy, into_lazy, Lazy, LazyMaintenancePlan);
    admission_case!(advisory, into_advisory, Advisory, AdvisoryMaintenancePlan);
    admission_case!(verifier, into_verifier, Verifier, VerifierMaintenancePlan);
    admission_case!(
        migration,
        into_migration,
        Migration,
        MigrationMaintenancePlan
    );

    pub(crate) fn denied(denial: IndexMaintenanceFailureOutcome) -> Self {
        Self {
            case: LayoutMutationAdmissionCase::Denied(denial),
        }
    }

    pub fn view(&self) -> LayoutMutationAdmissionView<'_> {
        match &self.case {
            LayoutMutationAdmissionCase::Exact(plan) => LayoutMutationAdmissionView::Exact(plan),
            LayoutMutationAdmissionCase::Lagged(plan, lag) => {
                LayoutMutationAdmissionView::Lagged(plan, lag)
            }
            LayoutMutationAdmissionCase::Rebuild(plan, lag) => {
                LayoutMutationAdmissionView::Rebuild(plan, lag)
            }
            LayoutMutationAdmissionCase::Lazy(plan, lag) => {
                LayoutMutationAdmissionView::Lazy(plan, lag)
            }
            LayoutMutationAdmissionCase::Advisory(plan, lag) => {
                LayoutMutationAdmissionView::Advisory(plan, lag)
            }
            LayoutMutationAdmissionCase::Verifier(plan, lag) => {
                LayoutMutationAdmissionView::Verifier(plan, lag)
            }
            LayoutMutationAdmissionCase::Migration(plan, lag) => {
                LayoutMutationAdmissionView::Migration(plan, lag)
            }
            LayoutMutationAdmissionCase::Denied(denial) => {
                LayoutMutationAdmissionView::Denied(denial)
            }
        }
    }

    pub fn case_id(&self) -> MaintenanceAdmissionCaseId {
        match &self.case {
            LayoutMutationAdmissionCase::Exact(_) => SUCCESS_CASES[0],
            LayoutMutationAdmissionCase::Lagged(..) => SUCCESS_CASES[1],
            LayoutMutationAdmissionCase::Rebuild(..) => SUCCESS_CASES[2],
            LayoutMutationAdmissionCase::Lazy(..) => SUCCESS_CASES[3],
            LayoutMutationAdmissionCase::Advisory(..) => SUCCESS_CASES[4],
            LayoutMutationAdmissionCase::Verifier(..) => SUCCESS_CASES[5],
            LayoutMutationAdmissionCase::Migration(..) => SUCCESS_CASES[6],
            LayoutMutationAdmissionCase::Denied(denial) => {
                MaintenanceAdmissionCaseId(denial.case_name())
            }
        }
    }

    pub fn into_exact(self) -> Result<ExactMaintenancePlan, Self> {
        match self.case {
            LayoutMutationAdmissionCase::Exact(plan) => Ok(plan),
            case => Err(Self { case }),
        }
    }
}
