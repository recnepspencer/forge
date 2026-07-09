#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionFanoutAcknowledgementPolicyClass {
    CanonicalMemberAcknowledgement,
}

impl BridgeSubscriptionFanoutAcknowledgementPolicyClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalMemberAcknowledgement => "canonical_member_acknowledgement",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionFanoutDiagnosticsPolicyClass {
    MinimalReferenceOnly,
}

impl BridgeSubscriptionFanoutDiagnosticsPolicyClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MinimalReferenceOnly => "minimal_reference_only",
        }
    }
}
