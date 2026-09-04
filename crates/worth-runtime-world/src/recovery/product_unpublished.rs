use std::sync::Arc;

use crate::basis::AdmittedCompositeRuntimeWorldBasis;
use crate::branch::{ProductBranchObservation, ProductBranchReferenceSnapshot};
use crate::history::ProductUnpublishedHistoryProtectionObligation;
use crate::identity::{
    CompositePublicationAttemptIdentity, ProductBranchIdentity, ProductBranchIncarnation,
    ProductUnpublishedOwnerEffectsIdentity,
};
use crate::lifecycle::RuntimeWorldInstant;
use crate::publication::{CompositeAttemptProgress, CompositeOwnerExecutionResults};
use crate::retention::{
    ReservedComponentPinPairCapacity, RetainedPartialRetentionObligation, RetentionObligationDenial,
};

use super::{ProductUnpublishedNextAction, ReservedProductUnpublishedSlot};

#[path = "product_unpublished/custody.rs"]
mod custody;

#[path = "product_unpublished/handle.rs"]
mod handle;
pub use handle::ProductUnpublishedRecoveryHandle;

#[path = "product_unpublished/record_inputs.rs"]
mod record_inputs;
pub(crate) use record_inputs::{
    InstalledSuccessorEvidence, PendingRetentionCustody, RetainedAttemptFacts,
    RetainedRecordCharges, RetainedSuccessorEvidence,
};

#[path = "product_unpublished/actions.rs"]
mod actions;
pub(crate) use actions::next_actions_for_progress;
pub(crate) use actions::RetainedNextActions;

/// Why at least one owner effect survived without a product-reference move.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProductUnpublishedCause {
    SiblingOwnerDenied,
    SettlementPending,
    CancellationAfterEffect,
    DeadlineAfterEffect,
    StaleProductHead,
    OwnerLost,
    ProductPublicationLost,
}

/// Whether recovery already owns exact successor pins or retains the reserved
/// pair capacity needed to retry an owner-denied acquisition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProductUnpublishedRetentionPosture {
    RetainedExact,
    ReacquisitionPending,
}

enum ProductUnpublishedRetentionCustody {
    Retained(RetainedPartialRetentionObligation),
    Pending {
        capacity: ReservedComponentPinPairCapacity,
        denial: RetentionObligationDenial,
    },
}

impl std::fmt::Debug for ProductUnpublishedRetentionCustody {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Retained(obligation) => {
                formatter.debug_tuple("Retained").field(obligation).finish()
            }
            Self::Pending { denial, .. } => formatter
                .debug_struct("Pending")
                .field("denial", denial)
                .finish_non_exhaustive(),
        }
    }
}

/// Catalog-owned recovery custody. The public returned value below is only a
/// view over this allocation, so dropping the caller capability cannot drop
/// the component pins, history protection, or deferred owner route.
#[derive(Debug)]
pub(crate) struct ProductUnpublishedOwnerEffectsRecord {
    identity: ProductUnpublishedOwnerEffectsIdentity,
    attempt_identity: CompositePublicationAttemptIdentity,
    expected_head: ProductBranchObservation,
    last_observed_head: Option<ProductBranchReferenceSnapshot>,
    progress: CompositeAttemptProgress,
    successor_basis: Option<AdmittedCompositeRuntimeWorldBasis>,
    component_results: CompositeOwnerExecutionResults,
    retention: ProductUnpublishedRetentionCustody,
    /// History protection over an installed successor occurrence, held only by
    /// a record whose attempt actually installed one. A pre-movement loser
    /// releases its reserved slot instead of spending it on a commit no head
    /// names, so its successor is evidence in the basis alone.
    successor_history_protection: Option<ProductUnpublishedHistoryProtectionObligation>,
    /// The exact product-branch occurrence whose creation charged this
    /// attempt's custody, when the attempt was a creation at all. A publication
    /// moves an existing head and creates no occurrence, so it carries `None`.
    /// The pair is the custody key: the name-keyed identity outlives
    /// retirement, so only identity plus incarnation names the occurrence whose
    /// component branches this record is answerable for.
    destination: Option<(ProductBranchIdentity, ProductBranchIncarnation)>,
    catalog_affinity: usize,
    live_obligations: usize,
    cause: ProductUnpublishedCause,
    next_actions: RetainedNextActions,
    deadline: Option<RuntimeWorldInstant>,
    age_ticks: u64,
    owner_effect_count: usize,
    metadata_bytes: usize,
}

/// Retained record for owner-local effects that were not represented by a
/// product-reference movement. It is not a commit, rollback token, or replay
/// artifact.
#[must_use = "product-unpublished owner effects must remain retained or be explicitly cleaned up"]
pub struct ProductUnpublishedOwnerEffects {
    record: Arc<ProductUnpublishedOwnerEffectsRecord>,
}

impl std::fmt::Debug for ProductUnpublishedOwnerEffects {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ProductUnpublishedOwnerEffects")
            .field("identity", &self.record.identity)
            .field("attempt_identity", &self.record.attempt_identity)
            .field("cause", &self.record.cause)
            .field("next_actions", &self.record.next_actions)
            .field("live_obligations", &self.record.live_obligations)
            .field("owner_effect_count", &self.record.owner_effect_count)
            .field("metadata_bytes", &self.record.metadata_bytes)
            .finish_non_exhaustive()
    }
}

impl ProductUnpublishedOwnerEffects {
    /// Exact reservation charge for the inline retained-record representation.
    /// Actions are stored in a fixed bounded array, so no allocator capacity or
    /// lower-bound vector hint can undercharge an installed record.
    pub(crate) const fn metadata_charge_hint() -> usize {
        std::mem::size_of::<ProductUnpublishedOwnerEffectsRecord>()
    }

    pub(crate) fn from_catalog_record(record: Arc<ProductUnpublishedOwnerEffectsRecord>) -> Self {
        Self { record }
    }

    /// The retained posture: this attempt owns its component pins outright.
    pub(crate) fn new_retained(
        facts: RetainedAttemptFacts,
        successor: RetainedSuccessorEvidence,
        retention_obligation: RetainedPartialRetentionObligation,
        charges: RetainedRecordCharges,
    ) -> Self {
        Self::install_new_record(
            facts,
            successor,
            ProductUnpublishedRetentionCustody::Retained(retention_obligation),
            charges,
        )
    }

    /// The reacquisition-pending posture: the owner effects are just as real,
    /// but the component pins could not be reacquired, so the record carries
    /// the reserved capacity and the denial that names why.
    pub(crate) fn new_reacquisition_pending(
        facts: RetainedAttemptFacts,
        successor: InstalledSuccessorEvidence,
        retention: PendingRetentionCustody,
        charges: RetainedRecordCharges,
    ) -> Self {
        let PendingRetentionCustody { capacity, denial } = retention;
        Self::install_new_record(
            facts,
            successor.into(),
            ProductUnpublishedRetentionCustody::Pending { capacity, denial },
            charges,
        )
    }

    /// Build and install the record both postures produce. The next actions
    /// are derived here from the progress and the cause rather than supplied,
    /// so no caller can install a record whose advertised actions disagree
    /// with the evidence it carries.
    fn install_new_record(
        facts: RetainedAttemptFacts,
        successor: RetainedSuccessorEvidence,
        retention: ProductUnpublishedRetentionCustody,
        charges: RetainedRecordCharges,
    ) -> Self {
        let RetainedAttemptFacts {
            identity,
            attempt_identity,
            expected_head,
            last_observed_head,
            progress,
            owner_results,
            destination,
        } = facts;
        let RetainedRecordCharges {
            recovery_slot,
            summary,
            cause,
            deadline,
        } = charges;
        let catalog_affinity = recovery_slot.catalog_affinity();
        let next_actions = next_actions_for_progress(&progress, cause);
        let mut record = Arc::new(ProductUnpublishedOwnerEffectsRecord {
            identity: identity.clone(),
            attempt_identity,
            expected_head,
            last_observed_head,
            progress,
            successor_basis: successor.basis,
            component_results: owner_results,
            retention,
            successor_history_protection: successor.history_protection,
            destination,
            catalog_affinity,
            live_obligations: summary.live_obligation_count,
            cause,
            next_actions: RetainedNextActions::from_vec(next_actions),
            deadline,
            age_ticks: 0,
            owner_effect_count: summary.owner_effect_count,
            metadata_bytes: Self::metadata_charge_hint(),
        });
        finalize_metadata_charge(&mut record);
        install_record(recovery_slot, identity, Arc::clone(&record));
        Self { record }
    }

    pub fn identity(&self) -> &ProductUnpublishedOwnerEffectsIdentity {
        &self.record.identity
    }

    pub fn attempt_identity(&self) -> &CompositePublicationAttemptIdentity {
        &self.record.attempt_identity
    }

    pub fn expected_head(&self) -> &ProductBranchObservation {
        &self.record.expected_head
    }

    pub fn last_observed_head(&self) -> Option<&ProductBranchReferenceSnapshot> {
        self.record.last_observed_head.as_ref()
    }

    pub fn progress(&self) -> &CompositeAttemptProgress {
        &self.record.progress
    }

    pub fn successor_basis(&self) -> Option<&AdmittedCompositeRuntimeWorldBasis> {
        self.record.successor_basis.as_ref()
    }

    pub fn component_results(&self) -> &CompositeOwnerExecutionResults {
        &self.record.component_results
    }

    /// Exact installed successor occurrence retained by this recovery record,
    /// when the attempt installed one at all. `None` is a record whose attempt
    /// lost before the product reference moved: it released its history slot
    /// and names its successor by basis alone. The identity is evidence only;
    /// it grants no History or publication authority.
    pub fn successor_commit(&self) -> Option<&crate::identity::CompositeCommitIdentity> {
        self.record
            .successor_history_protection
            .as_ref()
            .map(ProductUnpublishedHistoryProtectionObligation::commit_identity)
    }

    /// The product-branch occurrence this record's owner effects were created
    /// for, when the attempt created one. It names the custody a cleanup must
    /// drain; it is not authority to install or retire that branch.
    pub fn destination_branch(&self) -> Option<(&ProductBranchIdentity, ProductBranchIncarnation)> {
        self.record
            .destination
            .as_ref()
            .map(|(branch, incarnation)| (branch, *incarnation))
    }

    pub fn retention_posture(&self) -> ProductUnpublishedRetentionPosture {
        match &self.record.retention {
            ProductUnpublishedRetentionCustody::Retained(_) => {
                ProductUnpublishedRetentionPosture::RetainedExact
            }
            ProductUnpublishedRetentionCustody::Pending { .. } => {
                ProductUnpublishedRetentionPosture::ReacquisitionPending
            }
        }
    }

    pub(crate) fn retention_obligation(&self) -> Option<&RetainedPartialRetentionObligation> {
        match &self.record.retention {
            ProductUnpublishedRetentionCustody::Retained(obligation) => Some(obligation),
            ProductUnpublishedRetentionCustody::Pending { .. } => None,
        }
    }

    pub fn live_obligation_count(&self) -> usize {
        self.record.live_obligations
    }

    pub fn cause(&self) -> ProductUnpublishedCause {
        self.record.cause
    }

    pub fn next_actions(&self) -> &[ProductUnpublishedNextAction] {
        self.record.next_actions.as_slice()
    }

    pub fn deadline(&self) -> Option<RuntimeWorldInstant> {
        self.record.deadline
    }

    pub fn age_ticks(&self) -> u64 {
        self.record.age_ticks
    }

    pub fn owner_effect_count(&self) -> usize {
        self.record.owner_effect_count
    }

    pub fn metadata_bytes(&self) -> usize {
        self.record.metadata_bytes
    }

    pub fn recovery_handle(&self) -> ProductUnpublishedRecoveryHandle {
        ProductUnpublishedRecoveryHandle::new(
            self.record.identity.clone(),
            self.record.catalog_affinity,
        )
    }
}

fn finalize_metadata_charge(record: &mut Arc<ProductUnpublishedOwnerEffectsRecord>) {
    let record = Arc::get_mut(record).expect("new recovery record is uniquely owned");
    record.metadata_bytes = record.derived_metadata_bytes();
}

fn install_record(
    recovery_slot: ReservedProductUnpublishedSlot,
    identity: ProductUnpublishedOwnerEffectsIdentity,
    record: Arc<ProductUnpublishedOwnerEffectsRecord>,
) {
    if let Err((slot, denial)) = recovery_slot.install_record(identity, record) {
        drop(slot);
        panic!("reserved recovery capacity could not install its record: {denial:?}");
    }
}
