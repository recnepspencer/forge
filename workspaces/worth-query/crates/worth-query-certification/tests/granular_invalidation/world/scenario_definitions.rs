use super::{
    DeclaredDependency, DeclaredLocality, GranularInvalidationMutation,
    GranularInvalidationScenario,
};

const PROJECTED: &[&str] = &["projected-value"];
const DESK_ROLES: &[&str] = &["projected-value", "selection-or-membership", "grouping"];
const RANK_ROLES: &[&str] = &["projected-value", "ordering", "window-boundary"];
const FINANCIAL_RECORD: worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts =
    worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts::entity(0, 1, 1);
const LIFECYCLE_RECORD: worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts =
    worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts::entity(0, 2, 1);

pub(super) fn define(
    scenario: GranularInvalidationScenario,
    seed: u64,
) -> (Vec<DeclaredDependency>, Vec<GranularInvalidationMutation>) {
    match scenario {
        GranularInvalidationScenario::CurveDetailToLiveRisk => curve_world(
            "bridge-owned:curve-risk:dependency:2",
            [
                "financial-primary-curve-2",
                "financial-primary-curve-2",
                "financial-primary-curve-2",
            ],
            "financial-primary",
        ),
        GranularInvalidationScenario::OpaqueRegionPlatformTwin => curve_world_with_query_signal(
            "bridge-owned:curve-risk:dependency:2",
            [
                "opaque-a-scope-5-2",
                "opaque-a-scope-5-2",
                "opaque-a-scope-5-2",
            ],
            "region-7",
        ),
        GranularInvalidationScenario::SuppressedQuoteNoQueryPatch => quote_world(),
        GranularInvalidationScenario::OrderedPortfolioMembership => portfolio_world(seed),
        GranularInvalidationScenario::SharedLeaseDisclosureNoninterference => shared_world(),
        GranularInvalidationScenario::CorrespondenceRebindRestore => restore_world(),
    }
}

fn quote_world() -> (Vec<DeclaredDependency>, Vec<GranularInvalidationMutation>) {
    (
        vec![dependency(
            0,
            "PriceFacts",
            DeclaredLocality::ExactDetail("quotes", "instrument-17"),
            "QuoteMidField",
            PROJECTED,
            "bridge-owned:quote-risk:dependency:0",
            "financial-primary-quote",
            "financial-primary",
            true,
            5,
        )],
        vec![
            mutation(
                "small",
                "PriceFacts",
                "quotes",
                "instrument-17",
                "QuoteMidField",
                2,
                true,
            ),
            mutation(
                "large",
                "PriceFacts",
                "quotes",
                "instrument-17",
                "QuoteMidField",
                9,
                true,
            ),
        ],
    )
}

fn portfolio_world(seed: u64) -> (Vec<DeclaredDependency>, Vec<GranularInvalidationMutation>) {
    (
        vec![
            dependency(
                2,
                "PortfolioFacts",
                DeclaredLocality::WholePartition("usd-rates"),
                "PortfolioValueField",
                PROJECTED,
                "bridge-owned:portfolio-risk:dependency:2",
                "financial-primary-portfolio-2",
                "financial-primary",
                true,
                0,
            ),
            dependency(
                0,
                "PortfolioFacts",
                DeclaredLocality::WholePartition("usd-rates"),
                "PortfolioDeskField",
                DESK_ROLES,
                "bridge-owned:portfolio-risk:dependency:0",
                "financial-primary-portfolio-0",
                "financial-primary",
                true,
                0,
            ),
            dependency(
                1,
                "PortfolioFacts",
                DeclaredLocality::WholePartition("usd-rates"),
                "PortfolioRankField",
                RANK_ROLES,
                "bridge-owned:portfolio-risk:dependency:1",
                "financial-primary-portfolio-1",
                "financial-primary",
                true,
                0,
            ),
        ],
        vec![
            portfolio_mutation("value-forward", "PortfolioValueField", seed.max(1)),
            portfolio_mutation("value-reverse", "PortfolioValueField", seed.max(1)),
            portfolio_mutation("membership-removal", "PortfolioDeskField", 1),
            portfolio_mutation("membership-reentry", "PortfolioDeskField", 1),
            portfolio_mutation("ordering", "PortfolioRankField", 1),
            portfolio_mutation("window", "PortfolioRankField", 1),
        ],
    )
}

fn portfolio_mutation(
    identity: &'static str,
    field: &'static str,
    magnitude: u64,
) -> GranularInvalidationMutation {
    mutation(
        identity,
        "PortfolioFacts",
        "usd-rates",
        "portfolio-position-17",
        field,
        magnitude,
        true,
    )
}

fn shared_world() -> (Vec<DeclaredDependency>, Vec<GranularInvalidationMutation>) {
    let (dependencies, mutations) = curve_world(
        "bridge-owned:curve-risk:dependency:2",
        [
            "financial-primary-curve-2",
            "financial-primary-curve-2",
            "financial-primary-curve-2",
        ],
        "financial-primary",
    );
    (
        dependencies,
        vec![GranularInvalidationMutation {
            identity: "shared",
            ..mutations[0].clone()
        }],
    )
}

fn restore_world() -> (Vec<DeclaredDependency>, Vec<GranularInvalidationMutation>) {
    (
        vec![dependency(
            0,
            "IntentFacts",
            DeclaredLocality::ExactDetail("primary", "intent"),
            "IntentGateField",
            PROJECTED,
            "bridge-owned:temporal-ready:dependency:0",
            "temporal-primary-intent",
            "temporal-primary",
            true,
            0,
        )],
        vec![
            lifecycle_mutation(
                "delayed-old",
                "IntentFacts",
                "primary",
                "intent",
                "IntentGateField",
                10,
                false,
            ),
            lifecycle_mutation(
                "current",
                "IntentFacts",
                "primary",
                "intent",
                "IntentGateField",
                10,
                true,
            ),
        ],
    )
}

fn curve_world(
    performed_signal_partition: &'static str,
    query_signal_mappings: [&'static str; 3],
    query_signal_partition: &'static str,
) -> (Vec<DeclaredDependency>, Vec<GranularInvalidationMutation>) {
    curve_world_with_query_signal(
        performed_signal_partition,
        query_signal_mappings,
        query_signal_partition,
    )
}

fn curve_world_with_query_signal(
    performed_signal_partition: &'static str,
    query_signal_mappings: [&'static str; 3],
    query_signal_partition: &'static str,
) -> (Vec<DeclaredDependency>, Vec<GranularInvalidationMutation>) {
    (
        vec![
            dependency(
                2,
                "CurveFacts",
                DeclaredLocality::ExactDetail("usd-rates", "5y"),
                "CurveZeroRateField",
                PROJECTED,
                performed_signal_partition,
                query_signal_mappings[0],
                query_signal_partition,
                true,
                0,
            ),
            dependency(
                3,
                "CurveFacts",
                DeclaredLocality::Unscoped,
                "CurveZeroRateField",
                PROJECTED,
                performed_signal_partition,
                query_signal_mappings[1],
                query_signal_partition,
                false,
                0,
            ),
            dependency(
                4,
                "CurveFacts",
                DeclaredLocality::WholePartition("usd-rates"),
                "CurveZeroRateField",
                PROJECTED,
                performed_signal_partition,
                query_signal_mappings[2],
                query_signal_partition,
                false,
                0,
            ),
        ],
        vec![mutation(
            "detail-change",
            "CurveFacts",
            "usd-rates",
            "5y",
            "CurveZeroRateField",
            10,
            true,
        )],
    )
}

#[allow(clippy::too_many_arguments)]
fn dependency(
    ordinal: usize,
    aspect: &'static str,
    locality: DeclaredLocality,
    field: &'static str,
    roles: &'static [&'static str],
    performed_signal_partition: &'static str,
    query_signal_mapping: &'static str,
    query_signal_partition: &'static str,
    performs_signal: bool,
    tolerance: u64,
) -> DeclaredDependency {
    DeclaredDependency {
        ordinal,
        aspect,
        locality,
        field,
        roles,
        performed_signal_partition,
        query_signal_mapping,
        query_signal_partition,
        performs_signal,
        tolerance,
    }
}

fn mutation(
    identity: &'static str,
    aspect: &'static str,
    partition: &'static str,
    detail: &'static str,
    field: &'static str,
    magnitude: u64,
    current: bool,
) -> GranularInvalidationMutation {
    GranularInvalidationMutation {
        identity,
        aspect,
        partition,
        detail,
        relational_record_identity: FINANCIAL_RECORD,
        field,
        magnitude,
        current,
    }
}

fn lifecycle_mutation(
    identity: &'static str,
    aspect: &'static str,
    partition: &'static str,
    detail: &'static str,
    field: &'static str,
    magnitude: u64,
    current: bool,
) -> GranularInvalidationMutation {
    GranularInvalidationMutation {
        relational_record_identity: LIFECYCLE_RECORD,
        ..mutation(
            identity, aspect, partition, detail, field, magnitude, current,
        )
    }
}
