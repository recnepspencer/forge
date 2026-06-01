use crate::facade::{certify_milestone_three_closeout, milestone_three_closeout_requirements};
use crate::validation::reference_integrity::milestone_one_runtime_builder;

#[test]
fn milestone_three_closeout_requires_derived_reuse_legality_rows() {
    let requirements = milestone_three_closeout_requirements();
    let report = certify_milestone_three_closeout(
        || {
            milestone_one_runtime_builder()
                .expect(" milestone one runtime builder")
                .build()
        },
        "milestone-three-derived-reuse-legality",
    )
    .expect("milestone three closeout should certify");

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
