use super::*;

pub(super) fn aggregate_derived_digest(
    primitive_corpus: &PrimitiveCorpusReport,
    select: impl Fn(&MilestoneOneCertificationReport) -> DeterministicDigest,
) -> DeterministicDigest {
    digest_rows(primitive_corpus.cases.iter().map(|case| {
        let digest = select(&case.certification);
        format!(
            "{}:{}:{}:{}",
            case.stem, digest.algorithm, digest.digest_hex, digest.row_count
        )
    }))
}

pub(super) fn aggregate_truth_basis_digest(
    primitive_corpus: &PrimitiveCorpusReport,
) -> DeterministicDigest {
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

pub(super) fn build_derived_family_coverage_matrix(
    primitive_corpus: &PrimitiveCorpusReport,
) -> DerivedFamilyCoverageMatrix {
    DerivedFamilyCoverageMatrix {
        rows: primitive_corpus
            .coverage_matrix
            .entries
            .iter()
            .map(|entry| DerivedFamilyCoverageRow {
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

pub(super) fn build_derived_family_parity_matrix(
    parity_report: &PrimitiveCorpusParityReport,
) -> DerivedFamilyParityMatrix {
    DerivedFamilyParityMatrix {
        rows: parity_report
            .entries
            .iter()
            .map(|entry| DerivedFamilyParityRow {
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

pub(super) fn build_derived_branch_local_report(
    primitive_corpus: &PrimitiveCorpusReport,
) -> MilestoneTwoBranchLocalParityReport {
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

    MilestoneTwoBranchLocalParityReport {
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

pub(super) fn build_derived_replay_report(
    primitive_corpus: &PrimitiveCorpusReport,
) -> MilestoneTwoReplayParityReport {
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

    MilestoneTwoReplayParityReport {
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

pub(super) fn build_milestone_two_counter_report(
    primitive_corpus: &PrimitiveCorpusReport,
) -> MilestoneTwoCounters {
    let mut counters = MilestoneTwoCounters {
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
        counters.whole_view_rebuild_count +=
            usize::from(case.certification.derived_rebuild_report.whole_view_rebuild);
        counters.explicit_fallback_count += case
            .certification
            .derived_fallback_report
            .explicit_fallback_count;
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

pub(super) fn build_derived_invalidation_aggregate_report(
    primitive_corpus: &PrimitiveCorpusReport,
) -> DerivedInvalidationAggregateReport {
    let mut rows = BTreeMap::<(String, String, String), DerivedInvalidationAggregateRow>::new();
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
            let entry =
                rows.entry(key.clone())
                    .or_insert_with(|| DerivedInvalidationAggregateRow {
                        family: key.0.clone(),
                        target: key.1.clone(),
                        bridge_scope: key.2.clone(),
                        source_count: 0,
                        triggered_case_count: 0,
                    });
            entry.source_count += 1;
            entry.triggered_case_count += usize::from(row.triggered);
        }
    }

    DerivedInvalidationAggregateReport {
        touched_aspect_count,
        triggered_target_count,
        rows: rows.into_values().collect(),
    }
}

pub(super) fn build_derived_validator_coverage_report(
    primitive_corpus: &PrimitiveCorpusReport,
) -> DerivedValidatorCoverageReport {
    let mut rows = BTreeMap::<(String, String, String), DerivedValidatorCoverageRow>::new();
    for case in &primitive_corpus.cases {
        for validation_row in &case.certification.topology_validation_report.rows {
            if validation_row.phase == crate::validation::TopologyValidationPhase::Truth {
                continue;
            }
            let phase = match validation_row.phase {
                crate::validation::TopologyValidationPhase::DerivedMaterialization => {
                    "derived-materialization"
                }
                crate::validation::TopologyValidationPhase::DerivedInterpretation => {
                    "derived-interpretation"
                }
                crate::validation::TopologyValidationPhase::Truth => "truth",
            };
            let key = (
                case.family.clone(),
                validation_row.validator.clone(),
                phase.to_string(),
            );
            let entry = rows
                .entry(key.clone())
                .or_insert_with(|| DerivedValidatorCoverageRow {
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

    DerivedValidatorCoverageReport {
        rows: rows.into_values().collect(),
    }
}

pub(super) fn build_derived_rebuild_aggregate_report(
    primitive_corpus: &PrimitiveCorpusReport,
) -> DerivedRebuildAggregateReport {
    let mut rows = BTreeMap::<String, DerivedRebuildAggregateRow>::new();
    for case in &primitive_corpus.cases {
        let report = &case.certification.derived_rebuild_report;
        let entry = rows
            .entry(case.family.clone())
            .or_insert_with(|| DerivedRebuildAggregateRow {
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
    DerivedRebuildAggregateReport {
        rows: rows.into_values().collect(),
    }
}

pub(super) fn build_derived_fallback_aggregate_report(
    primitive_corpus: &PrimitiveCorpusReport,
) -> DerivedFallbackAggregateReport {
    let mut rows = BTreeMap::<String, DerivedFallbackAggregateRow>::new();
    for case in &primitive_corpus.cases {
        let report = &case.certification.derived_fallback_report;
        let entry =
            rows.entry(case.family.clone())
                .or_insert_with(|| DerivedFallbackAggregateRow {
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
    DerivedFallbackAggregateReport {
        rows: rows.into_values().collect(),
    }
}

pub(super) fn build_derived_equivalence_aggregate_report(
    primitive_corpus: &PrimitiveCorpusReport,
) -> DerivedEquivalenceContractAggregateReport {
    DerivedEquivalenceContractAggregateReport {
        rows: primitive_corpus
            .cases
            .iter()
            .map(|case| {
                let report = &case.certification.derived_equivalence_contract_report;
                DerivedEquivalenceContractAggregateRow {
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

pub(super) fn build_derived_failure_locality_report(
    primitive_corpus: &PrimitiveCorpusReport,
) -> FailureLocalityReport {
    FailureLocalityReport {
        rows: primitive_corpus
            .rejected_cases
            .iter()
            .map(
                |case| crate::certification::support::reporting::FailureLocalityRow {
                    family: case.family.clone(),
                    role: format!("{:?}", case.role),
                    validator_family: case.rejection.validator_family.clone(),
                    rejection_class: case.rejection.rejection_class.clone(),
                    diagnostic_code: case.rejection.diagnostic_code.clone(),
                    localized_entity_count: case.rejection.localized_entity_count,
                    localized_relation_count: case.rejection.localized_relation_count,
                },
            )
            .collect(),
    }
}




