struct MilestoneThirteenBoundary {
    responsibility: &'static str,
    source: &'static str,
    required_symbols: &'static [&'static str],
}

const OPERATIONAL_BOUNDARIES: &[MilestoneThirteenBoundary] = &[
    MilestoneThirteenBoundary {
        responsibility: "producer-wide direct subscriber collection",
        source: include_str!("../../../../logic/invalidation/routing/seeds.rs"),
        required_symbols: &["collect_live_subscribers_into", "runtime_subscribers_of"],
    },
    MilestoneThirteenBoundary {
        responsibility: "contract and scope admission after subscriber collection",
        source: include_str!("../../../../logic/invalidation/routing/planning.rs"),
        required_symbols: &["cares_about_change", "subscriber_invalidation_evidence"],
    },
    MilestoneThirteenBoundary {
        responsibility: "legacy transitive subscriber closure walk",
        source: include_str!("../../../../logic/invalidation/routing/application.rs"),
        required_symbols: &["execute_transitive_wave", "runtime_subscribers_of"],
    },
    MilestoneThirteenBoundary {
        responsibility: "legacy predicted counter authority",
        source: include_str!("../../../../data/proof/invalidation/plan.rs"),
        required_symbols: &["FrontierPredictedCounters", "pub fn new"],
    },
    MilestoneThirteenBoundary {
        responsibility: "legacy execution summary and counter authority",
        source: include_str!("../../../../data/proof/invalidation/execution.rs"),
        required_symbols: &["FrontierExecutionCounters", "FrontierExecutionSummary"],
    },
    MilestoneThirteenBoundary {
        responsibility: "runtime invalidation telemetry projection",
        source: include_str!("../../../../data/telemetry/execution.rs"),
        required_symbols: &[
            "direct_subscriber_candidates_examined",
            "direct_contract_rejections",
            "direct_causality_rejections",
            "invalidation_nodes_visited",
            "transitive_frontier_width",
        ],
    },
    MilestoneThirteenBoundary {
        responsibility: "producer-only reverse subscriber membership",
        source: include_str!("../../../../data/graph/topology/subscriber_edges.rs"),
        required_symbols: &["set_subscribers_sorted", "raw_subscribers_of"],
    },
    MilestoneThirteenBoundary {
        responsibility: "atomic producer output publication",
        source: include_str!("../../../../data/graph/runtime/effect/output_commit.rs"),
        required_symbols: &["CommittedProducedAspectDelta", "publish_output_commit"],
    },
    MilestoneThirteenBoundary {
        responsibility: "checkpoint cause and source-basis readmission",
        source: include_str!("../../../../logic/invalidation/causality/revalidation.rs"),
        required_symbols: &["readmit_checkpoint_causes", "node_invalidation_input"],
    },
    MilestoneThirteenBoundary {
        responsibility: "checkpoint graph authority reconstruction",
        source: include_str!("../../../../data/graph/runtime/graph/checkpoint.rs"),
        required_symbols: &[
            "restore_from_checkpoint_authority",
            "rebuild_checkpoint_topology",
        ],
    },
];

const PUBLIC_REACHABILITY_BOUNDARIES: &[MilestoneThirteenBoundary] = &[
    MilestoneThirteenBoundary {
        responsibility: "legacy public frontier plan constructors",
        source: include_str!("../../../../data/proof/invalidation/plan.rs"),
        required_symbols: &["pub struct FrontierPlan", "pub fn new"],
    },
    MilestoneThirteenBoundary {
        responsibility: "legacy public frontier execution constructors",
        source: include_str!("../../../../data/proof/invalidation/execution.rs"),
        required_symbols: &["pub struct FrontierExecutionSummary", "pub fn new"],
    },
    MilestoneThirteenBoundary {
        responsibility: "integration facade frontier re-exports",
        source: include_str!("../../../../facade/integration.rs"),
        required_symbols: &["FrontierExecutionSummary", "FrontierPlan"],
    },
    MilestoneThirteenBoundary {
        responsibility: "adapter facade frontier re-exports",
        source: include_str!("../../../../facade/adapters.rs"),
        required_symbols: &["FrontierExecutionSummary", "FrontierPlan"],
    },
    MilestoneThirteenBoundary {
        responsibility: "runtime dirty-batch entry point",
        source: include_str!("../../../../facade/runtime.rs"),
        required_symbols: &["mark_dirty_batch"],
    },
];

fn assert_inventory(boundaries: &[MilestoneThirteenBoundary]) {
    for boundary in boundaries {
        for symbol in boundary.required_symbols {
            assert!(
                boundary.source.contains(symbol),
                "{} no longer exposes inventoried symbol {symbol}",
                boundary.responsibility
            );
        }
    }
}

#[test]
fn phase_1_inventory_freezes_current_operational_cutover_boundaries() {
    assert_eq!(OPERATIONAL_BOUNDARIES.len(), 10);
    assert_inventory(OPERATIONAL_BOUNDARIES);
}

#[test]
fn phase_1_inventory_freezes_legacy_public_constructor_reachability() {
    assert_eq!(PUBLIC_REACHABILITY_BOUNDARIES.len(), 5);
    assert_inventory(PUBLIC_REACHABILITY_BOUNDARIES);
}
