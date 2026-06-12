#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerDirectFreshnessMode {
    LiveStrict,
    LiveCoalesced,
    BackgroundCoalesced,
    InvalidateOnly,
    PullOnFocus,
    PresenceOnly,
}

impl ForgeServerDirectFreshnessMode {
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
pub enum ForgeServerDirectDeliveryClass {
    AuthoritativeOrdered,
    ReplaceableLatestState,
    CoalescibleRegion,
    EphemeralPresence,
    AdvisoryHint,
}

impl ForgeServerDirectDeliveryClass {
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
pub struct ForgeServerDirectDeliveryRequest {
    freshness_mode: ForgeServerDirectFreshnessMode,
    delivery_class: ForgeServerDirectDeliveryClass,
    requested_resume: crate::ForgeServerQueryRequestedResume,
    canonical_digest: String,
}

impl ForgeServerDirectDeliveryRequest {
    pub fn new(
        freshness_mode: ForgeServerDirectFreshnessMode,
        delivery_class: ForgeServerDirectDeliveryClass,
        requested_resume: crate::ForgeServerQueryRequestedResume,
    ) -> Self {
        let canonical_digest = format!(
            "forge-server-direct-delivery-request-v1|freshness:{}|delivery:{}|resume:{}",
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

    pub fn freshness_mode(&self) -> ForgeServerDirectFreshnessMode {
        self.freshness_mode
    }

    pub fn delivery_class(&self) -> ForgeServerDirectDeliveryClass {
        self.delivery_class
    }

    pub fn requested_resume(&self) -> &crate::ForgeServerQueryRequestedResume {
        &self.requested_resume
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }

    pub fn request_digest(&self) -> &str {
        self.canonical_digest()
    }
}
