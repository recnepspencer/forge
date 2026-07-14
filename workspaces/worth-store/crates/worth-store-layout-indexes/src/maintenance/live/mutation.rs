use crate::{
    AdmittedLayoutMaterialization, AdmittedPhysicalArtifactFamily,
    BaselineLsmRunPublicationAdmission, LayoutMaterializationSourceKind,
    LayoutStrategyRegistrySnapshot, PhysicalArtifactFamily,
};
use worth_store_physical_isolation::CopyOnWritePublicationPlan;

use super::{
    IndexMaintenanceFailureOutcome, IndexMaintenanceMode, IndexPublicationProtocol,
    PhysicalMutationShape,
};

#[derive(Debug, Clone)]
enum LayoutMutationPlanKind {
    LsmAppend(BaselineLsmRunPublicationAdmission),
    CopyOnWrite(Box<CopyOnWritePublicationPlan>),
}

#[derive(Debug, Clone)]
pub struct CopyOnWriteLayoutMutationPlan {
    pub(super) family: AdmittedPhysicalArtifactFamily,
    pub(super) maintenance_mode: IndexMaintenanceMode,
    pub(super) publication: CopyOnWritePublicationPlan,
}

#[derive(Debug, Clone)]
pub struct LayoutMutationPlan {
    family: AdmittedPhysicalArtifactFamily,
    maintenance_mode: IndexMaintenanceMode,
    mutation_shape: PhysicalMutationShape,
    kind: LayoutMutationPlanKind,
}

#[derive(Debug)]
pub struct CopyOnWriteLayoutMutationRequest<'a> {
    strategy: LayoutStrategyRegistrySnapshot,
    plan: CopyOnWritePublicationPlan,
    materialization: &'a AdmittedLayoutMaterialization,
    current_security: &'a worth_store_security::StoreCurrentSecurityScopeWitnessSet,
}

impl<'a> CopyOnWriteLayoutMutationRequest<'a> {
    pub const fn new(
        strategy: LayoutStrategyRegistrySnapshot,
        plan: CopyOnWritePublicationPlan,
        materialization: &'a AdmittedLayoutMaterialization,
        current_security: &'a worth_store_security::StoreCurrentSecurityScopeWitnessSet,
    ) -> Self {
        Self {
            strategy,
            plan,
            materialization,
            current_security,
        }
    }
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
            LayoutMutationPlanKind::CopyOnWrite(_) => IndexPublicationProtocol::CopyOnWriteRootSwap,
        }
    }

    pub fn into_lsm_append(self) -> Result<BaselineLsmRunPublicationAdmission, Self> {
        match self.kind {
            LayoutMutationPlanKind::LsmAppend(admission) => Ok(admission),
            kind => Err(Self { kind, ..self }),
        }
    }

    pub fn into_copy_on_write(self) -> Result<CopyOnWriteLayoutMutationPlan, Self> {
        match self.kind {
            LayoutMutationPlanKind::CopyOnWrite(publication) => Ok(CopyOnWriteLayoutMutationPlan {
                family: self.family,
                maintenance_mode: self.maintenance_mode,
                publication: *publication,
            }),
            kind => Err(Self { kind, ..self }),
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
        if shape != PhysicalMutationShape::LogStructuredAppend {
            return LayoutMutationAdmissionOutcome::denied(
                IndexMaintenanceFailureOutcome::MutationShapeMismatch,
            );
        }
        LayoutMutationAdmissionOutcome::planned(LayoutMutationPlan {
            family: selected.admitted_family(),
            maintenance_mode: mode,
            mutation_shape: shape,
            kind: LayoutMutationPlanKind::LsmAppend(admission),
        })
    }

    pub fn admit_copy_on_write(
        self,
        request: CopyOnWriteLayoutMutationRequest<'_>,
    ) -> LayoutMutationAdmissionOutcome {
        let shape = request.strategy.request().mutation_shape();
        if shape != PhysicalMutationShape::PointRewrite {
            return LayoutMutationAdmissionOutcome::denied(
                IndexMaintenanceFailureOutcome::MutationShapeMismatch,
            );
        }
        let family = request.strategy.admitted_strategy().admitted_family();
        let expected_source = LayoutMaterializationSourceKind::BTreeRoot(
            request.plan.binding().old_root_validation().reference(),
        );
        if request.materialization.family() != family
            || request.materialization.coverage().require_exact().is_err()
            || request.materialization.source().kind() != expected_source
        {
            return LayoutMutationAdmissionOutcome::denied(
                IndexMaintenanceFailureOutcome::MutationSourceMaterializationMismatch,
            );
        }
        if family.security_identity() != request.current_security.key_scope().identity()
            || family.authority_identity() != request.current_security.authority_identity()
        {
            return LayoutMutationAdmissionOutcome::denied(
                IndexMaintenanceFailureOutcome::SecurityScopeMismatch,
            );
        }
        if family.authority_identity() != request.plan.binding().store_authority_identity() {
            return LayoutMutationAdmissionOutcome::denied(
                IndexMaintenanceFailureOutcome::PhysicalPublicationAuthorityMismatch,
            );
        }
        LayoutMutationAdmissionOutcome::planned(LayoutMutationPlan {
            family,
            maintenance_mode: request.strategy.request().maintenance_mode(),
            mutation_shape: shape,
            kind: LayoutMutationPlanKind::CopyOnWrite(Box::new(request.plan)),
        })
    }

    pub const fn deny_in_place_reachable_overwrite(self) -> LayoutMutationAdmissionOutcome {
        LayoutMutationAdmissionOutcome::denied(
            IndexMaintenanceFailureOutcome::InPlaceReachableOverwriteUnsupported,
        )
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
        LayoutMutationAdmissionCaseId("layout.maintenance.mutation.planned.copy_on_write"),
        LayoutMutationAdmissionCaseId("layout.maintenance.mutation.denied.shape"),
        LayoutMutationAdmissionCaseId("layout.maintenance.mutation.denied.security_scope"),
        LayoutMutationAdmissionCaseId("layout.maintenance.mutation.denied.authority"),
        LayoutMutationAdmissionCaseId("layout.maintenance.mutation.denied.materialization"),
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

    const fn denied(denial: IndexMaintenanceFailureOutcome) -> Self {
        Self {
            case: LayoutMutationAdmissionCase::Denied(denial),
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
                LayoutMutationPlanKind::CopyOnWrite(_) => LayoutMutationAdmissionCaseId(
                    "layout.maintenance.mutation.planned.copy_on_write",
                ),
            },
            LayoutMutationAdmissionCase::Denied(
                IndexMaintenanceFailureOutcome::InPlaceReachableOverwriteUnsupported,
            ) => LayoutMutationAdmissionCaseId("layout.maintenance.mutation.denied.in_place"),
            LayoutMutationAdmissionCase::Denied(
                IndexMaintenanceFailureOutcome::SecurityScopeMismatch,
            ) => LayoutMutationAdmissionCaseId("layout.maintenance.mutation.denied.security_scope"),
            LayoutMutationAdmissionCase::Denied(
                IndexMaintenanceFailureOutcome::PhysicalPublicationAuthorityMismatch,
            ) => LayoutMutationAdmissionCaseId("layout.maintenance.mutation.denied.authority"),
            LayoutMutationAdmissionCase::Denied(
                IndexMaintenanceFailureOutcome::MutationSourceMaterializationMismatch,
            ) => {
                LayoutMutationAdmissionCaseId("layout.maintenance.mutation.denied.materialization")
            }
            LayoutMutationAdmissionCase::Denied(_) => {
                LayoutMutationAdmissionCaseId("layout.maintenance.mutation.denied.shape")
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
