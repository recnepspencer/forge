use std::sync::Arc;

use crate::basis_lifecycle::BasisOperationLane;

use super::refresh::{WorthQueryLiveProjectionRefreshError, WorthQueryPendingOwnerImpact};
use super::refresh_work::WorthQueryLiveProjectionRefreshWork;
use super::source::WorthQueryProjectionLifecycleSource;

pub(crate) struct WorthQueryClassifiedOwnerDeliveryCompletion {
    delivery: crate::ordinary::live::WorthQueryManagedLiveDelivery,
    impact: Arc<crate::domain_installation::WorthQueryImpactDecision>,
    conditional: Arc<crate::domain_installation::WorthQueryConditionalProvenance>,
    owner_delivery_receipt: Arc<worth_runtime_bridge::facade::BridgeCorrespondenceDeliveryReceipt>,
    work: WorthQueryLiveProjectionRefreshWork,
}

impl WorthQueryClassifiedOwnerDeliveryCompletion {
    pub(crate) fn work(&self) -> WorthQueryLiveProjectionRefreshWork {
        self.work
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        crate::ordinary::live::WorthQueryManagedLiveDelivery,
        Arc<crate::domain_installation::WorthQueryImpactDecision>,
        Arc<crate::domain_installation::WorthQueryConditionalProvenance>,
        Arc<worth_runtime_bridge::facade::BridgeCorrespondenceDeliveryReceipt>,
        WorthQueryLiveProjectionRefreshWork,
    ) {
        (
            self.delivery,
            self.impact,
            self.conditional,
            self.owner_delivery_receipt,
            self.work,
        )
    }
}

pub(in crate::domain_installation::operation_execution) fn refresh_owner_delivery<
    D: 'static,
    O: 'static,
    F: 'static,
    L: BasisOperationLane,
    S: WorthQueryProjectionLifecycleSource<D, O, F, L>,
>(
    source: &S,
    handle: &crate::ordinary::live::WorthQueryManagedLiveHandle,
    workspace: &mut crate::runtime::WorthQueryWorkspace,
    pending: WorthQueryPendingOwnerImpact<'_>,
) -> Result<WorthQueryClassifiedOwnerDeliveryCompletion, WorthQueryLiveProjectionRefreshError> {
    let closure = pending.closure;
    let (admitted, mut work) =
        admit_pending_owner_delivery::<D, O, F>(handle, workspace, pending.delivery, closure)?;
    let classification = match admitted.retained_classification() {
        Some(retained) => {
            work.retain_impact_reuse();
            retained
        }
        None => classify_target(OwnerDeliveryClassificationPass {
            source,
            workspace,
            delivery: pending.delivery,
            admitted: &admitted,
            closure,
            work: &mut work,
        })?,
    };
    emit_and_drain_owner_delivery(
        handle,
        workspace,
        OwnerDeliveryEmission {
            admitted,
            closure,
            classification,
            receipt: pending.delivery,
            work,
        },
    )
}

fn admit_pending_owner_delivery<D: 'static, O: 'static, F: 'static>(
    handle: &crate::ordinary::live::WorthQueryManagedLiveHandle,
    workspace: &mut crate::runtime::WorthQueryWorkspace,
    delivery: &worth_runtime_bridge::facade::BridgeCorrespondenceDeliveryReceipt,
    closure: &crate::domain_installation::WorthQueryCompiledSemanticAspectDependencyClosure,
) -> Result<
    (
        crate::runtime::WorthQueryAdmittedStagedOwnerDelivery,
        WorthQueryLiveProjectionRefreshWork,
    ),
    WorthQueryLiveProjectionRefreshError,
> {
    let mut work = WorthQueryLiveProjectionRefreshWork::authority_checked();
    crate::domain_installation::preflight_owner_delivered_impact(closure, delivery).map_err(
        |denial| WorthQueryLiveProjectionRefreshError::Impact {
            denial,
            work,
            owner_delivery_retained: false,
        },
    )?;
    match workspace.admit_staged_conditional_owner_delivery::<D, O, F>(handle, delivery) {
        Ok(admitted) => {
            work.retain_causal_admission(admitted.work());
            Ok((admitted, work))
        }
        Err(stop) => {
            work.retain_causal_admission(stop.work());
            Err(WorthQueryLiveProjectionRefreshError::Impact {
                denial: stop.denial(),
                work,
                owner_delivery_retained: true,
            })
        }
    }
}

struct OwnerDeliveryEmission<'a> {
    admitted: crate::runtime::WorthQueryAdmittedStagedOwnerDelivery,
    closure: &'a crate::domain_installation::WorthQueryCompiledSemanticAspectDependencyClosure,
    classification: crate::runtime::WorthQueryRetainedOwnerDeliveryClassification,
    receipt: &'a worth_runtime_bridge::facade::BridgeCorrespondenceDeliveryReceipt,
    work: WorthQueryLiveProjectionRefreshWork,
}

fn emit_and_drain_owner_delivery(
    handle: &crate::ordinary::live::WorthQueryManagedLiveHandle,
    workspace: &mut crate::runtime::WorthQueryWorkspace,
    emission: OwnerDeliveryEmission<'_>,
) -> Result<WorthQueryClassifiedOwnerDeliveryCompletion, WorthQueryLiveProjectionRefreshError> {
    let OwnerDeliveryEmission {
        admitted,
        closure,
        classification,
        receipt,
        mut work,
    } = emission;
    let impact = Arc::clone(classification.impact());
    let conditional = Arc::clone(classification.conditional_arc());
    workspace
        .emit_classified_conditional_owner_delivery(
            admitted,
            closure,
            classification.conditional(),
            &impact,
        )
        .map_err(|error| emission_error(error, work))?;
    work.begin_drain();
    let delivery =
        handle
            .drain(workspace)
            .map_err(|error| WorthQueryLiveProjectionRefreshError::Runtime {
                error,
                work,
                owner_delivery_retained: false,
            })?;
    work.retain_delivery(&delivery);
    Ok(WorthQueryClassifiedOwnerDeliveryCompletion {
        delivery,
        impact,
        conditional,
        owner_delivery_receipt: Arc::new(receipt.clone()),
        work,
    })
}

struct OwnerDeliveryClassificationPass<'a, S> {
    source: &'a S,
    workspace: &'a mut crate::runtime::WorthQueryWorkspace,
    delivery: &'a worth_runtime_bridge::facade::BridgeCorrespondenceDeliveryReceipt,
    admitted: &'a crate::runtime::WorthQueryAdmittedStagedOwnerDelivery,
    closure: &'a crate::domain_installation::WorthQueryCompiledSemanticAspectDependencyClosure,
    work: &'a mut WorthQueryLiveProjectionRefreshWork,
}

fn classify_target<D, O, F, L: BasisOperationLane, S>(
    pass: OwnerDeliveryClassificationPass<'_, S>,
) -> Result<
    crate::runtime::WorthQueryRetainedOwnerDeliveryClassification,
    WorthQueryLiveProjectionRefreshError,
>
where
    S: WorthQueryProjectionLifecycleSource<D, O, F, L>,
{
    let OwnerDeliveryClassificationPass {
        source,
        workspace,
        delivery,
        admitted,
        closure,
        work,
    } = pass;
    let conditional = match admitted.retained_decision_seed() {
        Some(seed) => Arc::new(
            super::owner_conditional::reenter_owner_delivered_conditional(
                super::owner_conditional::WorthQueryOwnerConditionalReentryPass {
                    source,
                    delivery,
                    seed: &seed,
                    workspace,
                    work,
                },
            )
            .map_err(|stop| conditional_error(stop, *work))?,
        ),
        None => {
            let evaluated = super::owner_conditional::evaluate_owner_delivered_conditional(
                source, delivery, workspace,
            )
            .map_err(|stop| conditional_error(stop, *work))?;
            work.retain_conditional(evaluated.counters);
            admitted.retain_decision_seed(evaluated.retained_seed);
            Arc::new(evaluated.provenance)
        }
    };
    super::owner_conditional::require_settled_owner_decision(&conditional)
        .map_err(|stop| conditional_error(stop, *work))?;
    let impact = Arc::new(
        crate::domain_installation::classify_owner_delivered_impact(
            closure,
            delivery,
            &conditional,
        )
        .map_err(|denial| WorthQueryLiveProjectionRefreshError::Impact {
            denial,
            work: *work,
            owner_delivery_retained: true,
        })?,
    );
    work.retain_impact(&impact);
    Ok(admitted.retain_classification(
        crate::runtime::WorthQueryRetainedOwnerDeliveryClassification::new(conditional, impact),
    ))
}

fn conditional_error(
    stop: super::conditional_core::WorthQueryLifecycleConditionalCoreStop,
    work: WorthQueryLiveProjectionRefreshWork,
) -> WorthQueryLiveProjectionRefreshError {
    WorthQueryLiveProjectionRefreshError::Conditional {
        kind: stop.kind,
        detail: stop.detail,
        work,
        owner_delivery_retained: true,
    }
}

fn emission_error(
    error: crate::runtime::WorthQueryClassifiedOwnerDeliveryEmissionError,
    work: WorthQueryLiveProjectionRefreshWork,
) -> WorthQueryLiveProjectionRefreshError {
    match error {
        crate::runtime::WorthQueryClassifiedOwnerDeliveryEmissionError::Impact(denial) => {
            WorthQueryLiveProjectionRefreshError::Impact {
                denial,
                work,
                owner_delivery_retained: true,
            }
        }
        crate::runtime::WorthQueryClassifiedOwnerDeliveryEmissionError::Runtime(error) => {
            WorthQueryLiveProjectionRefreshError::Runtime {
                error,
                work,
                owner_delivery_retained: true,
            }
        }
    }
}
