mod canonical_identity;
#[cfg(test)]
mod claim_adversarial_tests;
mod courtroom;
mod fresh_recompute;
mod locality_contract;
mod locality_expectation;
mod locality_fresh_recompute;
mod necessity_manifest;
mod scenario_completion;
mod sealed_run;

use canonical_identity::{FinancialCanonicalCaseIdentity, FinancialCanonicalReportIdentity};
pub(in crate::tests::domains::fintech) use courtroom::run_financial_causality_courtroom;
pub(in crate::tests::domains::fintech) use fresh_recompute::FreshFinancialRecompute;
pub(in crate::tests::domains::fintech) use locality_expectation::FinancialLocalityExpectationManifest;
pub(in crate::tests::domains::fintech) use locality_fresh_recompute::FreshFinancialLocalityRecompute;
pub(in crate::tests::domains::fintech) use necessity_manifest::{
    FinancialNecessityEvidence, FinancialNecessityManifest,
};
pub(in crate::tests::domains::fintech) use scenario_completion::FinancialScenarioCompletion;
pub(in crate::tests::domains::fintech) use sealed_run::{
    FinancialAspectCausalityCertificationRun, FinancialCertificationPolicy,
    FinancialScenarioCertificationClaim,
};
