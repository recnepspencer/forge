use std::any::TypeId;
use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use std::sync::Arc;

use crate::domain_installation::execution_index::WorthQueryWorkflowExecutionDescriptor;
use crate::runtime::WorthQueryWorkspace;

use super::{
    WorthQueryDomainWorkflowStageExecutor, WorthQueryExecutableDomainOperation,
    WorthQueryWorkflowOperation, WorthQueryWorkflowStageExecutionContext,
    WorthQueryWorkflowStageExecutorFailure, WorthQueryWorkflowStageMaterial,
    WorthQueryWorkflowStageWorkspace, WorthQueryWorkflowValue,
};

type WorkflowExecutorMarker<D, O, F> = fn() -> (D, O, F);

trait ErasedWorkflowStageExecutor: Send + Sync {
    fn execute(
        &self,
        input: WorthQueryWorkflowValue,
        context: &WorthQueryWorkflowStageExecutionContext<'_>,
        workspace: &mut WorthQueryWorkspace,
    ) -> Result<WorthQueryWorkflowStageMaterial, WorthQueryWorkflowStageExecutorFailure>;
}

struct TypedWorkflowStageExecutor<D, O, F, E> {
    executor: E,
    marker: PhantomData<WorkflowExecutorMarker<D, O, F>>,
}

impl<D, O, F, E: WorthQueryDomainWorkflowStageExecutor<D, O, F>> ErasedWorkflowStageExecutor
    for TypedWorkflowStageExecutor<D, O, F, E>
where
    O: WorthQueryExecutableDomainOperation<D, F, Execution = WorthQueryWorkflowOperation>,
{
    fn execute(
        &self,
        input: WorthQueryWorkflowValue,
        context: &WorthQueryWorkflowStageExecutionContext<'_>,
        workspace: &mut WorthQueryWorkspace,
    ) -> Result<WorthQueryWorkflowStageMaterial, WorthQueryWorkflowStageExecutorFailure> {
        let mut workspace = WorthQueryWorkflowStageWorkspace::new(workspace);
        let mut material = match self.executor.execute_stage(input, context, &mut workspace) {
            Ok(material) => material,
            Err(failure) => {
                return Err(failure.with_executed_effects(workspace.into_executed_effects()));
            }
        };
        if workspace.installed_read_executions() != usize::from(context.requires_primary_read()) {
            return Err(WorthQueryWorkflowStageExecutorFailure::new(
                crate::domain_installation::WorthQueryOperationFailureClass::Indeterminate,
                "workflow stage did not use its installed primary read exactly once",
            )
            .with_executed_effects(workspace.into_executed_effects()));
        }
        material.retain_query_executed_effects(workspace.into_executed_effects());
        Ok(material)
    }
}

pub(crate) struct WorthQueryInstalledWorkflowStageExecutor {
    executor: Arc<dyn ErasedWorkflowStageExecutor>,
    pub(crate) installed_read: Option<crate::ordinary::read::WorthQueryReadDeclaration>,
}

struct WorkflowStageExecutorRegistration {
    executor: Arc<dyn ErasedWorkflowStageExecutor>,
    lowering_family: &'static str,
    deterministic: bool,
    execution_cost: crate::domain_installation::WorthQueryOperationCostClass,
    result_width_cost: crate::domain_installation::WorthQueryOperationCostClass,
    installed_read: Option<crate::ordinary::read::WorthQueryReadDeclaration>,
}

impl WorthQueryInstalledWorkflowStageExecutor {
    pub(crate) fn execute(
        &self,
        input: WorthQueryWorkflowValue,
        context: &WorthQueryWorkflowStageExecutionContext<'_>,
        workspace: &mut WorthQueryWorkspace,
    ) -> Result<WorthQueryWorkflowStageMaterial, WorthQueryWorkflowStageExecutorFailure> {
        self.executor.execute(input, context, workspace)
    }
}

#[derive(Default)]
pub(crate) struct WorthQueryPendingWorkflowStageExecutors {
    registrations: HashMap<(TypeId, TypeId, TypeId), WorkflowStageExecutorRegistration>,
    duplicate: bool,
}

impl WorthQueryPendingWorkflowStageExecutors {
    pub(crate) fn register<
        D: 'static,
        O,
        F: 'static,
        E: WorthQueryDomainWorkflowStageExecutor<D, O, F>,
    >(
        mut self,
        executor: E,
    ) -> Self
    where
        O: 'static
            + WorthQueryExecutableDomainOperation<D, F, Execution = WorthQueryWorkflowOperation>,
    {
        let key = (TypeId::of::<D>(), TypeId::of::<O>(), TypeId::of::<F>());
        self.duplicate |= self
            .registrations
            .insert(
                key,
                WorkflowStageExecutorRegistration {
                    installed_read: executor
                        .installed_read_declaration()
                        .map(|declaration| declaration.clone_for_installed_execution()),
                    executor: Arc::new(TypedWorkflowStageExecutor::<D, O, F, E> {
                        executor,
                        marker: PhantomData,
                    }),
                    lowering_family: E::LOWERING_FAMILY,
                    deterministic: E::DETERMINISTIC,
                    execution_cost: E::EXECUTION_COST,
                    result_width_cost: E::RESULT_WIDTH_COST,
                },
            )
            .is_some();
        self
    }

    pub(crate) fn install(
        self,
        workflow_operations: &[WorthQueryWorkflowExecutionDescriptor],
    ) -> Result<WorthQueryWorkflowStageExecutorRegistry, &'static str> {
        if self.duplicate {
            return Err("duplicate exact workflow stage executor registration");
        }
        let expected = workflow_operations
            .iter()
            .map(|descriptor| (descriptor.domain, descriptor.operation, descriptor.family))
            .collect::<HashSet<_>>();
        let actual = self.registrations.keys().copied().collect::<HashSet<_>>();
        if expected != actual {
            return Err("installed workflow operation and stage executor registration sets differ");
        }
        for descriptor in workflow_operations {
            if descriptor.has_unsupported_effect_family {
                return Err(
                    "workflow declares an effect family without a Query-owned stage execution door",
                );
            }
            let registration = self
                .registrations
                .get(&(descriptor.domain, descriptor.operation, descriptor.family))
                .expect("workflow executor registration set was closed");
            if registration.lowering_family != descriptor.lowering_family {
                return Err(
                    "workflow stage executor lowering family disagrees with installed semantics",
                );
            }
            if registration.deterministic != descriptor.deterministic_lowering {
                return Err("workflow executor determinism disagrees with installed semantics");
            }
            if descriptor.lookup_cost
                != crate::domain_installation::WorthQueryOperationCostClass::Constant
            {
                return Err("installed workflow lookup cost is not constant-time indexed lookup");
            }
            if registration.execution_cost != descriptor.execution_cost
                || registration.result_width_cost != descriptor.result_width_cost
            {
                return Err("workflow executor cost contract disagrees with installed semantics");
            }
            match (&registration.installed_read, descriptor.requires_installed_read) {
                (Some(declaration), true)
                    if declaration.identity().canonical_query_digest() == descriptor.query_digest
                        && declaration.identity().canonical_result_shape_digest()
                            == descriptor.result_shape_digest => {}
                (Some(_), true) => return Err(
                    "workflow executor read declaration disagrees with installed canonical semantics",
                ),
                (None, true) => {
                    return Err("workflow executor is missing its installed read declaration")
                }
                (Some(_), false) => {
                    return Err("workflow executor registered an undeclared installed read plan")
                }
                (None, false) => {}
            }
        }
        Ok(WorthQueryWorkflowStageExecutorRegistry {
            registrations: self
                .registrations
                .into_iter()
                .map(|(key, registration)| {
                    (
                        key,
                        Arc::new(WorthQueryInstalledWorkflowStageExecutor {
                            executor: registration.executor,
                            installed_read: registration.installed_read,
                        }),
                    )
                })
                .collect(),
        })
    }
}

pub(crate) struct WorthQueryWorkflowStageExecutorRegistry {
    registrations: HashMap<(TypeId, TypeId, TypeId), Arc<WorthQueryInstalledWorkflowStageExecutor>>,
}

impl WorthQueryWorkflowStageExecutorRegistry {
    pub(crate) fn get<D: 'static, O: 'static, F: 'static>(
        &self,
    ) -> Option<Arc<WorthQueryInstalledWorkflowStageExecutor>> {
        self.registrations
            .get(&(TypeId::of::<D>(), TypeId::of::<O>(), TypeId::of::<F>()))
            .map(Arc::clone)
    }
}
