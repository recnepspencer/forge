use crate::runtime::worker_host::{
    WorkerCompatibilityCertificationScenario, WorkerPortableGraphPublication,
};

use crate::runtime::tests::support::*;

pub(in crate::runtime::tests::worker_runtime) fn portable_dashboard_certification_scenario(
) -> WorkerCompatibilityCertificationScenario {
    WorkerCompatibilityCertificationScenario {
        publication: portable_dashboard_publication(),
        transaction_ops: dashboard_invalidation_burst(),
        feature_transaction_ops: vec![set_number("inventoryCount", 42.0)],
        main_transaction_ops: vec![set_number("trafficLoad", 0.64)],
        observed_signal_id: "dashboardView".to_owned(),
        async_signal_id: "dashboardView".to_owned(),
        async_payload_contract_id: 9001,
        async_payload_byte_len: 128,
        independent_region_recipe_ids: vec![
            "inventoryRegion".to_owned(),
            "trafficRegion".to_owned(),
            "financeRegion".to_owned(),
        ],
    }
}

pub(in crate::runtime::tests::worker_runtime) fn portable_dashboard_scenario_with_unpublished_region(
) -> WorkerCompatibilityCertificationScenario {
    WorkerCompatibilityCertificationScenario {
        independent_region_recipe_ids: vec![
            "inventoryRegion".to_owned(),
            "trafficRegion".to_owned(),
            "unpublishedRegion".to_owned(),
        ],
        ..portable_dashboard_certification_scenario()
    }
}

fn portable_dashboard_publication() -> WorkerPortableGraphPublication {
    WorkerPortableGraphPublication {
        policy: RuntimePolicySpec::default(),
        sources: vec![
            source("inventoryCount", 12.0),
            source("inventoryReserve", 5.0),
            source("trafficLoad", 0.21),
            source("trafficCapacity", 1.0),
            source("financeRevenue", 120.0),
            source("financeCost", 45.0),
        ],
        recipes: vec![
            recipe(
                "inventoryRegion",
                ["inventoryCount", "inventoryReserve"],
                Expr::Object {
                    fields: vec![
                        ("count".to_owned(), read("inventoryCount")),
                        ("reserve".to_owned(), read("inventoryReserve")),
                        (
                            "available".to_owned(),
                            Expr::Sum {
                                args: vec![read("inventoryCount"), read("inventoryReserve")],
                            },
                        ),
                    ],
                },
            ),
            recipe(
                "trafficRegion",
                ["trafficLoad", "trafficCapacity"],
                Expr::Object {
                    fields: vec![
                        ("load".to_owned(), read("trafficLoad")),
                        ("capacity".to_owned(), read("trafficCapacity")),
                    ],
                },
            ),
            recipe(
                "financeRegion",
                ["financeRevenue", "financeCost"],
                Expr::Object {
                    fields: vec![
                        ("revenue".to_owned(), read("financeRevenue")),
                        ("cost".to_owned(), read("financeCost")),
                        (
                            "gross".to_owned(),
                            Expr::Sum {
                                args: vec![read("financeRevenue"), read("financeCost")],
                            },
                        ),
                    ],
                },
            ),
            recipe(
                "dashboardView",
                ["inventoryRegion", "trafficRegion", "financeRegion"],
                Expr::Object {
                    fields: vec![
                        ("inventory".to_owned(), read("inventoryRegion")),
                        ("traffic".to_owned(), read("trafficRegion")),
                        ("finance".to_owned(), read("financeRegion")),
                    ],
                },
            ),
        ],
        output_ids: vec!["dashboardView".to_owned()],
    }
}

fn dashboard_invalidation_burst() -> Vec<TransactionOp> {
    vec![
        set_number("inventoryCount", 19.0),
        set_number("trafficLoad", 0.38),
        set_number("financeRevenue", 144.0),
    ]
}

fn source(id: &str, initial: f64) -> SourceSpec {
    SourceSpec {
        id: id.to_owned(),
        initial: SignalValue::Number(initial),
        produces_aspects: None,
    }
}

fn recipe<const N: usize>(id: &str, reads: [&str; N], expr: Expr) -> RecipeSpec {
    RecipeSpec {
        id: id.to_owned(),
        reads: reads
            .into_iter()
            .map(|read_id| RecipeReadSpec::LegacyId(read_id.to_owned()))
            .collect(),
        expr,
        when: None,
        identity: Some(IdentitySpec::Exact),
        produces_aspects: None,
    }
}

fn set_number(id: &str, value: f64) -> TransactionOp {
    TransactionOp::Set {
        id: id.to_owned(),
        value: SignalValue::Number(value),
        aspect: None,
        aspects: None,
    }
}
