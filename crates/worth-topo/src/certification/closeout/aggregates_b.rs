use super::*;

pub(super) fn build_failure_locality_report(
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

pub(super) fn build_closeout_branch_local_report(
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

pub(super) fn build_closeout_replay_report(
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

pub(super) fn build_closeout_rejection_class_report(
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
