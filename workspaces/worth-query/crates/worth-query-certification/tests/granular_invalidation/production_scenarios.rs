use crate::production_evidence::PerformedScenarioEvidence;
use crate::world::GranularInvalidationScenario;

pub fn run_scenario(
    scenario: GranularInvalidationScenario,
    seed: u64,
) -> PerformedScenarioEvidence {
    match scenario {
        GranularInvalidationScenario::CurveDetailToLiveRisk => {
            crate::financial_runtime_world::run_curve_certification(seed)
        }
        GranularInvalidationScenario::SuppressedQuoteNoQueryPatch => {
            crate::financial_runtime_world::run_quote_certification(seed)
        }
        GranularInvalidationScenario::OrderedPortfolioMembership => {
            crate::financial_runtime_world::run_portfolio_certification(seed)
        }
        GranularInvalidationScenario::SharedLeaseDisclosureNoninterference => {
            crate::financial_runtime_world::run_shared_certification(seed)
        }
        GranularInvalidationScenario::CorrespondenceRebindRestore => {
            crate::lifecycle_certification::run_correspondence_certification(seed)
        }
        GranularInvalidationScenario::OpaqueRegionPlatformTwin => {
            crate::financial_runtime_world::run_opaque_certification(seed)
        }
    }
}
