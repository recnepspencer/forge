use crate::keyspace::AdmittedPhysicalAccessIdentity;
use crate::planning::{AccessPlanIdentity, SelectedLsmLookup};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaselineLsmLookupAdmission {
    selected: SelectedLsmLookup,
    current_materialization: crate::CurrentLayoutMaterialization,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BaselineLsmLookupAdmissionCase {
    Admitted(BaselineLsmLookupAdmission),
    Stale(crate::StaleLayoutMaterialization),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaselineLsmLookupAdmissionOutcome {
    case: BaselineLsmLookupAdmissionCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaselineLsmLookupAdmissionView<'a> {
    Admitted(&'a BaselineLsmLookupAdmission),
    Stale(&'a crate::StaleLayoutMaterialization),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BaselineLsmLookupAdmissionCaseId(&'static str);

impl BaselineLsmLookupAdmissionCaseId {
    pub const fn name(self) -> &'static str {
        self.0
    }
}

pub fn baseline_lsm_lookup_admission_cases(
) -> impl Iterator<Item = BaselineLsmLookupAdmissionCaseId> {
    [
        BaselineLsmLookupAdmissionCaseId("layout.lsm_lookup.readiness.admitted"),
        BaselineLsmLookupAdmissionCaseId("layout.lsm_lookup.readiness.stale"),
    ]
    .into_iter()
}

impl BaselineLsmLookupAdmissionOutcome {
    fn admitted(admission: BaselineLsmLookupAdmission) -> Self {
        Self {
            case: BaselineLsmLookupAdmissionCase::Admitted(admission),
        }
    }

    fn stale(stale: crate::StaleLayoutMaterialization) -> Self {
        Self {
            case: BaselineLsmLookupAdmissionCase::Stale(stale),
        }
    }

    pub const fn view(&self) -> BaselineLsmLookupAdmissionView<'_> {
        match &self.case {
            BaselineLsmLookupAdmissionCase::Admitted(value) => {
                BaselineLsmLookupAdmissionView::Admitted(value)
            }
            BaselineLsmLookupAdmissionCase::Stale(value) => {
                BaselineLsmLookupAdmissionView::Stale(value)
            }
        }
    }

    pub const fn case_id(&self) -> BaselineLsmLookupAdmissionCaseId {
        match self.case {
            BaselineLsmLookupAdmissionCase::Admitted(_) => {
                BaselineLsmLookupAdmissionCaseId("layout.lsm_lookup.readiness.admitted")
            }
            BaselineLsmLookupAdmissionCase::Stale(_) => {
                BaselineLsmLookupAdmissionCaseId("layout.lsm_lookup.readiness.stale")
            }
        }
    }

    pub fn into_admitted(self) -> Result<BaselineLsmLookupAdmission, Self> {
        match self.case {
            BaselineLsmLookupAdmissionCase::Admitted(value) => Ok(value),
            case => Err(Self { case }),
        }
    }

    pub fn into_stale(self) -> Result<crate::StaleLayoutMaterialization, Self> {
        match self.case {
            BaselineLsmLookupAdmissionCase::Stale(value) => Ok(value),
            case => Err(Self { case }),
        }
    }
}

impl BaselineLsmLookupAdmission {
    pub fn admit(
        selected: SelectedLsmLookup,
        frontier: crate::CurrentMaterializationFrontier,
    ) -> BaselineLsmLookupAdmissionOutcome {
        let materialization = selected.materialization().clone();
        match materialization.classify_freshness_at(frontier) {
            Ok(crate::MaterializationFreshness::Current(current_materialization)) => {
                BaselineLsmLookupAdmissionOutcome::admitted(Self {
                    selected,
                    current_materialization,
                })
            }
            Ok(crate::MaterializationFreshness::Stale(stale)) => {
                BaselineLsmLookupAdmissionOutcome::stale(stale)
            }
            Err(denial) => unreachable!(
                "selected LSM lookup retains exact admitted materialization: {denial:?}"
            ),
        }
    }

    pub(crate) fn plan_binding(&self) -> &AccessPlanIdentity {
        self.selected.fingerprint()
    }
    pub(crate) fn request_identity(&self) -> AdmittedPhysicalAccessIdentity {
        self.selected.request_identity()
    }
    #[cfg(test)]
    pub(crate) fn selected(&self) -> &SelectedLsmLookup {
        &self.selected
    }
    pub const fn current_materialization(&self) -> &crate::CurrentLayoutMaterialization {
        &self.current_materialization
    }
}
