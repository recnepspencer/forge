use forge_runtime_bridge::facade::{
    BridgeMixedCauseDeliveryWindowPlan, BridgeMixedCauseOrderFamilyKind, BridgeMixedCauseOrdering,
    BridgeMixedCauseOrderingLaneKind,
};

use crate::identity::hash_parts;
use crate::subscription::QuerySubscriptionDeliveryCauseKind;

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
    ordering_digest: String,
    delivery_window_digest: String,
    lane_kind: ForgeQueryRuntimeMixedCauseLaneKind,
    ordered_member_kinds: Vec<ForgeQueryRuntimeMixedCauseMemberKind>,
    ordered_cause_digests: Vec<String>,
    suppressed_cause_digests: Vec<String>,
    denied_cause_digests: Vec<String>,
    coalescing_kind: ForgeQueryRuntimeDeliveryCoalescingKind,
    mixed_cause_digest: String,
}

impl ForgeQueryRuntimeMixedCauseDelivery {
    pub(crate) fn atomic_relational_patch(delivery_cause_digest: &str) -> Self {
        Self::atomic(
            ForgeQueryRuntimeMixedCauseMemberKind::TruthPatch,
            delivery_cause_digest,
            delivery_cause_digest,
        )
    }

    pub(crate) fn atomic_time_only(
        delivery_cause_kind: QuerySubscriptionDeliveryCauseKind,
        delivery_cause_digest: &str,
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
        Self::atomic(member_kind, delivery_cause_digest, delivery_cause_digest)
    }

    fn atomic(
        member_kind: ForgeQueryRuntimeMixedCauseMemberKind,
        ordering_digest: &str,
        delivery_window_digest: &str,
    ) -> Self {
        let mixed_cause_digest = hash_parts(&[
            "forge_query_runtime_mixed_cause_delivery_v1".to_string(),
            "lane:authoritative".to_string(),
            format!(
                "coalescing:{}",
                ForgeQueryRuntimeDeliveryCoalescingKind::Atomic.as_str()
            ),
            format!("ordered-kind:{}", member_kind.as_str()),
            format!("ordered-digest:{ordering_digest}"),
        ]);
        Self {
            ordering_digest: ordering_digest.to_string(),
            delivery_window_digest: delivery_window_digest.to_string(),
            lane_kind: ForgeQueryRuntimeMixedCauseLaneKind::Authoritative,
            ordered_member_kinds: vec![member_kind],
            ordered_cause_digests: vec![ordering_digest.to_string()],
            suppressed_cause_digests: Vec::new(),
            denied_cause_digests: Vec::new(),
            coalescing_kind: ForgeQueryRuntimeDeliveryCoalescingKind::Atomic,
            mixed_cause_digest,
        }
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
        let ordered_cause_digests = delivery_window
            .ordered_causes()
            .iter()
            .map(|cause| cause.digest().to_string())
            .collect::<Vec<_>>();
        let suppressed_cause_digests = ordering
            .suppressed()
            .iter()
            .map(|cause| cause.digest().to_string())
            .collect::<Vec<_>>();
        let denied_cause_digests = ordering
            .denied()
            .iter()
            .map(|cause| cause.digest().to_string())
            .collect::<Vec<_>>();
        let coalescing_kind = if ordered_member_kinds.len() > 1 {
            ForgeQueryRuntimeDeliveryCoalescingKind::Coalesced
        } else {
            ForgeQueryRuntimeDeliveryCoalescingKind::Atomic
        };
        let mixed_cause_digest = hash_parts(&[
            "forge_query_runtime_mixed_cause_delivery_v1".to_string(),
            format!("ordering:{}", ordering.digest()),
            format!("window:{}", delivery_window.digest()),
            format!("lane:{}", lane_kind.as_str()),
            format!("coalescing:{}", coalescing_kind.as_str()),
            format!("ordered:{}", ordered_cause_digests.join(",")),
            format!("suppressed:{}", suppressed_cause_digests.join(",")),
            format!("denied:{}", denied_cause_digests.join(",")),
        ]);
        Self {
            ordering_digest: ordering.digest().to_string(),
            delivery_window_digest: delivery_window.digest().to_string(),
            lane_kind,
            ordered_member_kinds,
            ordered_cause_digests,
            suppressed_cause_digests,
            denied_cause_digests,
            coalescing_kind,
            mixed_cause_digest,
        }
    }

    pub fn ordering_digest(&self) -> &str {
        &self.ordering_digest
    }

    pub fn delivery_window_digest(&self) -> &str {
        &self.delivery_window_digest
    }

    pub fn lane_kind(&self) -> ForgeQueryRuntimeMixedCauseLaneKind {
        self.lane_kind
    }

    pub fn ordered_member_kinds(&self) -> &[ForgeQueryRuntimeMixedCauseMemberKind] {
        &self.ordered_member_kinds
    }

    pub fn ordered_cause_digests(&self) -> &[String] {
        &self.ordered_cause_digests
    }

    pub fn suppressed_cause_digests(&self) -> &[String] {
        &self.suppressed_cause_digests
    }

    pub fn denied_cause_digests(&self) -> &[String] {
        &self.denied_cause_digests
    }

    pub fn coalescing_kind(&self) -> ForgeQueryRuntimeDeliveryCoalescingKind {
        self.coalescing_kind
    }

    pub fn mixed_cause_digest(&self) -> &str {
        &self.mixed_cause_digest
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
