use std::marker::PhantomData;

use worth_query_installation::facade::{ApplicationSchema, WorthQueryClockCoordinate};

use super::installation::WorthQueryConditionalClockHandle;
use super::WorthQueryConditionalOperationRegistry;
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;

mod erased;
pub(in crate::domain_computation::primary_graph) use erased::{
    ErasedClockObservationOutcome, ErasedClockObservationReceipt,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryConditionalClockObservationDenialKind {
    ForeignRuntime,
    BindingNotInstalled,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConditionalClockObservationDenial {
    kind: WorthQueryConditionalClockObservationDenialKind,
    subject: String,
}

impl WorthQueryConditionalClockObservationDenial {
    fn new(
        kind: WorthQueryConditionalClockObservationDenialKind,
        subject: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            subject: subject.into(),
        }
    }

    pub fn kind(&self) -> WorthQueryConditionalClockObservationDenialKind {
        self.kind
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryConditionalClockObservationFailureKind {
    SourceUnavailable,
    ObservationFailed,
    SourcePanicked,
    RuntimeRejected,
    ActiveSnapshotCapacityExhausted { maximum_active_snapshots: usize },
    RetentionCapacityExhausted,
    RetentionIdentityExhausted,
    SnapshotIdentityExhausted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConditionalClockObservationFailure {
    kind: WorthQueryConditionalClockObservationFailureKind,
    detail: String,
}

impl WorthQueryConditionalClockObservationFailure {
    pub fn kind(&self) -> WorthQueryConditionalClockObservationFailureKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

pub struct WorthQueryConditionalClockObservationReceipt<Clock> {
    granular_invalidation_installation:
        crate::domain_computation::primary_graph::WorthQueryGranularInvalidationInstallation,
    granular_source_read_basis:
        Option<crate::domain_computation::primary_graph::WorthQueryGranularSourceReadBasis>,
    sequence: u64,
    observed_time: WorthQueryClockCoordinate<Clock>,
    due_wake_count: usize,
    due_work_remaining: bool,
    authoritative_commit_count: usize,
    authoritative_work_remaining: bool,
    retained_due_wake_count: usize,
    retained_eligible_wake_count: usize,
    retained_suppressed_wake_count: usize,
    retained_deferred_wake_count: usize,
    retained_failed_wake_count: usize,
    committed_operation_count: usize,
    already_committed_operation_count: usize,
    failed_operation_count: usize,
    indeterminate_operation_count: usize,
    snapshot_capacity_backpressure: Option<usize>,
    retention_capacity_backpressure: bool,
    execution_provenance: Vec<super::WorthQueryConditionalExecutionProvenance>,
    granular_invalidations: Vec<worth_runtime_bridge::facade::BridgeGranularInvalidationDelivery>,
}

impl<Clock> WorthQueryConditionalClockObservationReceipt<Clock> {
    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    pub fn observed_time(&self) -> WorthQueryClockCoordinate<Clock> {
        WorthQueryClockCoordinate::from_nanoseconds(self.observed_time.nanoseconds())
    }

    pub fn due_wake_count(&self) -> usize {
        self.due_wake_count
    }

    pub fn due_work_remaining(&self) -> bool {
        self.due_work_remaining
    }

    /// Relevant authoritative commits examined before this clock advance.
    pub fn authoritative_commit_count(&self) -> usize {
        self.authoritative_commit_count
    }

    pub fn authoritative_work_remaining(&self) -> bool {
        self.authoritative_work_remaining
    }

    /// Wakes retained inside Query until governed operation re-entry consumes them.
    pub fn retained_due_wake_count(&self) -> usize {
        self.retained_due_wake_count
    }

    pub fn retained_eligible_wake_count(&self) -> usize {
        self.retained_eligible_wake_count
    }

    pub fn retained_suppressed_wake_count(&self) -> usize {
        self.retained_suppressed_wake_count
    }

    pub fn retained_deferred_wake_count(&self) -> usize {
        self.retained_deferred_wake_count
    }

    pub fn retained_failed_wake_count(&self) -> usize {
        self.retained_failed_wake_count
    }

    pub fn committed_operation_count(&self) -> usize {
        self.committed_operation_count
    }

    pub fn already_committed_operation_count(&self) -> usize {
        self.already_committed_operation_count
    }

    pub fn failed_operation_count(&self) -> usize {
        self.failed_operation_count
    }

    pub fn indeterminate_operation_count(&self) -> usize {
        self.indeterminate_operation_count
    }

    /// The owner-local snapshot ceiling that deferred operation re-entry.
    pub fn snapshot_capacity_backpressure(&self) -> Option<usize> {
        self.snapshot_capacity_backpressure
    }

    /// Whether owner-local retention pressure deferred operation re-entry.
    pub fn retention_capacity_backpressure(&self) -> bool {
        self.retention_capacity_backpressure
    }

    pub fn execution_provenance(&self) -> &[super::WorthQueryConditionalExecutionProvenance] {
        &self.execution_provenance
    }

    /// Consume the exact lower-runtime deliveries observed while this clock
    /// observation reconsidered authoritative commits. The returned carrier
    /// is transport evidence only; Query still performs candidate selection
    /// and current admission.
    pub fn take_granular_invalidation_batch(
        &mut self,
    ) -> crate::domain_computation::primary_graph::WorthQueryGranularInvalidationDeliveryBatch {
        crate::domain_computation::primary_graph::granular_invalidation::collect_granular_invalidations(
            self.granular_invalidation_installation.clone(),
            std::mem::take(&mut self.granular_invalidations),
            self.granular_source_read_basis.clone(),
        )
    }
}

pub enum WorthQueryConditionalClockObservationOutcome<Clock> {
    Accepted(WorthQueryConditionalClockObservationReceipt<Clock>),
    Duplicate(WorthQueryConditionalClockObservationReceipt<Clock>),
    Stale,
    Reordered,
    Closed,
    Failed(WorthQueryConditionalClockObservationFailure),
}

pub struct WorthQueryConditionalClockObservationPort<'runtime, Schema, Node, Clock> {
    runtime: &'runtime mut WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    handle: &'runtime WorthQueryConditionalClockHandle<Schema, Node, Clock>,
    marker: PhantomData<fn() -> (Node, Clock)>,
}

impl<'runtime, Schema, Node, Clock>
    WorthQueryConditionalClockObservationPort<'runtime, Schema, Node, Clock>
where
    Schema: ApplicationSchema,
{
    pub fn observe(&mut self) -> WorthQueryConditionalClockObservationOutcome<Clock> {
        let identity = self.handle.binding_identity();
        let truth = match super::signal_decision_reentry::WorthQueryConditionalTruthBasis::acquire(
            &self.runtime.runtime,
        ) {
            Ok(truth) => truth,
            Err(super::signal_decision_reentry::WorthQueryConditionalTruthBasisDenial::ActiveSnapshotCapacityExhausted {
                maximum_active_snapshots,
            }) => {
                return WorthQueryConditionalClockObservationOutcome::Failed(
                    WorthQueryConditionalClockObservationFailure {
                        kind: WorthQueryConditionalClockObservationFailureKind::ActiveSnapshotCapacityExhausted {
                            maximum_active_snapshots,
                        },
                        detail: "conditional clock observation exhausted active snapshot capacity".to_string(),
                    },
                );
            }
            Err(super::signal_decision_reentry::WorthQueryConditionalTruthBasisDenial::RetentionCapacityExhausted) => {
                return WorthQueryConditionalClockObservationOutcome::Failed(
                    WorthQueryConditionalClockObservationFailure {
                        kind: WorthQueryConditionalClockObservationFailureKind::RetentionCapacityExhausted,
                        detail: "conditional clock observation exhausted relational retention capacity".to_string(),
                    },
                );
            }
            Err(super::signal_decision_reentry::WorthQueryConditionalTruthBasisDenial::RetentionIdentityExhausted) => {
                return WorthQueryConditionalClockObservationOutcome::Failed(
                    WorthQueryConditionalClockObservationFailure {
                        kind: WorthQueryConditionalClockObservationFailureKind::RetentionIdentityExhausted,
                        detail: "conditional clock observation exhausted relational retention identity space".to_string(),
                    },
                );
            }
            Err(super::signal_decision_reentry::WorthQueryConditionalTruthBasisDenial::SnapshotIdentityExhausted) => {
                return WorthQueryConditionalClockObservationOutcome::Failed(
                    WorthQueryConditionalClockObservationFailure {
                        kind: WorthQueryConditionalClockObservationFailureKind::SnapshotIdentityExhausted,
                        detail: "conditional clock observation exhausted snapshot identity space".to_string(),
                    },
                );
            }
            Err(super::signal_decision_reentry::WorthQueryConditionalTruthBasisDenial::RuntimeRejected(detail)) => {
                return WorthQueryConditionalClockObservationOutcome::Failed(
                    WorthQueryConditionalClockObservationFailure {
                        kind: WorthQueryConditionalClockObservationFailureKind::RuntimeRejected,
                        detail: detail.to_string(),
                    },
                );
            }
        };
        let granular_invalidation_installation = self.runtime.granular_invalidation_installation();
        let granular_source_read_basis = truth.granular_source_read_basis();
        let mut owners = super::runtime_owners::ConditionalRuntimeOwners::take(self.runtime);
        let outcome = owners.observe_clock(identity, &self.handle.lease, &truth);
        match outcome {
            Some(outcome) => outcome.typed(
                granular_invalidation_installation,
                Some(granular_source_read_basis),
            ),
            None => WorthQueryConditionalClockObservationOutcome::Failed(
                WorthQueryConditionalClockObservationFailure {
                    kind: WorthQueryConditionalClockObservationFailureKind::RuntimeRejected,
                    detail: "conditional clock binding is no longer installed".to_string(),
                },
            ),
        }
    }
}

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub fn conditional_clock<'runtime, Node, Clock>(
        &'runtime mut self,
        handle: &'runtime WorthQueryConditionalClockHandle<Schema, Node, Clock>,
    ) -> Result<
        WorthQueryConditionalClockObservationPort<'runtime, Schema, Node, Clock>,
        WorthQueryConditionalClockObservationDenial,
    > {
        let registry = self
            .conditional_operations
            .get_mut()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        validate_handle(registry, handle)?;
        Ok(WorthQueryConditionalClockObservationPort {
            runtime: self,
            handle,
            marker: PhantomData,
        })
    }
}

fn validate_handle<Schema, Node, Clock>(
    registry: &WorthQueryConditionalOperationRegistry<Schema>,
    handle: &WorthQueryConditionalClockHandle<Schema, Node, Clock>,
) -> Result<(), WorthQueryConditionalClockObservationDenial> {
    if registry.contains_clock(handle.binding_identity(), &handle.lease) {
        Ok(())
    } else {
        Err(WorthQueryConditionalClockObservationDenial::new(
            WorthQueryConditionalClockObservationDenialKind::ForeignRuntime,
            handle.binding_identity(),
        ))
    }
}

#[cfg(test)]
mod tests;
