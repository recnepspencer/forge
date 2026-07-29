use std::collections::VecDeque;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use worth_query_admission::facade::authenticated_principal::WorthQueryRequestInterruption;
use worth_query_declaration::facade::application_schema::{
    ApplicationEffectPayload, ApplicationEffectRef,
};
use worth_query_installation::facade::{ApplicationSchema, ApplicationSchemaMember};
use worth_runtime_bridge::facade::{
    BridgeExecutionBasisTerminalDisposition, BridgeManagedQueueOccupancy,
};

use super::super::{
    WorthQueryAdmittedApplicationOperation, WorthQueryOperationScopeFingerprint,
    WorthQueryPrimaryGraphApplicationRuntime,
};
use super::{
    WorthQueryLiveCommitCause, WorthQueryLiveDeliveryControls, WorthQueryLiveDeliveryOpenDenial,
    WorthQueryLiveDeliveryOpenDenialKind, WorthQueryLiveDeliveryOutcome,
    WorthQueryLiveDeliveryOverflow, WorthQueryLiveSourcePoll,
};
use crate::domain_computation::managed_run::{
    admit_managed_lower_execution_basis, WorthQueryManagedLowerBinding,
    WorthQueryManagedLowerExecutionBasis, WorthQueryManagedTruthReadRequest,
};

static NEXT_LIVE_LEASE: AtomicU64 = AtomicU64::new(1);

/// Move-only Query authority for one installed live effect projection scope.
///
/// Descriptive scope fingerprints and effect references cannot construct it:
///
/// ```compile_fail
/// use worth_query_execution::facade::primary_graph::WorthQueryLiveEffectLease;
///
/// let _: WorthQueryLiveEffectLease<'static, (), (), (), (), (), ()> =
///     serde_json::from_str("{}").unwrap();
/// ```
pub struct WorthQueryLiveEffectLease<'runtime, Schema, Operation, Input, Scope, Effect, Payload> {
    runtime: &'runtime WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    basis: Option<WorthQueryManagedLowerExecutionBasis>,
    scope: WorthQueryOperationScopeFingerprint,
    cursor: u64,
    effect: ApplicationEffectRef<Schema, Effect, Payload>,
    payload_filter: Arc<dyn Fn(&Payload) -> bool + Send + Sync>,
    controls: WorthQueryLiveDeliveryControls,
    pending: VecDeque<WorthQueryPendingLiveCause<Payload>>,
    active_batch: Option<WorthQueryActiveLiveBatch>,
    #[cfg(test)]
    fail_next_queue_release: bool,
    _operation: PhantomData<fn(Input) -> (Operation, Scope)>,
}

struct WorthQueryActiveLiveBatch {
    batch: super::WorthQueryLiveCommitBatch,
    next_emission: usize,
}

struct WorthQueryPendingLiveCause<Payload> {
    commit_id: worth_relational::facade::history::CommitId,
    payload: Payload,
    occupancy: BridgeManagedQueueOccupancy,
}

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub fn open_live_effect_lease<Operation, Input, Scope, Effect, Payload>(
        &self,
        admission: &WorthQueryAdmittedApplicationOperation<Schema, Operation, Input, Scope>,
        effect: ApplicationEffectRef<Schema, Effect, Payload>,
        controls: WorthQueryLiveDeliveryControls,
    ) -> Result<
        WorthQueryLiveEffectLease<'_, Schema, Operation, Input, Scope, Effect, Payload>,
        WorthQueryLiveDeliveryOpenDenial,
    >
    where
        Payload: ApplicationEffectPayload + Clone,
    {
        self.open_live_effect_lease_matching(admission, effect, controls, |_| true)
    }

    pub fn open_live_effect_lease_matching<Operation, Input, Scope, Effect, Payload>(
        &self,
        admission: &WorthQueryAdmittedApplicationOperation<Schema, Operation, Input, Scope>,
        effect: ApplicationEffectRef<Schema, Effect, Payload>,
        controls: WorthQueryLiveDeliveryControls,
        payload_filter: impl Fn(&Payload) -> bool + Send + Sync + 'static,
    ) -> Result<
        WorthQueryLiveEffectLease<'_, Schema, Operation, Input, Scope, Effect, Payload>,
        WorthQueryLiveDeliveryOpenDenial,
    >
    where
        Payload: ApplicationEffectPayload + Clone,
    {
        admission
            .validate_projection_authority(
                self.runtime.authority_identity(),
                &self.installed_schema.binding_identity(),
            )
            .map_err(|denial| {
                WorthQueryLiveDeliveryOpenDenial::new(
                    WorthQueryLiveDeliveryOpenDenialKind::Authorization,
                    format!("{:?}", denial.kind()),
                )
            })?;
        let installed_effect = self
            .installed_schema
            .installed_declaration()
            .members()
            .iter()
            .any(|member| {
                matches!(
                    member,
                    ApplicationSchemaMember::Effect {
                        effect: installed,
                        payload_type,
                    } if installed == effect.name()
                        && payload_type == std::any::type_name::<Payload>()
                )
            });
        if !installed_effect {
            return Err(WorthQueryLiveDeliveryOpenDenial::new(
                WorthQueryLiveDeliveryOpenDenialKind::UninstalledEffect,
                effect.name(),
            ));
        }
        let strategy = admission
            .allowed_graph_contract()
            .execution_strategy()
            .ok_or_else(|| {
                WorthQueryLiveDeliveryOpenDenial::new(
                    WorthQueryLiveDeliveryOpenDenialKind::InvalidInstalledStrategySet,
                    admission.operation(),
                )
            })?;
        if controls.buffer_capacity() as u64 > strategy.envelope().queue_depth_ceiling() {
            return Err(WorthQueryLiveDeliveryOpenDenial::new(
                WorthQueryLiveDeliveryOpenDenialKind::BufferCapacityExceedsInstalled,
                admission.operation(),
            ));
        }
        let version = self
            .primary_provider
            .graph
            .with_runtime(|runtime| {
                runtime
                    .history()
                    .latest_commit()
                    .map(|head| head.version_id)
            })
            .ok_or_else(|| {
                WorthQueryLiveDeliveryOpenDenial::new(
                    WorthQueryLiveDeliveryOpenDenialKind::UnavailableProviderVersion,
                    admission.operation(),
                )
            })?;
        let attempt = NEXT_LIVE_LEASE.fetch_add(1, Ordering::Relaxed);
        let attempt_identity = format!("live-delivery:{attempt}");
        let binding = WorthQueryManagedLowerBinding::new(
            admission.operation(),
            &attempt_identity,
            strategy.envelope(),
        );
        let request = WorthQueryManagedTruthReadRequest::new(
            version,
            worth_runtime_bridge::facade::TruthBranchIdentity::from_relational_branch_id("main"),
            worth_runtime_bridge::facade::SnapshotReadPacket::new(Vec::new()),
        );
        let request_bridge = self.bridge.fork_managed_request_lane();
        let basis = admit_managed_lower_execution_basis(
            &request_bridge,
            &self.relational_source,
            binding,
            request,
        )
        .map_err(|failure| {
            WorthQueryLiveDeliveryOpenDenial::new(
                WorthQueryLiveDeliveryOpenDenialKind::BridgeBasisRejected,
                failure.detail.as_ref(),
            )
        })?;
        Ok(WorthQueryLiveEffectLease {
            runtime: self,
            basis: Some(basis),
            scope: admission.operation_scope_fingerprint(),
            cursor: self.primary_provider.live_delivery.open_cursor(),
            effect,
            payload_filter: Arc::new(payload_filter),
            controls,
            pending: VecDeque::new(),
            active_batch: None,
            #[cfg(test)]
            fail_next_queue_release: false,
            _operation: PhantomData,
        })
    }
}

impl<Schema, Operation, Input, Scope, Effect, Payload>
    WorthQueryLiveEffectLease<'_, Schema, Operation, Input, Scope, Effect, Payload>
where
    Schema: ApplicationSchema,
    Payload: ApplicationEffectPayload + Clone,
{
    pub fn buffered_cause_count(&self) -> usize {
        self.pending.len()
    }

    #[cfg(test)]
    pub(crate) fn fail_next_queue_release(&mut self) {
        self.fail_next_queue_release = true;
    }

    pub fn poll(
        &mut self,
        admission: &WorthQueryAdmittedApplicationOperation<Schema, Operation, Input, Scope>,
    ) -> WorthQueryLiveDeliveryOutcome<Payload> {
        if let Some(interruption) = self.controls.request().interruption() {
            return match interruption {
                WorthQueryRequestInterruption::Cancelled => {
                    WorthQueryLiveDeliveryOutcome::Cancelled
                }
                WorthQueryRequestInterruption::DeadlineExceeded => {
                    WorthQueryLiveDeliveryOutcome::DeadlineExceeded
                }
            };
        }
        if admission.operation_scope_fingerprint() != self.scope {
            return WorthQueryLiveDeliveryOutcome::ScopeMismatch;
        }
        if let Err(denial) = admission.validate_projection_authority(
            self.runtime.runtime.authority_identity(),
            &self.runtime.installed_schema.binding_identity(),
        ) {
            return WorthQueryLiveDeliveryOutcome::AuthorizationDenied(denial.kind());
        }
        let mut terminal = WorthQueryLiveDeliveryOutcome::Pending;
        while self.pending.len() < self.controls.buffer_capacity() {
            if self.active_batch.is_none() {
                let batch = match self
                    .runtime
                    .primary_provider
                    .live_delivery
                    .poll(self.cursor)
                {
                    WorthQueryLiveSourcePoll::Batch(batch) => batch,
                    WorthQueryLiveSourcePoll::Pending => break,
                    WorthQueryLiveSourcePoll::Overflow { missed } => {
                        terminal = WorthQueryLiveDeliveryOutcome::Overflow(
                            WorthQueryLiveDeliveryOverflow::new(missed),
                        );
                        break;
                    }
                    WorthQueryLiveSourcePoll::Closed => {
                        terminal = WorthQueryLiveDeliveryOutcome::Closed;
                        break;
                    }
                };
                self.active_batch = Some(WorthQueryActiveLiveBatch {
                    batch,
                    next_emission: 0,
                });
            }
            let active = self
                .active_batch
                .as_mut()
                .expect("live batch was installed above");
            let Some(emission) = active.batch.emissions.get(active.next_emission) else {
                self.cursor = active.batch.sequence.saturating_add(1);
                self.active_batch = None;
                continue;
            };
            let Some(payload) = emission
                .cloned_payload(&self.effect)
                .filter(|payload| (self.payload_filter)(payload))
            else {
                active.next_emission = active.next_emission.saturating_add(1);
                continue;
            };
            let Some(basis) = self.basis.as_mut() else {
                return WorthQueryLiveDeliveryOutcome::Closed;
            };
            let admission = match basis.bridge.enqueue_managed_queue(1) {
                Ok(admission) => admission,
                Err(_) => return WorthQueryLiveDeliveryOutcome::Unavailable,
            };
            let (_, occupancy) = admission.into_parts();
            active.next_emission = active.next_emission.saturating_add(1);
            self.pending.push_back(WorthQueryPendingLiveCause {
                commit_id: active.batch.commit_id,
                payload,
                occupancy,
            });
        }
        let Some(mut pending) = self.pending.pop_front() else {
            return terminal;
        };
        #[cfg(test)]
        if std::mem::take(&mut self.fail_next_queue_release) {
            self.pending.push_front(pending);
            return WorthQueryLiveDeliveryOutcome::Unavailable;
        }
        let Some(basis) = self.basis.as_mut() else {
            return WorthQueryLiveDeliveryOutcome::Closed;
        };
        if let Err(failure) = basis
            .bridge
            .release_managed_queue_occupancy(pending.occupancy)
        {
            pending.occupancy = failure.into_occupancy();
            self.pending.push_front(pending);
            return WorthQueryLiveDeliveryOutcome::Unavailable;
        }
        WorthQueryLiveDeliveryOutcome::Delivered(WorthQueryLiveCommitCause::new(
            pending.commit_id,
            pending.payload,
        ))
    }

    pub fn close(mut self) {
        if let Some(mut basis) = self.basis.take() {
            while let Some(pending) = self.pending.pop_front() {
                let _ = basis
                    .bridge
                    .release_managed_queue_occupancy(pending.occupancy);
            }
            let _ = basis
                .bridge
                .finalize(BridgeExecutionBasisTerminalDisposition::Cancelled);
        }
    }
}
