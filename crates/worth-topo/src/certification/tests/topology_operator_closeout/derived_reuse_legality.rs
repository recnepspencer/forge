use crate::facade::milestone_three_closeout_requirements;

#[test]
fn milestone_three_closeout_requires_derived_reuse_legality_rows() {
    let requirements = milestone_three_closeout_requirements();
    let report = crate::certification::test_support::cached_milestone_three_closeout_report();

    assert_eq!(
        report.derived_reuse_legality_rows.len(),
        requirements.required_family_rows.len()
    );
    assert!(report.derived_reuse_legality_rows.iter().all(|row| {
        !row.recompute_suppression_claimed()
            && !row.equivalence_contract_required()
            && row.replay_materialized_topology_equivalent()
            && row
                .row_digest()
                .starts_with(&format!("scenario={};", row.scenario().as_str()))
    }));
}
