use std::collections::{BTreeMap, BTreeSet};

use worth_foundational::facade::{
    canonicalization, prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisValue, CanonicalDerivedDigest,
    CanonicalDigestAlgorithmId, CanonicalIntegerWidth, CanonicalizationRuleVersion,
};
use worth_proof::TransitionOutcome;
use worth_runtime_bridge::facade::BridgeDiagnosticsTier;

use super::necessity_manifest::CrossRuntimeInvalidationNecessityManifest;
use super::production_evidence::{
    CertificationComparatorPolicy, CertificationExecutionLane, PerformedScenarioEvidence,
};
use super::production_scenarios::run_scenario;
use super::world::{GranularInvalidationScenario, GranularInvalidationWorldDefinition};

const CASE_DOMAIN: CanonicalBasisDomain =
    CanonicalBasisDomain::Future("WORTH.query.granular-invalidation-case");
const REPORT_DOMAIN: CanonicalBasisDomain =
    CanonicalBasisDomain::Future("WORTH.query.granular-invalidation-report");

#[derive(Clone)]
pub struct VerifiedGranularInvalidationClaim {
    pub(super) scenario: GranularInvalidationScenario,
    pub(super) seed: u64,
    digest: CanonicalDerivedDigest,
}

pub struct GranularInvalidationCertificationRun {
    claims: BTreeMap<GranularInvalidationScenario, VerifiedGranularInvalidationClaim>,
    report_digest: CanonicalDerivedDigest,
}

pub fn verify_production_scenario(
    world: GranularInvalidationWorldDefinition,
    expected: CrossRuntimeInvalidationNecessityManifest,
    actual: PerformedScenarioEvidence,
) -> Result<VerifiedGranularInvalidationClaim, &'static str> {
    if actual.scenario() != world.scenario || actual.seed() != world.seed {
        return Err("performed scenario identity differs from the declared world");
    }
    let observed = actual.identities();
    if observed.relational != expected.relational {
        return Err("performed relational identities differ from the independent manifest");
    }
    if observed.bridge != expected.bridge {
        return Err("performed Bridge identities differ from the independent manifest");
    }
    if observed.signal != expected.signal {
        return Err("performed Signal identities differ from the independent manifest");
    }
    if observed.impacts != expected.impacts {
        return Err("performed impact identities differ from the independent manifest");
    }
    if observed.maintenance != expected.maintenance {
        return Err("performed maintenance identities differ from the independent manifest");
    }
    if observed.deliveries != expected.deliveries {
        return Err("performed delivery identities differ from the independent manifest");
    }
    if observed.exclusions != expected.exclusions {
        return Err("performed exclusion identities differ from the independent manifest");
    }
    if actual.policy() != &expected_policy(world.scenario)
        || actual.diagnostics_tier() != BridgeDiagnosticsTier::Standard
        || actual.execution_lane() != CertificationExecutionLane::Scheduled
    {
        return Err("performed scenario policy differs from its installed contract");
    }
    if actual.runtime_ordinal() == 0 || actual.runtime_generation() == 0 {
        return Err("performed scenario carries stale runtime identity");
    }
    let expected_performed_signal_deliveries = expected
        .signal
        .iter()
        .filter(|identity| identity.contains(":performed-signal:"))
        .count();
    if actual.direct_truth_deliveries() != expected.bridge.len()
        || actual.performed_signal_deliveries() != expected_performed_signal_deliveries
    {
        return Err("direct truth and performed Signal counts were conflated");
    }
    if actual.counters().value("bridge.source-load-attempts") == 0 {
        return Err("performed Bridge counters contain no source load");
    }
    if actual.counters().value("query.admission.admitted-impacts")
        < expected_performed_signal_deliveries as u64
    {
        return Err("performed Query admission counters omit expected impacts");
    }
    if actual.counters().sum_prefix("query.admission.role.") < expected.impacts.len() as u64 {
        return Err("performed Query role counters omit expected semantic roles");
    }
    if !expected.maintenance.is_empty()
        && actual.counters().value("query.maintenance.operations") == 0
    {
        return Err("performed Query counters contain no maintenance");
    }
    let mut entries = vec![
        text_entry(CASE_DOMAIN, "scenario", world.scenario.name()),
        unsigned_entry(CASE_DOMAIN, "seed", world.seed),
        text_entry(
            CASE_DOMAIN,
            "diagnostics-tier",
            diagnostics_tier_name(actual.diagnostics_tier()),
        ),
        text_entry(CASE_DOMAIN, "execution-lane", "scheduled"),
        unsigned_entry(CASE_DOMAIN, "runtime-ordinal", actual.runtime_ordinal()),
        unsigned_entry(
            CASE_DOMAIN,
            "runtime-generation",
            actual.runtime_generation(),
        ),
        unsigned_entry(
            CASE_DOMAIN,
            "direct-truth-deliveries",
            actual.direct_truth_deliveries() as u64,
        ),
        unsigned_entry(
            CASE_DOMAIN,
            "performed-signal-deliveries",
            actual.performed_signal_deliveries() as u64,
        ),
    ];
    append_policy(&mut entries, actual.policy());
    append_identity_set(&mut entries, "R", &expected.relational);
    append_identity_set(&mut entries, "B", &expected.bridge);
    append_identity_set(&mut entries, "S", &expected.signal);
    append_identity_set(&mut entries, "I", &expected.impacts);
    append_identity_set(&mut entries, "M", &expected.maintenance);
    append_identity_set(&mut entries, "D", &expected.deliveries);
    append_identity_set(&mut entries, "X", &expected.exclusions);
    for (ordinal, (name, value)) in actual.counters().rows().enumerate() {
        entries.push(owned_text_entry(
            CASE_DOMAIN,
            format!("counter.{ordinal:03}.name"),
            name,
        ));
        entries.push(owned_unsigned_entry(
            CASE_DOMAIN,
            format!("counter.{ordinal:03}.value"),
            value,
        ));
    }
    let digest = canonical_digest(CASE_DOMAIN, entries)?;
    Ok(VerifiedGranularInvalidationClaim {
        scenario: world.scenario,
        seed: world.seed,
        digest,
    })
}

impl GranularInvalidationCertificationRun {
    pub fn seal(claims: Vec<VerifiedGranularInvalidationClaim>) -> Result<Self, &'static str> {
        if claims.len() != GranularInvalidationScenario::ALL.len() {
            return Err("certification requires exactly six scenario claims");
        }
        let seeds = claims
            .iter()
            .map(|claim| claim.seed)
            .collect::<BTreeSet<_>>();
        if seeds.len() != 1 {
            return Err("certification claims do not share one reproduction seed");
        }
        let mut by_scenario = BTreeMap::new();
        for claim in claims {
            if by_scenario.insert(claim.scenario, claim).is_some() {
                return Err("certification contains a duplicate scenario");
            }
        }
        if !GranularInvalidationScenario::ALL
            .iter()
            .all(|scenario| by_scenario.contains_key(scenario))
        {
            return Err("certification is missing a required scenario");
        }
        let entries = by_scenario
            .values()
            .enumerate()
            .map(|(ordinal, claim)| {
                CanonicalBasisEntry::new(
                    REPORT_DOMAIN,
                    CanonicalBasisLocus::Named(format!("case.{ordinal:02}").into()),
                    CanonicalBasisEntryKind::Identity,
                    CanonicalBasisValue::BytesDigest(
                        worth_foundational::facade::CanonicalDigestId::new(
                            *claim.digest.value().bytes(),
                        ),
                    ),
                )
            })
            .collect::<Vec<_>>();
        let report_digest = canonical_digest(REPORT_DOMAIN, entries)?;
        Ok(Self {
            claims: by_scenario,
            report_digest,
        })
    }

    pub fn case_count(&self) -> usize {
        self.claims.len()
    }

    pub fn report_digest(&self) -> &[u8; 32] {
        self.report_digest.value().bytes()
    }
}

pub fn production_claims(seed: u64) -> Vec<VerifiedGranularInvalidationClaim> {
    GranularInvalidationScenario::ALL
        .into_iter()
        .map(|scenario| {
            verify_evidence(scenario, seed, run_scenario(scenario, seed))
                .unwrap_or_else(|error| panic!("{} failed certification: {error}", scenario.name()))
        })
        .collect()
}

pub(super) fn verify_evidence(
    scenario: GranularInvalidationScenario,
    seed: u64,
    evidence: PerformedScenarioEvidence,
) -> Result<VerifiedGranularInvalidationClaim, &'static str> {
    let world = GranularInvalidationWorldDefinition::for_scenario(scenario, seed);
    let expected = CrossRuntimeInvalidationNecessityManifest::derive(&world);
    verify_production_scenario(world, expected, evidence)
}

fn expected_policy(scenario: GranularInvalidationScenario) -> CertificationComparatorPolicy {
    if scenario == GranularInvalidationScenario::SuppressedQuoteNoQueryPatch {
        CertificationComparatorPolicy::Tolerance {
            epsilon: 5,
            provider_identity: "worth.query.financial.quote-tolerance-5",
        }
    } else {
        CertificationComparatorPolicy::Exact
    }
}

fn append_policy(entries: &mut Vec<CanonicalBasisEntry>, policy: &CertificationComparatorPolicy) {
    match policy {
        CertificationComparatorPolicy::Exact => {
            entries.push(text_entry(CASE_DOMAIN, "policy.kind", "exact"));
        }
        CertificationComparatorPolicy::Tolerance {
            epsilon,
            provider_identity,
        } => {
            entries.push(text_entry(CASE_DOMAIN, "policy.kind", "tolerance"));
            entries.push(unsigned_entry(CASE_DOMAIN, "policy.epsilon", *epsilon));
            entries.push(text_entry(
                CASE_DOMAIN,
                "policy.provider",
                provider_identity,
            ));
        }
    }
}

const fn diagnostics_tier_name(tier: BridgeDiagnosticsTier) -> &'static str {
    match tier {
        BridgeDiagnosticsTier::Minimal => "minimal",
        BridgeDiagnosticsTier::Standard => "standard",
        BridgeDiagnosticsTier::Exhaustive => "exhaustive",
    }
}

fn canonical_digest(
    domain: CanonicalBasisDomain,
    entries: impl IntoIterator<Item = CanonicalBasisEntry>,
) -> Result<CanonicalDerivedDigest, &'static str> {
    let version = CanonicalizationRuleVersion::new("WORTH.query.granular-invalidation.v1")
        .ok_or("invalid canonicalization rule version")?;
    let TransitionOutcome::Success(ready) =
        prepare_canonical_basis_sequence(version, domain, entries)
    else {
        return Err("canonical case basis was denied");
    };
    let TransitionOutcome::Success(digest_ready) = canonicalization()
        .digest()
        .for_sequence(ready, CanonicalDigestAlgorithmId::sha256())
    else {
        return Err("canonical case digest was denied");
    };
    Ok(canonicalization().digest().derive(digest_ready))
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

fn owned_text_entry(
    domain: CanonicalBasisDomain,
    locus: String,
    value: &str,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        domain,
        CanonicalBasisLocus::Named(locus.into()),
        CanonicalBasisEntryKind::Identity,
        CanonicalBasisValue::ExactText(value.to_owned().into()),
    )
}

fn unsigned_entry(
    domain: CanonicalBasisDomain,
    locus: &'static str,
    value: u64,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        domain,
        CanonicalBasisLocus::Named(locus.into()),
        CanonicalBasisEntryKind::Identity,
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits64,
            value: value as u128,
        },
    )
}

fn owned_unsigned_entry(
    domain: CanonicalBasisDomain,
    locus: String,
    value: u64,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        domain,
        CanonicalBasisLocus::Named(locus.into()),
        CanonicalBasisEntryKind::Identity,
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits64,
            value: value as u128,
        },
    )
}

fn append_identity_set(
    entries: &mut Vec<CanonicalBasisEntry>,
    family: &'static str,
    identities: &BTreeSet<String>,
) {
    entries.push(unsigned_entry(
        CASE_DOMAIN,
        match family {
            "R" => "R.count",
            "B" => "B.count",
            "S" => "S.count",
            "I" => "I.count",
            "M" => "M.count",
            "D" => "D.count",
            _ => "X.count",
        },
        identities.len() as u64,
    ));
    entries.extend(identities.iter().enumerate().map(|(ordinal, identity)| {
        CanonicalBasisEntry::new(
            CASE_DOMAIN,
            CanonicalBasisLocus::Named(format!("{family}.{ordinal:04}").into()),
            CanonicalBasisEntryKind::Identity,
            CanonicalBasisValue::ExactText(identity.clone().into()),
        )
    }));
}
