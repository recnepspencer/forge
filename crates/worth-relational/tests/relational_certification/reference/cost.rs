use super::world::supply_chain::SupplyChainScale;
use super::world::supply_chain::{assert_oracle_matches, certified_supply_chain_world};
use worth_relational::facade::history::BranchId;

#[test]
fn phase4_reference_cost_probe_separates_setup_and_operation_work() {
    for fanout in [1usize, 64, 512] {
        let (mut world, expected) = certified_supply_chain_world(SupplyChainScale::court());
        assert_oracle_matches(&world, &expected);
        let setup = world.runtime.phase4_reference_cost_counters();
        let setup_catalog = world.runtime.history().immutable_commit_count();
        let after_setup_query = world.runtime.phase4_reference_cost_counters();
        assert_eq!(
            after_setup_query.branch_population_scans - setup.branch_population_scans,
            0,
            "immutable commit counting uses indexed canonical routes, not a root scan"
        );
        let mut previous = after_setup_query;
        for _ in 0..fanout {
            let (_, source_basis) = world
                .runtime
                .observe_fork_source(&BranchId("main".to_owned()))
                .expect("main remains a live fork source");
            world
                .runtime
                .fork_branch(
                    BranchId(format!("probe-{fanout}-{}", previous.reference_allocations)),
                    source_basis,
                )
                .expect("metadata-only fork succeeds");
            let current = world.runtime.phase4_reference_cost_counters();
            assert_eq!(
                current.branch_cell_lookups - previous.branch_cell_lookups,
                2
            );
            assert_eq!(current.catalog_lookups - previous.catalog_lookups, 1);
            assert_eq!(current.artifact_clones - previous.artifact_clones, 0);
            assert_eq!(
                current.branch_population_scans - previous.branch_population_scans,
                0,
                "fork operation must not scan the branch population"
            );
            assert_eq!(
                current.reference_allocations - previous.reference_allocations,
                1
            );
            assert_eq!(
                current.branch_cell_contacts - previous.branch_cell_contacts,
                3
            );
            previous = current;
        }
        assert_eq!(
            world.runtime.history().immutable_commit_count(),
            setup_catalog,
            "metadata-only forks must not append catalog artifacts"
        );
    }
}
