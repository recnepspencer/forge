#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerDirectFreshnessMode {
    LiveStrict,
    LiveCoalesced,
    BackgroundCoalesced,
    InvalidateOnly,
    PullOnFocus,
    PresenceOnly,
}

impl WorthServerDirectFreshnessMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LiveStrict => "live_strict",
            Self::LiveCoalesced => "live_coalesced",
            Self::BackgroundCoalesced => "background_coalesced",
            Self::InvalidateOnly => "invalidate_only",
            Self::PullOnFocus => "pull_on_focus",
            Self::PresenceOnly => "presence_only",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerDirectDeliveryClass {
    AuthoritativeOrdered,
    ReplaceableLatestState,
    CoalescibleRegion,
    EphemeralPresence,
    AdvisoryHint,
}

impl WorthServerDirectDeliveryClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeOrdered => "authoritative-ordered",
            Self::ReplaceableLatestState => "replaceable-latest-state",
            Self::CoalescibleRegion => "coalescible-region",
            Self::EphemeralPresence => "ephemeral-presence",
            Self::AdvisoryHint => "advisory-hint",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerDirectDeliveryRequest {
    freshness_mode: WorthServerDirectFreshnessMode,
    delivery_class: WorthServerDirectDeliveryClass,
    requested_resume: crate::WorthServerQueryRequestedResume,
    canonical_digest: String,
}

impl WorthServerDirectDeliveryRequest {
    pub fn new(
        freshness_mode: WorthServerDirectFreshnessMode,
        delivery_class: WorthServerDirectDeliveryClass,
        requested_resume: crate::WorthServerQueryRequestedResume,
    ) -> Self {
        let canonical_digest = format!(
            "worth-server-direct-delivery-request-v1|freshness:{}|delivery:{}|resume:{}",
            freshness_mode.as_str(),
            delivery_class.as_str(),
            requested_resume.canonical_label(),
        );
        Self {
            freshness_mode,
            delivery_class,
            requested_resume,
            canonical_digest,
        }
    }

    pub fn freshness_mode(&self) -> WorthServerDirectFreshnessMode {
        self.freshness_mode
    }

    pub fn delivery_class(&self) -> WorthServerDirectDeliveryClass {
        self.delivery_class
    }

    pub fn requested_resume(&self) -> &crate::WorthServerQueryRequestedResume {
        &self.requested_resume
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }

    pub fn request_digest(&self) -> &str {
        self.canonical_digest()
    }
}
