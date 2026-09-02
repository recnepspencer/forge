use crate::basis::AdmittedCompositeRuntimeWorldBasis;
use crate::branch::{
    ProductBranchObservation, RuntimeWorldBootstrapIntent, RuntimeWorldBootstrapOutcome,
    RuntimeWorldBranchAdmissionDenial, RuntimeWorldBranchRetirementDenial,
};
use crate::identity::ProductBranchIdentity;
use crate::publication::{
    CompositeComponentIntent, CompositeExecutionBorrow, LoweredOwnerComponentPlan,
    NoEffectCompositePublication, ProductBranchIntent, ReservedCompositePublicationAttempt,
    RuntimeWorldPublicationOutcome,
};
use crate::recovery::{ProductUnpublishedOwnerEffects, RecoveryContinuationContract};

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
    fn create_product_branch(
        &self,
        basis: AdmittedCompositeRuntimeWorldBasis,
        intent: ProductBranchIntent,
    ) -> Result<ProductBranchObservation, RuntimeWorldBranchAdmissionDenial>;

    fn retire_product_branch(
        &self,
        branch: ProductBranchIdentity,
    ) -> Result<(), RuntimeWorldBranchRetirementDenial>;
}

/// Shared internal seam for the serial owner execution pipeline.
pub(crate) trait RuntimeWorldPublicationService {
    fn prepare(
        &self,
        expected: ProductBranchObservation,
        intent: CompositeComponentIntent,
    ) -> Result<LoweredOwnerComponentPlan, NoEffectCompositePublication>;

    fn execute<Ctx, F>(
        &self,
        attempt: ReservedCompositePublicationAttempt,
        borrow: CompositeExecutionBorrow<'_, Ctx, F>,
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

    fn lifecycle_observation(&self) -> RuntimeWorldOwnerLifecycleObservation;
}
