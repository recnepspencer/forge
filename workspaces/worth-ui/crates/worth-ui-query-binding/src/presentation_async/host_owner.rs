use std::collections::{HashMap, HashSet};

use super::semantic_registry::WorthUiPresentationSemanticQueryObservation;
use worth_query::facade::runtime;
use worth_runtime_bridge::facade::{
    BridgeAsyncRequestTruthViewBasis, RelationalBridgeSnapshotIdentityParts, TruthBranchIdentity,
    TruthCommitIdentity, TruthSnapshotIdentity,
};

#[cfg(test)]
use super::WorthUiPresentationRequestBasis;
use super::{
    WorthUiPresentationAsyncObservation, WorthUiPresentationAsyncPosture,
    WorthUiPresentationAsyncRegistry, WorthUiPresentationRuntimeAdmission,
    WorthUiPresentationSemanticChange,
};

mod admission;
mod admission_recovery;
mod cancellation;
mod completion_semantic_changes;
mod correspondence;
mod installation;
mod pending_progress;
mod receipts;
mod rejection;
mod settlement;
mod stops;
mod superseded_observation;
mod superseded_settlement;
mod terminal_close;
mod transition_trace;
mod unresolved;

pub const WORTH_UI_PRESENTATION_PENDING_CAPACITY: usize = 64;

pub use correspondence::{
    WorthUiPresentationCancellationEffectsObservation,
    WorthUiPresentationCorrespondenceIssuanceDenial, WorthUiPresentationCorrespondenceIssuer,
    WorthUiPresentationEffectsIndeterminateObservation, WorthUiPresentationRuntimeCorrespondence,
    WorthUiPresentationSupersededPhysicalObservation, WorthUiPresentationValidatedCompletion,
};
pub use installation::{
    WorthUiPresentationAsyncHostCompletion, WorthUiPresentationAsyncHostPlan,
    WorthUiPresentationAsyncInstallation, WorthUiPresentationAsyncInstallationError,
    WorthUiPresentationQueryHostInstallationRequest,
};
pub use receipts::{
    WorthUiPresentationAdmissionRecovery, WorthUiPresentationCleanupRecovery,
    WorthUiPresentationIncompleteAdmission, WorthUiPresentationPendingReceipt,
    WorthUiPresentationPresentedReceipt, WorthUiPresentationRecoveryReceipt,
    WorthUiPresentationRecoveryRequiredReceipt, WorthUiPresentationUnresolvedReceipt,
};
pub use stops::{
    WorthUiPresentationAdmissionStop, WorthUiPresentationCleanupProgress,
    WorthUiPresentationRuntimeCleanupStop, WorthUiPresentationSettlementStop,
};
pub use terminal_close::{
    WorthUiPresentationAsyncCloseDenial, WorthUiPresentationAsyncCloseReceipt,
};
pub use transition_trace::{
    WorthUiPresentationTransitionKind, WorthUiPresentationTransitionObservation,
    WORTH_UI_PRESENTATION_TRANSITION_CAPACITY,
};

pub struct WorthUiPresentationAsyncOwner {
    correspondence_authority: std::sync::Arc<correspondence::PresentationCorrespondenceAuthority>,
    workspace: runtime::WorthQueryWorkspace,
    registry: WorthUiPresentationAsyncRegistry,
    next_truth_revision: u64,
    next_receipt_nonce: u64,
    pending: HashMap<PresentationAdmissionKey, PendingPresentationAdmission>,
    settling: HashMap<PresentationAdmissionKey, PendingPresentationAdmission>,
    superseded_pending: HashMap<PresentationAdmissionKey, PendingPresentationAdmission>,
    superseded_awaiting_completion:
        HashMap<(PresentationAdmissionKey, u64), WorthUiPresentationPresentedReceipt>,
    runtime_cleanups: HashMap<PresentationAdmissionKey, PendingRuntimeCleanup>,
    unresolved: HashMap<PresentationAdmissionKey, PendingPresentationAdmission>,
    terminal_closing: HashMap<PresentationAdmissionKey, PendingTerminalClose>,
    terminal_closed_resources: u64,
    retained: HashMap<
        super::semantic_transition::PresentationLineageKey,
        super::semantic_transition::RetainedPresentationSemanticState,
    >,
    current: HashMap<
        super::semantic_transition::PresentationLineageKey,
        (
            PresentationAdmissionKey,
            u64,
            WorthUiPresentationRuntimeAdmission,
        ),
    >,
    active_keys: HashSet<PresentationAdmissionKey>,
    transition_trace: Vec<WorthUiPresentationTransitionObservation>,
    transition_trace_overflowed: bool,
}

struct PendingRuntimeCleanup {
    nonce: u64,
    cleanup: super::runtime_bridge::WorthUiPresentationRuntimeCleanup,
}

struct PendingTerminalClose {
    admission: WorthUiPresentationRuntimeAdmission,
    semantic_retired: bool,
}

struct PendingPresentationAdmission {
    nonce: u64,
    lineage: super::semantic_transition::PresentationLineageKey,
    transition: super::semantic_transition::PresentationSemanticTransition,
    admission: WorthUiPresentationRuntimeAdmission,
    pending_publication_index: usize,
    pending_performed: Option<worth_signal::facade::adapters::InvalidationExecutionSummary>,
    pending_frontiers: Vec<WorthUiPresentationSemanticFrontierObservation>,
    supersession_query_admitted: bool,
    supersession_posture_observed: bool,
    supersession_semantic_retired: bool,
    predecessor_supersession_complete: bool,
    settlement: PresentationSettlementProgress,
    rejection: PresentationRejectionProgress,
    recovery_required: bool,
    superseding_pending_predecessor: bool,
    reconstructing_unresolved_predecessor: bool,
}

#[derive(Default)]
struct PresentationSettlementProgress {
    publication_index: usize,
    frontiers: Vec<WorthUiPresentationSemanticFrontierObservation>,
    completion_progress: Option<super::runtime_bridge::WorthUiPresentationCompletionProgress>,
    completion: Option<WorthUiPresentationAsyncObservation>,
    predecessor_observation: Option<WorthUiPresentationAsyncObservation>,
    predecessor_superseded: bool,
    predecessor_semantic_retired: bool,
    predecessor_query_closed: bool,
}

#[derive(Default)]
struct PresentationRejectionProgress {
    query_denied: bool,
    query_denial_observed: bool,
    semantic_retired: bool,
    query_closed: bool,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct PresentationAdmissionKey {
    attempt: worth_ui_host_contract::UiMountedPresentationAttemptIdentity,
    binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPresentationSemanticFrontierObservation {
    change: WorthUiPresentationSemanticChange,
    subscribers: Box<[super::WorthUiPresentationSemanticSubscriberIdentity]>,
    source_deliveries: u32,
    outcomes: Box<[worth_query::facade::domain::WorthQueryConditionalOutcomeClass]>,
    performed: Box<[worth_signal::facade::adapters::InvalidationExecutionSummary]>,
    scope_rejections: super::WorthUiPresentationScopeRejectionCounters,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPresentationConditionalOutcomeClass {
    ComputedChanged,
    ComputedRevertedClean,
    DependencyUnchanged,
    Suppressed,
    DeferredByCondition,
    DeferredTemporal,
    DeferredOnDemand,
}

impl WorthUiPresentationSemanticFrontierObservation {
    pub const fn change(&self) -> WorthUiPresentationSemanticChange {
        self.change
    }

    pub fn outcomes(&self) -> &[worth_query::facade::domain::WorthQueryConditionalOutcomeClass] {
        &self.outcomes
    }

    pub fn subscribers(&self) -> &[super::WorthUiPresentationSemanticSubscriberIdentity] {
        &self.subscribers
    }

    pub const fn source_deliveries(&self) -> u32 {
        self.source_deliveries
    }

    pub fn outcome_classes(
        &self,
    ) -> impl Iterator<Item = WorthUiPresentationConditionalOutcomeClass> + '_ {
        self.outcomes.iter().copied().map(|outcome| match outcome {
            worth_query::facade::domain::WorthQueryConditionalOutcomeClass::ComputedChanged => {
                WorthUiPresentationConditionalOutcomeClass::ComputedChanged
            }
            worth_query::facade::domain::WorthQueryConditionalOutcomeClass::ComputedRevertedClean => {
                WorthUiPresentationConditionalOutcomeClass::ComputedRevertedClean
            }
            worth_query::facade::domain::WorthQueryConditionalOutcomeClass::DependencyUnchanged => {
                WorthUiPresentationConditionalOutcomeClass::DependencyUnchanged
            }
            worth_query::facade::domain::WorthQueryConditionalOutcomeClass::Suppressed => {
                WorthUiPresentationConditionalOutcomeClass::Suppressed
            }
            worth_query::facade::domain::WorthQueryConditionalOutcomeClass::DeferredByCondition => {
                WorthUiPresentationConditionalOutcomeClass::DeferredByCondition
            }
            worth_query::facade::domain::WorthQueryConditionalOutcomeClass::DeferredTemporal => {
                WorthUiPresentationConditionalOutcomeClass::DeferredTemporal
            }
            worth_query::facade::domain::WorthQueryConditionalOutcomeClass::DeferredOnDemand => {
                WorthUiPresentationConditionalOutcomeClass::DeferredOnDemand
            }
        })
    }

    pub fn performed(&self) -> &[worth_signal::facade::adapters::InvalidationExecutionSummary] {
        &self.performed
    }

    pub const fn scope_rejections(&self) -> super::WorthUiPresentationScopeRejectionCounters {
        self.scope_rejections
    }
}

#[derive(Debug)]
pub enum WorthUiPresentationPendingAdmissionDenial {
    ForeignCorrespondenceAuthority,
    TruthRevisionExhausted,
    DuplicateAttemptBinding,
    PendingCapacityExhausted,
    IncompleteLineageAdmission,
    SettlingLineageAdmission,
    UnresolvedLineageAdmission,
    MissingSemanticBaseline,
    StalePredecessor,
    ForeignHostSurface,
    UnknownRemovedMechanic,
    UnknownReleasedPin,
    Runtime(WorthUiPresentationAdmissionStop),
    SemanticProgress(
        Box<WorthUiPresentationIncompleteAdmission>,
        WorthUiPresentationAdmissionStop,
    ),
    CleanupProgress(
        Box<WorthUiPresentationCleanupRecovery>,
        WorthUiPresentationCleanupProgress,
    ),
    MissingPerformedFrontier,
    NonPendingPosture,
}

#[derive(Debug)]
pub enum WorthUiPresentationSettlementDenial {
    InvalidPendingReceipt,
    ForeignPendingReceiptAuthority,
    ForeignCompletionAuthority,
    CompletionReceiptMismatch,
    Progress(WorthUiPresentationSettlementStop),
    SettlementAlreadyBegan,
    RuntimeCleanup(WorthUiPresentationRuntimeCleanupStop),
}

impl WorthUiPresentationAsyncOwner {
    fn record_transition(
        &mut self,
        kind: WorthUiPresentationTransitionKind,
        key: PresentationAdmissionKey,
    ) {
        if self.transition_trace.iter().any(|observation| {
            observation.kind() == kind
                && observation.attempt() == key.attempt
                && observation.binding() == key.binding
        }) {
            return;
        }
        if self.transition_trace.len() == WORTH_UI_PRESENTATION_TRANSITION_CAPACITY {
            self.transition_trace_overflowed = true;
            return;
        }
        self.transition_trace
            .push(WorthUiPresentationTransitionObservation::new(kind, key));
    }

    fn discard_pending_transition(&mut self, key: PresentationAdmissionKey) {
        self.transition_trace.retain(|observation| {
            observation.kind() != WorthUiPresentationTransitionKind::Pending
                || observation.attempt() != key.attempt
                || observation.binding() != key.binding
        });
    }

    pub fn observation(
        &self,
        receipt: &WorthUiPresentationPendingReceipt,
    ) -> Option<WorthUiPresentationAsyncObservation> {
        if !correspondence::is_correspondence_authority(
            &self.correspondence_authority,
            &receipt.authority,
        ) {
            return None;
        }
        let key = PresentationAdmissionKey {
            attempt: receipt.attempt,
            binding: receipt.binding,
        };
        self.pending
            .get(&key)
            .filter(|pending| pending.nonce == receipt.nonce)
            .map(|pending| &pending.admission)
            .or_else(|| {
                self.settling
                    .get(&key)
                    .filter(|pending| pending.nonce == receipt.nonce)
                    .map(|pending| &pending.admission)
            })
            .or_else(|| {
                self.superseded_pending
                    .get(&key)
                    .filter(|pending| pending.nonce == receipt.nonce)
                    .map(|pending| &pending.admission)
            })
            .or_else(|| {
                self.unresolved
                    .get(&key)
                    .filter(|pending| pending.nonce == receipt.nonce)
                    .map(|pending| &pending.admission)
            })
            .or_else(|| {
                self.current
                    .values()
                    .find(|(current_key, nonce, _)| *current_key == key && *nonce == receipt.nonce)
                    .map(|(_, _, admission)| admission)
            })
            .and_then(|admission| admission.observation(&self.workspace).ok())
    }

    pub fn recovery_observation(
        &self,
        receipt: &WorthUiPresentationIncompleteAdmission,
    ) -> Option<WorthUiPresentationAsyncObservation> {
        if !correspondence::is_correspondence_authority(
            &self.correspondence_authority,
            &receipt.authority,
        ) {
            return None;
        }
        let key = PresentationAdmissionKey {
            attempt: receipt.attempt,
            binding: receipt.binding,
        };
        self.pending
            .get(&key)
            .filter(|pending| pending.nonce == receipt.nonce)
            .and_then(|pending| pending.admission.observation(&self.workspace).ok())
    }
}

impl WorthUiPresentationPendingAdmissionDenial {
    pub fn into_recovery_receipt(self) -> Option<WorthUiPresentationAdmissionRecovery> {
        match self {
            Self::SemanticProgress(receipt, _) => {
                Some(WorthUiPresentationAdmissionRecovery::Incomplete(*receipt))
            }
            Self::CleanupProgress(receipt, _) => {
                Some(WorthUiPresentationAdmissionRecovery::Cleanup(*receipt))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
#[path = "host_owner_authority_tests.rs"]
mod authority_tests;
#[cfg(test)]
#[path = "host_owner_hostile_control_tests.rs"]
mod hostile_control_tests;
#[cfg(test)]
#[path = "host_owner_lifecycle_tests.rs"]
mod lifecycle_tests;
#[cfg(test)]
#[path = "host_owner_tests.rs"]
mod tests;
#[cfg(test)]
#[path = "host_owner_topology_tests.rs"]
mod topology_tests;
#[cfg(test)]
#[path = "host_owner_transition_trace_tests.rs"]
mod transition_trace_tests;
#[cfg(test)]
#[path = "host_owner_unresolved_tests.rs"]
mod unresolved_tests;
