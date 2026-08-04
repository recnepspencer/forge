use crate::{
    AdmittedPhysicalArtifactFamily, BaselineLsmRunPublicationAdmission, PhysicalArtifactFamily,
};

use super::{
    IndexMaintenanceFailureOutcome, IndexMaintenanceMode, IndexPublicationProtocol,
    PhysicalMutationShape,
};

#[derive(Debug, Clone)]
enum LayoutMutationPlanKind {
    LsmAppend(BaselineLsmRunPublicationAdmission),
}

#[derive(Debug, Clone)]
pub struct LayoutMutationPlan {
    family: AdmittedPhysicalArtifactFamily,
    maintenance_mode: IndexMaintenanceMode,
    mutation_shape: PhysicalMutationShape,
    kind: LayoutMutationPlanKind,
}

impl LayoutMutationPlan {
    pub const fn family(&self) -> PhysicalArtifactFamily {
        self.family.declaration().family()
    }
    pub const fn admitted_family(&self) -> AdmittedPhysicalArtifactFamily {
        self.family
    }
    pub const fn maintenance_mode(&self) -> IndexMaintenanceMode {
        self.maintenance_mode
    }
    pub const fn mutation_shape(&self) -> PhysicalMutationShape {
        self.mutation_shape
    }
    pub const fn publication_protocol(&self) -> IndexPublicationProtocol {
        match self.kind {
            LayoutMutationPlanKind::LsmAppend(_) => {
                IndexPublicationProtocol::LsmManifestReplacement
            }
        }
    }

    pub fn into_lsm_append(self) -> BaselineLsmRunPublicationAdmission {
        match self.kind {
            LayoutMutationPlanKind::LsmAppend(admission) => admission,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutMutationAdmission;

pub const fn layout_mutation_admission() -> LayoutMutationAdmission {
    LayoutMutationAdmission
}

impl LayoutMutationAdmission {
    pub fn admit_lsm_append(
        self,
        admission: BaselineLsmRunPublicationAdmission,
    ) -> LayoutMutationAdmissionOutcome {
        let selected = admission.selected();
        let strategy = selected.strategy_admission();
        let mode = strategy.request().maintenance_mode();
        let shape = strategy.request().mutation_shape();
        LayoutMutationAdmissionOutcome::planned(LayoutMutationPlan {
            family: selected.admitted_family(),
            maintenance_mode: mode,
            mutation_shape: shape,
            kind: LayoutMutationPlanKind::LsmAppend(admission),
        })
    }

    pub const fn deny_in_place_reachable_overwrite(self) -> LayoutMutationAdmissionOutcome {
        LayoutMutationAdmissionOutcome {
            case: LayoutMutationAdmissionCase::Denied(
                IndexMaintenanceFailureOutcome::InPlaceReachableOverwriteUnsupported,
            ),
        }
    }
}

#[derive(Debug, Clone)]
enum LayoutMutationAdmissionCase {
    Planned(Box<LayoutMutationPlan>),
    Denied(IndexMaintenanceFailureOutcome),
}

#[derive(Debug, Clone)]
pub struct LayoutMutationAdmissionOutcome {
    case: LayoutMutationAdmissionCase,
}

#[derive(Debug, Clone, Copy)]
pub enum LayoutMutationAdmissionView<'a> {
    Planned(&'a LayoutMutationPlan),
    Denied(&'a IndexMaintenanceFailureOutcome),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayoutMutationAdmissionCaseId(&'static str);

impl LayoutMutationAdmissionCaseId {
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

pub fn layout_mutation_admission_cases() -> impl Iterator<Item = LayoutMutationAdmissionCaseId> {
    [
        LayoutMutationAdmissionCaseId("layout.maintenance.mutation.planned.lsm_append"),
        LayoutMutationAdmissionCaseId("layout.maintenance.mutation.denied.in_place"),
    ]
    .into_iter()
}

impl LayoutMutationAdmissionOutcome {
    fn planned(plan: LayoutMutationPlan) -> Self {
        Self {
            case: LayoutMutationAdmissionCase::Planned(Box::new(plan)),
        }
    }

    pub const fn view(&self) -> LayoutMutationAdmissionView<'_> {
        match &self.case {
            LayoutMutationAdmissionCase::Planned(plan) => {
                LayoutMutationAdmissionView::Planned(plan)
            }
            LayoutMutationAdmissionCase::Denied(denial) => {
                LayoutMutationAdmissionView::Denied(denial)
            }
        }
    }

    pub const fn case_id(&self) -> LayoutMutationAdmissionCaseId {
        match &self.case {
            LayoutMutationAdmissionCase::Planned(plan) => match &plan.kind {
                LayoutMutationPlanKind::LsmAppend(_) => {
                    LayoutMutationAdmissionCaseId("layout.maintenance.mutation.planned.lsm_append")
                }
            },
            LayoutMutationAdmissionCase::Denied(_) => {
                LayoutMutationAdmissionCaseId("layout.maintenance.mutation.denied.in_place")
            }
        }
    }

    pub fn into_planned(self) -> Result<LayoutMutationPlan, Self> {
        match self.case {
            LayoutMutationAdmissionCase::Planned(plan) => Ok(*plan),
            case => Err(Self { case }),
        }
    }
}
