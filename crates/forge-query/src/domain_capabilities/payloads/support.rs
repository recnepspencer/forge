use super::common::{
    define_payload_family, ForgeQueryDomainCapabilityCategory,
    ForgeQueryDomainCapabilitySemanticPosture,
};

define_payload_family!(
    ForgeQuerySupportContributionPosture,
    ForgeQuerySupportContributionPayload,
    ForgeQueryDomainCapabilityCategory::SupportTraceability,
    {
        DeclarationSupport => "declaration-support",
        DeclarationTraceability => "declaration-traceability",
        NarrowedSupport => "narrowed-support",
    }
);

impl ForgeQuerySupportContributionPosture {
    pub const fn semantic_posture(self) -> ForgeQueryDomainCapabilitySemanticPosture {
        match self {
            Self::DeclarationSupport => {
                ForgeQueryDomainCapabilitySemanticPosture::SupportDeclarationSupport
            }
            Self::DeclarationTraceability => {
                ForgeQueryDomainCapabilitySemanticPosture::SupportDeclarationTraceability
            }
            Self::NarrowedSupport => {
                ForgeQueryDomainCapabilitySemanticPosture::SupportNarrowedSupport
            }
        }
    }
}
