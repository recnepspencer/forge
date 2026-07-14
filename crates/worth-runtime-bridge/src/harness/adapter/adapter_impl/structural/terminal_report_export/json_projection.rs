use std::collections::BTreeMap;

use serde_json::json;

use super::super::certification_bundle::{
    StructuralAmbiguityReport, StructuralDiffReport, StructuralHarnessCertificationBundle,
    StructuralIdentitySeparationReport, StructuralRetainedCandidateSet,
};
use super::super::counter_snapshot::StructuralHarnessCounterSnapshot;
use super::super::*;

pub(in crate::harness::adapter::adapter_impl) fn summary_json(
    execution: &StructuralHarnessExecution,
) -> serde_json::Value {
    let summary = execution.summary();
    json!({
        "structural_declaration_identity": summary.structural_declaration_identity.as_str(),
        "structural_contract_identity": summary.structural_contract_identity.as_str(),
        "structural_match_digest": summary.structural_match_digest,
        "structural_reuse_digest": summary.structural_reuse_digest,
        "branch_compare_digest": summary.branch_compare_digest,
        "replay_digest": summary.replay_digest,
        "diagnostics_digest": summary.diagnostics_digest,
        "failure_digest": summary.failure_digest,
        "outcome_class": format!("{:?}", summary.outcome_class),
        "counter_snapshot": counter_snapshot_json(&summary.counter_snapshot),
    })
}

pub(in crate::harness::adapter::adapter_impl) fn extensions_json(
    execution: &StructuralHarnessExecution,
    runtime_bridge: &crate::facade::RuntimeBridge,
) -> BTreeMap<String, serde_json::Value> {
    let mut extensions = BTreeMap::from([(
        "bridge_structural_certification_bundle".to_string(),
        certification_bundle_json(&execution.certification_bundle()),
    )]);
    match execution {
        StructuralHarnessExecution::Remap {
            contract,
            planned,
            reduced,
            artifact,
            record,
        }
        | StructuralHarnessExecution::RemapReplay {
            contract,
            planned,
            reduced,
            artifact,
            record,
            ..
        } => {
            let (key, value) = remap_record_extension(
                runtime_bridge,
                contract,
                planned,
                reduced,
                artifact,
                record,
            );
            extensions.insert(key, value);
        }
        StructuralHarnessExecution::Branch {
            contract,
            planned,
            reduced,
            artifact,
            record,
        }
        | StructuralHarnessExecution::BranchReplay {
            contract,
            planned,
            reduced,
            artifact,
            record,
            ..
        } => {
            let (key, value) = branch_record_extension(
                runtime_bridge,
                contract,
                planned,
                reduced,
                artifact,
                record,
            );
            extensions.insert(key, value);
        }
        StructuralHarnessExecution::Rejected { .. } => {}
    }
    extensions
}

fn certification_bundle_json(bundle: &StructuralHarnessCertificationBundle) -> serde_json::Value {
    json!({
        "structural_match_digest": bundle.structural_match_digest,
        "ambiguity_report": bundle
            .ambiguity_report
            .as_ref()
            .map(ambiguity_report_json),
        "remap_artifact_digest": bundle.remap_artifact_digest,
        "failure_digest": bundle.failure_digest,
        "structural_reuse_digest": bundle.structural_reuse_digest,
        "identity_separation_report": bundle
            .identity_separation_report
            .as_ref()
            .map(identity_separation_report_json),
        "replay_digest": bundle.replay_digest,
        "diagnostics_digest": bundle.diagnostics_digest,
        "branch_compare_digest": bundle.branch_compare_digest,
        "structural_diff_report": bundle.structural_diff_report.as_ref().map(structural_diff_report_json),
        "counter_snapshot": counter_snapshot_json(&bundle.counter_snapshot),
    })
}

fn remap_record_extension(
    runtime_bridge: &crate::facade::RuntimeBridge,
    contract: &AdmittedStructuralComparisonContract,
    planned: &PlannedStructuralMatchPacketSet,
    reduced: &ReducedStructuralMatchSet,
    artifact: &PublishedStructuralRemapArtifact,
    record: &BridgeCanonicalStructuralRemapRecord,
) -> (String, serde_json::Value) {
    let explanation = runtime_bridge
        .diagnostics()
        .explain_structural_remap_record(record);
    (
        "bridge_structural_remap_record".to_string(),
        json!({
            "record_identity": record.record_identity().as_str(),
            "structural_contract_identity": contract.contract_identity().as_str(),
            "structural_declaration_identity": declaration_identity(contract).as_str(),
            "planned_digest": planned.digest(),
            "reduced_digest": reduced.digest(),
            "artifact_digest": artifact.digest(),
            "outcome_class": format!("{:?}", reduced.outcome_class()),
            "explanation": {
                "record_identity": explanation.record_identity().as_str(),
                "declaration_identity": explanation.declaration_identity(),
                "semantics_version": explanation.semantics_version(),
                "candidate_count": explanation.candidate_count(),
                "outcome_class": format!("{:?}", explanation.outcome_class()),
            },
        }),
    )
}

fn branch_record_extension(
    runtime_bridge: &crate::facade::RuntimeBridge,
    contract: &AdmittedStructuralComparisonContract,
    planned: &PlannedStructuralMatchPacketSet,
    reduced: &ReducedStructuralMatchSet,
    artifact: &PublishedBranchComparisonArtifact,
    record: &BridgeCanonicalStructuralBranchComparisonRecord,
) -> (String, serde_json::Value) {
    let explanation = runtime_bridge
        .diagnostics()
        .explain_structural_branch_comparison_record(record);
    (
        "bridge_structural_branch_comparison_record".to_string(),
        json!({
            "record_identity": record.record_identity().as_str(),
            "structural_contract_identity": contract.contract_identity().as_str(),
            "structural_declaration_identity": declaration_identity(contract).as_str(),
            "planned_digest": planned.digest(),
            "reduced_digest": reduced.digest(),
            "artifact_digest": artifact.digest(),
            "branch_diff_count": reduced.branch_diff_count(),
            "explanation": {
                "record_identity": explanation.record_identity().as_str(),
                "declaration_identity": explanation.declaration_identity(),
                "semantics_version": explanation.semantics_version(),
                "candidate_count": explanation.candidate_count(),
                "branch_diff_count": explanation.branch_diff_count(),
            },
        }),
    )
}

fn ambiguity_report_json(report: &StructuralAmbiguityReport) -> serde_json::Value {
    json!({
        "outcome_class": format!("{:?}", report.outcome_class),
        "retained_candidates": retained_candidates_json(&report.retained_candidates),
    })
}

fn identity_separation_report_json(
    report: &StructuralIdentitySeparationReport,
) -> serde_json::Value {
    json!({
        "declaration_identity": report.declaration_identity.as_str(),
        "outcome_class": format!("{:?}", report.outcome_class),
        "retained_candidates": retained_candidates_json(&report.retained_candidates),
    })
}

fn structural_diff_report_json(report: &StructuralDiffReport) -> serde_json::Value {
    json!({
        "record_identity": report.record_identity.as_str(),
        "branch_diff_count": report.branch_diff_count,
        "retained_candidates": retained_candidates_json(&report.retained_candidates),
    })
}

fn retained_candidates_json(candidates: &StructuralRetainedCandidateSet) -> serde_json::Value {
    json!(candidates
        .candidates()
        .iter()
        .map(|candidate| candidate.identity())
        .collect::<Vec<_>>())
}

fn counter_snapshot_json(counters: &StructuralHarnessCounterSnapshot) -> serde_json::Value {
    json!({
        "structural_declaration_count": counters.structural_declaration_count,
        "structural_contract_count": counters.structural_contract_count,
        "structural_fingerprint_count": counters.structural_fingerprint_count,
        "structural_match_packet_count": counters.structural_match_packet_count,
        "structural_candidate_count": counters.structural_candidate_count,
        "structural_candidate_cohort_count": counters.structural_candidate_cohort_count,
        "structural_exact_match_count": counters.structural_exact_match_count,
        "structural_ambiguity_count": counters.structural_ambiguity_count,
        "structural_mismatch_count": counters.structural_mismatch_count,
        "structural_identity_conflict_count": counters.structural_identity_conflict_count,
        "structural_lineage_divergence_count": counters.structural_lineage_divergence_count,
        "structural_reuse_publication_count": counters.structural_reuse_publication_count,
        "branch_comparison_count": counters.branch_comparison_count,
        "branch_comparison_diff_count": counters.branch_comparison_diff_count,
        "branch_comparison_drift_rejection_count": counters.branch_comparison_drift_rejection_count,
        "structural_widened_scan_count": counters.structural_widened_scan_count,
        "structural_replay_request_count": counters.structural_replay_request_count,
        "structural_replay_mismatch_count": counters.structural_replay_mismatch_count,
    })
}
