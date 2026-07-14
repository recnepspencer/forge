use worth_store_physical_isolation::{
    PhysicalPublicationDenial, PhysicalPublicationReceipt, PhysicalRootPublicationRuntime,
};

use super::{CopyOnWriteLayoutMutationPlan, IndexMaintenanceMode};
use crate::{AdmittedPhysicalArtifactFamily, PhysicalArtifactFamily};

#[derive(Debug, Clone)]
pub struct CopyOnWriteLayoutMutationReceipt {
    family: AdmittedPhysicalArtifactFamily,
    maintenance_mode: IndexMaintenanceMode,
    publication: PhysicalPublicationReceipt,
}

impl CopyOnWriteLayoutMutationReceipt {
    pub const fn family(&self) -> PhysicalArtifactFamily {
        self.family.declaration().family()
    }

    pub const fn admitted_family(&self) -> AdmittedPhysicalArtifactFamily {
        self.family
    }

    pub const fn maintenance_mode(&self) -> IndexMaintenanceMode {
        self.maintenance_mode
    }

    pub const fn publication(&self) -> &PhysicalPublicationReceipt {
        &self.publication
    }
}

#[derive(Debug, Clone)]
enum CopyOnWriteLayoutMutationExecutionCase {
    Published(Box<CopyOnWriteLayoutMutationReceipt>),
    Denied(PhysicalPublicationDenial),
}

#[derive(Debug, Clone)]
pub struct CopyOnWriteLayoutMutationExecutionOutcome {
    case: CopyOnWriteLayoutMutationExecutionCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CopyOnWriteLayoutMutationExecutionCaseId {
    Published,
    PhysicalPublicationDenied,
}

impl CopyOnWriteLayoutMutationExecutionCaseId {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Published => "layout.mutation.execution.published",
            Self::PhysicalPublicationDenied => {
                "layout.mutation.execution.denied.physical_publication"
            }
        }
    }
}

pub fn copy_on_write_layout_mutation_execution_cases(
) -> impl Iterator<Item = CopyOnWriteLayoutMutationExecutionCaseId> {
    [
        CopyOnWriteLayoutMutationExecutionCaseId::Published,
        CopyOnWriteLayoutMutationExecutionCaseId::PhysicalPublicationDenied,
    ]
    .into_iter()
}

#[derive(Debug, Clone, Copy)]
pub enum CopyOnWriteLayoutMutationExecutionView<'a> {
    Published(&'a CopyOnWriteLayoutMutationReceipt),
    Denied(&'a PhysicalPublicationDenial),
}

impl CopyOnWriteLayoutMutationExecutionOutcome {
    pub const fn view(&self) -> CopyOnWriteLayoutMutationExecutionView<'_> {
        match &self.case {
            CopyOnWriteLayoutMutationExecutionCase::Published(value) => {
                CopyOnWriteLayoutMutationExecutionView::Published(value)
            }
            CopyOnWriteLayoutMutationExecutionCase::Denied(value) => {
                CopyOnWriteLayoutMutationExecutionView::Denied(value)
            }
        }
    }

    pub const fn case_id(&self) -> CopyOnWriteLayoutMutationExecutionCaseId {
        match &self.case {
            CopyOnWriteLayoutMutationExecutionCase::Published(_) => {
                CopyOnWriteLayoutMutationExecutionCaseId::Published
            }
            CopyOnWriteLayoutMutationExecutionCase::Denied(_) => {
                CopyOnWriteLayoutMutationExecutionCaseId::PhysicalPublicationDenied
            }
        }
    }

    pub fn into_published(
        self,
    ) -> Result<CopyOnWriteLayoutMutationReceipt, PhysicalPublicationDenial> {
        match self.case {
            CopyOnWriteLayoutMutationExecutionCase::Published(value) => Ok(*value),
            CopyOnWriteLayoutMutationExecutionCase::Denied(value) => Err(value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CopyOnWriteLayoutMutationExecution;

pub const fn copy_on_write_layout_mutation_execution() -> CopyOnWriteLayoutMutationExecution {
    CopyOnWriteLayoutMutationExecution
}

impl CopyOnWriteLayoutMutationExecution {
    pub fn execute(
        self,
        runtime: &mut PhysicalRootPublicationRuntime,
        plan: CopyOnWriteLayoutMutationPlan,
    ) -> CopyOnWriteLayoutMutationExecutionOutcome {
        match runtime.publish(plan.publication) {
            Ok(published) => CopyOnWriteLayoutMutationExecutionOutcome {
                case: CopyOnWriteLayoutMutationExecutionCase::Published(Box::new(
                    CopyOnWriteLayoutMutationReceipt {
                        family: plan.family,
                        maintenance_mode: plan.maintenance_mode,
                        publication: published.receipt().clone(),
                    },
                )),
            },
            Err(denial) => CopyOnWriteLayoutMutationExecutionOutcome {
                case: CopyOnWriteLayoutMutationExecutionCase::Denied(denial),
            },
        }
    }
}
