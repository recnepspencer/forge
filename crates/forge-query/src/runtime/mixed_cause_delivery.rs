use forge_runtime_bridge::facade::{
    BridgeDeniedMixedCause, BridgeMixedCauseDeliveryWindowPlan, BridgeMixedCauseOrderFamilyKind,
    BridgeMixedCauseOrdering, BridgeMixedCauseOrderingLaneKind, BridgeOrderedMixedCause,
    BridgeSuppressedMixedCause,
};

use crate::evidence_identity::ForgeQueryEvidenceIdentity;
use crate::subscription::QuerySubscriptionDeliveryCauseKind;

use super::evidence_identities::{
    runtime_mixed_cause_atomic_identity, runtime_mixed_cause_delivery_identity,
    runtime_mixed_cause_delivery_window_identity, runtime_mixed_cause_denied_cause_identity,
    runtime_mixed_cause_ordered_cause_identity, runtime_mixed_cause_ordering_identity,
    runtime_mixed_cause_suppressed_cause_identity,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryRuntimeMixedCauseLaneKind {
    Authoritative,
    Preview,
}

impl ForgeQueryRuntimeMixedCauseLaneKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Authoritative => "authoritative",
            Self::Preview => "preview",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryRuntimeDeliveryCoalescingKind {
    Atomic,
    Coalesced,
}

impl ForgeQueryRuntimeDeliveryCoalescingKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Atomic => "atomic",
            Self::Coalesced => "coalesced",
        }
    }

    pub fn as_public_str(self) -> &'static str {
        self.as_str()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryRuntimeMixedCauseMemberKind {
    TruthPatch,
    TemporalTruthPlusTime,
    TemporalTimeOnly,
    AsyncCompletion,
    AsyncDeniedCompletion,
    AsyncRetryLineage,
    AsyncRevalidationLineage,
}

impl ForgeQueryRuntimeMixedCauseMemberKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::TruthPatch => "truth_patch",
            Self::TemporalTruthPlusTime => "temporal_truth_plus_time",
            Self::TemporalTimeOnly => "temporal_time_only",
            Self::AsyncCompletion => "async_completion",
            Self::AsyncDeniedCompletion => "async_denied_completion",
            Self::AsyncRetryLineage => "async_retry_lineage",
            Self::AsyncRevalidationLineage => "async_revalidation_lineage",
        }
    }

    pub fn as_public_str(self) -> &'static str {
        self.as_str()
    }

    fn has_relational_patch(self) -> bool {
        matches!(self, Self::TruthPatch | Self::TemporalTruthPlusTime)
    }

    fn atomic_delivery_cause_kind(self) -> QuerySubscriptionDeliveryCauseKind {
        match self {
            Self::TruthPatch => QuerySubscriptionDeliveryCauseKind::RelationalPatch,
            Self::AsyncCompletion => QuerySubscriptionDeliveryCauseKind::AsyncCompletion,
            Self::AsyncDeniedCompletion => {
                QuerySubscriptionDeliveryCauseKind::AsyncDeniedCompletion
            }
            Self::AsyncRetryLineage => QuerySubscriptionDeliveryCauseKind::AsyncRetry,
            Self::AsyncRevalidationLineage => QuerySubscriptionDeliveryCauseKind::AsyncRevalidation,
            Self::TemporalTruthPlusTime | Self::TemporalTimeOnly => {
                QuerySubscriptionDeliveryCauseKind::MixedCause
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimeMixedCauseDelivery {
    ordering_identity: ForgeQueryEvidenceIdentity,
    delivery_window_identity: ForgeQueryEvidenceIdentity,
    lane_kind: ForgeQueryRuntimeMixedCauseLaneKind,
    ordered_member_kinds: Vec<ForgeQueryRuntimeMixedCauseMemberKind>,
    ordered_cause_identities: Vec<ForgeQueryEvidenceIdentity>,
    suppressed_cause_identities: Vec<ForgeQueryEvidenceIdentity>,
    denied_cause_identities: Vec<ForgeQueryEvidenceIdentity>,
    coalescing_kind: ForgeQueryRuntimeDeliveryCoalescingKind,
    mixed_cause_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryRuntimeMixedCauseDelivery {
    pub(crate) fn atomic_relational_patch(
        delivery_cause_identity: &ForgeQueryEvidenceIdentity,
    ) -> Self {
        Self::atomic(
            ForgeQueryRuntimeMixedCauseMemberKind::TruthPatch,
            delivery_cause_identity,
        )
    }

    pub(crate) fn atomic_time_only(
        delivery_cause_kind: QuerySubscriptionDeliveryCauseKind,
        delivery_cause_identity: &ForgeQueryEvidenceIdentity,
    ) -> Self {
        let member_kind = match delivery_cause_kind {
            QuerySubscriptionDeliveryCauseKind::FreshnessOnly
            | QuerySubscriptionDeliveryCauseKind::WindowEntry
            | QuerySubscriptionDeliveryCauseKind::WindowExit
            | QuerySubscriptionDeliveryCauseKind::Deadline
            | QuerySubscriptionDeliveryCauseKind::PreviousValueTransition => {
                ForgeQueryRuntimeMixedCauseMemberKind::TemporalTimeOnly
            }
            QuerySubscriptionDeliveryCauseKind::AsyncCompletion => {
                ForgeQueryRuntimeMixedCauseMemberKind::AsyncCompletion
            }
            QuerySubscriptionDeliveryCauseKind::AsyncDeniedCompletion => {
                ForgeQueryRuntimeMixedCauseMemberKind::AsyncDeniedCompletion
            }
            QuerySubscriptionDeliveryCauseKind::AsyncRetry => {
                ForgeQueryRuntimeMixedCauseMemberKind::AsyncRetryLineage
            }
            QuerySubscriptionDeliveryCauseKind::AsyncRevalidation => {
                ForgeQueryRuntimeMixedCauseMemberKind::AsyncRevalidationLineage
            }
            QuerySubscriptionDeliveryCauseKind::RelationalPatch
            | QuerySubscriptionDeliveryCauseKind::MixedCause => {
                ForgeQueryRuntimeMixedCauseMemberKind::TemporalTimeOnly
            }
        };
        Self::atomic(member_kind, delivery_cause_identity)
    }

    fn atomic(
        member_kind: ForgeQueryRuntimeMixedCauseMemberKind,
        delivery_cause_identity: &ForgeQueryEvidenceIdentity,
    ) -> Self {
        let atomic_identity = runtime_mixed_cause_atomic_identity(delivery_cause_identity);
        let mut delivery = Self {
            ordering_identity: atomic_identity.clone(),
            delivery_window_identity: atomic_identity,
            lane_kind: ForgeQueryRuntimeMixedCauseLaneKind::Authoritative,
            ordered_member_kinds: vec![member_kind],
            ordered_cause_identities: vec![delivery_cause_identity.clone()],
            suppressed_cause_identities: Vec::new(),
            denied_cause_identities: Vec::new(),
            coalescing_kind: ForgeQueryRuntimeDeliveryCoalescingKind::Atomic,
            mixed_cause_identity: ForgeQueryEvidenceIdentity::compose(
                crate::evidence_identity::ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
            )
            .seal(),
        };
        delivery.mixed_cause_identity = runtime_mixed_cause_delivery_identity(&delivery);
        delivery
    }

    pub(crate) fn from_bridge(
        ordering: &BridgeMixedCauseOrdering,
        delivery_window: &BridgeMixedCauseDeliveryWindowPlan,
    ) -> Self {
        let lane_kind = match ordering.lane_kind() {
            BridgeMixedCauseOrderingLaneKind::Authoritative => {
                ForgeQueryRuntimeMixedCauseLaneKind::Authoritative
            }
            BridgeMixedCauseOrderingLaneKind::Preview => {
                ForgeQueryRuntimeMixedCauseLaneKind::Preview
            }
        };
        let ordered_member_kinds = delivery_window
            .ordered_causes()
            .iter()
            .map(|cause| match cause.family_kind() {
                BridgeMixedCauseOrderFamilyKind::TruthPatch => {
                    ForgeQueryRuntimeMixedCauseMemberKind::TruthPatch
                }
                BridgeMixedCauseOrderFamilyKind::TemporalTruthPlusTime => {
                    ForgeQueryRuntimeMixedCauseMemberKind::TemporalTruthPlusTime
                }
                BridgeMixedCauseOrderFamilyKind::TemporalTimeOnly => {
                    ForgeQueryRuntimeMixedCauseMemberKind::TemporalTimeOnly
                }
                BridgeMixedCauseOrderFamilyKind::AsyncCompletion => {
                    ForgeQueryRuntimeMixedCauseMemberKind::AsyncCompletion
                }
                BridgeMixedCauseOrderFamilyKind::AsyncClassifiedDeniedCompletion => {
                    ForgeQueryRuntimeMixedCauseMemberKind::AsyncDeniedCompletion
                }
                BridgeMixedCauseOrderFamilyKind::AsyncRetryLineage => {
                    ForgeQueryRuntimeMixedCauseMemberKind::AsyncRetryLineage
                }
                BridgeMixedCauseOrderFamilyKind::AsyncRevalidationLineage => {
                    ForgeQueryRuntimeMixedCauseMemberKind::AsyncRevalidationLineage
                }
            })
            .collect::<Vec<_>>();
        let ordered_cause_identities = delivery_window
            .ordered_causes()
            .iter()
            .map(runtime_mixed_cause_ordered_cause_identity)
            .collect::<Vec<_>>();
        let suppressed_cause_identities = ordering
            .suppressed()
            .iter()
            .map(runtime_mixed_cause_suppressed_cause_identity)
            .collect::<Vec<_>>();
        let denied_cause_identities = ordering
            .denied()
            .iter()
            .map(runtime_mixed_cause_denied_cause_identity)
            .collect::<Vec<_>>();
        let coalescing_kind = if ordered_member_kinds.len() > 1 {
            ForgeQueryRuntimeDeliveryCoalescingKind::Coalesced
        } else {
            ForgeQueryRuntimeDeliveryCoalescingKind::Atomic
        };
        let ordering_identity = runtime_mixed_cause_ordering_identity(ordering);
        let delivery_window_identity =
            runtime_mixed_cause_delivery_window_identity(delivery_window, ordering);
        let mut delivery = Self {
            ordering_identity,
            delivery_window_identity,
            lane_kind,
            ordered_member_kinds,
            ordered_cause_identities,
            suppressed_cause_identities,
            denied_cause_identities,
            coalescing_kind,
            mixed_cause_identity: ForgeQueryEvidenceIdentity::compose(
                crate::evidence_identity::ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
            )
            .seal(),
        };
        delivery.mixed_cause_identity = runtime_mixed_cause_delivery_identity(&delivery);
        delivery
    }

    pub fn ordering_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.ordering_identity
    }

    pub fn ordering_for_reporting(&self) -> &str {
        self.ordering_identity.as_str()
    }

    pub fn delivery_window_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.delivery_window_identity
    }

    pub fn delivery_window_for_reporting(&self) -> &str {
        self.delivery_window_identity.as_str()
    }

    pub fn lane_kind(&self) -> ForgeQueryRuntimeMixedCauseLaneKind {
        self.lane_kind
    }

    pub fn ordered_member_kinds(&self) -> &[ForgeQueryRuntimeMixedCauseMemberKind] {
        &self.ordered_member_kinds
    }

    pub fn ordered_cause_identities(&self) -> &[ForgeQueryEvidenceIdentity] {
        &self.ordered_cause_identities
    }

    pub fn suppressed_cause_identities(&self) -> &[ForgeQueryEvidenceIdentity] {
        &self.suppressed_cause_identities
    }

    pub fn denied_cause_identities(&self) -> &[ForgeQueryEvidenceIdentity] {
        &self.denied_cause_identities
    }

    pub fn coalescing_kind(&self) -> ForgeQueryRuntimeDeliveryCoalescingKind {
        self.coalescing_kind
    }

    pub fn mixed_cause_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.mixed_cause_identity
    }

    pub fn mixed_cause_for_reporting(&self) -> &str {
        self.mixed_cause_identity.as_str()
    }

    pub fn has_relational_patch(&self) -> bool {
        self.ordered_member_kinds
            .iter()
            .copied()
            .any(ForgeQueryRuntimeMixedCauseMemberKind::has_relational_patch)
    }

    pub fn primary_delivery_cause_kind(&self) -> QuerySubscriptionDeliveryCauseKind {
        if self.ordered_member_kinds.len() == 1 {
            self.ordered_member_kinds[0].atomic_delivery_cause_kind()
        } else {
            QuerySubscriptionDeliveryCauseKind::MixedCause
        }
    }
}
