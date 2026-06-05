use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDomainOperatingContext,
};

use super::HadwigerResearchDomainEntry;

const REAL_CONTEXT_CAPABILITIES: &[ForgeQueryCapabilityFamily] = &[
    ForgeQueryCapabilityFamily::HistoricalEvaluation,
    ForgeQueryCapabilityFamily::WorkflowOrchestration,
];
const REAL_CONTEXT_SECTIONS: &[ForgeQueryConfigSectionFamily] = &[
    ForgeQueryConfigSectionFamily::Query,
    ForgeQueryConfigSectionFamily::Relational,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HadwigerResearchAssumptionRegime {
    FiniteLowerBoundSearch,
}

impl HadwigerResearchAssumptionRegime {
    fn as_str(self) -> &'static str {
        match self {
            Self::FiniteLowerBoundSearch => "finite_lower_bound_search",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HadwigerResearchCheckerSupportRegime {
    RealInProcess,
}

impl HadwigerResearchCheckerSupportRegime {
    fn as_str(self) -> &'static str {
        match self {
            Self::RealInProcess => "real_in_process",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HadwigerResearchInvalidationRegime {
    Conservative,
}

impl HadwigerResearchInvalidationRegime {
    fn as_str(self) -> &'static str {
        match self {
            Self::Conservative => "conservative",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HadwigerResearchOperatingContext {
    assumption_regime: HadwigerResearchAssumptionRegime,
    checker_support_regime: HadwigerResearchCheckerSupportRegime,
    invalidation_regime: HadwigerResearchInvalidationRegime,
}

impl HadwigerResearchOperatingContext {
    pub fn finite_lower_bound_real() -> Self {
        Self {
            assumption_regime: HadwigerResearchAssumptionRegime::FiniteLowerBoundSearch,
            checker_support_regime: HadwigerResearchCheckerSupportRegime::RealInProcess,
            invalidation_regime: HadwigerResearchInvalidationRegime::Conservative,
        }
    }

    pub fn assumption_regime(&self) -> HadwigerResearchAssumptionRegime {
        self.assumption_regime
    }

    pub fn checker_support_regime(&self) -> HadwigerResearchCheckerSupportRegime {
        self.checker_support_regime
    }

    pub fn invalidation_regime(&self) -> HadwigerResearchInvalidationRegime {
        self.invalidation_regime
    }
}

impl Default for HadwigerResearchOperatingContext {
    fn default() -> Self {
        Self::finite_lower_bound_real()
    }
}

impl ForgeQueryDomainOperatingContext<HadwigerResearchDomainEntry>
    for HadwigerResearchOperatingContext
{
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        REAL_CONTEXT_CAPABILITIES
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        REAL_CONTEXT_SECTIONS
    }

    fn context_identity_digest(&self) -> String {
        format!(
            "assumption:{}|checker:{}|invalidation:{}",
            self.assumption_regime.as_str(),
            self.checker_support_regime.as_str(),
            self.invalidation_regime.as_str()
        )
    }
}
