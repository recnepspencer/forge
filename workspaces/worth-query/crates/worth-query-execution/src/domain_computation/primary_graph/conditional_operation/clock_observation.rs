use std::marker::PhantomData;

use worth_query_installation::facade::{ApplicationSchema, WorthQueryClockCoordinate};

use super::installation::WorthQueryConditionalClockHandle;
use super::WorthQueryConditionalOperationRegistry;
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;

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
            Err(detail) => {
                return WorthQueryConditionalClockObservationOutcome::Failed(
                    WorthQueryConditionalClockObservationFailure {
                        kind: WorthQueryConditionalClockObservationFailureKind::RuntimeRejected,
                        detail: detail.to_string(),
                    },
                );
            }
        };
        let granular_invalidation_installation = self.runtime.granular_invalidation_installation();
        let mut owners = super::runtime_owners::ConditionalRuntimeOwners::take(self.runtime);
        let outcome = owners.observe_clock(identity, &self.handle.lease, &truth);
        match outcome {
            Some(outcome) => outcome.typed(granular_invalidation_installation),
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

pub(in crate::domain_computation::primary_graph) struct ErasedClockObservationReceipt {
    pub(super) sequence: u64,
    pub(super) observed_coordinate: u64,
    pub(super) due_wake_count: usize,
    pub(super) due_work_remaining: bool,
    pub(super) authoritative_commit_count: usize,
    pub(super) authoritative_work_remaining: bool,
    pub(super) retained_due_wake_count: usize,
    pub(super) retained_eligible_wake_count: usize,
    pub(super) retained_suppressed_wake_count: usize,
    pub(super) retained_deferred_wake_count: usize,
    pub(super) retained_failed_wake_count: usize,
    pub(super) committed_operation_count: usize,
    pub(super) already_committed_operation_count: usize,
    pub(super) failed_operation_count: usize,
    pub(super) indeterminate_operation_count: usize,
    pub(super) execution_provenance: Vec<super::WorthQueryConditionalExecutionProvenance>,
    pub(super) granular_invalidations:
        Vec<worth_runtime_bridge::facade::BridgeGranularInvalidationDelivery>,
}

pub(in crate::domain_computation::primary_graph) enum ErasedClockObservationOutcome {
    Accepted(ErasedClockObservationReceipt),
    Duplicate(ErasedClockObservationReceipt),
    Stale,
    Reordered,
    Closed,
    Failed {
        kind: WorthQueryConditionalClockObservationFailureKind,
        detail: String,
    },
}

impl ErasedClockObservationOutcome {
    fn typed<Clock>(
        self,
        installation: crate::domain_computation::primary_graph::WorthQueryGranularInvalidationInstallation,
    ) -> WorthQueryConditionalClockObservationOutcome<Clock> {
        match self {
            Self::Accepted(receipt) => {
                WorthQueryConditionalClockObservationOutcome::Accepted(receipt.typed(installation))
            }
            Self::Duplicate(receipt) => {
                WorthQueryConditionalClockObservationOutcome::Duplicate(receipt.typed(installation))
            }
            Self::Stale => WorthQueryConditionalClockObservationOutcome::Stale,
            Self::Reordered => WorthQueryConditionalClockObservationOutcome::Reordered,
            Self::Closed => WorthQueryConditionalClockObservationOutcome::Closed,
            Self::Failed { kind, detail } => WorthQueryConditionalClockObservationOutcome::Failed(
                WorthQueryConditionalClockObservationFailure { kind, detail },
            ),
        }
    }
}

impl ErasedClockObservationReceipt {
    fn typed<Clock>(
        self,
        granular_invalidation_installation: crate::domain_computation::primary_graph::WorthQueryGranularInvalidationInstallation,
    ) -> WorthQueryConditionalClockObservationReceipt<Clock> {
        WorthQueryConditionalClockObservationReceipt {
            granular_invalidation_installation,
            sequence: self.sequence,
            observed_time: WorthQueryClockCoordinate::from_nanoseconds(self.observed_coordinate),
            due_wake_count: self.due_wake_count,
            due_work_remaining: self.due_work_remaining,
            authoritative_commit_count: self.authoritative_commit_count,
            authoritative_work_remaining: self.authoritative_work_remaining,
            retained_due_wake_count: self.retained_due_wake_count,
            retained_eligible_wake_count: self.retained_eligible_wake_count,
            retained_suppressed_wake_count: self.retained_suppressed_wake_count,
            retained_deferred_wake_count: self.retained_deferred_wake_count,
            retained_failed_wake_count: self.retained_failed_wake_count,
            committed_operation_count: self.committed_operation_count,
            already_committed_operation_count: self.already_committed_operation_count,
            failed_operation_count: self.failed_operation_count,
            indeterminate_operation_count: self.indeterminate_operation_count,
            execution_provenance: self.execution_provenance,
            granular_invalidations: self.granular_invalidations,
        }
    }
}

#[cfg(test)]
mod tests;
