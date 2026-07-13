use worth_query::facade::runtime::{
    WorthQueryRuntimeFacadeFamily, WorthQueryRuntimeFamilySupport, WorthQueryRuntimeSupportProfile,
};
use worth_server::{
    WorthServerQueryDependencyAuditPathKind, WorthServerQueryDependencyClosurePosture,
    WorthServerQueryDependencyConsumerKitPosture, WorthServerQueryDependencyRuntimeReadiness,
};

#[path = "support/query_handoff/fixture.rs"]
mod query_handoff_fixture;
#[path = "support/query_handoff/runtime.rs"]
mod query_handoff_runtime;

use query_handoff_fixture::test_server;
use query_handoff_runtime::{ProfiledTestWorkspaceProvider, TestWorkspaceProvider};

#[test]
fn server_query_dependency_audit_consumes_production_owned_covered_path_inventory() {
    let server = test_server(TestWorkspaceProvider, false);

    let receipt = server.query_dependency_audit().run();
    let inventory = receipt.covered_path_inventory();
    let row_ids = receipt
        .rows()
        .iter()
        .map(|row| row.row_id().as_str())
        .collect::<Vec<_>>();

    assert_eq!(receipt.rows().len(), inventory.row_ids().len());
    assert_eq!(
        inventory.ordinary_row_count(),
        receipt
            .rows()
            .iter()
            .filter(|row| row.ordinary_path())
            .count()
    );
    assert_eq!(
        inventory.static_test_only_row_count(),
        receipt
            .rows()
            .iter()
            .filter(|row| !row.ordinary_path())
            .count()
    );
    assert_eq!(inventory.inventory_digest(), inventory.row_ids().join("|"));
    assert_eq!(
        row_ids,
        inventory
            .row_ids()
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        receipt
            .rows()
            .iter()
            .filter(|row| {
                row.path_kind()
                    == WorthServerQueryDependencyAuditPathKind::CertificationTestBackendSupport
            })
            .count(),
        inventory.static_test_only_row_count()
    );
    assert!(receipt.rows().iter().all(|row| !row.reason().is_empty()));
    assert!(!receipt.audit_digest().is_empty());
    assert!(receipt.ordinary_rows().iter().all(|row| row.scope_posture()
        != worth_server::WorthServerQueryDependencyScopePosture::Unclassified));
}

#[test]
fn server_paths_expose_real_query_support_pinning_provenance() {
    let server = test_server(TestWorkspaceProvider, false);

    let receipt = server.query_dependency_audit().run();

    for path_kind in [
        WorthServerQueryDependencyAuditPathKind::WorthNativeDirectRead,
        WorthServerQueryDependencyAuditPathKind::WorthNativeDirectState,
        WorthServerQueryDependencyAuditPathKind::WorthNativeDirectInspection,
        WorthServerQueryDependencyAuditPathKind::WorthNativeDirectProjection,
        WorthServerQueryDependencyAuditPathKind::CompatibilityHttpRead,
        WorthServerQueryDependencyAuditPathKind::QueryHandoffRead,
        WorthServerQueryDependencyAuditPathKind::QueryHandoffDownstreamDelivery,
    ] {
        let row = receipt
            .row(path_kind)
            .expect("read-shaped audit row should exist");
        let provenance = row
            .provenance()
            .support_pin()
            .expect("ordinary path should expose support pin provenance");
        assert_eq!(
            row.runtime_readiness(),
            WorthServerQueryDependencyRuntimeReadiness::QueryNineSevenSharedReadClosureReady
        );
        assert_eq!(
            row.closure_posture(),
            WorthServerQueryDependencyClosurePosture::Ready
        );
        assert_eq!(provenance.workspace_name(), "audit-workspace");
        assert!(!provenance.support_matrix_digest().is_empty());
        assert!(!provenance.support_snapshot_digest().is_empty());
        assert!(!provenance.contract_digest().is_empty());
        assert!(!provenance.report_digest().is_empty());
        assert_eq!(provenance.blocking_finding_count(), 0);
    }

    for path_kind in [
        WorthServerQueryDependencyAuditPathKind::WorthNativeDirectMutation,
        WorthServerQueryDependencyAuditPathKind::CompatibilityHttpMutation,
        WorthServerQueryDependencyAuditPathKind::QueryHandoffMutation,
    ] {
        let row = receipt
            .row(path_kind)
            .expect("mutation-shaped audit row should exist");
        let provenance = row
            .provenance()
            .support_pin()
            .expect("mutation path should expose support pin provenance");
        assert_eq!(
            row.runtime_readiness(),
            WorthServerQueryDependencyRuntimeReadiness::QueryNineSevenDeterministicSubmissionClosureReady
        );
        assert_eq!(
            row.closure_posture(),
            WorthServerQueryDependencyClosurePosture::Ready
        );
        assert_eq!(
            provenance.required_families(),
            &[
                WorthQueryRuntimeFacadeFamily::Write,
                WorthQueryRuntimeFacadeFamily::Submission,
                WorthQueryRuntimeFacadeFamily::Inspect,
            ]
        );
        assert_eq!(provenance.matched_required_count(), 3);
    }
}

#[test]
fn server_support_posture_and_boundary_rows_use_production_receipts() {
    let server = test_server(TestWorkspaceProvider, false);

    let receipt = server.query_dependency_audit().run();

    assert!(receipt.ordinary_rows().iter().all(|row| {
        matches!(
            row.consumer_kit_posture(),
            WorthServerQueryDependencyConsumerKitPosture::QuerySupportSnapshotAndPinningAdopted
                | WorthServerQueryDependencyConsumerKitPosture::QueryBoundaryAuditAdopted
        )
    }));

    let boundary_row = receipt
        .row(WorthServerQueryDependencyAuditPathKind::ServerConsumerBoundaryAudit)
        .expect("boundary audit row should exist");
    let boundary = boundary_row
        .provenance()
        .boundary_audit()
        .expect("boundary audit row should expose production inventory provenance");
    assert_eq!(
        boundary_row.runtime_readiness(),
        WorthServerQueryDependencyRuntimeReadiness::QueryNineEightConsumerKitClosureReady
    );
    assert_eq!(
        boundary_row.closure_posture(),
        WorthServerQueryDependencyClosurePosture::Ready
    );
    assert_eq!(boundary.finding_count(), 0);
    assert!(!boundary.source_inventory_identity().is_empty());
    assert!(!boundary.report_identity().is_empty());
    assert!(boundary
        .required_roots()
        .iter()
        .any(|root| root.contains("declaration_intake")));
    assert!(boundary
        .required_roots()
        .iter()
        .any(|root| root.contains("query_handoff")));
    assert!(boundary
        .required_roots()
        .iter()
        .any(|root| root.contains("compat_http")));
    assert!(boundary.source_paths().len() >= 4);

    let test_support_row = receipt
        .row(WorthServerQueryDependencyAuditPathKind::CertificationTestBackendSupport)
        .expect("static test support row should exist");
    let residue = test_support_row
        .provenance()
        .test_backend_residue()
        .expect("static support row should expose residue audit provenance");
    assert!(!test_support_row.ordinary_path());
    assert_eq!(
        test_support_row.consumer_kit_posture(),
        WorthServerQueryDependencyConsumerKitPosture::LocalFolklore
    );
    assert!(residue.finding_count() > 0);
    assert!(residue.scanned_file_count() > 0);
    assert!(!residue.report_identity().is_empty());
}

#[test]
fn hostile_support_profile_blocks_operation_runtime_closure_with_real_pin_provenance() {
    let server = test_server(
        ProfiledTestWorkspaceProvider::new(
            WorthQueryRuntimeSupportProfile::scaffold_backend_profile().with_family_support(
                WorthQueryRuntimeFamilySupport::unsupported(
                    WorthQueryRuntimeFacadeFamily::SharedRead,
                    "phase-one hostile audit fixture",
                ),
            ),
        ),
        false,
    );

    let receipt = server.query_dependency_audit().run();
    let row = receipt
        .row(WorthServerQueryDependencyAuditPathKind::WorthNativeDirectRead)
        .expect("read row should exist");
    let provenance = row
        .provenance()
        .support_pin()
        .expect("blocked row should still preserve pin provenance");

    assert!(!receipt.is_runtime_ready_for_phase_one());
    assert!(!receipt
        .rows_with_closure_posture(WorthServerQueryDependencyClosurePosture::Blocked)
        .is_empty());
    assert!(receipt.support_posture().blocked_row_count() > 0);
    assert_eq!(
        row.consumer_kit_posture(),
        WorthServerQueryDependencyConsumerKitPosture::QuerySupportSnapshotAndPinningBlocked
    );
    assert_eq!(provenance.workspace_name(), "audit-workspace");
    assert!(provenance.blocking_finding_count() > 0);
}
