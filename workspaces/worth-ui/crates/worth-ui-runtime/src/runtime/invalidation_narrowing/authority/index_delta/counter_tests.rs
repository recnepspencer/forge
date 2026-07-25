#[test]
fn shared_host_key_fanout_has_constant_semantic_work_and_logarithmic_index_work() {
    const NARROW_WIDTH: usize = 16;
    const WIDE_WIDTH: usize = 512;
    let narrow = exercise_shared_witness_delta(NARROW_WIDTH);
    let wide = exercise_shared_witness_delta(WIDE_WIDTH);

    assert_eq!(narrow.row_visits(), wide.row_visits());
    assert_eq!(narrow.membership_mutations(), wide.membership_mutations());
    assert_eq!(narrow.owner_mutations(), wide.owner_mutations());
    assert_eq!(wide.row_visits(), 2);
    assert_eq!(wide.membership_mutations(), 2);
    assert_eq!(wide.owner_mutations(), 4);
    let logical_mutations = wide.membership_mutations() + wide.owner_mutations();
    let fanout_growth_steps = (WIDE_WIDTH / NARROW_WIDTH).ilog2() as usize;
    let balanced_path_growth_bound = 2 * logical_mutations * fanout_growth_steps;
    assert!(
        wide.persistent_key_probes() >= narrow.persistent_key_probes()
            && wide.persistent_key_probes() - narrow.persistent_key_probes()
                <= balanced_path_growth_bound,
        "persistent lookup work must grow by balanced-tree depth, not shared-key fanout"
    );
    assert!(
        wide.persistent_node_copies() >= narrow.persistent_node_copies()
            && wide.persistent_node_copies() - narrow.persistent_node_copies()
                <= balanced_path_growth_bound,
        "copy-on-write work must grow by balanced-tree depth, not shared-key fanout"
    );
}

fn exercise_shared_witness_delta(width: usize) -> super::UiDerivedIndexDeltaCounters {
    let (runtime, roots, _) = crate::runtime::tests::production_catalog_activation_test_support::runtime_with_viewport_catalog(width);
    let predecessor = runtime.allocation_invalidation_index.borrow().clone();
    let row = predecessor
        .catalog
        .row_for_root(roots[0])
        .expect("production catalog contains changed row")
        .clone();
    let basis = &row.committed_invalidation_context().basis;
    let request = basis
        .host_allocation_requests()
        .next()
        .expect("viewport row owns one host request");
    let witness = basis
        .host_measurement_result(request)
        .expect("viewport row owns admitted host evidence")
        .authority_witness();

    let mut removal = predecessor.clone();
    let removal_work = removal
        .apply_index_delta(std::slice::from_ref(&row), &[])
        .expect("one exact predecessor row is removable");
    assert_eq!(removal_work.row_visits(), 1);
    assert_eq!(
        removal
            .host_target(witness)
            .expect("shared witness remains admitted")
            .target_count(),
        width - 1,
        "removing one owner cannot drop the other shared-key owners"
    );

    let mut replacement = predecessor;
    replacement
        .apply_index_delta(std::slice::from_ref(&row), std::slice::from_ref(&row))
        .expect("one-row replacement updates the same persistent keys");
    replacement
        .apply_index_delta(std::slice::from_ref(&row), std::slice::from_ref(&row))
        .expect("repeated one-row replacement retains exact ownership");
    replacement
        .apply_index_delta(std::slice::from_ref(&row), std::slice::from_ref(&row))
        .expect("persistent ownership does not drift across repetition")
}
