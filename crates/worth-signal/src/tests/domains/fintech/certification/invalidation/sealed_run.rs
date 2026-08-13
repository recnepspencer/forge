use std::collections::{BTreeMap, BTreeSet};

use crate::data::error::SignalError;
use crate::facade::DiagnosticsTier;
use crate::tests::domains::fintech::world::{
    CompiledFinancialWorld, FinancialComparatorProfile, FinancialOutputEquivalencePolicy,
    FinancialReproductionTuple, FinancialScaleTuple, FinancialScenarioIdentity, SemanticOutputKey,
};

use super::{
    FinancialCanonicalCaseIdentity, FinancialCanonicalReportIdentity, FinancialNecessityEvidence,
    FinancialScenarioCompletion, FreshFinancialRecompute,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) enum FinancialCertificationPolicy {
    Exact,
    HeterogeneousComparators,
    ProducerTolerance,
    ProducerLocalSlots,
    ExactPartitionLocality,
    DeltaThreshold,
    DependencyRewire,
    BranchRestoreReplay,
}

#[derive(Clone)]
pub(in crate::tests::domains::fintech) struct FinancialScenarioCertificationClaim {
    reproduction: FinancialReproductionTuple,
    scenario: FinancialScenarioIdentity,
    policy: FinancialCertificationPolicy,
    dependency_revision: u64,
    verified_dependency_revision: u64,
    verified_reproduction: FinancialReproductionTuple,
    canonical_identity: FinancialCanonicalCaseIdentity,
    completion: FinancialScenarioCompletion,
}

struct FinancialScenarioCertificationEvidence {
    scenario: FinancialScenarioIdentity,
    seed: u64,
    dependency_revision: u64,
    canonical_identity: FinancialCanonicalCaseIdentity,
    _seal: ScenarioEvidenceSeal,
}

struct ScenarioEvidenceSeal;

pub(in crate::tests::domains::fintech) struct FinancialAspectCausalityCertificationRun {
    seed: u64,
    evidence_by_scenario:
        BTreeMap<FinancialScenarioIdentity, FinancialScenarioCertificationEvidence>,
    report_identity: FinancialCanonicalReportIdentity,
    _seal: CertificationRunSeal,
}

struct CertificationRunSeal;

impl FinancialScenarioCertificationClaim {
    pub(super) fn verify(
        compiled: &CompiledFinancialWorld,
        fresh: &FreshFinancialRecompute,
        necessity: &FinancialNecessityEvidence,
        reproduction: FinancialReproductionTuple,
        scenario: FinancialScenarioIdentity,
        policy: FinancialCertificationPolicy,
        revision_key: SemanticOutputKey,
        completion: FinancialScenarioCompletion,
    ) -> Result<Self, SignalError> {
        compiled.verify_committed_financial_truth(necessity.required_work())?;
        if compiled.economic_snapshot() != &fresh.economic_snapshot() {
            return Err(SignalError::invalid_input(
                "incremental financial snapshot disagrees with fresh recompute",
            ));
        }
        if compiled.ledger().observed_work() != *necessity.required_work() {
            return Err(SignalError::invalid_input(
                "incremental financial work disagrees with necessity manifest",
            ));
        }
        let committed_values = compiled.committed_financial_values()?;
        let expected_values = necessity.expected_committed_values(fresh);
        if committed_values != expected_values {
            let mismatches = committed_values
                .iter()
                .filter(|(key, value)| expected_values.get(key) != Some(value))
                .map(|(key, value)| (*key, *value, expected_values.get(key).copied()))
                .collect::<Vec<_>>();
            return Err(SignalError::invalid_input(format!(
                "committed financial artifacts disagree with the independent necessity oracle for {scenario:?}: {mismatches:?}"
            )));
        }
        let dependency_revision = compiled
            .graph()
            .dependency_revision(compiled.handles().node_for(revision_key))?
            .0;
        let expected_revision = compiled.baseline_dependency_revision(revision_key)
            + u64::from(scenario == FinancialScenarioIdentity::InstrumentDependencyRewire) * 3;
        if dependency_revision != expected_revision {
            return Err(SignalError::invalid_input(format!(
                "financial dependency revision drifted: expected {expected_revision}, got {dependency_revision}"
            )));
        }
        validate_reproduction(compiled, reproduction, scenario)?;
        let canonical_identity =
            FinancialCanonicalCaseIdentity::from_verified_claim(reproduction, policy)?;
        let claim = Self {
            reproduction,
            scenario,
            policy,
            dependency_revision,
            verified_dependency_revision: expected_revision,
            verified_reproduction: reproduction,
            canonical_identity,
            completion,
        };
        claim.validate()?;
        Ok(claim)
    }

    fn seal(self) -> Result<FinancialScenarioCertificationEvidence, SignalError> {
        self.validate()?;
        Ok(FinancialScenarioCertificationEvidence {
            scenario: self.scenario,
            seed: self.reproduction.seed,
            dependency_revision: self.dependency_revision,
            canonical_identity: self.canonical_identity,
            _seal: ScenarioEvidenceSeal,
        })
    }

    fn validate(&self) -> Result<(), SignalError> {
        if self.reproduction.scenario != self.scenario {
            return Err(SignalError::invalid_input(
                "financial certification scenario does not match reproduction tuple",
            ));
        }
        if self.policy != expected_policy(self.scenario) {
            return Err(SignalError::invalid_input(
                "financial certification policy does not match scenario",
            ));
        }
        if !reproduction_policy_matches(self.scenario, &self.reproduction) {
            return Err(SignalError::invalid_input(
                "financial certification reproduction policy does not match scenario",
            ));
        }
        if self.reproduction != self.verified_reproduction
            || self.dependency_revision != self.verified_dependency_revision
            || self.dependency_revision == 0
        {
            return Err(SignalError::invalid_input(
                "financial certification evidence is stale or lacks a mutation step",
            ));
        }
        if self.completion.scenario() != self.scenario {
            return Err(SignalError::invalid_input(
                "financial scenario lacks required lifecycle evidence",
            ));
        }
        let expected_identity = FinancialCanonicalCaseIdentity::from_verified_claim(
            self.verified_reproduction,
            self.policy,
        )?;
        if self.canonical_identity != expected_identity {
            return Err(SignalError::invalid_input(
                "financial certification canonical case identity drifted",
            ));
        }
        Ok(())
    }
}

fn reproduction_policy_matches(
    scenario: FinancialScenarioIdentity,
    reproduction: &FinancialReproductionTuple,
) -> bool {
    let expected_comparators = match scenario {
        FinancialScenarioIdentity::HeterogeneousConsumerComparators
        | FinancialScenarioIdentity::ToleranceSuppressedRepricing => {
            FinancialComparatorProfile::ExactToleranceAndInstalledTolerance
        }
        _ => FinancialComparatorProfile::Exact,
    };
    let output_policy_matches = match scenario {
        FinancialScenarioIdentity::ToleranceSuppressedRepricing => matches!(
            reproduction.policy.producer_output_equivalence,
            FinancialOutputEquivalencePolicy::Tolerance { .. }
        ),
        _ => matches!(
            reproduction.policy.producer_output_equivalence,
            FinancialOutputEquivalencePolicy::Exact
        ),
    };
    reproduction.policy.consumer_comparators == expected_comparators
        && output_policy_matches
        && reproduction.policy.diagnostics == DiagnosticsTier::Development
}

fn validate_reproduction(
    compiled: &CompiledFinancialWorld,
    reproduction: FinancialReproductionTuple,
    scenario: FinancialScenarioIdentity,
) -> Result<(), SignalError> {
    let (mutation_step, economic_delta) = scenario_mutation_contract(scenario);
    let expected_comparators =
        if compiled.definition().consumers().iter().any(|consumer| {
            matches!(
            consumer.comparator,
            crate::tests::domains::fintech::world::FinancialComparatorPolicy::InstalledTolerance {
                ..
            }
        )
        }) {
            FinancialComparatorProfile::ExactToleranceAndInstalledTolerance
        } else {
            FinancialComparatorProfile::Exact
        };
    let mut non_exact_output_policies = compiled
        .definition()
        .factor_output_equivalence_policies()
        .filter(|policy| !matches!(policy, FinancialOutputEquivalencePolicy::Exact));
    let expected_output_policy = non_exact_output_policies
        .next()
        .unwrap_or(FinancialOutputEquivalencePolicy::Exact);
    if non_exact_output_policies.any(|policy| policy != expected_output_policy) {
        return Err(SignalError::invalid_input(
            "financial certification world has no single producer output policy",
        ));
    }
    if reproduction.seed != compiled.definition().seed()
        || reproduction.scale != FinancialScaleTuple::from_definition(compiled.definition())
        || reproduction.mutation_step != mutation_step
        || reproduction.economic_delta != economic_delta
        || reproduction.policy.consumer_comparators != expected_comparators
        || reproduction.policy.producer_output_equivalence != expected_output_policy
        || reproduction.policy.diagnostics != DiagnosticsTier::Development
    {
        return Err(SignalError::invalid_input(
            "financial reproduction tuple does not identify the verified world and mutation",
        ));
    }
    Ok(())
}

fn scenario_mutation_contract(scenario: FinancialScenarioIdentity) -> (u32, i64) {
    match scenario {
        FinancialScenarioIdentity::QuoteToRiskAspectTranslation => (1, 20_000),
        FinancialScenarioIdentity::HeterogeneousConsumerComparators => (6, 20_000),
        FinancialScenarioIdentity::ToleranceSuppressedRepricing => (6, 20_000),
        FinancialScenarioIdentity::ProducerLocalFactorSlotCollision => (1, 20_000),
        FinancialScenarioIdentity::PartitionedCurveBucketBump => (2, 11),
        FinancialScenarioIdentity::GatedRepricingRelease => (3, 3_000),
        FinancialScenarioIdentity::InstrumentDependencyRewire => (2, 100),
        FinancialScenarioIdentity::BranchShockRestoreReplay => (2, 20_000),
    }
}

impl FinancialAspectCausalityCertificationRun {
    pub(in crate::tests::domains::fintech) fn seal(
        claims: impl IntoIterator<Item = FinancialScenarioCertificationClaim>,
    ) -> Result<Self, SignalError> {
        let mut evidence_by_scenario = BTreeMap::new();
        let mut seed = None;
        for claim in claims {
            let evidence = claim.seal()?;
            if seed
                .replace(evidence.seed)
                .is_some_and(|old| old != evidence.seed)
            {
                return Err(SignalError::invalid_input(
                    "financial certification evidence uses mixed seeds",
                ));
            }
            if evidence_by_scenario
                .insert(evidence.scenario, evidence)
                .is_some()
            {
                return Err(SignalError::invalid_input(
                    "financial certification contains duplicate scenario evidence",
                ));
            }
        }
        let expected = required_scenarios();
        if evidence_by_scenario
            .keys()
            .copied()
            .collect::<BTreeSet<_>>()
            != expected
        {
            return Err(SignalError::invalid_input(
                "financial certification is missing required scenario evidence",
            ));
        }
        let report_identity = FinancialCanonicalReportIdentity::from_cases(
            evidence_by_scenario
                .values()
                .map(|evidence| &evidence.canonical_identity),
        )?;
        Ok(Self {
            seed: seed.expect("required evidence set is nonempty"),
            evidence_by_scenario,
            report_identity,
            _seal: CertificationRunSeal,
        })
    }

    pub(in crate::tests::domains::fintech) const fn seed(&self) -> u64 {
        self.seed
    }

    pub(in crate::tests::domains::fintech) fn scenario_count(&self) -> usize {
        self.evidence_by_scenario.len()
    }

    pub(in crate::tests::domains::fintech) fn minimum_dependency_revision(&self) -> u64 {
        self.evidence_by_scenario
            .values()
            .map(|evidence| evidence.dependency_revision)
            .min()
            .unwrap_or_default()
    }

    pub(in crate::tests::domains::fintech) fn canonical_report_id(&self) -> &[u8; 32] {
        self.report_identity.digest_bytes()
    }
}

fn expected_policy(scenario: FinancialScenarioIdentity) -> FinancialCertificationPolicy {
    match scenario {
        FinancialScenarioIdentity::QuoteToRiskAspectTranslation => {
            FinancialCertificationPolicy::Exact
        }
        FinancialScenarioIdentity::HeterogeneousConsumerComparators => {
            FinancialCertificationPolicy::HeterogeneousComparators
        }
        FinancialScenarioIdentity::ToleranceSuppressedRepricing => {
            FinancialCertificationPolicy::ProducerTolerance
        }
        FinancialScenarioIdentity::ProducerLocalFactorSlotCollision => {
            FinancialCertificationPolicy::ProducerLocalSlots
        }
        FinancialScenarioIdentity::PartitionedCurveBucketBump => {
            FinancialCertificationPolicy::ExactPartitionLocality
        }
        FinancialScenarioIdentity::GatedRepricingRelease => {
            FinancialCertificationPolicy::DeltaThreshold
        }
        FinancialScenarioIdentity::InstrumentDependencyRewire => {
            FinancialCertificationPolicy::DependencyRewire
        }
        FinancialScenarioIdentity::BranchShockRestoreReplay => {
            FinancialCertificationPolicy::BranchRestoreReplay
        }
    }
}

fn required_scenarios() -> BTreeSet<FinancialScenarioIdentity> {
    BTreeSet::from([
        FinancialScenarioIdentity::QuoteToRiskAspectTranslation,
        FinancialScenarioIdentity::HeterogeneousConsumerComparators,
        FinancialScenarioIdentity::ToleranceSuppressedRepricing,
        FinancialScenarioIdentity::ProducerLocalFactorSlotCollision,
        FinancialScenarioIdentity::PartitionedCurveBucketBump,
        FinancialScenarioIdentity::GatedRepricingRelease,
        FinancialScenarioIdentity::InstrumentDependencyRewire,
        FinancialScenarioIdentity::BranchShockRestoreReplay,
    ])
}

#[cfg(test)]
#[path = "sealed_run_tests.rs"]
mod tests;
