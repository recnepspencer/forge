#![allow(dead_code)]

use forge_server::{
    ForgeServerAbuseBudgetReceipt, ForgeServerBinaryCounterSet, ForgeServerCompatHttpRouteFamily,
    ForgeServerExternalCounterSet, ForgeServerTransferByteClass,
    ForgeServerTransferCleanupEvidence, ForgeServerTransferCleanupReason,
};

pub(crate) fn assert_budget_receipt(
    receipt: &ForgeServerAbuseBudgetReceipt,
    expected_route_family: ForgeServerCompatHttpRouteFamily,
    expected_byte_class: ForgeServerTransferByteClass,
    expected_denial_fragment: Option<&str>,
) {
    assert_eq!(receipt.route_family(), expected_route_family);
    assert_eq!(receipt.byte_class(), expected_byte_class);
    match expected_denial_fragment {
        Some(fragment) => assert!(
            receipt
                .denial()
                .expect("expected denial detail")
                .contains(fragment),
            "expected denial detail to contain `{fragment}`"
        ),
        None => assert!(receipt.denial().is_none()),
    }
}

pub(crate) fn assert_budget_scope(
    receipt: &ForgeServerAbuseBudgetReceipt,
    expected_tenant_id: &str,
    expected_workspace_id: &str,
    expected_branch_id: &str,
) {
    let expected_workspace_digest =
        format!("forge-server-workspace-target-v1:{expected_tenant_id}:{expected_workspace_id}");
    let expected_branch_digest =
        format!("forge-server-branch-target-v1:branch:{expected_branch_id}");
    assert_eq!(receipt.tenant_id(), expected_tenant_id);
    assert_eq!(receipt.workspace_digest(), expected_workspace_digest);
    assert_eq!(receipt.branch_digest(), expected_branch_digest);
    let canonical_digest = receipt.canonical_digest();
    assert!(canonical_digest.contains(&format!("tenant={expected_tenant_id}")));
    assert!(canonical_digest.contains(&format!("workspace={expected_workspace_digest}")));
    assert!(canonical_digest.contains(&format!("branch={expected_branch_digest}")));
}

pub(crate) fn assert_external_counter(
    counters: &ForgeServerExternalCounterSet,
    name: &str,
    expected: u64,
) {
    assert_eq!(
        counters.counter(name),
        Some(expected),
        "external counter `{name}`"
    );
}

pub(crate) fn assert_binary_counter(
    counters: &ForgeServerBinaryCounterSet,
    name: &str,
    expected: u64,
) {
    assert_eq!(
        counters.counter(name),
        Some(expected),
        "binary counter `{name}`"
    );
}

pub(crate) fn assert_cleanup_evidence(
    evidence: &ForgeServerTransferCleanupEvidence,
    expected_route_family: ForgeServerCompatHttpRouteFamily,
    expected_byte_class: ForgeServerTransferByteClass,
    expected_reason: ForgeServerTransferCleanupReason,
) {
    assert_eq!(evidence.route_family(), expected_route_family);
    assert_eq!(evidence.byte_class(), expected_byte_class);
    assert_eq!(evidence.reason(), expected_reason);
    assert!(evidence.attachment_bundle().provenance().is_some());
    assert!(evidence.attachment_bundle().receipt().is_some());
}

pub(crate) fn assert_cleanup_scope(
    evidence: &ForgeServerTransferCleanupEvidence,
    expected_tenant_id: &str,
    expected_workspace_id: &str,
    expected_branch_id: &str,
) {
    let expected_workspace_digest =
        format!("forge-server-workspace-target-v1:{expected_tenant_id}:{expected_workspace_id}");
    let expected_branch_digest =
        format!("forge-server-branch-target-v1:branch:{expected_branch_id}");
    assert_eq!(evidence.tenant_id(), expected_tenant_id);
    assert_eq!(evidence.workspace_digest(), expected_workspace_digest);
    assert_eq!(evidence.branch_digest(), expected_branch_digest);
    let canonical_digest = evidence.canonical_digest();
    assert!(canonical_digest.contains(&format!("tenant={expected_tenant_id}")));
    assert!(canonical_digest.contains(&format!("workspace={expected_workspace_digest}")));
    assert!(canonical_digest.contains(&format!("branch={expected_branch_digest}")));
}
