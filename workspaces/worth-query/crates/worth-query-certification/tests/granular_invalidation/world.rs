#[path = "world/scenario_definitions.rs"]
mod scenario_definitions;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum GranularInvalidationScenario {
    CurveDetailToLiveRisk,
    SuppressedQuoteNoQueryPatch,
    OrderedPortfolioMembership,
    SharedLeaseDisclosureNoninterference,
    CorrespondenceRebindRestore,
    OpaqueRegionPlatformTwin,
}

impl GranularInvalidationScenario {
    pub const ALL: [Self; 6] = [
        Self::CurveDetailToLiveRisk,
        Self::SuppressedQuoteNoQueryPatch,
        Self::OrderedPortfolioMembership,
        Self::SharedLeaseDisclosureNoninterference,
        Self::CorrespondenceRebindRestore,
        Self::OpaqueRegionPlatformTwin,
    ];

    pub const fn name(self) -> &'static str {
        match self {
            Self::CurveDetailToLiveRisk => "curve_detail_to_live_risk",
            Self::SuppressedQuoteNoQueryPatch => "suppressed_quote_no_query_patch",
            Self::OrderedPortfolioMembership => "ordered_portfolio_membership",
            Self::SharedLeaseDisclosureNoninterference => "shared_lease_disclosure_noninterference",
            Self::CorrespondenceRebindRestore => "correspondence_rebind_restore",
            Self::OpaqueRegionPlatformTwin => "opaque_region_platform_twin",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeclaredLocality {
    Unscoped,
    WholePartition(&'static str),
    ExactDetail(&'static str, &'static str),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclaredDependency {
    pub ordinal: usize,
    pub aspect: &'static str,
    pub locality: DeclaredLocality,
    pub field: &'static str,
    pub roles: &'static [&'static str],
    pub performed_signal_partition: &'static str,
    pub query_signal_mapping: &'static str,
    pub query_signal_partition: &'static str,
    pub performs_signal: bool,
    pub tolerance: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GranularInvalidationMutation {
    pub identity: &'static str,
    pub aspect: &'static str,
    pub partition: &'static str,
    pub detail: &'static str,
    pub relational_record_identity:
        worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts,
    pub field: &'static str,
    pub magnitude: u64,
    pub current: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GranularInvalidationWorldDefinition {
    pub scenario: GranularInvalidationScenario,
    pub seed: u64,
    pub dependencies: Vec<DeclaredDependency>,
    pub mutations: Vec<GranularInvalidationMutation>,
}

impl GranularInvalidationWorldDefinition {
    pub fn for_scenario(scenario: GranularInvalidationScenario, seed: u64) -> Self {
        let (dependencies, mutations) = scenario_definitions::define(scenario, seed);
        Self {
            scenario,
            seed,
            dependencies,
            mutations,
        }
    }
}
