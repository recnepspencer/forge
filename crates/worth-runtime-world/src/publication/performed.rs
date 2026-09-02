use std::sync::Arc;

use worth_proof::AuthorityWitness;
use worth_relational::facade::mvcc::PerformedRelationalCommit;
use worth_signal::facade::branch::{SignalBranchAdvanceOutcome, SignalBranchForkOutcome};

use crate::branch::ProductBranchObservation;
use crate::history::{CompositeComponentChangePosture, CompositeRuntimeWorldCommit};
use crate::identity::CompositePublicationAttemptIdentity;
use crate::retention::RetentionTransferReceipt;

worth_proof::authority_marker!(pub(crate) CompositePublicationAuthorityMarker);

/// Exact result of the Relational leg. The retained variant is evidence that
/// no Relational owner movement was requested, not an absent or guessed
/// result.
#[derive(Debug)]
pub struct CompositeRelationalOwnerResult {
    result: CompositeRelationalOwnerResultKind,
}

#[derive(Debug)]
enum CompositeRelationalOwnerResultKind {
    RetainedExact,
    Published(PerformedRelationalCommit),
}

/// Exact result of the Signal leg. A changed Signal component must carry the
/// owner-issued advance/fork result.
#[derive(Debug)]
pub struct CompositeSignalOwnerResult {
    result: CompositeSignalOwnerResultKind,
}

#[derive(Debug)]
enum CompositeSignalOwnerResultKind {
    RetainedExact,
    Advanced(SignalBranchAdvanceOutcome),
    Forked(SignalBranchForkOutcome),
}

/// The two owner results carried by one performed publication. They are
/// created only from the corresponding owner progress and cannot be mixed
/// independently with a commit posture.
#[derive(Debug)]
pub struct CompositeOwnerExecutionResults {
    relational: CompositeRelationalOwnerResult,
    signal: CompositeSignalOwnerResult,
}

impl CompositeOwnerExecutionResults {
    pub(crate) fn retained() -> Self {
        Self {
            relational: CompositeRelationalOwnerResult {
                result: CompositeRelationalOwnerResultKind::RetainedExact,
            },
            signal: CompositeSignalOwnerResult {
                result: CompositeSignalOwnerResultKind::RetainedExact,
            },
        }
    }

    pub(crate) fn relational_published(
        performed: PerformedRelationalCommit,
        signal: CompositeSignalOwnerResult,
    ) -> Self {
        Self {
            relational: CompositeRelationalOwnerResult {
                result: CompositeRelationalOwnerResultKind::Published(performed),
            },
            signal,
        }
    }

    pub(crate) fn signal_advanced(
        relational: CompositeRelationalOwnerResult,
        advanced: SignalBranchAdvanceOutcome,
    ) -> Self {
        Self {
            relational,
            signal: CompositeSignalOwnerResult {
                result: CompositeSignalOwnerResultKind::Advanced(advanced),
            },
        }
    }

    pub(crate) fn signal_forked(
        relational: CompositeRelationalOwnerResult,
        forked: SignalBranchForkOutcome,
    ) -> Self {
        Self {
            relational,
            signal: CompositeSignalOwnerResult {
                result: CompositeSignalOwnerResultKind::Forked(forked),
            },
        }
    }

    pub fn relational_posture(&self) -> CompositeComponentChangePosture {
        match self.relational.result {
            CompositeRelationalOwnerResultKind::RetainedExact => {
                CompositeComponentChangePosture::RetainExact
            }
            CompositeRelationalOwnerResultKind::Published(_) => {
                CompositeComponentChangePosture::Published
            }
        }
    }

    pub fn signal_posture(&self) -> CompositeComponentChangePosture {
        match self.signal.result {
            CompositeSignalOwnerResultKind::RetainedExact => {
                CompositeComponentChangePosture::RetainExact
            }
            CompositeSignalOwnerResultKind::Advanced(_)
            | CompositeSignalOwnerResultKind::Forked(_) => {
                CompositeComponentChangePosture::Published
            }
        }
    }

    pub(crate) fn relational_publication_identity(
        &self,
    ) -> Option<worth_relational::facade::history::RelationalCommitIdentity> {
        match &self.relational.result {
            CompositeRelationalOwnerResultKind::RetainedExact => None,
            CompositeRelationalOwnerResultKind::Published(result) => Some(result.commit_identity()),
        }
    }

    pub(crate) fn relational_publication_basis_identity(
        &self,
    ) -> Option<&worth_relational::facade::branch::RelationalBranchBasisAdmissionIdentity> {
        match &self.relational.result {
            CompositeRelationalOwnerResultKind::RetainedExact => None,
            CompositeRelationalOwnerResultKind::Published(result) => {
                Some(result.next_basis().admission_identity())
            }
        }
    }

    pub(crate) fn signal_publication_identity(
        &self,
    ) -> Option<crate::history::CompositeSignalPublicationIdentity> {
        match &self.signal.result {
            CompositeSignalOwnerResultKind::RetainedExact => None,
            CompositeSignalOwnerResultKind::Advanced(result) => Some(
                crate::history::CompositeSignalPublicationIdentity::Advanced(
                    result.advanced_basis().admission_identity().clone(),
                ),
            ),
            CompositeSignalOwnerResultKind::Forked(result) => {
                Some(crate::history::CompositeSignalPublicationIdentity::Forked(
                    result.created_basis().admission_identity().clone(),
                ))
            }
        }
    }
}

/// Structural counters frozen for the publication handoff. They are a
/// projection, not a substitute for owner evidence or retention authority.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CompositePublicationCostCounters {
    relational_owner_contacts: u64,
    signal_owner_contacts: u64,
    expected_head_rechecks: u64,
    unique_pin_hits: u64,
    unique_pin_acquisitions: u64,
    unique_pin_releases: u64,
    history_slots_reserved: u64,
    history_slots_installed: u64,
    product_cell_touches: u64,
    cas_attempts: u64,
    cas_wins: u64,
    cas_losses: u64,
    cancellation_observations: u64,
    retained_partial_creations: u64,
    retained_partial_cleanups: u64,
}

impl CompositePublicationCostCounters {
    pub(crate) const fn zero() -> Self {
        Self {
            relational_owner_contacts: 0,
            signal_owner_contacts: 0,
            expected_head_rechecks: 0,
            unique_pin_hits: 0,
            unique_pin_acquisitions: 0,
            unique_pin_releases: 0,
            history_slots_reserved: 0,
            history_slots_installed: 0,
            product_cell_touches: 0,
            cas_attempts: 0,
            cas_wins: 0,
            cas_losses: 0,
            cancellation_observations: 0,
            retained_partial_creations: 0,
            retained_partial_cleanups: 0,
        }
    }

    pub const fn relational_owner_contacts(self) -> u64 {
        self.relational_owner_contacts
    }

    pub const fn signal_owner_contacts(self) -> u64 {
        self.signal_owner_contacts
    }

    pub const fn expected_head_rechecks(self) -> u64 {
        self.expected_head_rechecks
    }

    pub const fn unique_pin_hits(self) -> u64 {
        self.unique_pin_hits
    }

    pub const fn unique_pin_acquisitions(self) -> u64 {
        self.unique_pin_acquisitions
    }

    pub const fn unique_pin_releases(self) -> u64 {
        self.unique_pin_releases
    }

    pub const fn history_slots_reserved(self) -> u64 {
        self.history_slots_reserved
    }

    pub const fn history_slots_installed(self) -> u64 {
        self.history_slots_installed
    }

    pub const fn product_cell_touches(self) -> u64 {
        self.product_cell_touches
    }

    pub const fn cas_attempts(self) -> u64 {
        self.cas_attempts
    }

    pub const fn cas_wins(self) -> u64 {
        self.cas_wins
    }

    pub const fn cas_losses(self) -> u64 {
        self.cas_losses
    }

    pub const fn cancellation_observations(self) -> u64 {
        self.cancellation_observations
    }

    pub const fn retained_partial_creations(self) -> u64 {
        self.retained_partial_creations
    }

    pub const fn retained_partial_cleanups(self) -> u64 {
        self.retained_partial_cleanups
    }
}

/// Cancellation observed after the branch reference moved is retained as
/// evidence; it cannot turn a performed movement back into no-effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompositeLateCancellationPosture {
    NotRequested,
    RequestedBeforeProductMovement,
    RequestedAfterProductMovement,
}

/// Linear proof that one immutable composite commit won the exact product
/// compare-and-publish transition.
#[must_use = "a performed publication must be handed to the product owner"]
pub struct PerformedCompositePublication {
    old_product_head: ProductBranchObservation,
    new_product_head: ProductBranchObservation,
    commit: Arc<CompositeRuntimeWorldCommit>,
    attempt_identity: CompositePublicationAttemptIdentity,
    component_results: CompositeOwnerExecutionResults,
    late_cancellation: CompositeLateCancellationPosture,
    retention_transfer: RetentionTransferReceipt,
    cost_counters: CompositePublicationCostCounters,
    _authority: AuthorityWitness<CompositePublicationAuthorityMarker>,
}

impl std::fmt::Debug for PerformedCompositePublication {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PerformedCompositePublication")
            .field("commit", &self.commit.identity())
            .field("old_product_head", &self.old_product_head)
            .field("new_product_head", &self.new_product_head)
            .field("attempt_identity", &self.attempt_identity)
            .field("component_results", &self.component_results)
            .field("late_cancellation", &self.late_cancellation)
            .field("cost_counters", &self.cost_counters)
            .finish_non_exhaustive()
    }
}

impl PerformedCompositePublication {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn owner_issued(
        old_product_head: ProductBranchObservation,
        new_product_head: ProductBranchObservation,
        commit: Arc<CompositeRuntimeWorldCommit>,
        attempt_identity: CompositePublicationAttemptIdentity,
        component_results: CompositeOwnerExecutionResults,
        late_cancellation: CompositeLateCancellationPosture,
        retention_transfer: RetentionTransferReceipt,
        cost_counters: CompositePublicationCostCounters,
    ) -> Self {
        Self {
            old_product_head,
            new_product_head,
            commit,
            attempt_identity,
            component_results,
            late_cancellation,
            retention_transfer,
            cost_counters,
            _authority: AuthorityWitness::from_authority_marker(
                CompositePublicationAuthorityMarker::seal(),
            ),
        }
    }

    pub fn old_product_head(&self) -> &ProductBranchObservation {
        &self.old_product_head
    }

    pub fn new_product_head(&self) -> &ProductBranchObservation {
        &self.new_product_head
    }

    pub fn commit(&self) -> &CompositeRuntimeWorldCommit {
        &self.commit
    }

    pub fn product_head(&self) -> &ProductBranchObservation {
        self.new_product_head()
    }

    pub fn attempt_identity(&self) -> &CompositePublicationAttemptIdentity {
        &self.attempt_identity
    }

    pub fn component_results(&self) -> &CompositeOwnerExecutionResults {
        &self.component_results
    }

    pub const fn late_cancellation(&self) -> CompositeLateCancellationPosture {
        self.late_cancellation
    }

    pub fn cost_counters(&self) -> CompositePublicationCostCounters {
        self.cost_counters
    }

    pub(crate) fn retention_transfer(&self) -> &RetentionTransferReceipt {
        &self.retention_transfer
    }
}
