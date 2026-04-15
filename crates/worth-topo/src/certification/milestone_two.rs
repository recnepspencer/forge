use std::collections::BTreeMap;

use forge_relational::facade::runtime::{RelationalReadView, RelationalRuntime};
use worth_schema::facade::{DerivedTopologyReadBasis, VerifiedTopologyCommit};

use crate::certification::bridge::certify_milestone_one_bridge_proof;
use crate::certification::corpus::certify_milestone_one_default_primitive_corpus_impl;
use crate::certification::error::WorthMilestoneOneCertificationError;
use crate::certification::requirements::milestone_two_closeout_requirements;
use crate::certification::read_view::WorthMilestoneOneCertificationHarness;
use crate::certification::report::{
    WorthDerivedFamilyCoverageMatrix, WorthDerivedFamilyCoverageRow,
    WorthDerivedEquivalenceContractAggregateReport, WorthDerivedEquivalenceContractAggregateRow,
    WorthDerivedFallbackAggregateReport, WorthDerivedFallbackAggregateRow,
    WorthDerivedInvalidationAggregateReport, WorthDerivedInvalidationAggregateRow,
    WorthDerivedFamilyParityMatrix, WorthDerivedFamilyParityRow, WorthDeterministicDigest,
    WorthDerivedValidatorCoverageReport, WorthDerivedValidatorCoverageRow,
    WorthMilestoneOneCertificationReport,
    WorthMilestoneTwoCloseoutReport,
    WorthMilestoneTwoBranchLocalParityReport, WorthMilestoneTwoReplayParityReport,
    WorthMilestoneTwoCounters, WorthMilestoneTwoDerivedCorpusReport,
    WorthMilestoneTwoDerivedReadReport, WorthPrimitiveCorpusParityReport, WorthPrimitiveCorpusReport,
    WorthDerivedRebuildAggregateReport, WorthDerivedRebuildAggregateRow, WorthFailureLocalityReport,
};
use crate::certification::shared::digest_rows;

pub fn certify_milestone_two_read_view_impl(
    read_view: &RelationalReadView,
    read_basis: DerivedTopologyReadBasis,
) -> Result<WorthMilestoneTwoDerivedReadReport, WorthMilestoneOneCertificationError> {
    let report = WorthMilestoneOneCertificationHarness::certify_read_view(read_view, read_basis)?;
    Ok(build_milestone_two_read_report(&report))
}

pub fn certify_milestone_two_verified_commit_impl(
    runtime: &mut RelationalRuntime,
    verified: &VerifiedTopologyCommit,
) -> Result<WorthMilestoneTwoDerivedReadReport, WorthMilestoneOneCertificationError> {
    let report = WorthMilestoneOneCertificationHarness::certify_verified_commit(runtime, verified)?;
    Ok(build_milestone_two_read_report(&report))
}

pub fn certify_milestone_two_default_derived_corpus_impl<F>(
    mut runtime_factory: F,
    stem: &str,
) -> Result<WorthMilestoneTwoDerivedCorpusReport, WorthMilestoneOneCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let primitive_corpus =
        certify_milestone_one_default_primitive_corpus_impl(&mut runtime_factory, stem)?;
    let bridge_proof_report = certify_milestone_one_bridge_proof(&format!("{stem}.bridge"))?;

    Ok(WorthMilestoneTwoDerivedCorpusReport {
        materialized_topology_digest: aggregate_derived_digest(&primitive_corpus, |report| {
            report
                .derived_equivalence_contract_report
                .materialized_topology_digest
                .clone()
        }),
        interpreted_topology_digest: aggregate_derived_digest(&primitive_corpus, |report| {
            report
                .derived_equivalence_contract_report
                .interpreted_topology_digest
                .clone()
        }),
        derived_validation_digest: aggregate_derived_digest(&primitive_corpus, |report| {
            report
                .derived_equivalence_contract_report
                .derived_validation_digest
                .clone()
        }),
        derived_truth_basis_digest: aggregate_truth_basis_digest(&primitive_corpus),
        derived_family_coverage_matrix: build_derived_family_coverage_matrix(&primitive_corpus),
        derived_family_parity_matrix: build_derived_family_parity_matrix(
            &primitive_corpus.parity_report,
        ),
        derived_branch_local_parity_report: build_derived_branch_local_report(&primitive_corpus),
        derived_replay_parity_report: build_derived_replay_report(&primitive_corpus),
        derived_bridge_family_coverage_report: bridge_proof_report.family_coverage_report.clone(),
        bridge_routing_digest: bridge_proof_report.bridge_routing_digest.clone(),
        bridge_historical_evaluation_digest: bridge_proof_report
            .bridge_historical_evaluation_digest
            .clone(),
        milestone_2_counter_report: build_milestone_two_counter_report(&primitive_corpus),
        primitive_corpus,
        bridge_proof_report,
    })
}

pub fn certify_milestone_two_closeout_impl<F>(
    runtime_factory: F,
    stem: &str,
) -> Result<WorthMilestoneTwoCloseoutReport, WorthMilestoneOneCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let requirements = milestone_two_closeout_requirements();
    let derived_corpus = certify_milestone_two_default_derived_corpus_impl(runtime_factory, stem)?;
    let primitive_corpus = &derived_corpus.primitive_corpus;

    let closeout = WorthMilestoneTwoCloseoutReport {
        materialized_topology_digest: derived_corpus.materialized_topology_digest.clone(),
        interpreted_topology_digest: derived_corpus.interpreted_topology_digest.clone(),
        derived_validation_digest: derived_corpus.derived_validation_digest.clone(),
        derived_truth_basis_digest: derived_corpus.derived_truth_basis_digest.clone(),
        bridge_routing_digest: derived_corpus.bridge_routing_digest.clone(),
        bridge_historical_evaluation_digest: derived_corpus
            .bridge_historical_evaluation_digest
            .clone(),
        derived_family_coverage_matrix: derived_corpus.derived_family_coverage_matrix.clone(),
        derived_family_parity_matrix: derived_corpus.derived_family_parity_matrix.clone(),
        derived_validator_coverage_report: build_derived_validator_coverage_report(primitive_corpus),
        derived_invalidation_report: build_derived_invalidation_aggregate_report(primitive_corpus),
        derived_rebuild_report: build_derived_rebuild_aggregate_report(primitive_corpus),
        derived_equivalence_contract_report: build_derived_equivalence_aggregate_report(
            primitive_corpus,
        ),
        derived_fallback_report: build_derived_fallback_aggregate_report(primitive_corpus),
        derived_failure_locality_report: build_derived_failure_locality_report(primitive_corpus),
        derived_branch_local_parity_report: derived_corpus
            .derived_branch_local_parity_report
            .clone(),
        derived_replay_parity_report: derived_corpus.derived_replay_parity_report.clone(),
        derived_bridge_family_coverage_report: derived_corpus
            .derived_bridge_family_coverage_report
            .clone(),
        milestone_2_counter_report: derived_corpus.milestone_2_counter_report.clone(),
        derived_corpus,
    };

    ensure_milestone_two_family_coverage_closure(
        &closeout.derived_family_coverage_matrix,
        &requirements,
    )?;
    ensure_milestone_two_parity_closure(&closeout.derived_family_parity_matrix, &requirements)?;
    ensure_milestone_two_validator_closure(
        &closeout.derived_validator_coverage_report,
        &requirements,
    )?;
    ensure_milestone_two_bridge_closure(
        &closeout.derived_bridge_family_coverage_report,
        &requirements,
    )?;
    ensure_milestone_two_failure_locality_closure(
        &closeout.derived_failure_locality_report,
        &requirements,
    )?;
    ensure_milestone_two_required_output_closure(&closeout, &requirements)?;

    Ok(closeout)
}

fn build_milestone_two_read_report(
    report: &WorthMilestoneOneCertificationReport,
) -> WorthMilestoneTwoDerivedReadReport {
    WorthMilestoneTwoDerivedReadReport {
        materialized_topology_digest: report
            .derived_equivalence_contract_report
            .materialized_topology_digest
            .clone(),
        interpreted_topology_digest: report
            .derived_equivalence_contract_report
            .interpreted_topology_digest
            .clone(),
        derived_validation_digest: report
            .derived_equivalence_contract_report
            .derived_validation_digest
            .clone(),
        derived_invalidation_report: report.derived_invalidation_report.clone(),
        derived_rebuild_report: report.derived_rebuild_report.clone(),
        derived_fallback_report: report.derived_fallback_report.clone(),
        derived_equivalence_contract_report: report.derived_equivalence_contract_report.clone(),
        derived_branch_local_parity_report: report.branch_local_topology_report.clone(),
        derived_replay_parity_report: report.milestone_1_replay_parity_report.clone(),
        milestone_2_counter_report: WorthMilestoneTwoCounters {
            derived_read_count: 1,
            touched_aspect_count: report.derived_invalidation_report.touched_aspect_count,
            triggered_invalidation_target_count: report
                .derived_invalidation_report
                .triggered_target_count,
            validation_row_count: report.topology_validation_report.rows.len(),
            whole_view_rebuild_count: usize::from(report.derived_rebuild_report.whole_view_rebuild),
            explicit_fallback_count: report.derived_fallback_report.explicit_fallback_count,
            replay_checked_count: usize::from(
                report
                    .milestone_1_replay_parity_report
                    .relational_replay_checked,
            ),
            branch_local_case_count: usize::from(report.branch_local_topology_report.branch_local),
        },
        read_artifact: report.read_artifact.clone(),
        certified_interpretation: report.certified_interpretation.clone(),
    }
}

fn aggregate_derived_digest(
    primitive_corpus: &WorthPrimitiveCorpusReport,
    select: impl Fn(&WorthMilestoneOneCertificationReport) -> WorthDeterministicDigest,
) -> WorthDeterministicDigest {
    digest_rows(primitive_corpus.cases.iter().map(|case| {
        let digest = select(&case.certification);
        format!(
            "{}:{}:{}:{}",
            case.stem, digest.algorithm, digest.digest_hex, digest.row_count
        )
    }))
}

fn aggregate_truth_basis_digest(
    primitive_corpus: &WorthPrimitiveCorpusReport,
) -> WorthDeterministicDigest {
    digest_rows(primitive_corpus.cases.iter().map(|case| {
        let report = &case.certification.derived_equivalence_contract_report;
        format!(
            "{}:{}:{}:{}:{}:{}",
            case.stem,
            report.authority_snapshot_id,
            report.authority_branch_id,
            report.truth_basis_digest_hex,
            report.touched_aspect_count,
            report.triggered_invalidation_targets.len()
        )
    }))
}

fn build_derived_family_coverage_matrix(
    primitive_corpus: &WorthPrimitiveCorpusReport,
) -> WorthDerivedFamilyCoverageMatrix {
    WorthDerivedFamilyCoverageMatrix {
        rows: primitive_corpus
            .coverage_matrix
            .entries
            .iter()
            .map(|entry| WorthDerivedFamilyCoverageRow {
                family: entry.family.clone(),
                admitted_case_count: entry.admitted_smallest_count
                    + entry.admitted_generic_count
                    + entry.admitted_hostile_count,
                out_of_class_rejection_count: entry.rejected_out_of_class_count,
                coverage_complete: entry.role_closure_complete,
            })
            .collect(),
    }
}

fn build_derived_family_parity_matrix(
    parity_report: &WorthPrimitiveCorpusParityReport,
) -> WorthDerivedFamilyParityMatrix {
    WorthDerivedFamilyParityMatrix {
        rows: parity_report
            .entries
            .iter()
            .map(|entry| WorthDerivedFamilyParityRow {
                family: entry.family.clone(),
                mainline_case_count: entry.mainline_case_count,
                branch_local_case_count: entry.branch_local_case_count,
                replay_verified_case_count: entry.mainline_replay_verified_case_count,
                branch_local_replay_verified_case_count: entry
                    .branch_local_replay_verified_case_count,
                cross_branch_parity_case_count: entry.cross_branch_parity_case_count,
                parity_complete: entry.parity_closure_complete,
            })
            .collect(),
    }
}

fn build_derived_branch_local_report(
    primitive_corpus: &WorthPrimitiveCorpusReport,
) -> WorthMilestoneTwoBranchLocalParityReport {
    let branch_ids = primitive_corpus
        .parity_report
        .entries
        .iter()
        .flat_map(|entry| entry.branch_ids.clone())
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let mainline_case_count = primitive_corpus
        .parity_report
        .entries
        .iter()
        .map(|entry| entry.mainline_case_count)
        .sum();
    let branch_local_case_count = primitive_corpus
        .parity_report
        .entries
        .iter()
        .map(|entry| entry.branch_local_case_count)
        .sum();

    WorthMilestoneTwoBranchLocalParityReport {
        mainline_case_count,
        branch_local_case_count,
        branch_ids,
        branch_local_closure_complete: primitive_corpus
            .parity_report
            .entries
            .iter()
            .all(|entry| entry.parity_closure_complete),
    }
}

fn build_derived_replay_report(
    primitive_corpus: &WorthPrimitiveCorpusReport,
) -> WorthMilestoneTwoReplayParityReport {
    let replay_checked_case_count = primitive_corpus
        .parity_report
        .entries
        .iter()
        .map(|entry| entry.mainline_replay_checked_case_count)
        .sum();
    let replay_verified_case_count = primitive_corpus
        .parity_report
        .entries
        .iter()
        .map(|entry| entry.mainline_replay_verified_case_count)
        .sum();
    let branch_local_replay_checked_case_count = primitive_corpus
        .parity_report
        .entries
        .iter()
        .map(|entry| entry.branch_local_replay_checked_case_count)
        .sum();
    let branch_local_replay_verified_case_count = primitive_corpus
        .parity_report
        .entries
        .iter()
        .map(|entry| entry.branch_local_replay_verified_case_count)
        .sum();
    let total_case_count = primitive_corpus.cases.len();

    WorthMilestoneTwoReplayParityReport {
        replay_checked_case_count,
        replay_verified_case_count,
        replay_mismatch_case_count: total_case_count.saturating_sub(replay_verified_case_count),
        branch_local_replay_checked_case_count,
        branch_local_replay_verified_case_count,
        replay_closure_complete: primitive_corpus
            .parity_report
            .entries
            .iter()
            .all(|entry| entry.parity_closure_complete),
    }
}

fn build_milestone_two_counter_report(
    primitive_corpus: &WorthPrimitiveCorpusReport,
) -> WorthMilestoneTwoCounters {
    let mut counters = WorthMilestoneTwoCounters {
        derived_read_count: 0,
        touched_aspect_count: 0,
        triggered_invalidation_target_count: 0,
        validation_row_count: 0,
        whole_view_rebuild_count: 0,
        explicit_fallback_count: 0,
        replay_checked_count: 0,
        branch_local_case_count: 0,
    };

    for case in &primitive_corpus.cases {
        counters.derived_read_count += 1;
        counters.touched_aspect_count += case
            .certification
            .derived_invalidation_report
            .touched_aspect_count;
        counters.triggered_invalidation_target_count += case
            .certification
            .derived_invalidation_report
            .triggered_target_count;
        counters.validation_row_count += case.certification.topology_validation_report.rows.len();
        counters.whole_view_rebuild_count += usize::from(
            case.certification.derived_rebuild_report.whole_view_rebuild,
        );
        counters.explicit_fallback_count +=
            case.certification.derived_fallback_report.explicit_fallback_count;
        counters.replay_checked_count += usize::from(
            case.certification
                .milestone_1_replay_parity_report
                .relational_replay_checked,
        );
        counters.branch_local_case_count +=
            usize::from(case.certification.branch_local_topology_report.branch_local);
    }

    counters
}

fn build_derived_invalidation_aggregate_report(
    primitive_corpus: &WorthPrimitiveCorpusReport,
) -> WorthDerivedInvalidationAggregateReport {
    let mut rows =
        BTreeMap::<(String, String, String), WorthDerivedInvalidationAggregateRow>::new();
    let mut touched_aspect_count = 0usize;
    let mut triggered_target_count = 0usize;

    for case in &primitive_corpus.cases {
        let report = &case.certification.derived_invalidation_report;
        touched_aspect_count += report.touched_aspect_count;
        triggered_target_count += report.triggered_target_count;
        for row in &report.rows {
            let key = (
                case.family.clone(),
                format!("{:?}", row.target),
                row.bridge_scope.clone(),
            );
            let entry = rows.entry(key.clone()).or_insert_with(|| {
                WorthDerivedInvalidationAggregateRow {
                    family: key.0.clone(),
                    target: key.1.clone(),
                    bridge_scope: key.2.clone(),
                    source_count: 0,
                    triggered_case_count: 0,
                }
            });
            entry.source_count += 1;
            entry.triggered_case_count += usize::from(row.triggered);
        }
    }

    WorthDerivedInvalidationAggregateReport {
        touched_aspect_count,
        triggered_target_count,
        rows: rows.into_values().collect(),
    }
}

fn build_derived_validator_coverage_report(
    primitive_corpus: &WorthPrimitiveCorpusReport,
) -> WorthDerivedValidatorCoverageReport {
    let mut rows =
        BTreeMap::<(String, String, String), WorthDerivedValidatorCoverageRow>::new();
    for case in &primitive_corpus.cases {
        for validation_row in &case.certification.topology_validation_report.rows {
            if validation_row.phase == crate::validators::WorthTopologyValidationPhase::Truth {
                continue;
            }
            let phase = match validation_row.phase {
                crate::validators::WorthTopologyValidationPhase::DerivedMaterialization => {
                    "derived-materialization"
                }
                crate::validators::WorthTopologyValidationPhase::DerivedInterpretation => {
                    "derived-interpretation"
                }
                crate::validators::WorthTopologyValidationPhase::Truth => "truth",
            };
            let key = (
                case.family.clone(),
                validation_row.validator.clone(),
                phase.to_string(),
            );
            let entry = rows.entry(key.clone()).or_insert_with(|| WorthDerivedValidatorCoverageRow {
                family: key.0.clone(),
                validator: key.1.clone(),
                phase: key.2.clone(),
                passed_count: 0,
                source_count: 0,
            });
            entry.passed_count += usize::from(validation_row.status == "passed");
            entry.source_count += 1;
        }
    }

    WorthDerivedValidatorCoverageReport {
        rows: rows.into_values().collect(),
    }
}

fn build_derived_rebuild_aggregate_report(
    primitive_corpus: &WorthPrimitiveCorpusReport,
) -> WorthDerivedRebuildAggregateReport {
    let mut rows = BTreeMap::<String, WorthDerivedRebuildAggregateRow>::new();
    for case in &primitive_corpus.cases {
        let report = &case.certification.derived_rebuild_report;
        let entry = rows
            .entry(case.family.clone())
            .or_insert_with(|| WorthDerivedRebuildAggregateRow {
                family: case.family.clone(),
                source_count: 0,
                whole_view_rebuild_count: 0,
                topology_entity_count: 0,
                topology_relation_count: 0,
                interpreted_wire_count: 0,
                interpreted_shell_count: 0,
                validation_row_count: 0,
            });
        entry.source_count += 1;
        entry.whole_view_rebuild_count += usize::from(report.whole_view_rebuild);
        entry.topology_entity_count += report.topology_entity_count;
        entry.topology_relation_count += report.topology_relation_count;
        entry.interpreted_wire_count += report.interpreted_wire_count;
        entry.interpreted_shell_count += report.interpreted_shell_count;
        entry.validation_row_count += report.validation_row_count;
    }
    WorthDerivedRebuildAggregateReport {
        rows: rows.into_values().collect(),
    }
}

fn build_derived_fallback_aggregate_report(
    primitive_corpus: &WorthPrimitiveCorpusReport,
) -> WorthDerivedFallbackAggregateReport {
    let mut rows = BTreeMap::<String, WorthDerivedFallbackAggregateRow>::new();
    for case in &primitive_corpus.cases {
        let report = &case.certification.derived_fallback_report;
        let entry = rows
            .entry(case.family.clone())
            .or_insert_with(|| WorthDerivedFallbackAggregateRow {
                family: case.family.clone(),
                source_count: 0,
                whole_view_materialization_count: 0,
                explicit_fallback_count: 0,
                precision_fallback_count: 0,
                precision_budget_fallback_count: 0,
            });
        entry.source_count += 1;
        entry.whole_view_materialization_count += usize::from(report.whole_view_materialization);
        entry.explicit_fallback_count += report.explicit_fallback_count;
        entry.precision_fallback_count += report.precision_fallback_count;
        entry.precision_budget_fallback_count += report.precision_budget_fallback_count;
    }
    WorthDerivedFallbackAggregateReport {
        rows: rows.into_values().collect(),
    }
}

fn build_derived_equivalence_aggregate_report(
    primitive_corpus: &WorthPrimitiveCorpusReport,
) -> WorthDerivedEquivalenceContractAggregateReport {
    WorthDerivedEquivalenceContractAggregateReport {
        rows: primitive_corpus
            .cases
            .iter()
            .map(|case| {
                let report = &case.certification.derived_equivalence_contract_report;
                WorthDerivedEquivalenceContractAggregateRow {
                    source: case.stem.clone(),
                    family: case.family.clone(),
                    truth_basis_digest_hex: report.truth_basis_digest_hex.clone(),
                    touched_aspect_count: report.touched_aspect_count,
                    triggered_invalidation_target_count: report
                        .triggered_invalidation_targets
                        .len(),
                    materialized_topology_digest: report.materialized_topology_digest.clone(),
                    interpreted_topology_digest: report.interpreted_topology_digest.clone(),
                    derived_validation_digest: report.derived_validation_digest.clone(),
                }
            })
            .collect(),
    }
}

fn build_derived_failure_locality_report(
    primitive_corpus: &WorthPrimitiveCorpusReport,
) -> WorthFailureLocalityReport {
    WorthFailureLocalityReport {
        rows: primitive_corpus
            .rejected_cases
            .iter()
            .map(|case| crate::certification::report::WorthFailureLocalityRow {
                family: case.family.clone(),
                role: format!("{:?}", case.role),
                validator_family: case.rejection.validator_family.clone(),
                rejection_class: case.rejection.rejection_class.clone(),
                diagnostic_code: case.rejection.diagnostic_code.clone(),
                localized_entity_count: case.rejection.localized_entity_count,
                localized_relation_count: case.rejection.localized_relation_count,
            })
            .collect(),
    }
}

fn ensure_milestone_two_family_coverage_closure(
    report: &WorthDerivedFamilyCoverageMatrix,
    requirements: &crate::certification::core::WorthCertificationSuiteRequirements,
) -> Result<(), WorthMilestoneOneCertificationError> {
    for family in &requirements.required_family_rows {
        let Some(row) = report.rows.iter().find(|row| row.family == *family) else {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone two closeout missing derived family coverage row for family `{family}`"
            )));
        };
        if !row.coverage_complete {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone two closeout derived family coverage is incomplete for family `{family}`"
            )));
        }
    }
    Ok(())
}

fn ensure_milestone_two_parity_closure(
    report: &WorthDerivedFamilyParityMatrix,
    requirements: &crate::certification::core::WorthCertificationSuiteRequirements,
) -> Result<(), WorthMilestoneOneCertificationError> {
    for family in &requirements.required_parity_rows {
        let Some(row) = report.rows.iter().find(|row| row.family == *family) else {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone two closeout missing derived parity row for family `{family}`"
            )));
        };
        if !row.parity_complete {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone two closeout derived parity is incomplete for family `{family}`"
            )));
        }
    }
    Ok(())
}

fn ensure_milestone_two_bridge_closure(
    report: &crate::certification::report::WorthBridgeFamilyCoverageReport,
    requirements: &crate::certification::core::WorthCertificationSuiteRequirements,
) -> Result<(), WorthMilestoneOneCertificationError> {
    for bridge_family in &requirements.required_bridge_rows {
        let Some(row) = report.rows.iter().find(|row| row.family == bridge_family.family) else {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone two closeout missing bridge family row for family `{}`",
                bridge_family.family
            )));
        };
        if !row.proof_complete {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone two closeout bridge proof is incomplete for family `{}`",
                bridge_family.family
            )));
        }
    }
    Ok(())
}

fn ensure_milestone_two_validator_closure(
    report: &WorthDerivedValidatorCoverageReport,
    requirements: &crate::certification::core::WorthCertificationSuiteRequirements,
) -> Result<(), WorthMilestoneOneCertificationError> {
    for expectation in &requirements.validator_expectations {
        for validator in &expectation.validators {
            let Some(row) = report
                .rows
                .iter()
                .find(|row| row.family == expectation.family && row.validator == *validator)
            else {
                return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                    "milestone two closeout missing derived validator coverage for family `{}` validator `{validator}`",
                    expectation.family
                )));
            };
            if row.passed_count == 0 {
                return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                    "milestone two closeout derived validator coverage is incomplete for family `{}` validator `{validator}`",
                    expectation.family
                )));
            }
        }
    }
    Ok(())
}

fn ensure_milestone_two_failure_locality_closure(
    report: &WorthFailureLocalityReport,
    requirements: &crate::certification::core::WorthCertificationSuiteRequirements,
) -> Result<(), WorthMilestoneOneCertificationError> {
    for family in &requirements.required_rejection_rows {
        if !report.rows.iter().any(|row| row.family == *family) {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone two closeout missing failure locality for family `{family}`"
            )));
        }
    }
    Ok(())
}

fn ensure_milestone_two_required_output_closure(
    closeout: &WorthMilestoneTwoCloseoutReport,
    requirements: &crate::certification::core::WorthCertificationSuiteRequirements,
) -> Result<(), WorthMilestoneOneCertificationError> {
    for output in &requirements.required_outputs {
        let present = match output {
            crate::certification::core::WorthCertificationRequiredOutput::MaterializedTopologyDigest => {
                closeout.materialized_topology_digest.row_count > 0
            }
            crate::certification::core::WorthCertificationRequiredOutput::InterpretedTopologyDigest => {
                closeout.interpreted_topology_digest.row_count > 0
            }
            crate::certification::core::WorthCertificationRequiredOutput::DerivedValidationDigest => {
                closeout.derived_validation_digest.row_count > 0
            }
            crate::certification::core::WorthCertificationRequiredOutput::DerivedTruthBasisDigest => {
                closeout.derived_truth_basis_digest.row_count > 0
            }
            crate::certification::core::WorthCertificationRequiredOutput::BridgeRoutingDigest => {
                closeout.bridge_routing_digest.row_count > 0
            }
            crate::certification::core::WorthCertificationRequiredOutput::BridgeHistoricalEvaluationDigest => {
                closeout.bridge_historical_evaluation_digest.row_count > 0
            }
            crate::certification::core::WorthCertificationRequiredOutput::DerivedFamilyCoverageMatrix => {
                !closeout.derived_family_coverage_matrix.rows.is_empty()
            }
            crate::certification::core::WorthCertificationRequiredOutput::DerivedFamilyParityMatrix => {
                !closeout.derived_family_parity_matrix.rows.is_empty()
            }
            crate::certification::core::WorthCertificationRequiredOutput::DerivedValidatorCoverageReport => {
                !closeout.derived_validator_coverage_report.rows.is_empty()
            }
            crate::certification::core::WorthCertificationRequiredOutput::DerivedInvalidationReport => {
                !closeout.derived_invalidation_report.rows.is_empty()
            }
            crate::certification::core::WorthCertificationRequiredOutput::DerivedRebuildReport => {
                !closeout.derived_rebuild_report.rows.is_empty()
            }
            crate::certification::core::WorthCertificationRequiredOutput::DerivedEquivalenceContractReport => {
                !closeout.derived_equivalence_contract_report.rows.is_empty()
            }
            crate::certification::core::WorthCertificationRequiredOutput::DerivedFallbackReport => {
                !closeout.derived_fallback_report.rows.is_empty()
            }
            crate::certification::core::WorthCertificationRequiredOutput::DerivedFailureLocalityReport => {
                !closeout.derived_failure_locality_report.rows.is_empty()
            }
            crate::certification::core::WorthCertificationRequiredOutput::DerivedBranchLocalParityReport => {
                !closeout.derived_branch_local_parity_report.branch_ids.is_empty()
            }
            crate::certification::core::WorthCertificationRequiredOutput::DerivedReplayParityReport => {
                closeout.derived_replay_parity_report.replay_checked_case_count > 0
            }
            crate::certification::core::WorthCertificationRequiredOutput::DerivedBridgeFamilyCoverageReport => {
                !closeout.derived_bridge_family_coverage_report.rows.is_empty()
            }
            crate::certification::core::WorthCertificationRequiredOutput::MilestoneTwoCounterReport => {
                closeout.milestone_2_counter_report.derived_read_count > 0
            }
            _ => true,
        };
        if !present {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone two closeout missing required output `{output:?}`"
            )));
        }
    }
    Ok(())
}
