use crate::publication_digest::hash_parts;
use worth_query_execution::facade::domain_computation::{
    canonical_indexed_operation_material, canonical_operation_material,
};

use super::super::super::{
    WorthQueryAdmittedDomainEvidenceSidecar, WorthQueryAdmittedStructuralCounter,
    WorthQueryCandidateRecord, WorthQueryCandidateRecordDisposition,
    WorthQueryDecisionCausalParent, WorthQueryDecisionRecord, WorthQueryTransformationRecord,
};
use super::vocabulary::{disposition_name, error_name};

pub(super) fn counter_sidecar_digest(records: &[WorthQueryAdmittedStructuralCounter]) -> String {
    hash_parts(&[canonical_indexed_operation_material(
        "domain.evidence.optional-counter",
        records.iter().map(super::core::counter_material),
    )])
}

pub(super) fn decision_sidecar_digest(records: &[WorthQueryDecisionRecord]) -> String {
    hash_parts(&[canonical_indexed_operation_material(
        "domain.evidence.decision.record",
        records.iter().map(decision_record_material),
    )])
}

pub(super) fn candidate_sidecar_digest(records: &[WorthQueryCandidateRecord]) -> String {
    hash_parts(&[canonical_indexed_operation_material(
        "domain.evidence.candidate.record",
        records.iter().map(candidate_record_material),
    )])
}

pub(super) fn transformation_sidecar_digest(records: &[WorthQueryTransformationRecord]) -> String {
    hash_parts(&[canonical_indexed_operation_material(
        "domain.evidence.transformation.record",
        records.iter().map(transformation_record_material),
    )])
}

pub(super) fn sidecar_material<T>(sidecar: &WorthQueryAdmittedDomainEvidenceSidecar<T>) -> String {
    match sidecar {
        WorthQueryAdmittedDomainEvidenceSidecar::NotApplicable => "not-applicable".into(),
        WorthQueryAdmittedDomainEvidenceSidecar::Omitted => "omitted".into(),
        WorthQueryAdmittedDomainEvidenceSidecar::DigestOnly { digest } => {
            format!("digest-only:{digest}")
        }
        WorthQueryAdmittedDomainEvidenceSidecar::Materialized { digest, .. } => {
            format!("materialized:{digest}")
        }
        WorthQueryAdmittedDomainEvidenceSidecar::PartiallyMaterialized { digest, .. } => {
            format!("partially-materialized:{digest}")
        }
    }
}

fn decision_record_material(record: &WorthQueryDecisionRecord) -> String {
    canonical_operation_material(vec![
        ("kind", record.kind().as_str().into()),
        ("reason", record.reason_family().into()),
        ("artifact.family", record.artifact_key_family().into()),
        ("artifact.key", record.artifact_key().into()),
        (
            "causal_parent",
            causal_parent_material(record.causal_parent()),
        ),
        ("payload.version", record.payload_version().to_string()),
        ("payload", record.payload().into()),
        ("recovery", record.recovery_relevant().to_string()),
    ])
}

fn causal_parent_material(parent: &WorthQueryDecisionCausalParent) -> String {
    match parent {
        WorthQueryDecisionCausalParent::None => "none".into(),
        WorthQueryDecisionCausalParent::Single(identity) => {
            canonical_operation_material(vec![("single", identity.clone())])
        }
        WorthQueryDecisionCausalParent::Ordered(identities) => {
            canonical_indexed_operation_material("parent", identities.iter().cloned())
        }
    }
}

fn candidate_record_material(record: &WorthQueryCandidateRecord) -> String {
    canonical_operation_material(vec![
        ("identity", record.identity().into()),
        (
            "disposition",
            match record.disposition() {
                WorthQueryCandidateRecordDisposition::Considered => "considered",
                WorthQueryCandidateRecordDisposition::Rejected => "rejected",
                WorthQueryCandidateRecordDisposition::Incumbent => "incumbent",
            }
            .into(),
        ),
    ])
}

fn transformation_record_material(record: &WorthQueryTransformationRecord) -> String {
    canonical_operation_material(vec![
        ("source", record.source_occurrence_identity().into()),
        (
            "outputs",
            canonical_indexed_operation_material(
                "output",
                record.output_occurrence_identities().iter().cloned(),
            ),
        ),
        ("disposition", disposition_name(record.disposition()).into()),
        ("error", error_name(record.error()).into()),
    ])
}
