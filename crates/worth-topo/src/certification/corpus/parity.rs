use super::*;

pub(crate) fn build_primitive_corpus_parity_report(
    cases: &[WorthPrimitiveCorpusCaseReport],
    branch_local_cases: Option<&[WorthPrimitiveCorpusCaseReport]>,
) -> WorthPrimitiveCorpusParityReport {
    let mut rows = BTreeMap::<String, WorthPrimitiveCorpusParityEntry>::new();

    for family in canonical_milestone_one_primitive_families() {
        rows.insert(family.to_string(), empty_corpus_parity_entry(family));
    }

    for case in cases {
        let row = rows
            .entry(case.family.clone())
            .or_insert_with(|| empty_corpus_parity_entry(&case.family));
        row.mainline_case_count += 1;
        row.branch_ids.push(
            case.certification
                .branch_local_topology_report
                .branch_id
                .0
                .clone(),
        );
        let is_branch_local = case.certification.branch_local_topology_report.branch_local;
        if is_branch_local {
            row.branch_local_case_count += 1;
        }
        if case
            .certification
            .milestone_1_replay_parity_report
            .relational_replay_checked
        {
            row.mainline_replay_checked_case_count += 1;
            if is_branch_local {
                row.branch_local_replay_checked_case_count += 1;
            }
        }
        if case
            .certification
            .milestone_1_replay_parity_report
            .relational_replay_verified
        {
            row.mainline_replay_verified_case_count += 1;
            if is_branch_local {
                row.branch_local_replay_verified_case_count += 1;
            }
        }
        if case
            .certification
            .milestone_1_replay_parity_report
            .interpretation_digest_match
            && case
                .certification
                .milestone_1_replay_parity_report
                .truth_digest_match
            && case
                .certification
                .milestone_1_replay_parity_report
                .validation_digest_match
        {
            row.mainline_digest_parity_case_count += 1;
            if is_branch_local {
                row.branch_local_digest_parity_case_count += 1;
            }
        }
    }

    let mut branch_lookup = BTreeMap::<String, &WorthPrimitiveCorpusCaseReport>::new();
    if let Some(branch_local_cases) = branch_local_cases {
        for case in branch_local_cases {
            let row = rows
                .entry(case.family.clone())
                .or_insert_with(|| empty_corpus_parity_entry(&case.family));
            row.branch_local_case_count += 1;
            row.branch_ids.push(
                case.certification
                    .branch_local_topology_report
                    .branch_id
                    .0
                    .clone(),
            );
            if case
                .certification
                .milestone_1_replay_parity_report
                .relational_replay_checked
            {
                row.branch_local_replay_checked_case_count += 1;
            }
            if case
                .certification
                .milestone_1_replay_parity_report
                .relational_replay_verified
            {
                row.branch_local_replay_verified_case_count += 1;
            }
            if case
                .certification
                .milestone_1_replay_parity_report
                .interpretation_digest_match
                && case
                    .certification
                    .milestone_1_replay_parity_report
                    .truth_digest_match
                && case
                    .certification
                    .milestone_1_replay_parity_report
                    .validation_digest_match
            {
                row.branch_local_digest_parity_case_count += 1;
            }
            branch_lookup.insert(parity_case_key(case), case);
        }
    }

    if branch_local_cases.is_some() {
        for case in cases {
            if let Some(branch_case) = branch_lookup.get(&parity_case_key(case)) {
                let row = rows
                    .entry(case.family.clone())
                    .or_insert_with(|| empty_corpus_parity_entry(&case.family));
                if case.certification.topology_truth_digest
                    == branch_case.certification.topology_truth_digest
                    && case.certification.topology_validation_digest
                        == branch_case.certification.topology_validation_digest
                    && case.certification.read_artifact.interpretations
                        == branch_case.certification.read_artifact.interpretations
                    && case.certification.certified_interpretation.interpretations
                        == branch_case
                            .certification
                            .certified_interpretation
                            .interpretations
                {
                    row.cross_branch_parity_case_count += 1;
                }
            }
        }
    }

    for row in rows.values_mut() {
        row.branch_ids.sort();
        row.branch_ids.dedup();
        let cross_branch_scope_satisfied = match branch_local_cases {
            Some(_) => {
                row.branch_local_case_count == row.mainline_case_count
                    && row.branch_local_replay_checked_case_count == row.branch_local_case_count
                    && row.branch_local_replay_verified_case_count == row.branch_local_case_count
                    && row.branch_local_digest_parity_case_count == row.branch_local_case_count
                    && row.cross_branch_parity_case_count == row.mainline_case_count
                    && row.branch_ids.len() >= 2
            }
            None => row.branch_local_case_count == row.mainline_case_count,
        };
        row.parity_closure_complete = row.mainline_case_count > 0
            && row.mainline_replay_checked_case_count == row.mainline_case_count
            && row.mainline_replay_verified_case_count == row.mainline_case_count
            && row.mainline_digest_parity_case_count == row.mainline_case_count
            && cross_branch_scope_satisfied;
    }

    WorthPrimitiveCorpusParityReport {
        entries: rows.into_values().collect(),
    }
}
