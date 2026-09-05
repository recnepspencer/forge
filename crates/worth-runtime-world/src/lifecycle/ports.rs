use worth_signal::facade::{SignalError, SignalTransaction};

use crate::branch::ProductBranchReferenceCell;
use crate::branch::{
    ProductBranchCreationIntent, ProductBranchObservation, ProductBranchRetirementReport,
    RuntimeWorldBootstrapIntent, RuntimeWorldBootstrapOutcome, RuntimeWorldBranchAdmissionDenial,
    RuntimeWorldBranchRetirementDenial,
};
use crate::identity::ProductBranchIdentity;
use crate::lifecycle::RuntimeWorldInstant;
use crate::publication::{
    CompositeLateCancellationPosture, CompositePublicationIntent, CompositePublicationReady,
    CompositePublicationStage, NoEffectCompositePublication, OwnerExecutionOutcome,
    PreparedCompositePublicationWithSignal, PreparedCompositePublicationWithoutSignal,
    ReservedBranchCreationAttempt, RuntimeWorldCancellationToken, RuntimeWorldPublicationOutcome,
};
use crate::recovery::{ProductUnpublishedOwnerEffects, RecoveryContinuationContract};

use super::close::{RuntimeWorldCloseDenial, RuntimeWorldCloseReport};

/// Lifecycle state of the managed Runtime World owner.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeWorldOwnerLifecycleObservation {
    Open,
    Closing,
    Closed,
}

/// Typed post-close failure for weak Runtime World ports.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeWorldOwnerUnavailable {
    _private: (),
}

impl RuntimeWorldOwnerUnavailable {
    pub(crate) const fn new() -> Self {
        Self { _private: () }
    }
}

/// Owner-issued input for one composite branch-creation attempt. The source
/// observation carries the exact source identity, reference occurrence, and
/// component basis. Creation never borrows a Signal transaction: forking a
/// component branch is not a mutation of that branch.
pub(crate) struct RuntimeWorldBranchCreationRequest<'a> {
    source: ProductBranchObservation,
    intent: ProductBranchCreationIntent,
    cancellation: &'a RuntimeWorldCancellationToken,
}

impl<'a> RuntimeWorldBranchCreationRequest<'a> {
    pub(crate) fn new(
        source: ProductBranchObservation,
        intent: ProductBranchCreationIntent,
        cancellation: &'a RuntimeWorldCancellationToken,
    ) -> Self {
        Self {
            source,
            intent,
            cancellation,
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        ProductBranchObservation,
        ProductBranchCreationIntent,
        &'a RuntimeWorldCancellationToken,
    ) {
        (self.source, self.intent, self.cancellation)
    }
}

/// Terminal owned by the branch-creation service. A performed value is a new
/// product reference occurrence; a partial value keeps performed owner work
/// in the bounded recovery authority without publishing a product head.
#[derive(Debug)]
pub(crate) enum RuntimeWorldBranchCreationOutcome {
    Performed(ProductBranchObservation),
    ProductUnpublished(ProductUnpublishedOwnerEffects),
}

/// Shared internal seam for exact product-head observation.
pub(crate) trait RuntimeWorldObservationService {
    fn observe_product_branch(
        &self,
        branch: &ProductBranchIdentity,
    ) -> Result<ProductBranchObservation, RuntimeWorldBranchAdmissionDenial>;
}

/// Shared internal seam for product-reference creation and retirement. The
/// per-owner creation plans live in `ProductBranchCreationIntent`; retirement
/// reports the component work it did not perform instead of silently
/// deleting a component branch.
pub(crate) trait RuntimeWorldBranchService {
    fn create_product_branch(
        &self,
        request: RuntimeWorldBranchCreationRequest<'_>,
    ) -> Result<RuntimeWorldBranchCreationOutcome, RuntimeWorldBranchAdmissionDenial>;

    /// Retire the installed occurrence proved by this observation. Later head
    /// movement is allowed; a recreated name is a different occurrence.
    fn retire_product_branch(
        &self,
        observed: &ProductBranchObservation,
    ) -> Result<ProductBranchRetirementReport, RuntimeWorldBranchRetirementDenial>;
}

/// Shared internal seam for the serial owner execution pipeline. Preparation
/// lowers and reserves in one step, so no caller can hold a lowered plan
/// without the bounded capacities that authorize executing it.
pub(crate) trait RuntimeWorldPreparationService {
    /// The prepared type is chosen by the intent's compile-visible stage: a
    /// `WithoutSignal` intent cannot yield a Signal-advancing reservation.
    fn prepare_publication<S>(
        &self,
        expected: ProductBranchObservation,
        intent: CompositePublicationIntent<S>,
        cancellation: &RuntimeWorldCancellationToken,
        deadline: Option<RuntimeWorldInstant>,
    ) -> Result<S::Prepared, NoEffectCompositePublication>
    where
        S: CompositePublicationStage;

    fn prepare_creation(
        &self,
        source: ProductBranchObservation,
        intent: ProductBranchCreationIntent,
        cancellation: &RuntimeWorldCancellationToken,
        deadline: Option<RuntimeWorldInstant>,
    ) -> Result<ReservedBranchCreationAttempt, RuntimeWorldBranchAdmissionDenial>;
}

/// Shared internal seam for the two owner-execution routes. There is one
/// method per compile-visible Signal decision; neither can be reached with
/// the other's prepared reservation.
pub(crate) trait RuntimeWorldOwnerExecutionService {
    type SignalDefinition: Copy + Ord + std::fmt::Debug + 'static;
    type SignalIdentity: Copy + Ord;
    type SignalEvent;
    type SignalContext;
    type SignalTransactionKey: Copy + Ord;

    fn execute_without_signal(
        &self,
        prepared: PreparedCompositePublicationWithoutSignal,
        cancellation: &RuntimeWorldCancellationToken,
    ) -> OwnerExecutionOutcome;

    fn execute_with_signal<F>(
        &self,
        prepared: PreparedCompositePublicationWithSignal,
        runtime_ctx: &mut Self::SignalContext,
        cancellation: &RuntimeWorldCancellationToken,
        apply: F,
    ) -> OwnerExecutionOutcome
    where
        F: FnOnce(
            &mut SignalTransaction<
                '_,
                Self::SignalDefinition,
                Self::SignalIdentity,
                Self::SignalEvent,
                Self::SignalContext,
                Self::SignalTransactionKey,
            >,
        ) -> Result<(), SignalError>;
}

pub(crate) trait RuntimeWorldProductPublicationService {
    fn publish(
        &self,
        ready: CompositePublicationReady,
        cell: &ProductBranchReferenceCell,
        late_cancellation: CompositeLateCancellationPosture,
    ) -> RuntimeWorldPublicationOutcome;
}

/// Shared internal seam for retained owner effects. Recovery never fabricates
/// a product commit or promotes a partial record to performed publication.
pub(crate) trait RuntimeWorldRecoveryService {
    fn continue_effects(
        &self,
        effects: ProductUnpublishedOwnerEffects,
    ) -> Result<RecoveryContinuationContract, RuntimeWorldOwnerUnavailable>;
}

/// Shared internal seam for one-shot root bootstrap and owner close. Close
/// reports what it drained and what it deliberately left to the component
/// owners rather than returning a bare unit.
pub(crate) trait RuntimeWorldLifecycleService {
    fn bootstrap_root(&self, intent: RuntimeWorldBootstrapIntent) -> RuntimeWorldBootstrapOutcome;

    fn close(&self) -> Result<RuntimeWorldCloseReport, RuntimeWorldCloseDenial>;
}
