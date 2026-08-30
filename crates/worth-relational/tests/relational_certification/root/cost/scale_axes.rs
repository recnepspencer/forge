use super::world::supply_chain::{
    assert_oracle_matches, certified_supply_chain_world, commit_branch_batch_with_result,
    lower_cargo_footprint_batch, SupplyChainScale,
};
use worth_relational::facade::history::BranchId;
use worth_relational::facade::inspection::RelationalMvccCostScope;
use worth_relational::facade::transactions::WorkerIntentBatch;

#[test]
fn production_write_footprints_report_exact_records_and_local_publication_cost() {
    let scale = SupplyChainScale::standard();
    let (world, expected) = certified_supply_chain_world(scale);
    assert_oracle_matches(&world, &expected);
    let mut physical_costs = Vec::new();
    for footprint in [1, 64, 4_096] {
        let branch = BranchId(format!("footprint-{footprint}"));
        let (_, source) = world
            .runtime
            .observe_fork_source(&BranchId("main".to_owned()))
            .unwrap();
        world.runtime.fork_branch(branch.clone(), source).unwrap();
        let identity = world.runtime.branch_identity(&branch).unwrap();
        let scope = RelationalMvccCostScope::capture(&world.runtime, vec![identity]);
        let batch = lower_cargo_footprint_batch(&world.handles, scale, footprint);
        let committed = commit_branch_batch_with_result(&world.runtime, branch, batch);
        assert_eq!(
            committed.publication_summary().unwrap().patch_record_count,
            footprint
        );
        world
            .runtime
            .snapshots()
            .release_snapshot(&committed.snapshot)
            .unwrap();
        let cost = world.runtime.observe_mvcc_cost(&scope).unwrap();
        let delta = cost.sharing_cost_delta();
        assert_eq!(delta.transaction_validation_attempts, 1);
        assert_eq!(delta.candidate_preparations, 1);
        assert_eq!(delta.publication_attempts, 1);
        assert_eq!(delta.branch_population_scans, 0);
        assert_eq!(delta.copied_commit_envelopes, 0);
        physical_costs.push((
            footprint,
            delta.publication_touched_region_count,
            delta.publication_persistent_index_path_nodes,
            delta.copied_truth_bytes,
            delta.publication_new_authoritative_bytes,
        ));
    }
    assert!(physical_costs.windows(2).all(|pair| pair[0].1 == pair[1].1));
    assert!(physical_costs.windows(2).all(|pair| pair[0].2 == pair[1].2));
    assert!(physical_costs.windows(2).all(|pair| pair[0].3 == pair[1].3));
    let new_bytes = physical_costs
        .iter()
        .map(|sample| (sample.0 as f64, sample.4 as f64))
        .collect::<Vec<_>>();
    let (slope, intercept) = fitted_line(&new_bytes);
    assert!((2_200.0..=2_250.0).contains(&slope));
    assert!(maximum_residual(&new_bytes, slope, intercept) <= 256.0);
}

#[test]
fn selected_publication_cost_is_flat_through_ordinary_retained_histories() {
    assert_flat_history_samples(&[1, 64, 1_024]);
}

#[test]
#[ignore = "scheduled retained-root ceiling"]
fn selected_publication_cost_is_flat_through_documented_retention_ceiling() {
    // The owner caps all simultaneously retained retired roots at 4,096, and
    // the fresh Court world honestly consumes part of that shared capacity.
    // The largest repeatable integer-ratio fixture is therefore 1/63/3,969;
    // it preserves a geometric slope without bypassing owner backpressure.
    assert_flat_history_samples(&[1, 63, 3_969]);
}

fn assert_flat_history_samples(history_lengths: &[usize]) {
    let mut samples = Vec::new();
    for &history_len in history_lengths {
        let (mut world, _) = certified_supply_chain_world(SupplyChainScale::court());
        let scale = SupplyChainScale::court();
        let branch = BranchId(format!("history-{history_len}"));
        let (_, source) = world
            .runtime
            .observe_fork_source(&BranchId("main".to_owned()))
            .unwrap();
        world.runtime.fork_branch(branch.clone(), source).unwrap();
        let first_batch = lower_cargo_footprint_batch(&world.handles, scale, 1);
        let first = commit_branch_batch_with_result(&world.runtime, branch.clone(), first_batch);
        world
            .runtime
            .snapshots()
            .release_snapshot(&first.snapshot)
            .unwrap();
        let identity = world.runtime.branch_identity(&branch).unwrap();
        let (_, first_basis) = world.runtime.observe_branch(&identity).unwrap();
        let mut retained_bases = vec![world.runtime.retain_component_basis(&first_basis).unwrap()];
        for sequence in 1..history_len {
            let committed = commit_branch_batch_with_result(
                &world.runtime,
                branch.clone(),
                WorkerIntentBatch::new(format!("retained-history-{sequence}")),
            );
            world
                .runtime
                .snapshots()
                .release_snapshot(&committed.snapshot)
                .unwrap();
            let (_, basis) = world.runtime.observe_branch(&identity).unwrap();
            retained_bases.push(world.runtime.retain_component_basis(&basis).unwrap());
        }
        assert_eq!(retained_bases.len(), history_len);
        let scope = RelationalMvccCostScope::capture(&world.runtime, vec![identity]);
        let final_batch = lower_cargo_footprint_batch(&world.handles, scale, 64);
        let final_commit = commit_branch_batch_with_result(&world.runtime, branch, final_batch);
        let retained = world.runtime.run_branch_root_reclamation_pass();
        assert!(retained.roots_still_retained() >= history_len as u64);
        let delta = world
            .runtime
            .observe_mvcc_cost(&scope)
            .unwrap()
            .sharing_cost_delta();
        samples.push((
            delta.publication_touched_region_count,
            delta.publication_persistent_index_path_nodes,
            delta.branch_population_scans,
            delta.copied_commit_envelopes,
            delta.transaction_validation_attempts,
            delta.retained_history_head_lookups,
            delta.candidate_preparations,
            delta.publication_attempts,
        ));
        for retained_basis in retained_bases {
            world
                .runtime
                .release_component_basis(retained_basis)
                .unwrap();
        }
        world
            .runtime
            .snapshots()
            .release_snapshot(&final_commit.snapshot)
            .unwrap();
    }
    assert!(samples.windows(2).all(|pair| pair[0] == pair[1]));
}

fn fitted_line(samples: &[(f64, f64)]) -> (f64, f64) {
    let count = samples.len() as f64;
    let mean_x = samples.iter().map(|sample| sample.0).sum::<f64>() / count;
    let mean_y = samples.iter().map(|sample| sample.1).sum::<f64>() / count;
    let numerator = samples
        .iter()
        .map(|sample| (sample.0 - mean_x) * (sample.1 - mean_y))
        .sum::<f64>();
    let denominator = samples
        .iter()
        .map(|sample| (sample.0 - mean_x).powi(2))
        .sum::<f64>();
    let slope = numerator / denominator;
    (slope, mean_y - slope * mean_x)
}

fn maximum_residual(samples: &[(f64, f64)], slope: f64, intercept: f64) -> f64 {
    samples
        .iter()
        .map(|sample| (sample.1 - (intercept + slope * sample.0)).abs())
        .fold(0.0, f64::max)
}
