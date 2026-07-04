use super::closeout::current_worth_workload_ordinary_consumer_sweep_closeout;
use super::workload_composition_explainer_ledger::WorthWorkloadCompositionExplainerDisposition;

#[test]
fn workload_composition_explainers_import_planner_owned_lanes() {
    let source_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/workload_composition/worth_workload/ordinary_consumer_sweep/closeout.rs");
    let source = std::fs::read_to_string(&source_path)
        .expect("ordinary sweep closeout source should remain readable");

    assert!(
        source.contains("current_worth_touched_graph_conflict_public_facade_with_artifact_policy"),
        "workload-composition status surface must load the planner-owned public facade directly"
    );
    assert!(
        source.contains("WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy::MinimalOperationalTruth"),
        "workload-composition status surface must name the planner-owned diagnostic artifact policy directly"
    );
    assert!(
        !source.contains("current_worth_touched_graph_conflict_public_closeout,"),
        "workload-composition status surface must not import the legacy public-closeout helper directly"
    );
    assert!(
        !source.contains("current_worth_touched_graph_conflict_milestone_fifteen_seed"),
        "workload-composition status surface must not reopen seed lowering outside the planner-owned facade"
    );
}

#[test]
fn workload_composition_local_explainers_are_deleted_or_capped() {
    let closeout = current_worth_workload_ordinary_consumer_sweep_closeout()
        .expect("ordinary sweep closeout should build");
    let explainer_ledger = closeout.workload_composition_explainer_ledger();

    assert_eq!(explainer_ledger.migrated_count(), 1);
    assert_eq!(explainer_ledger.capped_residue_count(), 0);
    assert_eq!(explainer_ledger.query_gap_count(), 0);

    let explainer_row = explainer_ledger
        .rows()
        .iter()
        .find(|row| row.surface_name() == "current_worth_workload_ordinary_consumer_sweep_closeout")
        .expect("workload-composition explainer row should be counted independently");
    assert_eq!(
        explainer_row.disposition(),
        WorthWorkloadCompositionExplainerDisposition::MigratedOrdinaryConsumer
    );
    assert_eq!(
        explainer_row.source_path(),
        "crates/worth-kernel/src/workload_composition/worth_workload/ordinary_consumer_sweep/closeout.rs"
    );

    let public_closeout_cluster = closeout
        .cluster_ledgers()
        .iter()
        .find(|ledger| ledger.cluster_kind().as_str() == "public-closeout")
        .expect("public-closeout cluster should still exist");
    assert!(
        public_closeout_cluster
            .rows()
            .iter()
            .all(|row| row.surface_name()
                != "current_worth_workload_ordinary_consumer_sweep_closeout"),
        "public-closeout debt must not count the workload-composition explainer surface"
    );
    assert!(
        closeout
            .residue_rows()
            .iter()
            .all(|row| row.surface_name()
                != "current_worth_workload_ordinary_consumer_sweep_closeout"),
        "ordinary workload-composition explainers must not survive as uncapped residue rows"
    );
}
