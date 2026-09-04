use std::sync::Arc;

use crate::basis::AdmittedCompositeRuntimeWorldBasis;
use crate::branch::{ProductBranchObservation, ProductBranchReferenceSnapshot};
use crate::history::{
    CompositeHistoryCatalog, CompositeRuntimeWorldCommit, ReservedCompositeCommitCapacity,
};
use crate::identity::{
    CompositeCommitIdentity, CompositePublicationAttemptIdentity,
    ProductUnpublishedOwnerEffectsIdentity,
};
use crate::lifecycle::owner::RuntimeWorldOperationReservation;
use crate::lifecycle::RuntimeWorldInstant;
use crate::recovery::{
    ProductUnpublishedCause, ProductUnpublishedOwnerEffectSummary, ProductUnpublishedOwnerEffects,
    ReservedProductUnpublishedSlot,
};
use crate::retention::{ReservedComponentPinPairCapacity, RetentionObligationDenial};

use super::product_cas::CompositePublicationReady;
use super::{
    CompositeAttemptProgress, CompositeOwnerExecutionResults, ReservedCompositePublicationAttempt,
    ReservedPublicationAttemptParts,
};

#[path = "owner_execution/retention.rs"]
mod retention;

pub(crate) use retention::{
    retain_attempt_effects, RetainedAttemptInputs, RetainedOwnerEffectInputs,
};

pub(crate) const RETENTION_PENDING_LIVE_OBLIGATION_COUNT: usize = 3;

/// Owner effects have been settled into exact progress, but product
/// publication has not yet crossed its final compare-and-publish point.
pub struct OwnerExecutionSettlement {
    attempt: ReservedCompositePublicationAttempt,
    progress: CompositeAttemptProgress,
    successor_basis: Option<AdmittedCompositeRuntimeWorldBasis>,
}

impl std::fmt::Debug for OwnerExecutionSettlement {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("OwnerExecutionSettlement")
            .field("progress", &self.progress)
            .field("successor_basis", &self.successor_basis)
            .finish_non_exhaustive()
    }
}

impl OwnerExecutionSettlement {
    pub(crate) fn new(
        attempt: ReservedCompositePublicationAttempt,
        progress: CompositeAttemptProgress,
    ) -> Self {
        Self {
            attempt,
            progress,
            successor_basis: None,
        }
    }

    pub(crate) fn with_successor_basis(
        attempt: ReservedCompositePublicationAttempt,
        progress: CompositeAttemptProgress,
        successor_basis: AdmittedCompositeRuntimeWorldBasis,
    ) -> Self {
        Self {
            attempt,
            progress,
            successor_basis: Some(successor_basis),
        }
    }

    pub fn progress(&self) -> &CompositeAttemptProgress {
        &self.progress
    }

    pub(crate) fn successor_basis(&self) -> Option<&AdmittedCompositeRuntimeWorldBasis> {
        self.successor_basis.as_ref()
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        ReservedCompositePublicationAttempt,
        CompositeAttemptProgress,
    ) {
        (self.attempt, self.progress)
    }

    /// Retain an exact owner-progress image at an intermediate boundary. This
    /// path is used when a later owner or product-head check denies the
    /// attempt; it never performs the final product CAS.
    pub(crate) fn retain_with_cause(
        self,
        successor_basis: AdmittedCompositeRuntimeWorldBasis,
        cause: ProductUnpublishedCause,
        last_observed_head: Option<ProductBranchReferenceSnapshot>,
    ) -> ProductUnpublishedOwnerEffects {
        let (attempt, progress) = self.into_parts();
        let (progress, owner_results) = progress
            .into_recovery_results()
            .expect("owner-effect retention carries representable progress");
        retain_publication_effects(
            attempt.into_parts(),
            RetainedOwnerEffectInputs {
                progress,
                successor_basis,
                owner_results,
                cause,
                last_observed_head,
                destination: PUBLICATION_CREATES_NO_OCCURRENCE,
            },
            false,
        )
    }
}

/// A publication moves the head of a product branch that already exists, so it
/// reserves no branch occurrence and charges no custody. Every publication
/// retention terminal says so with this one name rather than a bare `None`
/// whose meaning would have to be re-derived at each site.
const PUBLICATION_CREATES_NO_OCCURRENCE: Option<(
    crate::identity::ProductBranchIdentity,
    crate::identity::ProductBranchIncarnation,
)> = None;

/// Retain a publication attempt's owner effects. Only a publication reaches
/// this wrapper, so the plan match it enforces is the publication plan.
fn retain_publication_effects(
    parts: ReservedPublicationAttemptParts,
    effects: RetainedOwnerEffectInputs,
    enforce_plan_match: bool,
) -> ProductUnpublishedOwnerEffects {
    let ReservedPublicationAttemptParts {
        identity,
        expected_head,
        plan,
        capacities,
        cancellation: _,
        deadline,
        counters: _,
    } = parts;
    if enforce_plan_match {
        assert!(
            effects.owner_results.matches_plan(&plan),
            "retained owner results must match the reserved component plan"
        );
    }
    retain_attempt_effects(
        RetainedAttemptInputs {
            attempt_identity: identity,
            expected_head,
            capacities,
            deadline,
        },
        effects,
    )
}

impl OwnerExecutionSettlement {
    /// Consume exact settled owner evidence into one successor commit and bind
    /// its reserved publication pins. A post-effect pin denial becomes the
    /// retained recovery terminal; it can never be reclassified as no-effect.
    pub(crate) fn ready(
        self,
        successor_basis: AdmittedCompositeRuntimeWorldBasis,
    ) -> Result<CompositePublicationReady, ProductUnpublishedOwnerEffects> {
        let cancellation_observed = self.attempt.cancellation_posture()
            == super::CompositeAttemptCancellationPosture::CancellationObserved;
        let (attempt, progress) = self.into_parts();
        let (progress, owner_results) = match progress.into_ready_results() {
            Ok(ready) => ready,
            Err(progress) => {
                let (progress, owner_results) = progress
                    .into_recovery_results()
                    .expect("only incomplete owner progress enters recovery here");
                return Err(retain_publication_effects(
                    attempt.into_parts(),
                    RetainedOwnerEffectInputs {
                        progress,
                        successor_basis,
                        owner_results,
                        cause: ProductUnpublishedCause::SettlementPending,
                        last_observed_head: None,
                        destination: PUBLICATION_CREATES_NO_OCCURRENCE,
                    },
                    true,
                ));
            }
        };
        if cancellation_observed {
            return Err(retain_publication_effects(
                attempt.into_parts(),
                RetainedOwnerEffectInputs {
                    progress,
                    successor_basis,
                    owner_results,
                    cause: ProductUnpublishedCause::CancellationAfterEffect,
                    last_observed_head: None,
                    destination: PUBLICATION_CREATES_NO_OCCURRENCE,
                },
                true,
            ));
        }
        publish_ready(
            attempt.into_parts(),
            progress,
            successor_basis,
            owner_results,
        )
    }
}

/// Bind the reserved publication pins to the settled successor commit. A pin
/// denial here is post-effect and becomes the retained recovery terminal.
fn publish_ready(
    parts: ReservedPublicationAttemptParts,
    progress: CompositeAttemptProgress,
    successor_basis: AdmittedCompositeRuntimeWorldBasis,
    owner_results: CompositeOwnerExecutionResults,
) -> Result<CompositePublicationReady, ProductUnpublishedOwnerEffects> {
    let ReservedPublicationAttemptParts {
        identity,
        expected_head,
        plan,
        capacities,
        cancellation,
        deadline,
        counters,
    } = parts;
    assert!(
        owner_results.matches_plan(&plan),
        "settled owner results must match the reserved component plan"
    );
    SettledPublication {
        identity,
        expected_head,
        cancellation,
        deadline,
        counters,
        progress,
        successor_basis,
        owner_results,
    }
    .into_ready(capacities)
}

/// One settled publication, separated from the bounded reservations it will
/// consume. Every terminal below is reached from this same evidence, so a
/// denied pin binding cannot describe a different publication than a
/// successful one.
struct SettledPublication {
    identity: CompositePublicationAttemptIdentity,
    expected_head: ProductBranchObservation,
    cancellation: super::CompositeAttemptCancellationPosture,
    deadline: Option<RuntimeWorldInstant>,
    counters: super::CompositePublicationCostCounters,
    progress: CompositeAttemptProgress,
    successor_basis: AdmittedCompositeRuntimeWorldBasis,
    owner_results: CompositeOwnerExecutionResults,
}

/// The reservations a denied pin binding still owns. They are named as one
/// bundle so the retention-pending terminal cannot silently drop any of them.
struct RetentionPendingResources {
    product_unpublished_identity: ProductUnpublishedOwnerEffectsIdentity,
    reserved_commit_capacity: ReservedCompositeCommitCapacity,
    reserved_recovery_slot: ReservedProductUnpublishedSlot,
    history: CompositeHistoryCatalog,
    operation: RuntimeWorldOperationReservation,
}

impl SettledPublication {
    fn into_ready(
        self,
        capacities: super::ReservedAttemptCapacities,
    ) -> Result<CompositePublicationReady, ProductUnpublishedOwnerEffects> {
        let (
            reserved_commit_identity,
            product_unpublished_identity,
            reserved_commit_capacity,
            reserved_recovery_slot,
            reserved_component_pin_pair,
            reserved_publication_capacity,
            history,
            operation,
        ) = capacities.into_parts();
        let commit = self.successor_commit(reserved_commit_identity);
        let publication_retention =
            match reserved_component_pin_pair.bind_publication(commit.basis()) {
                Ok(retention) => retention,
                Err((capacity, denial)) => {
                    let resources = RetentionPendingResources {
                        product_unpublished_identity,
                        reserved_commit_capacity,
                        reserved_recovery_slot,
                        history,
                        operation,
                    };
                    return Err(self.into_retention_pending(resources, &commit, capacity, denial));
                }
            };
        Ok(CompositePublicationReady::new(
            super::product_cas::CompositePublicationReadyInputs {
                identity: self.identity,
                expected_head: self.expected_head,
                commit,
                owner_results: self.owner_results,
                progress: self.progress,
                product_unpublished_identity,
                reserved_commit_capacity,
                reserved_recovery_slot,
                reserved_publication_capacity,
                history,
                operation,
                publication_retention,
                cancellation: self.cancellation,
                deadline: self.deadline,
                counters: self.counters,
            },
        ))
    }

    /// The one successor occurrence this publication would install.
    fn successor_commit(
        &self,
        reserved_commit_identity: CompositeCommitIdentity,
    ) -> Arc<CompositeRuntimeWorldCommit> {
        Arc::new(
            CompositeRuntimeWorldCommit::from_ordinary_publication(
                reserved_commit_identity,
                self.expected_head.snapshot().commit(),
                self.successor_basis.clone(),
                self.identity.clone(),
                &self.owner_results,
                None,
            )
            .expect("owner-issued results and admitted successor form the reserved commit"),
        )
    }

    /// A pin denial after the owner effects are real. The successor occurrence
    /// is installed and protected exactly as a performed publication would
    /// install it; only the product reference is left where it was.
    fn into_retention_pending(
        self,
        resources: RetentionPendingResources,
        commit: &Arc<CompositeRuntimeWorldCommit>,
        capacity: ReservedComponentPinPairCapacity,
        denial: RetentionObligationDenial,
    ) -> ProductUnpublishedOwnerEffects {
        let RetentionPendingResources {
            product_unpublished_identity,
            reserved_commit_capacity,
            reserved_recovery_slot,
            history,
            mut operation,
        } = resources;
        let successor_history =
            retention::install_retained_commit(history, reserved_commit_capacity, commit);
        operation
            .begin_recovery()
            .expect("a publishing attempt enters retained recovery");
        let summary = ProductUnpublishedOwnerEffectSummary::from_progress(
            &self.progress,
            RETENTION_PENDING_LIVE_OBLIGATION_COUNT,
            ProductUnpublishedOwnerEffects::metadata_charge_hint(),
        );
        ProductUnpublishedOwnerEffects::new_reacquisition_pending(
            crate::recovery::RetainedAttemptFacts {
                identity: product_unpublished_identity,
                attempt_identity: self.identity,
                expected_head: self.expected_head,
                last_observed_head: None,
                progress: self.progress,
                owner_results: self.owner_results,
                destination: PUBLICATION_CREATES_NO_OCCURRENCE,
            },
            crate::recovery::InstalledSuccessorEvidence {
                basis: self.successor_basis,
                history_protection: successor_history,
            },
            crate::recovery::PendingRetentionCustody { capacity, denial },
            crate::recovery::RetainedRecordCharges {
                recovery_slot: reserved_recovery_slot,
                summary,
                // Retention could not reacquire its pins after a real owner
                // effect, which is the owner-issued authority this attempt
                // depended on going away underneath it.
                cause: ProductUnpublishedCause::OwnerLost,
                deadline: self.deadline,
            },
        )
    }
}
