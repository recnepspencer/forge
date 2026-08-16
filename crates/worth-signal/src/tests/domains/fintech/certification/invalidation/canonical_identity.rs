use crate::data::error::SignalError;
use crate::facade::DiagnosticsTier;
use crate::tests::domains::fintech::world::{
    FinancialComparatorProfile, FinancialOutputEquivalencePolicy, FinancialReproductionTuple,
    FinancialScenarioIdentity,
};
use worth_foundational::facade::{
    canonicalization, prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisSequence, CanonicalBasisValue,
    CanonicalDerivedDigest, CanonicalDigestAlgorithmId, CanonicalDigestId, CanonicalIntegerWidth,
    CanonicalizationRuleVersion,
};
use worth_proof::TransitionOutcome;

use super::FinancialCertificationPolicy;

const CASE_DOMAIN: CanonicalBasisDomain =
    CanonicalBasisDomain::Future("WORTH.signal.financial-certification-case");
const REPORT_DOMAIN: CanonicalBasisDomain =
    CanonicalBasisDomain::Future("WORTH.signal.financial-certification-report");
const RULE_VERSION: &str = "WORTH.signal.financial-certification.v1";

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct FinancialCanonicalCaseIdentity {
    basis: CanonicalBasisSequence,
    digest: CanonicalDerivedDigest,
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct FinancialCanonicalReportIdentity {
    basis: CanonicalBasisSequence,
    digest: CanonicalDerivedDigest,
}

impl FinancialCanonicalCaseIdentity {
    pub(super) fn from_verified_claim(
        reproduction: FinancialReproductionTuple,
        policy: FinancialCertificationPolicy,
    ) -> Result<Self, SignalError> {
        let scale = reproduction.scale;
        let output_policy = reproduction.policy.producer_output_equivalence;
        let entries = [
            text_entry(
                CASE_DOMAIN,
                "scenario",
                scenario_name(reproduction.scenario),
            ),
            unsigned_entry(CASE_DOMAIN, "seed", reproduction.seed as u128),
            unsigned_entry(CASE_DOMAIN, "scale.factors", scale.factors as u128),
            unsigned_entry(CASE_DOMAIN, "scale.positions", scale.positions as u128),
            unsigned_entry(CASE_DOMAIN, "scale.books", scale.books as u128),
            unsigned_entry(CASE_DOMAIN, "scale.desks", scale.desks as u128),
            text_entry(CASE_DOMAIN, "policy", policy_name(policy)),
            text_entry(
                CASE_DOMAIN,
                "policy.consumer_comparators",
                comparator_profile_name(reproduction.policy.consumer_comparators),
            ),
            text_entry(
                CASE_DOMAIN,
                "policy.producer_output_equivalence",
                output_policy_name(output_policy),
            ),
            unsigned_entry(
                CASE_DOMAIN,
                "policy.producer_output_epsilon",
                output_policy_epsilon(output_policy) as u128,
            ),
            text_entry(
                CASE_DOMAIN,
                "policy.diagnostics",
                diagnostics_name(reproduction.policy.diagnostics),
            ),
            unsigned_entry(
                CASE_DOMAIN,
                "mutation.step",
                reproduction.mutation_step as u128,
            ),
            signed_entry(
                CASE_DOMAIN,
                "mutation.economic_delta",
                reproduction.economic_delta as i128,
            ),
        ];
        let (basis, digest) = canonical_identity(CASE_DOMAIN, entries)?;
        Ok(Self { basis, digest })
    }

    pub(super) fn from_extended_entries(
        entries: impl IntoIterator<Item = CanonicalBasisEntry>,
    ) -> Result<Self, SignalError> {
        let (basis, digest) = canonical_identity(CASE_DOMAIN, entries)?;
        Ok(Self { basis, digest })
    }

    pub(super) fn digest_id(&self) -> CanonicalDigestId {
        CanonicalDigestId::new(*self.digest.value().bytes())
    }

    pub(in crate::tests::domains::fintech) fn digest_bytes(&self) -> &[u8; 32] {
        self.digest.value().bytes()
    }
}

impl FinancialCanonicalReportIdentity {
    pub(in crate::tests::domains::fintech) fn from_cases<'a>(
        cases: impl IntoIterator<Item = &'a FinancialCanonicalCaseIdentity>,
    ) -> Result<Self, SignalError> {
        let mut digests = cases
            .into_iter()
            .map(FinancialCanonicalCaseIdentity::digest_id)
            .collect::<Vec<_>>();
        if digests.is_empty() {
            return Err(SignalError::invalid_input(
                "financial certification report contains no case identity",
            ));
        }
        digests.sort_by_key(|digest| *digest.bytes());
        if digests
            .windows(2)
            .any(|pair| pair[0].bytes() == pair[1].bytes())
        {
            return Err(SignalError::invalid_input(
                "financial certification report contains duplicate case identity",
            ));
        }
        let entries = digests
            .into_iter()
            .enumerate()
            .map(|(index, digest)| {
                CanonicalBasisEntry::new(
                    REPORT_DOMAIN,
                    CanonicalBasisLocus::Named(format!("case.{index:04}").into()),
                    CanonicalBasisEntryKind::Identity,
                    CanonicalBasisValue::BytesDigest(digest),
                )
            })
            .collect::<Vec<_>>();
        let (basis, digest) = canonical_identity(REPORT_DOMAIN, entries)?;
        Ok(Self { basis, digest })
    }

    pub(in crate::tests::domains::fintech) fn digest_bytes(&self) -> &[u8; 32] {
        self.digest.value().bytes()
    }
}

fn canonical_identity(
    domain: CanonicalBasisDomain,
    entries: impl IntoIterator<Item = CanonicalBasisEntry>,
) -> Result<(CanonicalBasisSequence, CanonicalDerivedDigest), SignalError> {
    let version = CanonicalizationRuleVersion::new(RULE_VERSION)
        .expect("financial certification rule version is a valid stable literal");
    let ready = match prepare_canonical_basis_sequence(version, domain, entries) {
        TransitionOutcome::Success(ready) => ready,
        denied => {
            return Err(SignalError::internal(format!(
                "financial certification canonical basis was denied: {denied:?}"
            )))
        }
    };
    let basis = ready.payload().clone();
    let digest_ready = match canonicalization()
        .digest()
        .for_sequence(ready, CanonicalDigestAlgorithmId::sha256())
    {
        TransitionOutcome::Success(ready) => ready,
        denied => {
            return Err(SignalError::internal(format!(
                "financial certification digest was denied: {denied:?}"
            )))
        }
    };
    Ok((basis, canonicalization().digest().derive(digest_ready)))
}

fn text_entry(
    domain: CanonicalBasisDomain,
    locus: &'static str,
    value: &'static str,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        domain,
        CanonicalBasisLocus::Named(locus.into()),
        CanonicalBasisEntryKind::Identity,
        CanonicalBasisValue::ExactText(value.into()),
    )
}

fn unsigned_entry(
    domain: CanonicalBasisDomain,
    locus: &'static str,
    value: u128,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        domain,
        CanonicalBasisLocus::Named(locus.into()),
        CanonicalBasisEntryKind::Identity,
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits128,
            value,
        },
    )
}

fn signed_entry(
    domain: CanonicalBasisDomain,
    locus: &'static str,
    value: i128,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        domain,
        CanonicalBasisLocus::Named(locus.into()),
        CanonicalBasisEntryKind::Identity,
        CanonicalBasisValue::SignedInteger {
            width: CanonicalIntegerWidth::Bits128,
            value,
        },
    )
}

const fn scenario_name(scenario: FinancialScenarioIdentity) -> &'static str {
    match scenario {
        FinancialScenarioIdentity::QuoteToRiskAspectTranslation => "quote-to-risk-translation",
        FinancialScenarioIdentity::HeterogeneousConsumerComparators => "heterogeneous-comparators",
        FinancialScenarioIdentity::ToleranceSuppressedRepricing => "tolerance-suppression",
        FinancialScenarioIdentity::ProducerLocalFactorSlotCollision => "producer-local-slots",
        FinancialScenarioIdentity::PartitionedCurveBucketBump => "partitioned-curve-bump",
        FinancialScenarioIdentity::GatedRepricingRelease => "gated-repricing-release",
        FinancialScenarioIdentity::InstrumentDependencyRewire => "instrument-dependency-rewire",
        FinancialScenarioIdentity::BranchShockRestoreReplay => "branch-restore-replay",
    }
}

const fn policy_name(policy: FinancialCertificationPolicy) -> &'static str {
    match policy {
        FinancialCertificationPolicy::Exact => "exact",
        FinancialCertificationPolicy::HeterogeneousComparators => "heterogeneous-comparators",
        FinancialCertificationPolicy::ProducerTolerance => "producer-tolerance",
        FinancialCertificationPolicy::ProducerLocalSlots => "producer-local-slots",
        FinancialCertificationPolicy::ExactPartitionLocality => "exact-partition-locality",
        FinancialCertificationPolicy::DeltaThreshold => "delta-threshold",
        FinancialCertificationPolicy::DependencyRewire => "dependency-rewire",
        FinancialCertificationPolicy::BranchRestoreReplay => "branch-restore-replay",
    }
}

const fn comparator_profile_name(profile: FinancialComparatorProfile) -> &'static str {
    match profile {
        FinancialComparatorProfile::Exact => "exact",
        FinancialComparatorProfile::ExactToleranceAndInstalledTolerance => {
            "exact-tolerance-installed"
        }
    }
}

const fn output_policy_name(policy: FinancialOutputEquivalencePolicy) -> &'static str {
    match policy {
        FinancialOutputEquivalencePolicy::Exact => "exact",
        FinancialOutputEquivalencePolicy::Tolerance { .. } => "tolerance",
    }
}

const fn output_policy_epsilon(policy: FinancialOutputEquivalencePolicy) -> u64 {
    match policy {
        FinancialOutputEquivalencePolicy::Exact => 0,
        FinancialOutputEquivalencePolicy::Tolerance { epsilon } => epsilon,
    }
}

fn diagnostics_name(tier: DiagnosticsTier) -> &'static str {
    tier.label()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::domains::fintech::world::{
        compile_financial_world, FinancialWorldDefinition,
    };

    #[test]
    fn canonical_case_identity_is_stable_and_mutation_sensitive() {
        let baseline =
            compile_financial_world(FinancialWorldDefinition::deterministic(41)).unwrap();
        let reproduction = baseline.reproduction();
        let first = FinancialCanonicalCaseIdentity::from_verified_claim(
            reproduction,
            FinancialCertificationPolicy::Exact,
        )
        .unwrap();
        let second = FinancialCanonicalCaseIdentity::from_verified_claim(
            reproduction,
            FinancialCertificationPolicy::Exact,
        )
        .unwrap();
        assert_eq!(first, second);

        let mut perturbed = reproduction;
        perturbed.policy.producer_output_equivalence =
            FinancialOutputEquivalencePolicy::Tolerance { epsilon: 999 };
        let perturbed = FinancialCanonicalCaseIdentity::from_verified_claim(
            perturbed,
            FinancialCertificationPolicy::Exact,
        )
        .unwrap();
        assert_ne!(first.digest_bytes(), perturbed.digest_bytes());
    }
}
