use super::common::{
    define_payload_family, WorthQueryDomainCapabilityCategory,
    WorthQueryDomainCapabilitySemanticPosture,
};

define_payload_family!(
    WorthQuerySupportContributionPosture,
    WorthQuerySupportContributionPayload,
    WorthQueryDomainCapabilityCategory::SupportTraceability,
    {
        DeclarationSupport => "declaration-support",
        DeclarationTraceability => "declaration-traceability",
        NarrowedSupport => "narrowed-support",
    }
);

impl WorthQuerySupportContributionPosture {
    pub const fn semantic_posture(self) -> WorthQueryDomainCapabilitySemanticPosture {
        match self {
            Self::DeclarationSupport => {
                WorthQueryDomainCapabilitySemanticPosture::SupportDeclarationSupport
            }
            Self::DeclarationTraceability => {
                WorthQueryDomainCapabilitySemanticPosture::SupportDeclarationTraceability
            }
            Self::NarrowedSupport => {
                WorthQueryDomainCapabilitySemanticPosture::SupportNarrowedSupport
            }
        }
    }
}
