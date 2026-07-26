use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use std::sync::Arc;

use super::{
    WorthQueryDirectOperation, WorthQueryDomainOperationExecutor,
    WorthQueryExecutableDomainOperation, WorthQueryOperationExecutionContext,
    WorthQueryOperationExecutorFailure, WorthQueryOperationPublicationMode,
    WorthQueryOperationWorkspace,
};
use crate::domain_installation::execution_index::WorthQueryDomainOperationExecutionDescriptor;
use crate::runtime::WorthQueryWorkspace;

type OperationExecutorMarker<D, O, F> = fn() -> (D, O, F);

pub(crate) trait ErasedDomainOperationExecutor: Send + Sync {
    fn execute(
        &self,
        input: Box<dyn Any>,
        context: &WorthQueryOperationExecutionContext<'_>,
        workspace: &mut WorthQueryWorkspace,
    ) -> Result<ErasedOperationExecution, WorthQueryOperationExecutorFailure>;
}

pub(crate) struct ErasedOperationExecution {
    material: Box<dyn Any>,
    installed_read_executions: usize,
}

struct TypedDomainOperationExecutor<D, O, F, E> {
    executor: E,
    _marker: PhantomData<OperationExecutorMarker<D, O, F>>,
}

impl<D: 'static, O, F: 'static, E> ErasedDomainOperationExecutor
    for TypedDomainOperationExecutor<D, O, F, E>
where
    O: WorthQueryExecutableDomainOperation<D, F, Execution = WorthQueryDirectOperation>,
    E: WorthQueryDomainOperationExecutor<D, O, F>,
{
    fn execute(
        &self,
        input: Box<dyn Any>,
        context: &WorthQueryOperationExecutionContext<'_>,
        workspace: &mut WorthQueryWorkspace,
    ) -> Result<ErasedOperationExecution, WorthQueryOperationExecutorFailure> {
        let input = input.downcast::<O::Input>().map_err(|_| {
            WorthQueryOperationExecutorFailure::new(
                crate::domain_installation::WorthQueryOperationFailureClass::Indeterminate,
                "registered operation input type drifted",
            )
        })?;
        let mut workspace = WorthQueryOperationWorkspace::new(workspace);
        let material = self.executor.execute(*input, context, &mut workspace)?;
        let installed_read_executions = workspace.installed_read_executions();
        if installed_read_executions != usize::from(context.has_installed_read()) {
            return Err(WorthQueryOperationExecutorFailure::new(
                crate::domain_installation::WorthQueryOperationFailureClass::Indeterminate,
                "executor did not use its installed canonical read exactly once",
            ));
        }
        Ok(ErasedOperationExecution {
            material: Box::new(material),
            installed_read_executions,
        })
    }
}

pub(crate) struct WorthQueryInstalledDomainOperationExecutor {
    pub(crate) executor: Arc<dyn ErasedDomainOperationExecutor>,
    pub(crate) installed_read: Option<crate::ordinary::read::WorthQueryReadDeclaration>,
    pub(crate) resource_support: super::WorthQueryExecutionResourceSupport,
}

impl WorthQueryInstalledDomainOperationExecutor {
    pub(crate) fn execute<D: 'static, O, F: 'static>(
        &self,
        input: O::Input,
        context: &WorthQueryOperationExecutionContext<'_>,
        workspace: &mut WorthQueryWorkspace,
    ) -> Result<
        (
            super::WorthQueryOperationExecutionMaterial<O::Output>,
            usize,
        ),
        WorthQueryOperationExecutorFailure,
    >
    where
        O: WorthQueryExecutableDomainOperation<D, F, Execution = WorthQueryDirectOperation>,
    {
        let execution = self.executor.execute(Box::new(input), context, workspace)?;
        execution
            .material
            .downcast::<super::WorthQueryOperationExecutionMaterial<O::Output>>()
            .map(|material| (*material, execution.installed_read_executions))
            .map_err(|_| {
                WorthQueryOperationExecutorFailure::new(
                    crate::domain_installation::WorthQueryOperationFailureClass::Indeterminate,
                    "registered operation output type drifted",
                )
            })
    }
}

struct PendingExecutor {
    executor: Arc<dyn ErasedDomainOperationExecutor>,
    publishes: bool,
    lowering_family: &'static str,
    deterministic: bool,
    execution_cost: crate::domain_installation::WorthQueryOperationCostClass,
    result_width_cost: crate::domain_installation::WorthQueryOperationCostClass,
    installed_read: Option<crate::ordinary::read::WorthQueryReadDeclaration>,
    resource_support: super::WorthQueryExecutionResourceSupport,
}

#[derive(Default)]
pub(crate) struct WorthQueryPendingDomainOperationExecutors {
    registrations: HashMap<(TypeId, TypeId, TypeId), PendingExecutor>,
    duplicate: bool,
    invalid_publication_output: bool,
}

impl WorthQueryPendingDomainOperationExecutors {
    pub(crate) fn register<D: 'static, O, F: 'static, E>(mut self, executor: E) -> Self
    where
        O: WorthQueryExecutableDomainOperation<D, F, Execution = WorthQueryDirectOperation>,
        E: WorthQueryDomainOperationExecutor<D, O, F>,
    {
        let key = (TypeId::of::<D>(), TypeId::of::<O>(), TypeId::of::<F>());
        let resource_support = executor.execution_resource_support();
        let registration = PendingExecutor {
            installed_read: executor
                .installed_read_declaration()
                .map(|declaration| declaration.clone_for_installed_execution()),
            executor: Arc::new(TypedDomainOperationExecutor::<D, O, F, E> {
                executor,
                _marker: PhantomData,
            }),
            publishes: <O::Publication as WorthQueryOperationPublicationMode>::PUBLISHES,
            lowering_family: E::LOWERING_FAMILY,
            deterministic: E::DETERMINISTIC,
            execution_cost: E::EXECUTION_COST,
            result_width_cost: E::RESULT_WIDTH_COST,
            resource_support,
        };
        self.invalid_publication_output |= registration.publishes
            && TypeId::of::<O::Output>()
                != TypeId::of::<crate::ordinary::read::WorthQueryReadCompletion>();
        self.duplicate |= self.registrations.insert(key, registration).is_some();
        self
    }

    pub(crate) fn install(
        self,
        installed: &[WorthQueryDomainOperationExecutionDescriptor],
    ) -> Result<WorthQueryDomainOperationExecutorRegistry, &'static str> {
        if self.duplicate {
            return Err("duplicate exact domain operation executor registration");
        }
        if self.invalid_publication_output {
            return Err(
                "publishing operation executor output is not Query read publication material",
            );
        }
        let installed_keys = installed
            .iter()
            .filter(|descriptor| !descriptor.has_workflow)
            .map(|descriptor| (descriptor.domain, descriptor.operation, descriptor.family))
            .collect::<HashSet<_>>();
        if installed_keys.len() != self.registrations.len() {
            return Err("installed operation and executor registration sets differ");
        }
        for descriptor in installed
            .iter()
            .filter(|descriptor| !descriptor.has_workflow)
        {
            if descriptor.has_unsupported_effect_family || descriptor.requires_primary_mutation {
                return Err(
                    "direct operation declares effects without a Query-owned execution door",
                );
            }
            let registration = self
                .registrations
                .get(&(descriptor.domain, descriptor.operation, descriptor.family))
                .ok_or("installed domain operation is missing its exact executor")?;
            if registration.publishes != descriptor.publishes {
                return Err("executor publication marker disagrees with installed semantics");
            }
            if registration.lowering_family != descriptor.lowering_family {
                return Err("executor lowering family disagrees with installed semantics");
            }
            if registration.deterministic != descriptor.deterministic_lowering {
                return Err("executor determinism disagrees with installed semantics");
            }
            if descriptor.lookup_cost
                != crate::domain_installation::WorthQueryOperationCostClass::Constant
            {
                return Err("installed operation lookup cost is not constant-time indexed lookup");
            }
            if registration.execution_cost != descriptor.execution_cost
                || registration.result_width_cost != descriptor.result_width_cost
            {
                return Err("executor cost contract disagrees with installed semantics");
            }
            match (
                &registration.installed_read,
                descriptor.requires_installed_read,
            ) {
                (Some(declaration), true)
                    if declaration.identity().canonical_query_digest()
                        == descriptor.query_digest
                        && declaration.identity().canonical_result_shape_digest()
                            == descriptor.result_shape_digest => {}
                (Some(_), true) => {
                    return Err(
                        "executor read declaration disagrees with installed canonical semantics",
                    )
                }
                (None, true) => {
                    return Err("operation executor is missing its installed read declaration")
                }
                (Some(_), false) => {
                    return Err("executor registered an undeclared installed read plan")
                }
                (None, false) => {}
            }
        }
        Ok(WorthQueryDomainOperationExecutorRegistry {
            registrations: self
                .registrations
                .into_iter()
                .map(|(key, value)| {
                    (
                        key,
                        Arc::new(WorthQueryInstalledDomainOperationExecutor {
                            executor: value.executor,
                            installed_read: value.installed_read,
                            resource_support: value.resource_support,
                        }),
                    )
                })
                .collect(),
        })
    }
}

pub(crate) struct WorthQueryDomainOperationExecutorRegistry {
    registrations:
        HashMap<(TypeId, TypeId, TypeId), Arc<WorthQueryInstalledDomainOperationExecutor>>,
}

impl WorthQueryDomainOperationExecutorRegistry {
    pub(crate) fn get<D: 'static, O: 'static, F: 'static>(
        &self,
    ) -> Option<Arc<WorthQueryInstalledDomainOperationExecutor>> {
        self.registrations
            .get(&(TypeId::of::<D>(), TypeId::of::<O>(), TypeId::of::<F>()))
            .map(Arc::clone)
    }
}
