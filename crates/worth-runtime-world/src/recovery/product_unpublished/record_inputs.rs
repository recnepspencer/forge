use crate::basis::AdmittedCompositeRuntimeWorldBasis;
use crate::branch::{ProductBranchObservation, ProductBranchReferenceSnapshot};
use crate::history::ProductUnpublishedHistoryProtectionObligation;
use crate::identity::{
    CompositePublicationAttemptIdentity, ProductBranchIdentity, ProductBranchIncarnation,
    ProductUnpublishedOwnerEffectsIdentity,
};
use crate::lifecycle::RuntimeWorldInstant;
use crate::publication::{CompositeAttemptProgress, CompositeOwnerExecutionResults};
use crate::retention::{ReservedComponentPinPairCapacity, RetentionObligationDenial};

use super::super::{ProductUnpublishedOwnerEffectSummary, ReservedProductUnpublishedSlot};
use super::ProductUnpublishedCause;

/// What a retained record knows about the attempt that produced it, before any
/// posture is chosen. Both posture constructors take it whole: an identity
/// without its attempt, or a progress without the owner results it was read
/// from, would describe an attempt that never ran, so no caller may name a
/// subset of it.
pub(crate) struct RetainedAttemptFacts {
    pub(crate) identity: ProductUnpublishedOwnerEffectsIdentity,
    pub(crate) attempt_identity: CompositePublicationAttemptIdentity,
    pub(crate) expected_head: ProductBranchObservation,
    pub(crate) last_observed_head: Option<ProductBranchReferenceSnapshot>,
    pub(crate) progress: CompositeAttemptProgress,
    pub(crate) owner_results: CompositeOwnerExecutionResults,
    /// The product-branch occurrence whose creation charged this attempt's
    /// custody, when the attempt created one at all.
    pub(crate) destination: Option<(ProductBranchIdentity, ProductBranchIncarnation)>,
}

/// What a retained record is charged for and answerable to: the catalog slot
/// it installs into, the obligation counts it declares, why it exists, and
/// when it expires. The four travel together because the record's own
/// bookkeeping is derived from all of them at once and is wrong if any is
/// chosen independently of the rest.
pub(crate) struct RetainedRecordCharges {
    pub(crate) recovery_slot: ReservedProductUnpublishedSlot,
    pub(crate) summary: ProductUnpublishedOwnerEffectSummary,
    pub(crate) cause: ProductUnpublishedCause,
    pub(crate) deadline: Option<RuntimeWorldInstant>,
}

/// The successor a retained record still names. Either half may be absent: an
/// attempt denied before it admitted a basis names no basis, and an attempt
/// that lost the product CAS before materializing released its history slot
/// and so installed no commit to protect.
pub(crate) struct RetainedSuccessorEvidence {
    pub(crate) basis: Option<AdmittedCompositeRuntimeWorldBasis>,
    pub(crate) history_protection: Option<ProductUnpublishedHistoryProtectionObligation>,
}

/// The successor a reacquisition-pending record always holds. That posture is
/// only reachable after the attempt installed its commit into the history slot
/// it reserved, so neither half is optional here; the type carries the
/// guarantee instead of each caller re-deriving it.
pub(crate) struct InstalledSuccessorEvidence {
    pub(crate) basis: AdmittedCompositeRuntimeWorldBasis,
    pub(crate) history_protection: ProductUnpublishedHistoryProtectionObligation,
}

impl From<InstalledSuccessorEvidence> for RetainedSuccessorEvidence {
    fn from(installed: InstalledSuccessorEvidence) -> Self {
        Self {
            basis: Some(installed.basis),
            history_protection: Some(installed.history_protection),
        }
    }
}

/// The component-pin custody a reacquisition-pending record holds in place of
/// an owned pin pair: the capacity still reserved for the retry, and the
/// denial naming why the pins could not be reacquired. Neither is meaningful
/// without the other, since the capacity exists only to answer that denial.
pub(crate) struct PendingRetentionCustody {
    pub(crate) capacity: ReservedComponentPinPairCapacity,
    pub(crate) denial: RetentionObligationDenial,
}
