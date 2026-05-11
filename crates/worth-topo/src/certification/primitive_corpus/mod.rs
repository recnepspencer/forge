use std::collections::BTreeMap;

use forge_relational::facade::history::BranchId;
use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::topology_authoring::{
    MilestoneOnePrimitiveCase, MilestoneOnePrimitiveExpectedOutcome, MilestoneOnePrimitiveRole,
    MilestoneOnePrimitiveScenario,
};
use schema::facade::MutationOrigin;

use crate::certification::error::MilestoneOneCertificationError;
use crate::certification::rejections::summarize_primitive_rejection;
use crate::certification::shared::{
    admitted_range_expected_branch_local_count, admitted_range_expected_mainline_count,
    canonical_milestone_one_primitive_families, empty_corpus_coverage_entry,
    empty_corpus_parity_entry, parity_case_key, primitive_family_name,
};
use crate::certification::support::reporting::{
    AdmittedRangeSweepReport, AdmittedRangeSweepRow, PrimitiveCorpusCaseReport,
    PrimitiveCorpusCoverageEntry, PrimitiveCorpusCoverageMatrix, PrimitiveCorpusParityEntry,
    PrimitiveCorpusParityReport, PrimitiveCorpusRejectedCaseReport, PrimitiveCorpusReport,
};
use crate::test_support::primitive_corpus::authored_topology::{
    milestone_one_admitted_range_scenarios, milestone_one_default_corpus_scenarios,
    milestone_one_out_of_class_range_scenarios,
};
use crate::test_support::primitive_corpus::branch_replay_cases::{
    milestone_one_default_branch_local_admitted_scenarios,
    milestone_one_heavy_branch_local_scenarios,
};
use crate::test_support::primitive_corpus::derived_topology::certified_verified_commit;
use crate::test_support::primitive_corpus::validated_topology::{
    verified_primitive, verified_primitive_on_branch,
};

mod parity;
use self::parity::build_primitive_corpus_parity_report;

pub fn certify_milestone_one_primitive_corpus_impl<F>(
    mut runtime_factory: F,
    stem: &str,
    primitives: &[MilestoneOnePrimitiveCase],
) -> Result<PrimitiveCorpusReport, MilestoneOneCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let mut cases = Vec::with_capacity(primitives.len());
    let rejected_cases = Vec::new();
    for (index, primitive) in primitives.iter().enumerate() {
        let case_stem = format!("{stem}.case.{index}");
        let mut runtime = runtime_factory();
        let verified = verified_primitive(&mut runtime, &case_stem, primitive)?;
        let certification = certified_verified_commit(&mut runtime, &verified)?;
        cases.push(PrimitiveCorpusCaseReport {
            stem: case_stem,
            family: primitive_family_name(primitive).to_string(),
            role: MilestoneOnePrimitiveRole::Generic,
            primitive: primitive.clone(),
            expected_outcome: MilestoneOnePrimitiveExpectedOutcome::Admit,
            certification,
        });
    }

    let coverage_matrix = build_primitive_corpus_coverage_matrix(&cases, &rejected_cases);
    let parity_report = build_primitive_corpus_parity_report(&cases, None);
    Ok(PrimitiveCorpusReport {
        coverage_matrix,
        parity_report,
        cases,
        rejected_cases,
    })
}

pub fn certify_milestone_one_default_primitive_corpus_impl<F>(
    mut runtime_factory: F,
    stem: &str,
) -> Result<PrimitiveCorpusReport, MilestoneOneCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let scenarios = milestone_one_default_corpus_scenarios();
    let mut report =
        certify_milestone_one_primitive_scenarios_impl(&mut runtime_factory, stem, &scenarios)?;
    let branch_local_scenarios = milestone_one_default_branch_local_admitted_scenarios();
    let branch_local = certify_milestone_one_branch_local_primitive_scenarios_impl(
        &mut runtime_factory,
        &format!("{stem}.branch_local"),
        "feature",
        &branch_local_scenarios,
    )?;
    report.parity_report =
        build_primitive_corpus_parity_report(&report.cases, Some(&branch_local.cases));
    Ok(report)
}

pub fn certify_milestone_one_primitive_scenarios_impl<F>(
    runtime_factory: &mut F,
    stem: &str,
    scenarios: &[MilestoneOnePrimitiveScenario],
) -> Result<PrimitiveCorpusReport, MilestoneOneCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let mut cases = Vec::new();
    let mut rejected_cases = Vec::new();
    for (index, scenario) in scenarios.iter().enumerate() {
        let case_stem = format!("{stem}.case.{index}");
        let mut runtime = runtime_factory();
        match scenario.expected_outcome {
            MilestoneOnePrimitiveExpectedOutcome::Admit => {
                let verified = verified_primitive(&mut runtime, &case_stem, &scenario.primitive)?;
                let certification = certified_verified_commit(&mut runtime, &verified)?;
                cases.push(PrimitiveCorpusCaseReport {
                    stem: case_stem,
                    family: scenario.family.clone(),
                    role: scenario.role,
                    primitive: scenario.primitive.clone(),
                    expected_outcome: scenario.expected_outcome,
                    certification,
                });
            }
            MilestoneOnePrimitiveExpectedOutcome::Reject => {
                let rejection =
                    match verified_primitive(&mut runtime, &case_stem, &scenario.primitive) {
                        Ok(_) => {
                            return Err(MilestoneOneCertificationError::ReadView(format!(
                                "out-of-class scenario `{}` unexpectedly admitted",
                                scenario.family
                            )));
                        }
                        Err(error) => summarize_primitive_rejection(&error),
                    };
                rejected_cases.push(PrimitiveCorpusRejectedCaseReport {
                    stem: case_stem,
                    family: scenario.family.clone(),
                    role: scenario.role,
                    primitive: scenario.primitive.clone(),
                    expected_outcome: scenario.expected_outcome,
                    rejection,
                });
            }
        }
    }

    let coverage_matrix = build_primitive_corpus_coverage_matrix(&cases, &rejected_cases);
    let parity_report = build_primitive_corpus_parity_report(&cases, None);
    Ok(PrimitiveCorpusReport {
        coverage_matrix,
        parity_report,
        cases,
        rejected_cases,
    })
}

pub fn certify_milestone_one_branch_local_primitive_scenarios_impl<F>(
    runtime_factory: &mut F,
    stem: &str,
    branch_id: &str,
    scenarios: &[MilestoneOnePrimitiveScenario],
) -> Result<PrimitiveCorpusReport, MilestoneOneCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let mut cases = Vec::new();
    let mut rejected_cases = Vec::new();
    for (index, scenario) in scenarios.iter().enumerate() {
        let case_stem = format!("{stem}.case.{index}");
        let mut runtime = runtime_factory();
        runtime
            .history_authority()
            .create_branch(
                BranchId(branch_id.to_string()),
                &BranchId("main".to_string()),
            )
            .map_err(|error| {
                MilestoneOneCertificationError::ReadView(format!(
                    "failed to create branch `{branch_id}`: {error:?}"
                ))
            })?;
        match scenario.expected_outcome {
            MilestoneOnePrimitiveExpectedOutcome::Admit => {
                let verified = verified_primitive_on_branch(
                    &mut runtime,
                    &case_stem,
                    &scenario.primitive,
                    BranchId(branch_id.to_string()),
                    MutationOrigin::BranchLocalApplication,
                )?;
                let certification = certified_verified_commit(&mut runtime, &verified)?;
                cases.push(PrimitiveCorpusCaseReport {
                    stem: case_stem,
                    family: scenario.family.clone(),
                    role: scenario.role,
                    primitive: scenario.primitive.clone(),
                    expected_outcome: scenario.expected_outcome,
                    certification,
                });
            }
            MilestoneOnePrimitiveExpectedOutcome::Reject => {
                let rejection = match verified_primitive_on_branch(
                    &mut runtime,
                    &case_stem,
                    &scenario.primitive,
                    BranchId(branch_id.to_string()),
                    MutationOrigin::BranchLocalApplication,
                ) {
                    Ok(_) => {
                        return Err(MilestoneOneCertificationError::ReadView(format!(
                            "out-of-class branch-local scenario `{}` unexpectedly admitted",
                            scenario.family
                        )));
                    }
                    Err(error) => summarize_primitive_rejection(&error),
                };
                rejected_cases.push(PrimitiveCorpusRejectedCaseReport {
                    stem: case_stem,
                    family: scenario.family.clone(),
                    role: scenario.role,
                    primitive: scenario.primitive.clone(),
                    expected_outcome: scenario.expected_outcome,
                    rejection,
                });
            }
        }
    }

    let coverage_matrix = build_primitive_corpus_coverage_matrix(&cases, &rejected_cases);
    let parity_report = build_primitive_corpus_parity_report(&cases, None);
    Ok(PrimitiveCorpusReport {
        coverage_matrix,
        parity_report,
        cases,
        rejected_cases,
    })
}

pub(crate) fn certify_milestone_one_admitted_range_sweeps<F>(
    runtime_factory: &mut F,
    stem: &str,
) -> Result<AdmittedRangeSweepReport, MilestoneOneCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let mainline = certify_milestone_one_primitive_scenarios_impl(
        runtime_factory,
        &format!("{stem}.mainline"),
        &milestone_one_admitted_range_scenarios(),
    )?;
    let branch_local = certify_milestone_one_branch_local_primitive_scenarios_impl(
        runtime_factory,
        &format!("{stem}.branch_local"),
        "feature",
        &milestone_one_heavy_branch_local_scenarios(),
    )?;
    let out_of_class = certify_milestone_one_primitive_scenarios_impl(
        runtime_factory,
        &format!("{stem}.out_of_class"),
        &milestone_one_out_of_class_range_scenarios(),
    )?;

    let mut rows = BTreeMap::<String, AdmittedRangeSweepRow>::new();
    for family in canonical_milestone_one_primitive_families() {
        rows.insert(
            family.to_string(),
            AdmittedRangeSweepRow {
                family: family.to_string(),
                mainline_case_count: 0,
                branch_local_case_count: 0,
                mainline_replay_verified_case_count: 0,
                branch_local_replay_verified_case_count: 0,
                out_of_class_case_count: 0,
                out_of_class_rejection_count: 0,
                sweep_closure_complete: false,
            },
        );
    }

    for case in &mainline.cases {
        let row = rows.get_mut(&case.family).expect("canonical family row");
        row.mainline_case_count += 1;
        if case
            .certification
            .milestone_1_replay_parity_report
            .relational_replay_verified
        {
            row.mainline_replay_verified_case_count += 1;
        }
    }

    for case in &branch_local.cases {
        let row = rows.get_mut(&case.family).expect("canonical family row");
        row.branch_local_case_count += 1;
        if case
            .certification
            .milestone_1_replay_parity_report
            .relational_replay_verified
        {
            row.branch_local_replay_verified_case_count += 1;
        }
    }

    for case in &out_of_class.rejected_cases {
        let row = rows.get_mut(&case.family).expect("canonical family row");
        row.out_of_class_case_count += 1;
        if case.rejection.rejection_class == "OutOfClass" {
            row.out_of_class_rejection_count += 1;
        }
    }

    for row in rows.values_mut() {
        let expected_mainline = admitted_range_expected_mainline_count(&row.family);
        let expected_branch_local = admitted_range_expected_branch_local_count(&row.family);
        row.sweep_closure_complete = row.mainline_case_count == expected_mainline
            && row.mainline_replay_verified_case_count == expected_mainline
            && row.branch_local_case_count == expected_branch_local
            && row.branch_local_replay_verified_case_count == expected_branch_local
            && row.out_of_class_case_count >= 1
            && row.out_of_class_rejection_count >= 1;
    }

    Ok(AdmittedRangeSweepReport {
        rows: rows.into_values().collect(),
    })
}

pub(crate) fn build_primitive_corpus_coverage_matrix(
    cases: &[PrimitiveCorpusCaseReport],
    rejected_cases: &[PrimitiveCorpusRejectedCaseReport],
) -> PrimitiveCorpusCoverageMatrix {
    let mut rows = BTreeMap::<String, PrimitiveCorpusCoverageEntry>::new();

    for family in canonical_milestone_one_primitive_families() {
        rows.insert(family.to_string(), empty_corpus_coverage_entry(family));
    }

    for case in cases {
        let row = rows
            .entry(case.family.clone())
            .or_insert_with(|| empty_corpus_coverage_entry(&case.family));
        match case.role {
            MilestoneOnePrimitiveRole::Smallest => row.admitted_smallest_count += 1,
            MilestoneOnePrimitiveRole::Generic => row.admitted_generic_count += 1,
            MilestoneOnePrimitiveRole::HostileAdmitted => row.admitted_hostile_count += 1,
            MilestoneOnePrimitiveRole::OutOfClass => {}
        }
    }

    for case in rejected_cases {
        let row = rows
            .entry(case.family.clone())
            .or_insert_with(|| empty_corpus_coverage_entry(&case.family));
        if case.role == MilestoneOnePrimitiveRole::OutOfClass {
            row.rejected_out_of_class_count += 1;
        }
    }

    for row in rows.values_mut() {
        row.role_closure_complete = row.admitted_smallest_count > 0
            && row.admitted_generic_count > 0
            && row.admitted_hostile_count > 0
            && row.rejected_out_of_class_count > 0;
    }

    PrimitiveCorpusCoverageMatrix {
        entries: rows.into_values().collect(),
    }
}
