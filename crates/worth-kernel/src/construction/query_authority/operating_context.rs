use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDomainOperatingContext,
};

use super::domain::PrimitiveConstructionQueryDomain;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PrimitiveConstructionOperatingMode {
    CurrentHeadAuthoritative,
    CertificationReplay,
}

impl PrimitiveConstructionOperatingMode {
    fn identity_part(self) -> &'static str {
        match self {
            Self::CurrentHeadAuthoritative => "current-head-authoritative",
            Self::CertificationReplay => "certification-replay",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionOperatingContext {
    mode: PrimitiveConstructionOperatingMode,
}

impl PrimitiveConstructionOperatingContext {
    pub(crate) fn current_head_authoritative() -> Self {
        Self {
            mode: PrimitiveConstructionOperatingMode::CurrentHeadAuthoritative,
        }
    }

    #[cfg(test)]
    pub(crate) fn certification_replay() -> Self {
        Self {
            mode: PrimitiveConstructionOperatingMode::CertificationReplay,
        }
    }

    pub(crate) fn mode(&self) -> PrimitiveConstructionOperatingMode {
        self.mode
    }
}

impl ForgeQueryDomainOperatingContext<PrimitiveConstructionQueryDomain>
    for PrimitiveConstructionOperatingContext
{
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
        ]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::Relational,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!(
            "worth.kernel.primitive_construction.operating_context.{}.v1",
            self.mode.identity_part()
        )
    }
}
