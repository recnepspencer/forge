use super::{UiAllocationReceiptGeneration, UiAllocationReceiptIdentity, UiAllocationReuseVerdict};

/// Runtime truth posture for the most recently committed allocation receipt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationReceiptFreshnessPosture {
    Current,
    Coalescing,
    StaleButBounded,
    RecomputePending,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiAllocationReceiptLagBound {
    observed_frames: u8,
    maximum_frames: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationFreshnessTransitionCause {
    ResolvedCommit,
    CoalescingWindowOpened,
    PartialQuerySettlement,
    LeafRemeasureRequired,
    CoalescingLagExceeded,
    BoundedLagExpired,
    ReplacementRequired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiAllocationFreshnessTransition {
    from: Option<UiAllocationReceiptFreshnessPosture>,
    to: UiAllocationReceiptFreshnessPosture,
    cause: UiAllocationFreshnessTransitionCause,
    lag: Option<UiAllocationReceiptLagBound>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationFreshnessTransitionDenial {
    LagExceedsPolicy,
    StalePostureRequiresLagBound,
    RecomputePendingIsTerminal,
    InvalidSuccessor,
}

/// Immutable commitment lineage. Attempt denials are deliberately not freshness states.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAllocationReceiptReport {
    receipt_identity: UiAllocationReceiptIdentity,
    receipt_generation: UiAllocationReceiptGeneration,
    reuse_verdict: UiAllocationReuseVerdict,
    transitions: Box<[UiAllocationFreshnessTransition]>,
    counters: Option<super::UiAllocationCounterReport>,
}

impl UiAllocationReceiptReport {
    pub(crate) fn new(
        receipt_identity: UiAllocationReceiptIdentity,
        receipt_generation: UiAllocationReceiptGeneration,
        reuse_verdict: UiAllocationReuseVerdict,
    ) -> Self {
        let (freshness, cause) = match reuse_verdict {
            UiAllocationReuseVerdict::StructureReuseLeafRemeasure(_) => (
                UiAllocationReceiptFreshnessPosture::RecomputePending,
                UiAllocationFreshnessTransitionCause::LeafRemeasureRequired,
            ),
            UiAllocationReuseVerdict::NewCommit | UiAllocationReuseVerdict::FullReuse => (
                UiAllocationReceiptFreshnessPosture::Current,
                UiAllocationFreshnessTransitionCause::ResolvedCommit,
            ),
            UiAllocationReuseVerdict::Denied(_) => {
                unreachable!("denied attempts produce denial reports, never receipt reports")
            }
        };
        Self {
            receipt_identity,
            receipt_generation,
            reuse_verdict,
            transitions: vec![UiAllocationFreshnessTransition {
                from: None,
                to: freshness,
                cause,
                lag: None,
            }]
            .into_boxed_slice(),
            counters: None,
        }
    }

    pub(crate) fn transition(
        &self,
        to: UiAllocationReceiptFreshnessPosture,
        cause: UiAllocationFreshnessTransitionCause,
        lag: Option<UiAllocationReceiptLagBound>,
    ) -> Result<Self, UiAllocationFreshnessTransitionDenial> {
        let from = self.freshness();
        if from == UiAllocationReceiptFreshnessPosture::RecomputePending {
            return Err(UiAllocationFreshnessTransitionDenial::RecomputePendingIsTerminal);
        }
        let admitted = matches!(
            (from, to, cause),
            (
                UiAllocationReceiptFreshnessPosture::Current,
                UiAllocationReceiptFreshnessPosture::Coalescing,
                UiAllocationFreshnessTransitionCause::CoalescingWindowOpened,
            ) | (
                UiAllocationReceiptFreshnessPosture::Current,
                UiAllocationReceiptFreshnessPosture::StaleButBounded,
                UiAllocationFreshnessTransitionCause::PartialQuerySettlement,
            ) | (
                UiAllocationReceiptFreshnessPosture::Coalescing,
                UiAllocationReceiptFreshnessPosture::StaleButBounded,
                UiAllocationFreshnessTransitionCause::PartialQuerySettlement,
            ) | (
                UiAllocationReceiptFreshnessPosture::Current,
                UiAllocationReceiptFreshnessPosture::RecomputePending,
                UiAllocationFreshnessTransitionCause::LeafRemeasureRequired
                    | UiAllocationFreshnessTransitionCause::BoundedLagExpired
                    | UiAllocationFreshnessTransitionCause::ReplacementRequired,
            ) | (
                UiAllocationReceiptFreshnessPosture::Coalescing,
                UiAllocationReceiptFreshnessPosture::RecomputePending,
                UiAllocationFreshnessTransitionCause::CoalescingLagExceeded
                    | UiAllocationFreshnessTransitionCause::BoundedLagExpired
                    | UiAllocationFreshnessTransitionCause::ReplacementRequired,
            ) | (
                UiAllocationReceiptFreshnessPosture::StaleButBounded,
                UiAllocationReceiptFreshnessPosture::RecomputePending,
                UiAllocationFreshnessTransitionCause::LeafRemeasureRequired
                    | UiAllocationFreshnessTransitionCause::BoundedLagExpired
                    | UiAllocationFreshnessTransitionCause::ReplacementRequired,
            )
        );
        if !admitted {
            return Err(UiAllocationFreshnessTransitionDenial::InvalidSuccessor);
        }
        if to == UiAllocationReceiptFreshnessPosture::StaleButBounded && lag.is_none() {
            return Err(UiAllocationFreshnessTransitionDenial::StalePostureRequiresLagBound);
        }
        if to != UiAllocationReceiptFreshnessPosture::RecomputePending
            && lag.is_some_and(|bound| bound.observed_frames > bound.maximum_frames)
        {
            return Err(UiAllocationFreshnessTransitionDenial::LagExceedsPolicy);
        }
        let mut transitions = self.transitions.to_vec();
        transitions.push(UiAllocationFreshnessTransition {
            from: Some(from),
            to,
            cause,
            lag,
        });
        let mut successor = self.clone();
        successor.transitions = transitions.into_boxed_slice();
        Ok(successor)
    }

    pub(super) fn apply_committed_transaction_freshness(
        self,
        transaction: &super::UiAllocationReplanTransaction,
    ) -> Self {
        let maximum_lag = transaction
            .ingress_policy_verdicts()
            .iter()
            .filter_map(|verdict| match verdict {
                crate::runtime::UiAllocationIngressPolicyVerdict::Current => None,
                crate::runtime::UiAllocationIngressPolicyVerdict::PartialQueryStaleButBounded {
                    max_lag_frames,
                    ..
                } => Some(*max_lag_frames),
            })
            .min();
        let Some(maximum_lag) = maximum_lag else {
            return self;
        };
        let (posture, cause) = if maximum_lag == 0 {
            (
                UiAllocationReceiptFreshnessPosture::RecomputePending,
                UiAllocationFreshnessTransitionCause::BoundedLagExpired,
            )
        } else {
            (
                UiAllocationReceiptFreshnessPosture::StaleButBounded,
                UiAllocationFreshnessTransitionCause::PartialQuerySettlement,
            )
        };
        self.transition(
            posture,
            cause,
            Some(UiAllocationReceiptLagBound::new(1, maximum_lag)),
        )
        .expect("admitted Query partial-settlement policy defines receipt freshness")
    }

    pub fn receipt_identity(&self) -> &UiAllocationReceiptIdentity {
        &self.receipt_identity
    }
    pub fn receipt_generation(&self) -> UiAllocationReceiptGeneration {
        self.receipt_generation
    }
    pub fn reuse_verdict(&self) -> &UiAllocationReuseVerdict {
        &self.reuse_verdict
    }
    pub fn freshness(&self) -> UiAllocationReceiptFreshnessPosture {
        self.transitions
            .last()
            .expect("report has an initial transition")
            .to
    }
    pub fn transitions(&self) -> &[UiAllocationFreshnessTransition] {
        &self.transitions
    }
    pub fn current_lag(&self) -> Option<UiAllocationReceiptLagBound> {
        self.transitions
            .last()
            .and_then(|transition| transition.lag)
    }
    pub fn counters(&self) -> Option<&super::UiAllocationCounterReport> {
        self.counters.as_ref()
    }
    pub(super) fn attach_counters(&mut self, counters: super::UiAllocationCounterReport) {
        self.counters = Some(counters);
    }
}

impl UiAllocationReceiptLagBound {
    pub(crate) const fn new(observed_frames: u8, maximum_frames: u8) -> Self {
        Self {
            observed_frames,
            maximum_frames,
        }
    }
    pub const fn observed_frames(self) -> u8 {
        self.observed_frames
    }
    pub const fn maximum_frames(self) -> u8 {
        self.maximum_frames
    }
    pub const fn is_within_bound(self) -> bool {
        self.observed_frames <= self.maximum_frames
    }
}

impl UiAllocationFreshnessTransition {
    pub const fn from(self) -> Option<UiAllocationReceiptFreshnessPosture> {
        self.from
    }
    pub const fn to(self) -> UiAllocationReceiptFreshnessPosture {
        self.to
    }
    pub const fn cause(self) -> UiAllocationFreshnessTransitionCause {
        self.cause
    }
    pub const fn lag(self) -> Option<UiAllocationReceiptLagBound> {
        self.lag
    }
}
