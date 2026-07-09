use crate::facade::merge::{
    RelationalSchemaReconciliationBasisRow, RelationalSchemaReconciliationCorrespondenceLinkRow,
    RelationalSchemaReconciliationWitness, RelationalSchemaReconciliationWitnessDenial,
    RelationalSchemaReconciliationWitnessPosture, RelationalSchemaReconciliationWitnessRow,
};
use crate::facade::schema::{SchemaReconciliationClassification, SchemaReconciliationPolicy};
use crate::transactions::data::RecordRef;
use sha2::{Digest, Sha256};

#[derive(serde::Serialize)]
pub(super) struct SchemaWitnessPayload<'a> {
    request_digest: &'a str,
    branch_basis_digest: &'a str,
    rows: Vec<SchemaWitnessRowPayload<'a>>,
    witness_digest: String,
}

#[derive(serde::Serialize)]
pub(super) struct SchemaWitnessRowPayload<'a> {
    record: &'a RecordRef,
    target_record: Option<&'a RecordRef>,
    basis: &'a RelationalSchemaReconciliationBasisRow,
    source_only_aspect_count: usize,
    target_only_aspect_count: usize,
    divergent_aspect_count: usize,
    unavailable_aspect_count: usize,
    decision_boundary: crate::facade::merge::MergePolicyDecisionBoundary,
    relation_endpoint_divergence: bool,
    correspondence_linkage: Option<&'a RelationalSchemaReconciliationCorrespondenceLinkRow>,
    classification: SchemaReconciliationClassification,
    policy: Option<SchemaReconciliationPolicy>,
    denial: Option<RelationalSchemaReconciliationWitnessDenial>,
    posture: RelationalSchemaReconciliationWitnessPosture,
    row_digest: &'a str,
}

pub(super) fn witness_payload<'a>(
    witness: &'a RelationalSchemaReconciliationWitness,
    rows: Vec<SchemaWitnessRowPayload<'a>>,
    witness_digest: Option<&str>,
) -> SchemaWitnessPayload<'a> {
    let witness_digest = witness_digest.map(str::to_string).unwrap_or_else(|| {
        schema_witness_digest(
            witness.request_digest(),
            witness.branch_basis_digest(),
            &rows,
        )
    });
    SchemaWitnessPayload {
        request_digest: witness.request_digest(),
        branch_basis_digest: witness.branch_basis_digest(),
        rows,
        witness_digest,
    }
}

pub(super) fn row_payloads<'a>(
    rows: &'a [RelationalSchemaReconciliationWitnessRow],
    classification: Option<SchemaReconciliationClassification>,
    posture: Option<RelationalSchemaReconciliationWitnessPosture>,
    denial: Option<RelationalSchemaReconciliationWitnessDenial>,
) -> Vec<SchemaWitnessRowPayload<'a>> {
    rows.iter()
        .map(|row| SchemaWitnessRowPayload {
            record: row.record(),
            target_record: row.target_record(),
            basis: row.basis(),
            source_only_aspect_count: row.source_only_aspect_count(),
            target_only_aspect_count: row.target_only_aspect_count(),
            divergent_aspect_count: row.divergent_aspect_count(),
            unavailable_aspect_count: row.unavailable_aspect_count(),
            decision_boundary: row.decision_boundary(),
            relation_endpoint_divergence: row.relation_endpoint_divergence(),
            correspondence_linkage: row.correspondence_linkage(),
            classification: classification.unwrap_or(row.classification()),
            policy: row.policy(),
            denial: denial.or(row.denial()),
            posture: posture.unwrap_or(row.posture()),
            row_digest: row.row_digest(),
        })
        .collect()
}

fn schema_witness_digest(
    request_digest: &str,
    branch_basis_digest: &str,
    rows: &[SchemaWitnessRowPayload<'_>],
) -> String {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"WORTH.relational.merge.schema_reconciliation_witness.v1");
    bytes.extend_from_slice(request_digest.as_bytes());
    bytes.extend_from_slice(branch_basis_digest.as_bytes());
    bytes.extend_from_slice(rows.len().to_string().as_bytes());
    for row in rows {
        bytes.extend_from_slice(&rmp_serde::to_vec_named(row).expect("encode row payload"));
    }
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}
