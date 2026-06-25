use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters, TopologyRuntimePostureCapability,
    TopologyRuntimePostureStatus, TopologyRuntimeSupport,
};
use crate::validation::reference_integrity::build_milestone_one_runtime;
use forge_query::facade::{
    ForgeQueryBranchOptions, ForgeQueryIntentDeclaration, ForgeQuerySessionLabel,
};

#[test]
fn current_head_runtime_posture_rows_freeze_admitted_and_denied_capabilities() {
    let support = TopologyRuntimeSupport::current_head_authoritative();

    assert_eq!(
        support.runtime_posture_rows().len(),
        TopologyRuntimePostureCapability::ALL.len()
    );
    for capability in TopologyRuntimePostureCapability::ALL {
        let row = support
            .runtime_posture_rows()
            .iter()
            .find(|row| row.capability() == capability)
            .expect("current-head posture row should exist");
        let expected_status = match capability {
            TopologyRuntimePostureCapability::CurrentHeadLiveReads
            | TopologyRuntimePostureCapability::PostWriteMaterialization
            | TopologyRuntimePostureCapability::BranchPreviewBasis
            | TopologyRuntimePostureCapability::AuthoritativeWrites => {
                TopologyRuntimePostureStatus::Admitted
            }
            TopologyRuntimePostureCapability::CurrentHeadMaterialization
            | TopologyRuntimePostureCapability::BranchLocalIntentStaging
            | TopologyRuntimePostureCapability::BranchLocalDeclarationExecution
            | TopologyRuntimePostureCapability::HistoricalBasis => {
                TopologyRuntimePostureStatus::Denied
            }
        };
        assert_eq!(row.status(), expected_status);
        assert!(!row.row_digest().is_empty());
    }
}

#[test]
fn snapshot_runtime_posture_rows_freeze_historical_read_only_capabilities() {
    let support = TopologyRuntimeSupport::snapshot_read_only();

    assert_eq!(
        support.runtime_posture_rows().len(),
        TopologyRuntimePostureCapability::ALL.len()
    );
    for capability in TopologyRuntimePostureCapability::ALL {
        let expected_status = match capability {
            TopologyRuntimePostureCapability::HistoricalBasis => {
                TopologyRuntimePostureStatus::Admitted
            }
            TopologyRuntimePostureCapability::CurrentHeadLiveReads
            | TopologyRuntimePostureCapability::CurrentHeadMaterialization
            | TopologyRuntimePostureCapability::PostWriteMaterialization
            | TopologyRuntimePostureCapability::BranchPreviewBasis
            | TopologyRuntimePostureCapability::BranchLocalIntentStaging
            | TopologyRuntimePostureCapability::BranchLocalDeclarationExecution
            | TopologyRuntimePostureCapability::AuthoritativeWrites => {
                TopologyRuntimePostureStatus::Denied
            }
        };
        assert_eq!(support.runtime_posture_status(capability), expected_status);
    }
}

#[test]
fn current_head_runtime_admits_preview_and_branch_sessions() {
    let runtime = build_milestone_one_runtime().expect("runtime");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, ".runtime-posture.branch-preview").expect("workspace");

    let preview = workspace
        .preview(ForgeQuerySessionLabel::scoped_strs("topology", ["preview"]).expect("label"))
        .expect("preview session");
    assert_eq!(preview.basis_admission().label(), "topology.preview");

    let branch = workspace
        .branch(ForgeQuerySessionLabel::scoped_strs("topology", ["branch"]).expect("label"))
        .expect("branch session");
    assert_eq!(branch.basis_admission().label(), "topology.branch");
}

#[test]
fn current_head_runtime_admits_branch_sessions_but_denies_branch_local_intent_staging_and_topology_declaration_execution(
) {
    let runtime = build_milestone_one_runtime().expect("runtime");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let support = adapters.support().clone();
    let mut workspace =
        topology_runtime(adapters, ".runtime-posture.branch-intent").expect("workspace");

    assert_eq!(
        support.runtime_posture_status(TopologyRuntimePostureCapability::BranchLocalIntentStaging),
        TopologyRuntimePostureStatus::Denied
    );
    assert_eq!(
        support.runtime_posture_status(
            TopologyRuntimePostureCapability::BranchLocalDeclarationExecution
        ),
        TopologyRuntimePostureStatus::Denied
    );

    let mut branch = workspace
        .branch_with_options(
            ForgeQuerySessionLabel::scoped_strs("topology", ["branch-intent"]).expect("label"),
            ForgeQueryBranchOptions::sandboxed_write_intent(),
        )
        .expect("branch session should be admitted");
    let error = branch
        .execute_intent(ForgeQueryIntentDeclaration::strategy_commit(
            "topology-branch-stage",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            forge_query::facade::ForgeQueryIntentInput::object([(
                "entity",
                forge_query::facade::ForgeQueryIntentInput::string("topology-branch-stage"),
            )]),
        ))
        .expect_err("branch-local intent staging should remain denied");
    assert!(error
        .to_string()
        .contains("intent commit strategies are not admitted by this runtime batch"));
}
