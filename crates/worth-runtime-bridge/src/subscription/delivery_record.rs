use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeAdmittedSubscriptionIdentity, BridgeSubscriptionBasisIdentity,
    BridgeSubscriptionCounters, BridgeSubscriptionDeliveryContentDigest,
    BridgeSubscriptionDeliveryDiagnosticsReferenceIdentity,
    BridgeSubscriptionDeliveryFamilyIdentity, BridgeSubscriptionDeliveryMemberIdentity,
    BridgeSubscriptionDeliveryWindowIdentity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeSubscriptionDeliveryMemberClass {
    Update,
    Removal,
    ContinuityRemap,
    ReplayMember,
    HeartbeatNoOp,
}

impl BridgeSubscriptionDeliveryMemberClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Update => "update",
            Self::Removal => "removal",
            Self::ContinuityRemap => "continuity_remap",
            Self::ReplayMember => "replay_member",
            Self::HeartbeatNoOp => "heartbeat_no_op",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeSubscriptionDeliveryContentOmissionReason {
    ContentDigestOnly,
    RouteFocusedDelivery,
    HeartbeatNoOp,
}

impl BridgeSubscriptionDeliveryContentOmissionReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContentDigestOnly => "content_digest_only",
            Self::RouteFocusedDelivery => "route_focused_delivery",
            Self::HeartbeatNoOp => "heartbeat_no_op",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionDeliveryMemberInput {
    route_or_slice_identity: Arc<str>,
    upstream_causality_digest: Arc<str>,
    member_class: BridgeSubscriptionDeliveryMemberClass,
    delivery_content_digest: Option<BridgeSubscriptionDeliveryContentDigest>,
    content_omitted_reason: Option<BridgeSubscriptionDeliveryContentOmissionReason>,
}

impl BridgeSubscriptionDeliveryMemberInput {
    pub fn delivery_content_digest(
        route_or_slice_identity: impl Into<Arc<str>>,
        upstream_causality_digest: impl Into<Arc<str>>,
        member_class: BridgeSubscriptionDeliveryMemberClass,
        delivery_content_digest: BridgeSubscriptionDeliveryContentDigest,
    ) -> Self {
        Self {
            route_or_slice_identity: route_or_slice_identity.into(),
            upstream_causality_digest: upstream_causality_digest.into(),
            member_class,
            delivery_content_digest: Some(delivery_content_digest),
            content_omitted_reason: None,
        }
    }

    pub fn omitted_content(
        route_or_slice_identity: impl Into<Arc<str>>,
        upstream_causality_digest: impl Into<Arc<str>>,
        member_class: BridgeSubscriptionDeliveryMemberClass,
        content_omitted_reason: BridgeSubscriptionDeliveryContentOmissionReason,
    ) -> Self {
        Self {
            route_or_slice_identity: route_or_slice_identity.into(),
            upstream_causality_digest: upstream_causality_digest.into(),
            member_class,
            delivery_content_digest: None,
            content_omitted_reason: Some(content_omitted_reason),
        }
    }

    pub fn route_or_slice_identity(&self) -> &str {
        self.route_or_slice_identity.as_ref()
    }

    pub fn upstream_causality_digest(&self) -> &str {
        self.upstream_causality_digest.as_ref()
    }

    pub fn member_class(&self) -> BridgeSubscriptionDeliveryMemberClass {
        self.member_class
    }

    pub fn delivery_content_digest_value(&self) -> Option<&str> {
        self.delivery_content_digest
            .as_ref()
            .map(BridgeSubscriptionDeliveryContentDigest::as_str)
    }

    pub fn content_omitted_reason(
        &self,
    ) -> Option<BridgeSubscriptionDeliveryContentOmissionReason> {
        self.content_omitted_reason
    }

    pub(crate) fn canonical_input_basis(&self) -> String {
        let content_basis = self
            .delivery_content_digest_value()
            .map(str::to_owned)
            .unwrap_or_else(|| {
                self.content_omitted_reason()
                    .map(BridgeSubscriptionDeliveryContentOmissionReason::as_str)
                    .unwrap_or("missing_content_basis")
                    .to_owned()
            });
        format!(
            "route-or-slice={}|causality={}|class={}|content={}",
            self.route_or_slice_identity(),
            self.upstream_causality_digest(),
            self.member_class().as_str(),
            content_basis,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionDeliveryMemberRecord {
    delivery_member_identity: BridgeSubscriptionDeliveryMemberIdentity,
    admitted_subscription_identity: BridgeAdmittedSubscriptionIdentity,
    delivery_family_identity: BridgeSubscriptionDeliveryFamilyIdentity,
    delivery_window_identity: BridgeSubscriptionDeliveryWindowIdentity,
    canonical_sequence: usize,
    route_or_slice_identity: Arc<str>,
    basis_identity: BridgeSubscriptionBasisIdentity,
    upstream_causality_digest: Arc<str>,
    member_class: BridgeSubscriptionDeliveryMemberClass,
    delivery_content_digest: Option<BridgeSubscriptionDeliveryContentDigest>,
    content_omitted_reason: Option<BridgeSubscriptionDeliveryContentOmissionReason>,
    diagnostics_tier_class: Arc<str>,
    counter_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionDeliveryMemberRecord {
    pub(crate) fn new(
        admitted_subscription_identity: BridgeAdmittedSubscriptionIdentity,
        delivery_family_identity: BridgeSubscriptionDeliveryFamilyIdentity,
        delivery_window_identity: BridgeSubscriptionDeliveryWindowIdentity,
        basis_identity: BridgeSubscriptionBasisIdentity,
        canonical_sequence: usize,
        diagnostics_tier_class: Arc<str>,
        counter_digest: Arc<str>,
        input: BridgeSubscriptionDeliveryMemberInput,
    ) -> Self {
        let content_basis = input
            .delivery_content_digest_value()
            .map(str::to_owned)
            .unwrap_or_else(|| {
                input
                    .content_omitted_reason()
                    .map(BridgeSubscriptionDeliveryContentOmissionReason::as_str)
                    .unwrap_or("missing_content_basis")
                    .to_owned()
            });
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-delivery-member|admitted={}|family={}|window={}|sequence={}|route-or-slice={}|basis={}|causality={}|class={}|content={}|diagnostics-tier={}|counter-digest={}",
            admitted_subscription_identity.as_str(),
            delivery_family_identity.as_str(),
            delivery_window_identity.as_str(),
            canonical_sequence,
            input.route_or_slice_identity(),
            basis_identity.as_str(),
            input.upstream_causality_digest(),
            input.member_class().as_str(),
            content_basis,
            diagnostics_tier_class.as_ref(),
            counter_digest.as_ref(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            delivery_member_identity: BridgeSubscriptionDeliveryMemberIdentity::admit_bridge_owned(
                format!("bridge-subscription-delivery-member-id:sha256:{digest:x}"),
            ),
            admitted_subscription_identity,
            delivery_family_identity,
            delivery_window_identity,
            canonical_sequence,
            route_or_slice_identity: input.route_or_slice_identity,
            basis_identity,
            upstream_causality_digest: input.upstream_causality_digest,
            member_class: input.member_class,
            delivery_content_digest: input.delivery_content_digest,
            content_omitted_reason: input.content_omitted_reason,
            diagnostics_tier_class,
            counter_digest,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-delivery-member:sha256:{digest:x}"
            )),
        }
    }

    pub fn delivery_member_identity(&self) -> &BridgeSubscriptionDeliveryMemberIdentity {
        &self.delivery_member_identity
    }

    pub fn delivery_window_identity(&self) -> &BridgeSubscriptionDeliveryWindowIdentity {
        &self.delivery_window_identity
    }

    pub fn admitted_subscription_identity(&self) -> &BridgeAdmittedSubscriptionIdentity {
        &self.admitted_subscription_identity
    }

    pub fn delivery_family_identity(&self) -> &BridgeSubscriptionDeliveryFamilyIdentity {
        &self.delivery_family_identity
    }

    pub fn canonical_sequence(&self) -> usize {
        self.canonical_sequence
    }

    pub fn route_or_slice_identity(&self) -> &str {
        self.route_or_slice_identity.as_ref()
    }

    pub fn basis_identity(&self) -> &BridgeSubscriptionBasisIdentity {
        &self.basis_identity
    }

    pub fn member_class(&self) -> BridgeSubscriptionDeliveryMemberClass {
        self.member_class
    }

    pub fn delivery_content_digest_value(&self) -> Option<&str> {
        self.delivery_content_digest
            .as_ref()
            .map(BridgeSubscriptionDeliveryContentDigest::as_str)
    }

    pub fn content_omitted_reason(
        &self,
    ) -> Option<BridgeSubscriptionDeliveryContentOmissionReason> {
        self.content_omitted_reason
    }

    pub fn diagnostics_tier_class(&self) -> &str {
        self.diagnostics_tier_class.as_ref()
    }

    pub fn counter_digest(&self) -> &str {
        self.counter_digest.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionDeliveryDiagnosticsReference {
    diagnostics_reference_identity: BridgeSubscriptionDeliveryDiagnosticsReferenceIdentity,
    delivery_window_identity: BridgeSubscriptionDeliveryWindowIdentity,
    counter_digest: Arc<str>,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionDeliveryDiagnosticsReference {
    pub(crate) fn new(
        delivery_window_identity: BridgeSubscriptionDeliveryWindowIdentity,
        counter_digest: Arc<str>,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-delivery-diagnostics-reference|window={}|counter-digest={}",
            delivery_window_identity.as_str(),
            counter_digest.as_ref(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            diagnostics_reference_identity:
                BridgeSubscriptionDeliveryDiagnosticsReferenceIdentity::admit_bridge_owned(format!(
                    "bridge-subscription-delivery-diagnostics-reference-id:sha256:{digest:x}"
                )),
            delivery_window_identity,
            counter_digest,
            counters: BridgeSubscriptionCounters::from_delivery_diagnostics_reference(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-delivery-diagnostics-reference:sha256:{digest:x}"
            )),
        }
    }

    pub fn diagnostics_reference_identity(
        &self,
    ) -> &BridgeSubscriptionDeliveryDiagnosticsReferenceIdentity {
        &self.diagnostics_reference_identity
    }

    pub fn delivery_window_identity(&self) -> &BridgeSubscriptionDeliveryWindowIdentity {
        &self.delivery_window_identity
    }

    pub fn counter_digest(&self) -> &str {
        self.counter_digest.as_ref()
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
