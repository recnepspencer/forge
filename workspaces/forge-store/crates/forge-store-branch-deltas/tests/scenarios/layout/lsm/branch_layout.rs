use forge_store_branch_deltas::{
    branch_semantic_authority, reject_branch_delta_read_plan,
    BranchDeltaLayoutAccessDenialKind, BranchDeltaLayerId, BranchDeltaReadPlan,
    BranchDeltaReadRequest,
};

#[test]
fn phase23_branch_delta_family_binds_layers_to_admitted_lineage_support() {
    let request = BranchDeltaReadRequest::new(BranchDeltaLayerId(7), "main/feature-a");
    let plan = BranchDeltaReadPlan::new(request, 24);
    let witness = branch_semantic_authority().admit_same_branch_descendant("main/feature-a");

    let report = witness
        .admit_branch_delta_layout(&plan)
        .expect("admitted branch delta layer");
    assert_eq!(
        report.family_id(),
        forge_store_contracts::DurableArtifactFamilyId::BranchDeltaArtifact
    );
    assert_eq!(report.layer_id(), BranchDeltaLayerId(7));
    assert_eq!(report.branch_lineage(), "main/feature-a");
    assert_eq!(report.declared_delta_rows(), 24);
    assert_eq!(report.support_estimate().planned_range_lookups(), 1);
    assert_eq!(report.support_estimate().planned_maintenance_reads(), 1);
    assert_eq!(report.support_estimate().planned_range_steps(), 24);

    let denial = reject_branch_delta_read_plan(&plan).unwrap_err();
    assert_eq!(
        denial.kind(),
        BranchDeltaLayoutAccessDenialKind::BranchDeltaPlanCannotStandInForLayoutAuthority
    );

    let denial = branch_semantic_authority()
        .admit_same_branch_descendant("main/feature-b")
        .admit_branch_delta_layout(&plan)
        .unwrap_err();
    assert_eq!(
        denial.kind(),
        BranchDeltaLayoutAccessDenialKind::BranchDeltaLineageDoesNotMatchWitness
    );
}
