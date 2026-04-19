use std::collections::{BTreeMap, BTreeSet};

use forge_relational::facade::runtime::RelationalRuntime;

use crate::certification::bridge::certify_milestone_one_bridge_proof;
use crate::certification::core::{
    WorthCertificationRequiredOutput, WorthCertificationSuiteRequirements,
};
use crate::certification::corpus::{
    certify_milestone_one_admitted_range_sweeps,
    certify_milestone_one_default_primitive_corpus_impl,
};
use crate::certification::error::WorthMilestoneOneCertificationError;
use crate::certification::read_view::WorthMilestoneOneCertificationHarness;
use crate::certification::rejections::certify_milestone_one_illegal_topology_rejections;
use crate::certification::report::{
    WorthAdmittedRangeSweepReport, WorthDeterministicDigest, WorthFailureLocalityReport,
    WorthFailureLocalityRow, WorthIllegalTopologyRejectionReport,
    WorthMilestoneOneBranchLocalAggregateReport, WorthMilestoneOneCertificationReport,
    WorthMilestoneOneCloseoutReport, WorthMilestoneOneCounters,
    WorthMilestoneOneRejectionClassReport, WorthMilestoneOneRejectionClassRow,
    WorthMilestoneOneReplayAggregateReport, WorthMilestoneOneValidationAggregateReport,
    WorthMilestoneOneValidationAggregateRow, WorthMilestoneOneValidatorCoverageReport,
    WorthMilestoneOneValidatorCoverageRow, WorthNamingAttachmentAggregateReport,
    WorthNamingAttachmentAggregateRow, WorthPrimitiveCorpusReport, WorthReplayParityStatus,
    WorthTopologyLocalizationAggregateEntityRow, WorthTopologyLocalizationAggregateRelationRow,
    WorthTopologyLocalizationAggregateReport,
};
use crate::certification::requirements::milestone_one_closeout_requirements;
use crate::certification::shared::digest_rows;
use crate::fixtures::validated_topology::seeded_bootstrap;

pub fn certify_milestone_one_closeout_impl<F>(
    mut runtime_factory: F,
    stem: &str,
) -> Result<WorthMilestoneOneCloseoutReport, WorthMilestoneOneCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let requirements = milestone_one_closeout_requirements();
    let mut baseline_runtime = runtime_factory();
    let seeded =
        seeded_bootstrap(&mut baseline_runtime, &format!("{stem}.bootstrap")).map_err(|error| {
            WorthMilestoneOneCertificationError::ReadView(format!(
                "worth milestone one closeout failed to seed bootstrap truth: {error:?}"
            ))
        })?;
    let baseline_read = baseline_runtime
        .read_truth()
        .read_snapshot(&seeded.snapshot)
        .ok_or_else(|| {
            WorthMilestoneOneCertificationError::ReadView(format!(
                "worth milestone one closeout could not open seeded snapshot {:?}",
                seeded.snapshot
            ))
        })?;
    let seeded_bootstrap = WorthMilestoneOneCertificationHarness::certify_read_view_with_batch(
        &baseline_read,
        seeded.read_basis,
        Some(&seeded.persisted_truth.batch),
        1,
    )?;
    let primitive_corpus = certify_milestone_one_default_primitive_corpus_impl(
        &mut runtime_factory,
        &format!("{stem}.corpus"),
    )?;
    let admitted_range_sweeps = certify_milestone_one_admitted_range_sweeps(
        &mut runtime_factory,
        &format!("{stem}.sweeps"),
    )?;
    let illegal_topology_rejection_report = certify_milestone_one_illegal_topology_rejections(
        &mut runtime_factory,
        &format!("{stem}.illegal"),
    )?;
    let bridge_proof_report = certify_milestone_one_bridge_proof(&format!("{stem}.bridge"))?;
    let topology_truth_digest =
        build_closeout_digest(&seeded_bootstrap, &primitive_corpus, |report| {
            report.topology_truth_digest.clone()
        });
    let naming_truth_digest =
        build_closeout_digest(&seeded_bootstrap, &primitive_corpus, |report| {
            report.naming_truth_digest.clone()
        });
    let topology_validation_digest =
        build_closeout_digest(&seeded_bootstrap, &primitive_corpus, |report| {
            report.topology_validation_digest.clone()
        });
    let topology_validation_report =
        build_closeout_validation_report(&seeded_bootstrap, &primitive_corpus);
    let topology_localization_report =
        build_closeout_localization_report(&seeded_bootstrap, &primitive_corpus);
    let naming_attachment_report =
        build_closeout_naming_attachment_report(&seeded_bootstrap, &primitive_corpus);
    let primitive_family_coverage_matrix = primitive_corpus.coverage_matrix.clone();
    let primitive_corpus_parity_report = primitive_corpus.parity_report.clone();
    let validator_coverage_report =
        build_closeout_validator_coverage_report(&topology_validation_report);
    let branch_local_topology_report =
        build_closeout_branch_local_report(&seeded_bootstrap, &primitive_corpus);
    let milestone_1_replay_parity_report =
        build_closeout_replay_report(&seeded_bootstrap, &primitive_corpus);
    let rejection_class_report = build_closeout_rejection_class_report(
        &primitive_corpus,
        &illegal_topology_rejection_report,
    );
    let failure_locality_report =
        build_failure_locality_report(&primitive_corpus, &illegal_topology_rejection_report);
    let bridge_family_coverage_report = bridge_proof_report.family_coverage_report.clone();
    let counter_report = build_closeout_counter_report(
        &seeded_bootstrap,
        &primitive_corpus,
        &illegal_topology_rejection_report,
    );

    ensure_family_coverage_closure(&primitive_family_coverage_matrix, &requirements)?;
    ensure_parity_closure(&primitive_corpus_parity_report, &requirements)?;
    ensure_validator_expectation_closure(&validator_coverage_report, &requirements)?;
    ensure_rejection_class_closure(&rejection_class_report, &requirements)?;
    ensure_sweep_closure(&admitted_range_sweeps, &requirements)?;
    ensure_failure_locality_closure(&failure_locality_report, &requirements)?;
    ensure_bridge_coverage_closure(&bridge_proof_report.family_coverage_report, &requirements)?;

    let closeout = WorthMilestoneOneCloseoutReport {
        topology_truth_digest,
        naming_truth_digest,
        topology_validation_digest,
        topology_validation_report,
        topology_localization_report,
        naming_attachment_report,
        primitive_family_coverage_matrix,
        primitive_corpus_parity_report,
        admitted_range_sweep_report: admitted_range_sweeps,
        validator_coverage_report,
        branch_local_topology_report,
        milestone_1_replay_parity_report,
        rejection_class_report,
        failure_locality_report,
        bridge_family_coverage_report,
        seeded_bootstrap,
        primitive_corpus,
        illegal_topology_rejection_report,
        bridge_proof_report,
        milestone_1_counter_report: counter_report,
    };

    ensure_required_output_closure(&closeout, &requirements)?;
    Ok(closeout)
}

fn build_closeout_counter_report(
    seeded_bootstrap: &WorthMilestoneOneCertificationReport,
    primitive_corpus: &WorthPrimitiveCorpusReport,
    illegal_topology_rejection_report: &WorthIllegalTopologyRejectionReport,
) -> WorthMilestoneOneCounters {
    let mut counter_report = seeded_bootstrap.counters.clone();
    counter_report.commit_boundary_rejection_count = illegal_topology_rejection_report.case_count;
    for case in &primitive_corpus.cases {
        counter_report.topology_entity_upsert_count +=
            case.certification.counters.topology_entity_upsert_count;
        counter_report.topology_relation_upsert_count +=
            case.certification.counters.topology_relation_upsert_count;
        counter_report.topology_relation_remove_count +=
            case.certification.counters.topology_relation_remove_count;
        counter_report.commit_boundary_validator_count +=
            case.certification.counters.commit_boundary_validator_count;
        counter_report.derived_topology_interpretation_count += case
            .certification
            .counters
            .derived_topology_interpretation_count;
        counter_report.derived_topology_full_fallback_count += case
            .certification
            .counters
            .derived_topology_full_fallback_count;
        counter_report.naming_target_lookup_count +=
            case.certification.counters.naming_target_lookup_count;
        counter_report.primitive_family_member_count +=
            case.certification.counters.primitive_family_member_count;
        counter_report.replay_history_length += case.certification.counters.replay_history_length;
        counter_report.replay_interpretation_rerun_count += case
            .certification
            .counters
            .replay_interpretation_rerun_count;
    }
    counter_report
}

fn build_closeout_digest(
    seeded_bootstrap: &WorthMilestoneOneCertificationReport,
    primitive_corpus: &WorthPrimitiveCorpusReport,
    select: impl Fn(&WorthMilestoneOneCertificationReport) -> WorthDeterministicDigest,
) -> WorthDeterministicDigest {
    digest_rows(
        std::iter::once(("seeded_bootstrap".to_string(), select(seeded_bootstrap)))
            .chain(
                primitive_corpus
                    .cases
                    .iter()
                    .map(|case| (case.stem.clone(), select(&case.certification))),
            )
            .map(|(source, digest)| {
                format!(
                    "{source}:{}:{}:{}",
                    digest.algorithm, digest.digest_hex, digest.row_count
                )
            }),
    )
}

fn build_closeout_validation_report(
    seeded_bootstrap: &WorthMilestoneOneCertificationReport,
    primitive_corpus: &WorthPrimitiveCorpusReport,
) -> WorthMilestoneOneValidationAggregateReport {
    let mut rows = Vec::new();
    rows.extend(
        seeded_bootstrap
            .topology_validation_report
            .rows
            .iter()
            .map(|row| WorthMilestoneOneValidationAggregateRow {
                source: "seeded_bootstrap".to_string(),
                family: "SeededBootstrap".to_string(),
                validator: row.validator.clone(),
                status: row.status.clone(),
            }),
    );
    rows.push(WorthMilestoneOneValidationAggregateRow {
        source: "seeded_bootstrap".to_string(),
        family: "SeededBootstrap".to_string(),
        validator: "naming".to_string(),
        status: if seeded_bootstrap.named_truth_validated {
            "passed".to_string()
        } else {
            "failed".to_string()
        },
    });
    rows.extend(primitive_corpus.cases.iter().flat_map(|case| {
        case.certification
            .topology_validation_report
            .rows
            .iter()
            .map(move |row| WorthMilestoneOneValidationAggregateRow {
                source: case.stem.clone(),
                family: case.family.clone(),
                validator: row.validator.clone(),
                status: row.status.clone(),
            })
    }));
    rows.extend(primitive_corpus.cases.iter().map(|case| {
        WorthMilestoneOneValidationAggregateRow {
            source: case.stem.clone(),
            family: case.family.clone(),
            validator: "naming".to_string(),
            status: if case.certification.named_truth_validated {
                "passed".to_string()
            } else {
                "failed".to_string()
            },
        }
    }));
    WorthMilestoneOneValidationAggregateReport { rows }
}

fn build_closeout_localization_report(
    seeded_bootstrap: &WorthMilestoneOneCertificationReport,
    primitive_corpus: &WorthPrimitiveCorpusReport,
) -> WorthTopologyLocalizationAggregateReport {
    let mut topology_entities = Vec::new();
    let mut topology_relations = Vec::new();
    topology_entities.extend(
        seeded_bootstrap
            .topology_localization_report
            .topology_entities
            .iter()
            .map(|row| WorthTopologyLocalizationAggregateEntityRow {
                source: "seeded_bootstrap".to_string(),
                entity_id: row.entity_id,
                kind_name: row.kind_name.clone(),
            }),
    );
    topology_relations.extend(
        seeded_bootstrap
            .topology_localization_report
            .topology_relations
            .iter()
            .map(|row| WorthTopologyLocalizationAggregateRelationRow {
                source: "seeded_bootstrap".to_string(),
                relation_id: row.relation_id,
                kind_name: row.kind_name.clone(),
            }),
    );
    for case in &primitive_corpus.cases {
        topology_entities.extend(
            case.certification
                .topology_localization_report
                .topology_entities
                .iter()
                .map(|row| WorthTopologyLocalizationAggregateEntityRow {
                    source: case.stem.clone(),
                    entity_id: row.entity_id,
                    kind_name: row.kind_name.clone(),
                }),
        );
        topology_relations.extend(
            case.certification
                .topology_localization_report
                .topology_relations
                .iter()
                .map(|row| WorthTopologyLocalizationAggregateRelationRow {
                    source: case.stem.clone(),
                    relation_id: row.relation_id,
                    kind_name: row.kind_name.clone(),
                }),
        );
    }
    WorthTopologyLocalizationAggregateReport {
        topology_entities,
        topology_relations,
    }
}

fn build_closeout_naming_attachment_report(
    seeded_bootstrap: &WorthMilestoneOneCertificationReport,
    primitive_corpus: &WorthPrimitiveCorpusReport,
) -> WorthNamingAttachmentAggregateReport {
    let mut attachments = Vec::new();
    let mut orphan_persistent_name_ids = BTreeSet::new();
    attachments.extend(
        seeded_bootstrap
            .naming_attachment_report
            .attachments
            .iter()
            .map(|row| WorthNamingAttachmentAggregateRow {
                source: "seeded_bootstrap".to_string(),
                topology_entity_id: row.topology_entity_id,
                topology_kind_name: row.topology_kind_name.clone(),
                attached_persistent_name_ids: row.attached_persistent_name_ids.clone(),
            }),
    );
    orphan_persistent_name_ids.extend(
        seeded_bootstrap
            .naming_attachment_report
            .orphan_persistent_name_ids
            .iter()
            .copied(),
    );
    for case in &primitive_corpus.cases {
        attachments.extend(
            case.certification
                .naming_attachment_report
                .attachments
                .iter()
                .map(|row| WorthNamingAttachmentAggregateRow {
                    source: case.stem.clone(),
                    topology_entity_id: row.topology_entity_id,
                    topology_kind_name: row.topology_kind_name.clone(),
                    attached_persistent_name_ids: row.attached_persistent_name_ids.clone(),
                }),
        );
        orphan_persistent_name_ids.extend(
            case.certification
                .naming_attachment_report
                .orphan_persistent_name_ids
                .iter()
                .copied(),
        );
    }
    WorthNamingAttachmentAggregateReport {
        fully_named: orphan_persistent_name_ids.is_empty(),
        orphan_persistent_name_ids: orphan_persistent_name_ids.into_iter().collect(),
        attachments,
    }
}

fn build_closeout_validator_coverage_report(
    aggregate: &WorthMilestoneOneValidationAggregateReport,
) -> WorthMilestoneOneValidatorCoverageReport {
    let mut rows = BTreeMap::<(String, String), WorthMilestoneOneValidatorCoverageRow>::new();
    for row in &aggregate.rows {
        let entry = rows
            .entry((row.family.clone(), row.validator.clone()))
            .or_insert_with(|| WorthMilestoneOneValidatorCoverageRow {
                family: row.family.clone(),
                validator: row.validator.clone(),
                passed_count: 0,
                source_count: 0,
            });
        entry.source_count += 1;
        if row.status == "passed" {
            entry.passed_count += 1;
        }
    }
    WorthMilestoneOneValidatorCoverageReport {
        rows: rows.into_values().collect(),
    }
}

fn build_failure_locality_report(
    primitive_corpus: &WorthPrimitiveCorpusReport,
    illegal_topology_rejections: &WorthIllegalTopologyRejectionReport,
) -> WorthFailureLocalityReport {
    let mut rows = Vec::new();
    rows.extend(
        primitive_corpus
            .rejected_cases
            .iter()
            .map(|case| WorthFailureLocalityRow {
                family: case.family.clone(),
                role: format!("{:?}", case.role),
                validator_family: case.rejection.validator_family.clone(),
                rejection_class: case.rejection.rejection_class.clone(),
                diagnostic_code: case.rejection.diagnostic_code,
                localized_entity_count: case.rejection.localized_entity_count,
                localized_relation_count: case.rejection.localized_relation_count,
            }),
    );
    rows.extend(
        illegal_topology_rejections
            .cases
            .iter()
            .map(|case| WorthFailureLocalityRow {
                family: case.family.clone(),
                role: case.role.clone(),
                validator_family: case.rejection.validator_family.clone(),
                rejection_class: case.rejection.rejection_class.clone(),
                diagnostic_code: case.rejection.diagnostic_code,
                localized_entity_count: case.rejection.localized_entity_count,
                localized_relation_count: case.rejection.localized_relation_count,
            }),
    );
    WorthFailureLocalityReport { rows }
}

fn build_closeout_branch_local_report(
    seeded_bootstrap: &WorthMilestoneOneCertificationReport,
    primitive_corpus: &WorthPrimitiveCorpusReport,
) -> WorthMilestoneOneBranchLocalAggregateReport {
    let mainline_case_count = 1 + primitive_corpus
        .parity_report
        .entries
        .iter()
        .map(|entry| entry.mainline_case_count)
        .sum::<usize>();
    let branch_local_case_count = primitive_corpus
        .parity_report
        .entries
        .iter()
        .map(|entry| entry.branch_local_case_count)
        .sum::<usize>();
    let mut branch_ids = BTreeSet::new();
    branch_ids.insert(
        seeded_bootstrap
            .branch_local_topology_report
            .branch_id
            .0
            .clone(),
    );
    for entry in &primitive_corpus.parity_report.entries {
        branch_ids.extend(entry.branch_ids.iter().cloned());
    }
    WorthMilestoneOneBranchLocalAggregateReport {
        mainline_case_count,
        branch_local_case_count,
        branch_ids: branch_ids.into_iter().collect(),
        branch_local_closure_complete: branch_local_case_count > 0
            && primitive_corpus
                .parity_report
                .entries
                .iter()
                .all(|entry| entry.parity_closure_complete),
    }
}

fn build_closeout_replay_report(
    seeded_bootstrap: &WorthMilestoneOneCertificationReport,
    primitive_corpus: &WorthPrimitiveCorpusReport,
) -> WorthMilestoneOneReplayAggregateReport {
    let replay_checked_case_count = usize::from(
        seeded_bootstrap
            .milestone_1_replay_parity_report
            .relational_replay_checked,
    ) + primitive_corpus
        .parity_report
        .entries
        .iter()
        .map(|entry| entry.mainline_replay_checked_case_count)
        .sum::<usize>();
    let replay_verified_case_count = usize::from(
        seeded_bootstrap
            .milestone_1_replay_parity_report
            .relational_replay_verified,
    ) + primitive_corpus
        .parity_report
        .entries
        .iter()
        .map(|entry| entry.mainline_replay_verified_case_count)
        .sum::<usize>();
    let replay_mismatch_case_count = usize::from(matches!(
        seeded_bootstrap
            .milestone_1_replay_parity_report
            .parity_status,
        WorthReplayParityStatus::Mismatch
    )) + primitive_corpus
        .parity_report
        .entries
        .iter()
        .map(|entry| {
            entry
                .mainline_case_count
                .saturating_sub(entry.mainline_digest_parity_case_count)
        })
        .sum::<usize>();
    let branch_local_replay_checked_case_count = primitive_corpus
        .parity_report
        .entries
        .iter()
        .map(|entry| entry.branch_local_replay_checked_case_count)
        .sum::<usize>();
    let branch_local_replay_verified_case_count = primitive_corpus
        .parity_report
        .entries
        .iter()
        .map(|entry| entry.branch_local_replay_verified_case_count)
        .sum::<usize>();
    WorthMilestoneOneReplayAggregateReport {
        replay_checked_case_count,
        replay_verified_case_count,
        replay_mismatch_case_count,
        branch_local_replay_checked_case_count,
        branch_local_replay_verified_case_count,
        replay_closure_complete: primitive_corpus
            .parity_report
            .entries
            .iter()
            .all(|entry| entry.parity_closure_complete),
    }
}

fn build_closeout_rejection_class_report(
    primitive_corpus: &WorthPrimitiveCorpusReport,
    illegal_topology_rejections: &WorthIllegalTopologyRejectionReport,
) -> WorthMilestoneOneRejectionClassReport {
    let mut rows = BTreeMap::<(String, String), WorthMilestoneOneRejectionClassRow>::new();
    for case in &primitive_corpus.rejected_cases {
        let key = (case.family.clone(), case.rejection.rejection_class.clone());
        let entry = rows
            .entry(key.clone())
            .or_insert_with(|| WorthMilestoneOneRejectionClassRow {
                family: key.0.clone(),
                rejection_class: key.1.clone(),
                case_count: 0,
            });
        entry.case_count += 1;
    }
    for case in &illegal_topology_rejections.cases {
        let key = (case.family.clone(), case.rejection.rejection_class.clone());
        let entry = rows
            .entry(key.clone())
            .or_insert_with(|| WorthMilestoneOneRejectionClassRow {
                family: key.0.clone(),
                rejection_class: key.1.clone(),
                case_count: 0,
            });
        entry.case_count += 1;
    }
    WorthMilestoneOneRejectionClassReport {
        rows: rows.into_values().collect(),
    }
}

fn ensure_validator_expectation_closure(
    report: &WorthMilestoneOneValidatorCoverageReport,
    requirements: &WorthCertificationSuiteRequirements,
) -> Result<(), WorthMilestoneOneCertificationError> {
    for expectation in &requirements.validator_expectations {
        for validator in &expectation.validators {
            let satisfied = report.rows.iter().any(|row| {
                row.family == expectation.family
                    && row.validator == *validator
                    && row.passed_count >= 1
            });
            if !satisfied {
                return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                    "milestone one closeout missing validator coverage for family `{}` validator `{validator}`",
                    expectation.family
                )));
            }
        }
    }
    Ok(())
}

fn ensure_family_coverage_closure(
    report: &crate::certification::report::WorthPrimitiveCorpusCoverageMatrix,
    requirements: &WorthCertificationSuiteRequirements,
) -> Result<(), WorthMilestoneOneCertificationError> {
    for family in &requirements.required_family_rows {
        let Some(row) = report.entries.iter().find(|row| row.family == *family) else {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone one closeout missing family coverage row for family `{family}`"
            )));
        };
        if !row.role_closure_complete {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone one closeout family coverage is incomplete for family `{family}`"
            )));
        }
    }
    Ok(())
}

fn ensure_parity_closure(
    report: &crate::certification::report::WorthPrimitiveCorpusParityReport,
    requirements: &WorthCertificationSuiteRequirements,
) -> Result<(), WorthMilestoneOneCertificationError> {
    for family in &requirements.required_parity_rows {
        let Some(row) = report.entries.iter().find(|row| row.family == *family) else {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone one closeout missing parity row for family `{family}`"
            )));
        };
        if !row.parity_closure_complete {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone one closeout parity is incomplete for family `{family}`"
            )));
        }
    }
    Ok(())
}

fn ensure_rejection_class_closure(
    report: &WorthMilestoneOneRejectionClassReport,
    requirements: &WorthCertificationSuiteRequirements,
) -> Result<(), WorthMilestoneOneCertificationError> {
    for family in &requirements.required_rejection_rows {
        let has_row = report
            .rows
            .iter()
            .any(|row| row.family == *family && row.case_count > 0);
        if !has_row {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone one closeout missing rejection-class coverage for family `{family}`"
            )));
        }
    }
    Ok(())
}

fn ensure_sweep_closure(
    report: &WorthAdmittedRangeSweepReport,
    requirements: &WorthCertificationSuiteRequirements,
) -> Result<(), WorthMilestoneOneCertificationError> {
    for family in &requirements.required_family_rows {
        let Some(row) = report.rows.iter().find(|row| row.family == *family) else {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone one closeout missing admitted-range sweep row for family `{family}`"
            )));
        };
        if !row.sweep_closure_complete {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone one closeout admitted-range sweep is incomplete for family `{family}`"
            )));
        }
    }
    Ok(())
}

fn ensure_failure_locality_closure(
    report: &WorthFailureLocalityReport,
    requirements: &WorthCertificationSuiteRequirements,
) -> Result<(), WorthMilestoneOneCertificationError> {
    for family in &requirements.required_rejection_rows {
        let has_row = report.rows.iter().any(|row| row.family == *family);
        if !has_row {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone one closeout missing failure locality row for family `{family}`"
            )));
        }
    }
    Ok(())
}

fn ensure_bridge_coverage_closure(
    report: &crate::certification::report::WorthBridgeFamilyCoverageReport,
    requirements: &WorthCertificationSuiteRequirements,
) -> Result<(), WorthMilestoneOneCertificationError> {
    for bridge_family in &requirements.required_bridge_rows {
        let Some(row) = report
            .rows
            .iter()
            .find(|row| row.family == bridge_family.family)
        else {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone one closeout missing bridge coverage row for family `{}`",
                bridge_family.family
            )));
        };
        if !row.proof_complete {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone one closeout bridge proof is incomplete for family `{}`",
                bridge_family.family
            )));
        }
    }
    Ok(())
}

fn ensure_required_output_closure(
    closeout: &WorthMilestoneOneCloseoutReport,
    requirements: &WorthCertificationSuiteRequirements,
) -> Result<(), WorthMilestoneOneCertificationError> {
    for output in &requirements.required_outputs {
        let present = match output {
            WorthCertificationRequiredOutput::TopologyTruthDigest => {
                closeout.topology_truth_digest.row_count > 0
            }
            WorthCertificationRequiredOutput::NamingTruthDigest => {
                closeout.naming_truth_digest.row_count > 0
            }
            WorthCertificationRequiredOutput::TopologyValidationDigest => {
                closeout.topology_validation_digest.row_count > 0
            }
            WorthCertificationRequiredOutput::TopologyValidationReport => {
                !closeout.topology_validation_report.rows.is_empty()
            }
            WorthCertificationRequiredOutput::TopologyLocalizationReport => {
                !closeout
                    .topology_localization_report
                    .topology_entities
                    .is_empty()
                    || !closeout
                        .topology_localization_report
                        .topology_relations
                        .is_empty()
            }
            WorthCertificationRequiredOutput::NamingAttachmentReport => {
                !closeout.naming_attachment_report.attachments.is_empty()
            }
            WorthCertificationRequiredOutput::PrimitiveFamilyCoverageMatrix => {
                !closeout.primitive_family_coverage_matrix.entries.is_empty()
            }
            WorthCertificationRequiredOutput::PrimitiveCorpusParityReport => {
                !closeout.primitive_corpus_parity_report.entries.is_empty()
            }
            WorthCertificationRequiredOutput::AdmittedRangeSweepReport => {
                !closeout.admitted_range_sweep_report.rows.is_empty()
            }
            WorthCertificationRequiredOutput::ValidatorCoverageReport => {
                !closeout.validator_coverage_report.rows.is_empty()
            }
            WorthCertificationRequiredOutput::BranchLocalTopologyReport => {
                !closeout.branch_local_topology_report.branch_ids.is_empty()
            }
            WorthCertificationRequiredOutput::ReplayParityReport => {
                closeout
                    .milestone_1_replay_parity_report
                    .replay_checked_case_count
                    > 0
            }
            WorthCertificationRequiredOutput::RejectionClassReport => {
                !closeout.rejection_class_report.rows.is_empty()
            }
            WorthCertificationRequiredOutput::FailureLocalityReport => {
                !closeout.failure_locality_report.rows.is_empty()
            }
            WorthCertificationRequiredOutput::BridgeFamilyCoverageReport => {
                !closeout.bridge_family_coverage_report.rows.is_empty()
            }
            WorthCertificationRequiredOutput::BridgeProofReport => {
                closeout.bridge_proof_report.proof_case_count > 0
            }
            WorthCertificationRequiredOutput::CounterReport => {
                closeout
                    .milestone_1_counter_report
                    .commit_boundary_validator_count
                    > 0
            }
            _ => true,
        };
        if !present {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone one closeout missing required output `{output:?}`"
            )));
        }
    }
    Ok(())
}
