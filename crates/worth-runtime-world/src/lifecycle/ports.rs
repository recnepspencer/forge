use crate::branch::ProductBranchReferenceCell;
use crate::branch::{
    ProductBranchObservation, RuntimeWorldBootstrapIntent, RuntimeWorldBootstrapOutcome,
    RuntimeWorldBranchAdmissionDenial, RuntimeWorldBranchRetirementDenial,
};
use crate::identity::ProductBranchIdentity;
use crate::lifecycle::RuntimeWorldCancellationToken;
use crate::lifecycle::RuntimeWorldInstant;
use crate::publication::{
    CompositeExecutionBorrow, CompositeLateCancellationPosture, CompositePublicationCostCounters,
    CompositePublicationReady, LoweredOwnerComponentPlan, NoEffectCompositePublication,
    OwnerExecutionOutcome, ProductBranchIntent, ReservedCompositePublicationAttempt,
    RuntimeWorldPublicationOutcome,
};
use crate::recovery::{ProductUnpublishedOwnerEffects, RecoveryContinuationContract};

use super::close::RuntimeWorldCloseDenial;

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
/// component basis; the borrow carries only the synchronous Signal authority
/// required by the requested posture.
pub(crate) struct RuntimeWorldBranchCreationRequest<'a, D, I, E, Ctx, T = ()>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    source: ProductBranchObservation,
    intent: ProductBranchIntent,
    signal: CompositeExecutionBorrow<'a, D, I, E, Ctx, T>,
}

impl<'a, D, I, E, Ctx, T> RuntimeWorldBranchCreationRequest<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(crate) fn new(
        source: ProductBranchObservation,
        intent: ProductBranchIntent,
        signal: CompositeExecutionBorrow<'a, D, I, E, Ctx, T>,
    ) -> Self {
        Self {
            source,
            intent,
            signal,
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        ProductBranchObservation,
        ProductBranchIntent,
        CompositeExecutionBorrow<'a, D, I, E, Ctx, T>,
    ) {
        (self.source, self.intent, self.signal)
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
/// component posture pair lives in `ProductBranchIntent`; retirement cannot
/// delete a component branch as a side effect.
pub(crate) trait RuntimeWorldBranchService {
    type SignalDefinition: Copy + Ord + std::fmt::Debug + 'static;
    type SignalIdentity: Copy + Ord;
    type SignalEvent;
    type SignalContext;
    type SignalTransactionKey: Copy + Ord;

    fn create_product_branch(
        &self,
        request: RuntimeWorldBranchCreationRequest<
            '_,
            Self::SignalDefinition,
            Self::SignalIdentity,
            Self::SignalEvent,
            Self::SignalContext,
            Self::SignalTransactionKey,
        >,
    ) -> Result<RuntimeWorldBranchCreationOutcome, RuntimeWorldBranchAdmissionDenial>;

    fn retire_product_branch(
        &self,
        branch: ProductBranchIdentity,
    ) -> Result<(), RuntimeWorldBranchRetirementDenial>;
}

/// Shared internal seam for the serial owner execution pipeline.
pub(crate) trait RuntimeWorldPreparationService {
    fn prepare(
        &self,
        expected: ProductBranchObservation,
        intent: ProductBranchIntent,
    ) -> Result<LoweredOwnerComponentPlan, NoEffectCompositePublication>;

    fn reserve(
        &self,
        plan: LoweredOwnerComponentPlan,
        cancellation: &RuntimeWorldCancellationToken,
        deadline: Option<RuntimeWorldInstant>,
    ) -> Result<ReservedCompositePublicationAttempt, NoEffectCompositePublication>;
}

pub(crate) trait RuntimeWorldOwnerExecutionService {
    type SignalDefinition: Copy + Ord + std::fmt::Debug + 'static;
    type SignalIdentity: Copy + Ord;
    type SignalEvent;
    type SignalContext;
    type SignalTransactionKey: Copy + Ord;

    fn execute(
        &self,
        attempt: ReservedCompositePublicationAttempt,
        borrow: CompositeExecutionBorrow<
            '_,
            Self::SignalDefinition,
            Self::SignalIdentity,
            Self::SignalEvent,
            Self::SignalContext,
            Self::SignalTransactionKey,
        >,
        cancellation: &RuntimeWorldCancellationToken,
    ) -> OwnerExecutionOutcome;
}

pub(crate) trait RuntimeWorldProductPublicationService {
    fn publish(
        &self,
        ready: CompositePublicationReady,
        cell: &ProductBranchReferenceCell,
        late_cancellation: CompositeLateCancellationPosture,
        cost_counters: CompositePublicationCostCounters,
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

/// Shared internal seam for one-shot root bootstrap and owner close.
pub(crate) trait RuntimeWorldLifecycleService {
    fn bootstrap_root(&self, intent: RuntimeWorldBootstrapIntent) -> RuntimeWorldBootstrapOutcome;

    fn close(&self) -> Result<(), RuntimeWorldCloseDenial>;
}
