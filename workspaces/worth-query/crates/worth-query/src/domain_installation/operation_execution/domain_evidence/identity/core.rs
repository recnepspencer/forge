use crate::domain_installation::operation_identity_basis::{
    canonical_indexed_operation_material, canonical_operation_material,
};

use super::super::super::{
    WorthQueryCandidateSearchSummary, WorthQueryDomainEvidenceBinding,
    WorthQueryDomainEvidenceCore, WorthQueryDomainEvidenceGovernance,
};
use super::vocabulary::{
    classification_name, correspondence_name, deletion_name, disposition_name, error_name,
    feasibility_name, incumbent_name, legal_hold_name, loss_name, optimality_name, redaction_name,
    retention_name, search_name, termination_name,
};

pub(super) fn binding_material(binding: &WorthQueryDomainEvidenceBinding) -> String {
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

pub(super) fn governance_material(governance: &WorthQueryDomainEvidenceGovernance) -> String {
    canonical_operation_material(vec![
        (
            "audiences",
            canonical_indexed_operation_material(
                "domain.evidence.governance.audience",
                governance.audiences().iter().cloned(),
            ),
        ),
        (
            "classification",
            classification_name(governance.classification()).into(),
        ),
        ("redaction", redaction_name(governance.redaction()).into()),
        ("retention", retention_name(governance.retention()).into()),
        ("deletion", deletion_name(governance.deletion()).into()),
        (
            "legal-hold",
            legal_hold_name(governance.legal_hold()).into(),
        ),
    ])
}

pub(super) fn core_material(core: &WorthQueryDomainEvidenceCore) -> String {
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
