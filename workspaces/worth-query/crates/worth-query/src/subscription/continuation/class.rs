#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SubscriptionContinuationClass {
    IdentityRemap,
    CorrespondenceAdvisory,
    IdentityBreak,
    CollectionMembershipRemap,
    GroupedMembershipRemap,
    PreviewPromotionRemap,
    UnsupportedContinuation,
}

impl SubscriptionContinuationClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::IdentityRemap => "identity_remap",
            Self::CorrespondenceAdvisory => "correspondence_advisory",
            Self::IdentityBreak => "identity_break",
            Self::CollectionMembershipRemap => "collection_membership_remap",
            Self::GroupedMembershipRemap => "grouped_membership_remap",
            Self::PreviewPromotionRemap => "preview_promotion_remap",
            Self::UnsupportedContinuation => "unsupported_continuation",
        }
    }
}
