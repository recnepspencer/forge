use super::*;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub(super) struct SemanticExecutionKey {
    attempt: worth_ui_host_contract::UiMountedPresentationAttemptIdentity,
    binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    change: usize,
}

pub(super) struct PendingSemanticExecution {
    partitions: Box<[PresentationSemanticPartition]>,
    next_partition: usize,
    current: Option<PendingPartitionExecution>,
    deliveries: Vec<worth_runtime_bridge::facade::CorrespondenceDeliveryCounters>,
    query: Vec<WorthUiPresentationSemanticQueryObservation>,
    subscribers: Vec<WorthUiPresentationSemanticSubscriberIdentity>,
    scope_rejections: WorthUiPresentationScopeRejectionCounters,
}

struct PendingPartitionExecution {
    delivery: worth_runtime_bridge::facade::CorrespondenceDeliveryCounters,
    attempt: std::num::NonZeroU64,
    instances: Vec<RegisteredSemanticInstance>,
    next_instance: usize,
}

impl SemanticExecutionKey {
    pub(super) fn for_admission(
        admission: &WorthUiPresentationRuntimeAdmission,
        change: usize,
    ) -> Self {
        Self {
            attempt: admission.basis().attempt(),
            binding: admission.basis().binding(),
            change,
        }
    }

    pub(super) fn same_admission(self, other: Self) -> bool {
        self.attempt == other.attempt && self.binding == other.binding
    }
}

impl WorthUiPresentationAsyncRegistry {
    pub(crate) fn publish_and_execute_publication(
        &mut self,
        workspace: &mut runtime::WorthQueryWorkspace,
        admission: &WorthUiPresentationRuntimeAdmission,
        publication: &PresentationSemanticPublication,
    ) -> Result<WorthUiPresentationSemanticExecution, WorthUiPresentationSemanticExecutionDenial>
    {
        let key = SemanticExecutionKey::for_admission(admission, publication.change().ordinal());
        let progress =
            self.semantic_executions
                .remove(&key)
                .unwrap_or_else(|| PendingSemanticExecution {
                    partitions: publication.partitions().into(),
                    next_partition: 0,
                    current: None,
                    deliveries: Vec::new(),
                    query: Vec::new(),
                    subscribers: Vec::new(),
                    scope_rejections: Default::default(),
                });
        self.resume_semantic_execution(workspace, key, publication.change(), progress)
    }

    fn resume_semantic_execution(
        &mut self,
        workspace: &mut runtime::WorthQueryWorkspace,
        key: SemanticExecutionKey,
        change: WorthUiPresentationSemanticChange,
        mut progress: PendingSemanticExecution,
    ) -> Result<WorthUiPresentationSemanticExecution, WorthUiPresentationSemanticExecutionDenial>
    {
        while progress.next_partition < progress.partitions.len() {
            if progress.current.is_none() {
                let partition = &progress.partitions[progress.next_partition];
                let selection = self
                    .instances
                    .instances(partition)
                    .ok_or(WorthUiPresentationSemanticExecutionDenial::MissingSourceInstance)?;
                progress.scope_rejections = progress
                    .scope_rejections
                    .checked_add(selection.rejections)
                    .ok_or(WorthUiPresentationSemanticExecutionDenial::ScopeCounterOverflow)?;
                let instances = selection.instances;
                let source = instances
                    .first()
                    .ok_or(WorthUiPresentationSemanticExecutionDenial::MissingSourceInstance)?;
                let delivery = deliver_change(workspace, &source.query, change)?;
                let attempt = self.execution_attempt(partition)?;
                progress.current = Some(PendingPartitionExecution {
                    delivery: delivery.counters(),
                    attempt,
                    instances,
                    next_instance: 0,
                });
            }
            let current = progress
                .current
                .as_mut()
                .expect("partition execution was initialized");
            while let Some(instance) = current.instances.get(current.next_instance) {
                match execute_instance(workspace, &instance.query, current.attempt) {
                    Ok(report) => {
                        let performed = report
                            .performed_signal_invalidation()
                            .expect("successful semantic execution retains performed Signal truth")
                            .summary();
                        progress
                            .query
                            .push(WorthUiPresentationSemanticQueryObservation {
                                outcome: report.provenance().class(),
                                performed,
                            });
                        progress.subscribers.push(instance.subscriber);
                        current.next_instance += 1;
                    }
                    Err(denial) => {
                        self.semantic_executions.insert(key, progress);
                        return Err(denial);
                    }
                }
            }
            let completed = progress
                .current
                .take()
                .expect("completed partition execution remains retained");
            progress.deliveries.push(completed.delivery);
            progress.next_partition += 1;
        }
        Ok(WorthUiPresentationSemanticExecution {
            deliveries: progress.deliveries.into_boxed_slice(),
            query: progress.query.into_boxed_slice(),
            subscribers: progress.subscribers.into_boxed_slice(),
            scope_rejections: progress.scope_rejections,
        })
    }

    fn execution_attempt(
        &mut self,
        partition: &PresentationSemanticPartition,
    ) -> Result<std::num::NonZeroU64, WorthUiPresentationSemanticExecutionDenial> {
        if let Some(attempt) = self.execution_attempts.get(partition).copied() {
            return Ok(attempt);
        }
        self.next_execution_attempt = if self.next_execution_attempt == 0 {
            1
        } else {
            self.next_execution_attempt
                .checked_add(2)
                .ok_or(WorthUiPresentationSemanticExecutionDenial::ExecutionAttemptExhausted)?
        };
        let attempt = std::num::NonZeroU64::new(self.next_execution_attempt)
            .expect("checked execution attempt is non-zero");
        self.execution_attempts.insert(partition.clone(), attempt);
        Ok(attempt)
    }
}

fn deliver_change(
    workspace: &mut runtime::WorthQueryWorkspace,
    source: &runtime::WorthQueryInstalledOwnedConditionalInstance,
    change: WorthUiPresentationSemanticChange,
) -> Result<
    worth_runtime_bridge::facade::BridgeCorrespondenceDeliveryReceipt,
    WorthUiPresentationSemanticExecutionDenial,
> {
    let delivery = super::super::semantic_invalidation::publish_presentation_semantic_change(
        workspace,
        source,
        change.ordinal(),
    )
    .map_err(WorthUiPresentationSemanticExecutionDenial::Delivery)?;
    match delivery {
        worth_proof::TransitionOutcome::Success(receipt) => Ok(receipt),
        worth_proof::TransitionOutcome::Denied(denial) => Err(
            WorthUiPresentationSemanticExecutionDenial::DeliveryDenied(Box::new(denial)),
        ),
        worth_proof::TransitionOutcome::Deferred(denial) => {
            Err(WorthUiPresentationSemanticExecutionDenial::DeliveryDeferred(denial))
        }
        worth_proof::TransitionOutcome::Stale(denial) => Err(
            WorthUiPresentationSemanticExecutionDenial::DeliveryStale(denial),
        ),
        worth_proof::TransitionOutcome::RebindRequired(denial) => {
            Err(WorthUiPresentationSemanticExecutionDenial::DeliveryRebindRequired(denial))
        }
        worth_proof::TransitionOutcome::Failed(denial) => Err(
            WorthUiPresentationSemanticExecutionDenial::DeliveryFailed(denial),
        ),
    }
}

fn execute_instance(
    workspace: &mut runtime::WorthQueryWorkspace,
    instance: &runtime::WorthQueryInstalledOwnedConditionalInstance,
    attempt: std::num::NonZeroU64,
) -> Result<
    worth_query::facade::domain::WorthQueryOwnedConditionalExecutionReport,
    WorthUiPresentationSemanticExecutionDenial,
> {
    let domain = workspace
        .domain(super::super::semantic_invalidation::WorthUiPresentationAsyncDomainEntry)
        .map_err(WorthUiPresentationSemanticExecutionDenial::Domain)?;
    let bound = workspace
        .observe_operating_world()
        .map_err(|error| {
            WorthUiPresentationSemanticExecutionDenial::OperatingWorld(Box::new(error))
        })?
        .family(super::super::semantic_invalidation::WorthUiPresentationAsyncOperationFamily)
        .bind(
            &domain,
            super::super::semantic_invalidation::WorthUiPresentationAsyncOperation,
        )
        .map_err(|error| WorthUiPresentationSemanticExecutionDenial::Binding(Box::new(error)))?;
    let admitted = match bound.admit_execution_resources(
        (),
        crate::installed_domain::execution_resources::operation_execution_resource_request(),
        workspace,
    ) {
        worth_proof::TransitionOutcome::Success(admitted) => admitted,
        worth_proof::TransitionOutcome::Denied(denial)
        | worth_proof::TransitionOutcome::Deferred(denial)
        | worth_proof::TransitionOutcome::Stale(denial)
        | worth_proof::TransitionOutcome::RebindRequired(denial)
        | worth_proof::TransitionOutcome::Failed(denial) => {
            return Err(WorthUiPresentationSemanticExecutionDenial::Resources(
                denial,
            ));
        }
    };
    admitted
        .execute_owned_conditional_instance(instance, attempt, workspace)
        .map_err(|error| WorthUiPresentationSemanticExecutionDenial::Query(Box::new(error)))
}
