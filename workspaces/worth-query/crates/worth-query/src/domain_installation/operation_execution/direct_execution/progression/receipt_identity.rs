//! Canonical identity of one completed direct execution receipt.

use crate::domain_installation::operation_identity_basis::{
    canonical_indexed_operation_material, canonical_operation_material, graph_call_kind_material,
    operation_result_state_material, operation_warning_material,
};
use crate::domain_installation::{WorthQueryConditionalProvenance, WorthQueryOperationResultState};
use crate::identity::hash_parts;
use crate::memory_workspace::WorthQuerySnapshotIdentity;

use super::{WorthQueryBoundGraphExecutionReceipt, WorthQueryOperationExecutionWarning};

pub(super) struct DirectExecutionIdentityInput<'a> {
    pub(super) binding_identity: &'a str,
    pub(super) capability_identity: u64,
    pub(super) execution_snapshot: &'a WorthQuerySnapshotIdentity,
    pub(super) result_state: WorthQueryOperationResultState,
    pub(super) warnings: &'a [WorthQueryOperationExecutionWarning],
    pub(super) graph_receipts: &'a [WorthQueryBoundGraphExecutionReceipt],
    pub(super) output_identity: &'a str,
    pub(super) conditional: &'a [WorthQueryConditionalProvenance],
    pub(super) domain_evidence:
        Option<&'a crate::domain_installation::WorthQueryAdmittedDomainEvidence>,
    pub(super) execution_resources:
        &'a crate::domain_installation::WorthQueryExecutionResourceAttemptEvidence,
}

pub(super) fn direct_execution_receipt_identity(input: DirectExecutionIdentityInput<'_>) -> String {
    let graph_evidence = canonical_indexed_operation_material(
        "direct.graph",
        input.graph_receipts.iter().map(|receipt| {
            canonical_operation_material(vec![
                ("graph.role", receipt.role().to_owned()),
                (
                    "graph.kind",
                    graph_call_kind_material(receipt.kind()).into(),
                ),
                ("graph.evidence", receipt.evidence_identity().to_owned()),
                (
                    "graph.projection",
                    receipt
                        .graph_read_product()
                        .map(|projection| projection.call_identity())
                        .unwrap_or("not-projected")
                        .to_owned(),
                ),
            ])
        }),
    );
    let warning_evidence = canonical_indexed_operation_material(
        "direct.warning",
        input.warnings.iter().map(operation_warning_material),
    );
    hash_parts(&[
        "worth_query_bound_execution_v1".into(),
        format!("binding:{}", input.binding_identity),
        format!("capability:{}", input.capability_identity),
        format!(
            "snapshot:{}",
            input.execution_snapshot.evidence_identity().as_str()
        ),
        format!(
            "result_state:{}",
            operation_result_state_material(Some(input.result_state))
        ),
        format!("warnings:{warning_evidence}"),
        format!("graph_evidence:{graph_evidence}"),
        format!("output:{}", input.output_identity),
        format!("resources:{}", input.execution_resources.identity()),
        format!(
            "domain_evidence:{}",
            input
                .domain_evidence
                .map(crate::domain_installation::WorthQueryAdmittedDomainEvidence::identity)
                .unwrap_or("not-required")
        ),
        format!(
            "conditional:{}",
            canonical_indexed_operation_material(
                "direct.conditional",
                input.conditional.iter().map(
                    crate::domain_installation::operation_execution::conditional_trace_semantic_material,
                ),
            )
        ),
    ])
}
