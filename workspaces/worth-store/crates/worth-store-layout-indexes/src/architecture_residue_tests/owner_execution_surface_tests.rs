use super::{source, source_tree};

#[test]
fn certification_observes_owner_executions_without_a_generic_execution_receipt() {
    let execution = source("src/access/execution/mod.rs");
    let observation = source("src/access/execution/view.rs");

    assert!(!execution.contains("ExecutedAccessReceipt"));
    for removed in [
        "src/access/execution/lowering_facade.rs",
        "src/access/execution/physical_execution.rs",
        "src/access/execution/transition_authority.rs",
        "src/access/execution/executed_evidence.rs",
        "src/access/execution/lowered_plan.rs",
        "src/access/execution/ready_plan.rs",
        "src/access/execution/outcomes/mod.rs",
        "src/layout_counters.rs",
        "src/compile_fail",
    ] {
        assert!(!std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(removed)
            .exists());
    }
    assert!(std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/integrity/readmission/authority.rs")
        .exists());
    assert!(observation.contains("pub enum ExecutedLayoutOperation"));
    assert!(observation.contains("BTreeLookup(Box<crate::BaselineBTreeLookupExecution>)"));
    assert!(observation.contains("LsmLookup(Box<crate::BaselineLsmLookupExecution>)"));
    assert!(observation.contains("DegradedScan(Box<super::DegradedScanExecution>)"));
}

#[test]
fn selection_outcomes_expose_exact_capabilities_through_an_ordinary_owner_facade() {
    let outcome = source("src/planning/selection_outcome.rs").replace('\r', "");
    let runtime = source_tree("src/access/execution/degraded_scan/operation");
    let request_declaration = source("src/access/execution/degraded_scan/operation/request.rs");

    for extraction in [
        "into_btree_lookup",
        "into_btree_replay_recovery",
        "into_lsm_lookup",
        "into_lsm_run_publication",
        "into_lsm_replay_recovery",
        "into_lsm_compaction",
        "into_degraded",
    ] {
        assert!(
            outcome.contains(&format!("pub fn {extraction}")),
            "selection outcome does not expose exact extraction {extraction}"
        );
    }
    assert!(!outcome.contains("#[cfg(test)]\n    pub fn into_degraded"));
    for required in [
        "layout_declarations()",
        "admit_physical_artifact_family",
        "admit_physical_key_domain",
        "admit_page_key",
        "admit_current_catalog_root_materialization",
        "AccessPlanSelector.select_admitted_with_budget",
        ".into_degraded()",
        "classify_readiness",
        "execute_ready",
    ] {
        assert!(
            runtime.contains(required),
            "ordinary degraded owner facade omits lifecycle step {required}"
        );
    }
    for inaccessible_input in [
        "AdmittedPhysicalArtifactFamily",
        "AdmittedConcretePhysicalKey",
        "AdmittedLayoutMaterialization",
        "CurrentMaterializationFrontier",
    ] {
        assert!(
            !request_declaration.contains(inaccessible_input),
            "public degraded request requires inaccessible capability {inaccessible_input}"
        );
    }
}

#[test]
fn btree_traversal_plan_and_replay_source_share_store_authority_and_source_lease_owner() {
    let read_source = source("src/strategy/btree/execution/read_source.rs");
    let physical_root = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../worth-store-physical-isolation/src/root_protocol/root_kinds.rs"),
    )
    .unwrap();
    let isolation_request = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../worth-store-physical-isolation/src/recovery_source_lease/request.rs"),
    )
    .unwrap();

    assert!(read_source.contains(
        "plan.root().store_authority_identity() != self.witness.store_authority_identity()"
    ));
    assert!(read_source.contains("PhysicalReadPlanAdmissionDenial::StoreAuthorityMismatch"));
    assert!(physical_root.contains(
        "store_authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity"
    ));
    assert!(isolation_request.contains("pub struct RecoverySourceLeaseRequest"));
    assert!(isolation_request.contains("pub fn new("));
    assert!(isolation_request.contains("source_identity: [u8; 32]"));
}
