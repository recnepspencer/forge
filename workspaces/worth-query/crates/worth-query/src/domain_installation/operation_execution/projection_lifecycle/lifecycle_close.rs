use crate::basis_lifecycle::BasisOperationLane;
use crate::ordinary::live::{WorthQueryManagedLiveCloseOutcome, WorthQueryManagedLiveCloseReceipt};
use crate::runtime::WorthQueryWorkspace;
use worth_proof::{Artifact, NoAssumptionBasis, NoProofs, PhaseMarker};

use super::operational_owner::WorthQueryOperationalProjection;
use super::states::WorthQueryProjectionLifecycleEvidence;
use super::WorthQueryLiveProjectionReceipt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryProjectionLifecycleCloseCause {
    Cancellation,
    Disposal,
    Replacement,
    Rebind,
    ReplacementRollback,
    RebindRollback,
}

impl WorthQueryProjectionLifecycleCloseCause {
    fn identity_name(self) -> &'static str {
        match self {
            Self::Cancellation => "cancellation",
            Self::Disposal => "disposal",
            Self::Replacement => "replacement",
            Self::Rebind => "rebind",
            Self::ReplacementRollback => "replacement_rollback",
            Self::RebindRollback => "rebind_rollback",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryProjectionLifecycleTransitionCounters {
    pub close_attempts: usize,
    pub close_completions: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryProjectionLifecycleCloseReceipt {
    cause: WorthQueryProjectionLifecycleCloseCause,
    predecessor_identity: String,
    ordinary: WorthQueryManagedLiveCloseReceipt,
    counters: WorthQueryProjectionLifecycleTransitionCounters,
}

impl WorthQueryProjectionLifecycleCloseReceipt {
    pub fn cause(&self) -> WorthQueryProjectionLifecycleCloseCause {
        self.cause
    }

    pub fn predecessor_identity(&self) -> &str {
        &self.predecessor_identity
    }

    pub fn ordinary(&self) -> &WorthQueryManagedLiveCloseReceipt {
        &self.ordinary
    }

    pub fn counters(&self) -> WorthQueryProjectionLifecycleTransitionCounters {
        self.counters
    }
}

pub(super) struct WorthQueryClosedProjection<S, P: PhaseMarker> {
    source: S,
    proof: WorthQueryClosedProjectionProof<P>,
    live_receipt: WorthQueryLiveProjectionReceipt,
    close_receipt: WorthQueryProjectionLifecycleCloseReceipt,
    conditional_provenance: Vec<super::super::super::WorthQueryConditionalProvenance>,
}

type WorthQueryClosedProjectionProof<P> =
    Artifact<P, WorthQueryProjectionLifecycleEvidence, NoProofs, NoAssumptionBasis>;

impl<S, P: PhaseMarker> WorthQueryClosedProjection<S, P> {
    pub(super) fn source(&self) -> &S {
        &self.source
    }

    pub(super) fn proof(&self) -> &WorthQueryClosedProjectionProof<P> {
        &self.proof
    }

    pub(super) fn live_receipt(&self) -> &WorthQueryLiveProjectionReceipt {
        &self.live_receipt
    }

    pub(super) fn close_receipt(&self) -> &WorthQueryProjectionLifecycleCloseReceipt {
        &self.close_receipt
    }

    pub(super) fn conditional_provenance(
        &self,
    ) -> &[super::super::super::WorthQueryConditionalProvenance] {
        &self.conditional_provenance
    }

    pub(super) fn transition<T: PhaseMarker>(
        self,
        identity_family: &'static str,
    ) -> WorthQueryClosedProjection<S, T> {
        let prior_identity = self.proof.payload().identity.clone();
        let identity = crate::identity::hash_parts(&[
            identity_family.into(),
            format!("predecessor:{prior_identity}"),
            format!(
                "closeout:{}",
                self.close_receipt.ordinary.closeout_identity().as_str()
            ),
        ]);
        WorthQueryClosedProjection {
            source: self.source,
            proof: Artifact::new(WorthQueryProjectionLifecycleEvidence {
                identity,
                predecessor_identity: prior_identity,
            }),
            live_receipt: self.live_receipt,
            close_receipt: self.close_receipt,
            conditional_provenance: self.conditional_provenance,
        }
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        S,
        WorthQueryLiveProjectionReceipt,
        WorthQueryProjectionLifecycleCloseReceipt,
        Vec<super::super::super::WorthQueryConditionalProvenance>,
    ) {
        (
            self.source,
            self.live_receipt,
            self.close_receipt,
            self.conditional_provenance,
        )
    }
}

pub(super) enum WorthQueryProjectionCloseCoreOutcome<
    S,
    L: BasisOperationLane,
    From: PhaseMarker,
    To: PhaseMarker,
> {
    Closed(WorthQueryClosedProjection<S, To>),
    Stopped(WorthQueryProjectionCloseCoreStop<S, L, From>),
}

pub(super) struct WorthQueryProjectionCloseCoreStop<S, L: BasisOperationLane, P: PhaseMarker> {
    owner: WorthQueryOperationalProjection<S, L, P>,
    kind: WorthQueryProjectionCloseCoreStopKind,
    counters: WorthQueryProjectionLifecycleTransitionCounters,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum WorthQueryProjectionCloseCoreStopKind {
    Runtime(String),
}

impl<S, L: BasisOperationLane, P: PhaseMarker> WorthQueryProjectionCloseCoreStop<S, L, P> {
    pub(super) fn into_parts(
        self,
    ) -> (
        WorthQueryOperationalProjection<S, L, P>,
        WorthQueryProjectionCloseCoreStopKind,
        WorthQueryProjectionLifecycleTransitionCounters,
    ) {
        (self.owner, self.kind, self.counters)
    }
}

pub(super) fn close_operational_projection<
    S,
    L: BasisOperationLane,
    From: PhaseMarker,
    To: PhaseMarker,
>(
    owner: WorthQueryOperationalProjection<S, L, From>,
    cause: WorthQueryProjectionLifecycleCloseCause,
    workspace: &mut WorthQueryWorkspace,
) -> WorthQueryProjectionCloseCoreOutcome<S, L, From, To> {
    let mut counters = WorthQueryProjectionLifecycleTransitionCounters::default();
    let (source, proof, handle, live_receipt, conditional_provenance) = owner.into_parts();
    let predecessor_identity = proof.payload().identity.clone();
    counters.close_attempts = 1;
    match handle.close_with_cause(workspace, ordinary_close_cause(cause)) {
        WorthQueryManagedLiveCloseOutcome::Closed(ordinary) => {
            counters.close_completions = 1;
            let identity = crate::identity::hash_parts(&[
                "worth_query_projection_lifecycle_close_v1".into(),
                format!("cause:{}", cause.identity_name()),
                format!("predecessor:{predecessor_identity}"),
                format!("resource:{}", ordinary.resource_name()),
                format!("closeout:{}", ordinary.closeout_identity().as_str()),
            ]);
            WorthQueryProjectionCloseCoreOutcome::Closed(WorthQueryClosedProjection {
                source,
                proof: Artifact::new(WorthQueryProjectionLifecycleEvidence {
                    identity,
                    predecessor_identity: predecessor_identity.clone(),
                }),
                live_receipt,
                close_receipt: WorthQueryProjectionLifecycleCloseReceipt {
                    cause,
                    predecessor_identity,
                    ordinary,
                    counters,
                },
                conditional_provenance,
            })
        }
        WorthQueryManagedLiveCloseOutcome::Stopped(stop) => {
            let detail = stop.error().to_string();
            let handle = stop.into_handle();
            WorthQueryProjectionCloseCoreOutcome::Stopped(WorthQueryProjectionCloseCoreStop {
                owner: WorthQueryOperationalProjection::from_parts(
                    source,
                    proof,
                    handle,
                    live_receipt,
                    conditional_provenance,
                ),
                kind: WorthQueryProjectionCloseCoreStopKind::Runtime(detail),
                counters,
            })
        }
    }
}

fn ordinary_close_cause(
    cause: WorthQueryProjectionLifecycleCloseCause,
) -> crate::ordinary::live::WorthQueryManagedLiveCloseCause {
    use crate::ordinary::live::WorthQueryManagedLiveCloseCause as Ordinary;
    match cause {
        WorthQueryProjectionLifecycleCloseCause::Cancellation => Ordinary::Cancellation,
        WorthQueryProjectionLifecycleCloseCause::Disposal => Ordinary::Disposal,
        WorthQueryProjectionLifecycleCloseCause::Replacement => Ordinary::Replacement,
        WorthQueryProjectionLifecycleCloseCause::Rebind => Ordinary::Rebind,
        WorthQueryProjectionLifecycleCloseCause::ReplacementRollback => {
            Ordinary::ReplacementRollback
        }
        WorthQueryProjectionLifecycleCloseCause::RebindRollback => Ordinary::RebindRollback,
    }
}
