use crate::domain_installation::operation_identity_basis::{
    canonical_indexed_operation_material, canonical_operation_material,
};
use crate::identity::hash_parts;
use worth_query_installation::facade::{
    WorthQueryCandidateOptimalityPosture, WorthQueryCandidateSearchPosture,
    WorthQuerySourceOutputCorrespondence, WorthQueryTransformationDisposition,
    WorthQueryTransformationErrorPosture, WorthQueryTransformationLossPosture,
};

use super::{
    WorthQueryAdmittedDomainEvidenceSidecar, WorthQueryCandidateFeasibilityClass,
    WorthQueryCandidateIncumbentDisposition, WorthQueryCandidateRecord,
    WorthQueryCandidateRecordDisposition, WorthQueryCandidateSearchSummary,
    WorthQueryCandidateTerminationClass, WorthQueryDecisionCausalParent, WorthQueryDecisionRecord,
    WorthQueryDomainEvidenceBinding, WorthQueryDomainEvidenceCore, WorthQueryTransformationRecord,
};

pub(super) fn domain_evidence_identity(
    contract_identity: &str,
    binding: &WorthQueryDomainEvidenceBinding,
    core: &WorthQueryDomainEvidenceCore,
    decision_sidecar: &WorthQueryAdmittedDomainEvidenceSidecar<WorthQueryDecisionRecord>,
    candidate_sidecar: &WorthQueryAdmittedDomainEvidenceSidecar<WorthQueryCandidateRecord>,
    transformation_sidecar: &WorthQueryAdmittedDomainEvidenceSidecar<
        WorthQueryTransformationRecord,
    >,
) -> String {
    hash_parts(&[
        "worth_query_admitted_domain_evidence_v1".into(),
        format!("contract:{contract_identity}"),
        format!("binding:{}", binding_material(binding)),
        format!("core:{}", core_material(core)),
        format!("decision_sidecar:{}", sidecar_material(decision_sidecar)),
        format!("candidate_sidecar:{}", sidecar_material(candidate_sidecar)),
        format!(
            "transformation_sidecar:{}",
            sidecar_material(transformation_sidecar)
        ),
    ])
}

pub(crate) fn domain_evidence_core_material(core: &WorthQueryDomainEvidenceCore) -> String {
    core_material(core)
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

fn binding_material(binding: &WorthQueryDomainEvidenceBinding) -> String {
    canonical_operation_material(vec![
        ("operation", binding.operation_identity().into()),
        ("binding", binding.binding_identity().into()),
        (
            "run",
            binding.run_identity().unwrap_or("not-required").into(),
        ),
        (
            "stage",
            binding.stage_identity().unwrap_or("not-required").into(),
        ),
        ("basis", binding.basis_identity().into()),
        ("snapshot", binding.execution_snapshot_identity().into()),
        ("output", binding.output_occurrence_identity().into()),
    ])
}

fn core_material(core: &WorthQueryDomainEvidenceCore) -> String {
    canonical_operation_material(vec![
        (
            "counters",
            canonical_indexed_operation_material(
                "domain.evidence.counter",
                core.counters().iter().map(|counter| {
                    canonical_operation_material(vec![
                        ("name", counter.schema().name().as_str().into()),
                        ("initial", counter.initial().to_string()),
                        ("observed", counter.observed().to_string()),
                        (
                            "provider_certification",
                            counter
                                .provider_certification()
                                .unwrap_or("not-required")
                                .into(),
                        ),
                    ])
                }),
            ),
        ),
        (
            "decisions",
            canonical_indexed_operation_material(
                "domain.evidence.decision.summary",
                core.decisions().iter().map(|summary| {
                    let counts = summary.counts();
                    canonical_operation_material(vec![
                        ("kind", summary.schema().kind().as_str().into()),
                        ("occurrences", counts.occurrence_count().to_string()),
                        ("parents", counts.causal_parent_count().to_string()),
                        ("artifacts", counts.affected_artifact_count().to_string()),
                        ("recovery", counts.recovery_relevant_count().to_string()),
                    ])
                }),
            ),
        ),
        (
            "candidate_search",
            core.candidate_search()
                .map(candidate_summary_material)
                .unwrap_or_else(|| "not-applicable".into()),
        ),
        (
            "transformation",
            core.transformation()
                .map(|summary| {
                    let parts = summary.parts();
                    canonical_operation_material(vec![
                        ("source.family", parts.source_occurrence.family().into()),
                        ("source.value", parts.source_occurrence.value().into()),
                        ("output", parts.output_occurrence_identity.clone()),
                        ("family", parts.transformation_family.clone()),
                        ("version", parts.transformation_version.to_string()),
                        (
                            "correspondence",
                            correspondence_name(parts.correspondence).into(),
                        ),
                        ("disposition", disposition_name(parts.disposition).into()),
                        ("error", error_name(parts.error).into()),
                        ("loss", loss_name(parts.loss).into()),
                    ])
                })
                .unwrap_or_else(|| "not-applicable".into()),
        ),
        ("authority", "descriptive-only".into()),
    ])
}

fn candidate_summary_material(summary: &WorthQueryCandidateSearchSummary) -> String {
    let parts = summary.parts();
    canonical_operation_material(vec![
        ("universe.family", parts.universe.family().into()),
        ("universe.value", parts.universe.value().into()),
        ("considered", parts.considered_count.to_string()),
        ("termination.family", parts.termination_family.clone()),
        ("termination", termination_name(parts.termination).into()),
        ("completeness", search_name(&parts.completeness)),
        ("feasibility.family", parts.feasibility_family.clone()),
        ("feasibility", feasibility_name(parts.feasibility).into()),
        (
            "comparison.family",
            parts.comparison_authority.family().into(),
        ),
        (
            "comparison.value",
            parts.comparison_authority.value().into(),
        ),
        ("optimality", optimality_name(&parts.optimality)),
        ("rejected", parts.rejected_count.to_string()),
        ("incumbent.family", parts.incumbent_family.clone()),
        ("incumbent", incumbent_name(parts.incumbent).into()),
    ])
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

fn sidecar_material<T>(sidecar: &WorthQueryAdmittedDomainEvidenceSidecar<T>) -> String {
    match sidecar {
        WorthQueryAdmittedDomainEvidenceSidecar::NotApplicable => "not-applicable".into(),
        WorthQueryAdmittedDomainEvidenceSidecar::Omitted => "omitted".into(),
        WorthQueryAdmittedDomainEvidenceSidecar::DigestOnly { digest } => {
            format!("digest-only:{digest}")
        }
        WorthQueryAdmittedDomainEvidenceSidecar::Materialized { digest, .. } => {
            format!("materialized:{digest}")
        }
    }
}

fn termination_name(value: WorthQueryCandidateTerminationClass) -> &'static str {
    match value {
        WorthQueryCandidateTerminationClass::Completed => "completed",
        WorthQueryCandidateTerminationClass::Exhausted => "exhausted",
        WorthQueryCandidateTerminationClass::BoundReached => "bound-reached",
        WorthQueryCandidateTerminationClass::SampleCompleted => "sample-completed",
        WorthQueryCandidateTerminationClass::HeuristicStop => "heuristic-stop",
        WorthQueryCandidateTerminationClass::Interrupted => "interrupted",
    }
}

fn feasibility_name(value: WorthQueryCandidateFeasibilityClass) -> &'static str {
    match value {
        WorthQueryCandidateFeasibilityClass::NotApplicable => "not-applicable",
        WorthQueryCandidateFeasibilityClass::NoFeasibleCandidate => "none-feasible",
        WorthQueryCandidateFeasibilityClass::FeasibleCandidateFound => "feasible-found",
        WorthQueryCandidateFeasibilityClass::AllConsideredFeasible => "all-considered-feasible",
        WorthQueryCandidateFeasibilityClass::Unknown => "unknown",
    }
}

fn incumbent_name(value: WorthQueryCandidateIncumbentDisposition) -> &'static str {
    match value {
        WorthQueryCandidateIncumbentDisposition::NotApplicable => "not-applicable",
        WorthQueryCandidateIncumbentDisposition::None => "none",
        WorthQueryCandidateIncumbentDisposition::Selected => "selected",
        WorthQueryCandidateIncumbentDisposition::Reused => "reused",
        WorthQueryCandidateIncumbentDisposition::RejectedAll => "rejected-all",
    }
}

fn search_name(value: &WorthQueryCandidateSearchPosture) -> String {
    match value {
        WorthQueryCandidateSearchPosture::NotApplicable => "not-applicable".into(),
        WorthQueryCandidateSearchPosture::Exhaustive => "exhaustive".into(),
        WorthQueryCandidateSearchPosture::ProvenTopK { count } => {
            format!("proven-top-k:{count}")
        }
        WorthQueryCandidateSearchPosture::Bounded { bound_identity } => {
            format!("bounded:{bound_identity}")
        }
        WorthQueryCandidateSearchPosture::Sampled { sample_identity } => {
            format!("sampled:{sample_identity}")
        }
        WorthQueryCandidateSearchPosture::Heuristic => "heuristic".into(),
        WorthQueryCandidateSearchPosture::Incomplete => "incomplete".into(),
    }
}

fn optimality_name(value: &WorthQueryCandidateOptimalityPosture) -> String {
    match value {
        WorthQueryCandidateOptimalityPosture::NotApplicable => "not-applicable".into(),
        WorthQueryCandidateOptimalityPosture::ProvenOptimal => "proven-optimal".into(),
        WorthQueryCandidateOptimalityPosture::ProvenTopK { count } => {
            format!("proven-top-k:{count}")
        }
        WorthQueryCandidateOptimalityPosture::BoundedGap { bound_identity } => {
            format!("bounded-gap:{bound_identity}")
        }
        WorthQueryCandidateOptimalityPosture::BestInDeclaredSample { sample_identity } => {
            format!("best-in-sample:{sample_identity}")
        }
        WorthQueryCandidateOptimalityPosture::ParetoForDeclaredSet { set_identity } => {
            format!("pareto:{set_identity}")
        }
        WorthQueryCandidateOptimalityPosture::FeasibleOnly => "feasible-only".into(),
        WorthQueryCandidateOptimalityPosture::Unknown => "unknown".into(),
    }
}

fn correspondence_name(value: WorthQuerySourceOutputCorrespondence) -> &'static str {
    match value {
        WorthQuerySourceOutputCorrespondence::OneToOne => "one-to-one",
        WorthQuerySourceOutputCorrespondence::OneToMany => "one-to-many",
        WorthQuerySourceOutputCorrespondence::ManyToOne => "many-to-one",
        WorthQuerySourceOutputCorrespondence::ManyToMany => "many-to-many",
        WorthQuerySourceOutputCorrespondence::Partial => "partial",
    }
}

fn disposition_name(value: WorthQueryTransformationDisposition) -> &'static str {
    match value {
        WorthQueryTransformationDisposition::Preserved => "preserved",
        WorthQueryTransformationDisposition::Normalized => "normalized",
        WorthQueryTransformationDisposition::Approximated => "approximated",
        WorthQueryTransformationDisposition::Repaired => "repaired",
        WorthQueryTransformationDisposition::Omitted => "omitted",
        WorthQueryTransformationDisposition::Unsupported => "unsupported",
        WorthQueryTransformationDisposition::Quarantined => "quarantined",
    }
}

fn error_name(value: WorthQueryTransformationErrorPosture) -> &'static str {
    match value {
        WorthQueryTransformationErrorPosture::Exact => "exact",
        WorthQueryTransformationErrorPosture::Bounded => "bounded",
        WorthQueryTransformationErrorPosture::Estimated => "estimated",
        WorthQueryTransformationErrorPosture::Unknown => "unknown",
    }
}

fn loss_name(value: WorthQueryTransformationLossPosture) -> &'static str {
    match value {
        WorthQueryTransformationLossPosture::Lossless => "lossless",
        WorthQueryTransformationLossPosture::DeclaredLossy => "declared-lossy",
        WorthQueryTransformationLossPosture::LossClassifiedByDomain => "domain-classified",
    }
}
