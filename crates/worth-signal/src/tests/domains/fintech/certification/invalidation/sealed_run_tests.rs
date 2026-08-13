use super::*;
use crate::tests::domains::fintech::certification::invalidation::courtroom::build_financial_causality_claims;

#[test]
fn sealed_financial_causality_run_rejects_incomplete_or_mismatched_evidence() {
    let claims = build_financial_causality_claims().unwrap();
    let run = FinancialAspectCausalityCertificationRun::seal(claims.clone()).unwrap();
    assert_eq!(run.seed(), 41);
    assert_eq!(run.scenario_count(), 8);
    assert!(run.minimum_dependency_revision() > 0);
    let mut reordered = build_financial_causality_claims().unwrap();
    reordered.reverse();
    let second_run = FinancialAspectCausalityCertificationRun::seal(reordered).unwrap();
    assert_eq!(run.canonical_report_id(), second_run.canonical_report_id());

    let mut missing = claims.clone();
    missing.pop();
    assert!(FinancialAspectCausalityCertificationRun::seal(missing).is_err());

    let mut duplicate = claims.clone();
    duplicate.push(claims[0].clone());
    assert!(FinancialAspectCausalityCertificationRun::seal(duplicate).is_err());

    let mut wrong_scenario = claims.clone();
    wrong_scenario[0].reproduction.scenario = FinancialScenarioIdentity::BranchShockRestoreReplay;
    assert!(FinancialAspectCausalityCertificationRun::seal(wrong_scenario).is_err());

    let mut wrong_policy = claims.clone();
    wrong_policy[0].policy = FinancialCertificationPolicy::BranchRestoreReplay;
    assert!(FinancialAspectCausalityCertificationRun::seal(wrong_policy).is_err());

    let mut wrong_reproduction_policy = claims.clone();
    wrong_reproduction_policy[0]
        .reproduction
        .policy
        .consumer_comparators = FinancialComparatorProfile::ExactToleranceAndInstalledTolerance;
    assert!(FinancialAspectCausalityCertificationRun::seal(wrong_reproduction_policy).is_err());

    let mut wrong_diagnostics_tier = claims.clone();
    wrong_diagnostics_tier[0].reproduction.policy.diagnostics = DiagnosticsTier::Operational;
    assert!(FinancialAspectCausalityCertificationRun::seal(wrong_diagnostics_tier).is_err());

    let mut stale = claims.clone();
    stale[0].dependency_revision = 0;
    assert!(FinancialAspectCausalityCertificationRun::seal(stale).is_err());

    let mut wrong_nonzero_revision = claims.clone();
    wrong_nonzero_revision[0].dependency_revision += 1;
    assert!(FinancialAspectCausalityCertificationRun::seal(wrong_nonzero_revision).is_err());

    let mut wrong_reproduction = claims.clone();
    for claim in &mut wrong_reproduction {
        claim.reproduction.seed = 99;
    }
    assert!(FinancialAspectCausalityCertificationRun::seal(wrong_reproduction).is_err());

    let mut wrong_mutation = claims.clone();
    wrong_mutation[0].reproduction.mutation_step += 1;
    wrong_mutation[0].reproduction.economic_delta += 1;
    assert!(FinancialAspectCausalityCertificationRun::seal(wrong_mutation).is_err());

    let mut wrong_completion = claims;
    wrong_completion[1].completion = wrong_completion[0].completion.clone();
    assert!(FinancialAspectCausalityCertificationRun::seal(wrong_completion).is_err());
}
