use std::collections::{BTreeMap, BTreeSet};
use worth_query_certification::facade::{
    canonical_hostile_matrix, certify_hostile_provider, certify_provider_pair,
    WorthQueryCertificationCounter, WorthQueryCertificationCounterSetDenial,
    WorthQueryCertificationCounters, WorthQueryCertificationFailure,
    WorthQueryCertificationHostileAttack, WorthQueryCertificationJourneyCheckpoint as Checkpoint,
    WorthQueryCertificationObservation, WorthQueryCertificationObservationDenial,
    WorthQueryCertificationProvider, WorthQueryCertificationScenario,
    WorthQueryCertificationScenarioKind as Kind, WorthQueryCertificationSuite,
    WorthQueryCertificationSuiteDenial,
};

use crate::hostile_provider_fixture::HostileHarnessProvider;

struct SemanticProviderFixture {
    identity: &'static str,
    drift: Option<&'static str>,
}

impl SemanticProviderFixture {
    fn conforming(identity: &'static str) -> Self {
        Self {
            identity,
            drift: None,
        }
    }
}

impl WorthQueryCertificationProvider for SemanticProviderFixture {
    fn provider_identity(&self) -> &str {
        self.identity
    }

    fn execute(
        &mut self,
        scenario: &WorthQueryCertificationScenario,
    ) -> Result<WorthQueryCertificationObservation, String> {
        let fact = self
            .drift
            .unwrap_or_else(|| provider_semantic_fact(scenario.kind()));
        WorthQueryCertificationObservation::new(
            [("domain-result".to_owned(), fact.to_owned())],
            WorthQueryCertificationCounters::exact([(
                WorthQueryCertificationCounter::ProviderContacts,
                1,
            )])
            .unwrap(),
        )
        .map_err(|denial| format!("invalid fixture observation: {denial:?}"))
    }
}

#[test]
fn provider_pair_reports_semantics_and_real_journey_requirements_separately() {
    let suite = complete_suite();
    let mut first = SemanticProviderFixture::conforming("ui-reference-provider");
    let mut second = SemanticProviderFixture::conforming("ui-alternate-provider");
    let report = certify_provider_pair(&suite, &mut first, &mut second).unwrap();

    assert_eq!(report.scenarios().len(), Kind::ALL.len());
    assert_eq!(
        report.provider_identities(),
        &[
            "ui-reference-provider".to_owned(),
            "ui-alternate-provider".to_owned()
        ]
    );
    let required = report
        .scenarios()
        .iter()
        .flat_map(|scenario| scenario.required_journey_checkpoints().iter().copied())
        .collect::<BTreeSet<_>>();
    assert_eq!(required, Checkpoint::ALL.into_iter().collect());
}

#[test]
fn canonical_hostile_runner_invokes_every_registered_attack_once() {
    let mut provider = HostileHarnessProvider::conforming("query-hostile-fixture");
    let report = certify_hostile_provider(&mut provider).unwrap();
    assert_eq!(
        report.hostile_case_count(),
        canonical_hostile_matrix().len()
    );
    assert_eq!(
        provider.observed_attacks(),
        WorthQueryCertificationHostileAttack::ALL
    );
}

#[test]
fn incomplete_semantic_fixture_is_rejected_before_provider_execution() {
    let scenario = WorthQueryCertificationScenario::with_oracle(
        "workflow",
        Kind::Workflow,
        [("domain-result".to_owned(), "workflow".to_owned())],
        expected_counters(),
    )
    .unwrap();
    let denial = WorthQueryCertificationSuite::complete([scenario]).unwrap_err();
    let WorthQueryCertificationSuiteDenial::MissingScenarioKinds(missing) = denial else {
        panic!("one unique scenario should fail only for incomplete semantic coverage")
    };
    assert_eq!(missing.len(), Kind::ALL.len() - 1);
}

#[test]
fn semantic_and_structural_drift_cannot_hide_behind_green_execution() {
    let suite = complete_suite();
    let mut first = SemanticProviderFixture::conforming("reference");
    let mut second = SemanticProviderFixture::conforming("alternate");
    first.drift = Some("shared-wrong-answer");
    second.drift = Some("shared-wrong-answer");
    assert!(matches!(
        certify_provider_pair(&suite, &mut first, &mut second),
        Err(WorthQueryCertificationFailure::OracleMismatch { .. })
    ));
}

#[test]
fn duplicate_counters_are_rejected_instead_of_silently_overwritten() {
    let denial = WorthQueryCertificationCounters::exact([
        (WorthQueryCertificationCounter::ProviderContacts, 1),
        (WorthQueryCertificationCounter::ProviderContacts, 2),
    ])
    .unwrap_err();
    assert_eq!(
        denial,
        WorthQueryCertificationCounterSetDenial::DuplicateCounter(
            WorthQueryCertificationCounter::ProviderContacts
        )
    );
}

#[test]
fn hostile_counter_drift_fails_even_when_the_denial_kind_matches() {
    let mut second = HostileHarnessProvider::with_counter_drift(
        "alternate",
        WorthQueryCertificationHostileAttack::StaleGeneration,
    );
    assert!(matches!(
        certify_hostile_provider(&mut second),
        Err(WorthQueryCertificationFailure::HostileEvidenceMismatch {
            attack: WorthQueryCertificationHostileAttack::StaleGeneration,
            ..
        })
    ));
}

#[test]
fn hostile_registry_is_complete_unique_and_exactly_accounted() {
    let matrix = canonical_hostile_matrix();
    let attacks = matrix
        .iter()
        .map(|case| case.attack())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        attacks,
        WorthQueryCertificationHostileAttack::ALL
            .into_iter()
            .collect()
    );
    for case in matrix {
        assert_eq!(
            case.expected().counters().values(),
            &BTreeMap::from([(WorthQueryCertificationCounter::BoundaryChecks, 1)]),
            "{:?} must perform exactly one boundary check and no downstream work",
            case.attack()
        );
    }
}

#[test]
fn duplicate_semantic_facts_are_rejected_instead_of_silently_overwritten() {
    let denial = WorthQueryCertificationObservation::new(
        [
            ("result".to_owned(), "first".to_owned()),
            ("result".to_owned(), "second".to_owned()),
        ],
        WorthQueryCertificationCounters::default(),
    )
    .unwrap_err();
    assert_eq!(
        denial,
        WorthQueryCertificationObservationDenial::DuplicateSemanticFact("result".to_owned())
    );
}

#[test]
fn provider_pair_requires_distinct_valid_identities() {
    let suite = complete_suite();
    let mut first = SemanticProviderFixture::conforming("same-provider");
    let mut second = SemanticProviderFixture::conforming("same-provider");
    assert_eq!(
        certify_provider_pair(&suite, &mut first, &mut second).unwrap_err(),
        WorthQueryCertificationFailure::SameProviderIdentity
    );

    let mut invalid = SemanticProviderFixture::conforming("invalid provider");
    let mut valid = SemanticProviderFixture::conforming("valid-provider");
    assert!(matches!(
        certify_provider_pair(&suite, &mut invalid, &mut valid),
        Err(WorthQueryCertificationFailure::InvalidProviderIdentity(identity))
            if identity == "invalid provider"
    ));
}

fn complete_suite() -> WorthQueryCertificationSuite {
    WorthQueryCertificationSuite::complete(Kind::ALL.into_iter().map(|kind| {
        WorthQueryCertificationScenario::with_oracle(
            oracle_semantic_fact(kind),
            kind,
            [(
                "domain-result".to_owned(),
                oracle_semantic_fact(kind).to_owned(),
            )],
            expected_counters(),
        )
        .unwrap()
    }))
    .unwrap()
}

fn expected_counters() -> WorthQueryCertificationCounters {
    WorthQueryCertificationCounters::exact([(WorthQueryCertificationCounter::ProviderContacts, 1)])
        .unwrap()
}

fn oracle_semantic_fact(kind: Kind) -> &'static str {
    match kind {
        Kind::Workflow => "workflow",
        Kind::Replay => "replay",
        Kind::ConditionalNode => "conditional",
        Kind::SemanticAspectCorrespondence => "correspondence",
        Kind::Reversal => "reversal",
        Kind::Lineage => "lineage",
        Kind::DependencyImpact => "impact",
        Kind::CounterContract => "counters",
    }
}

fn provider_semantic_fact(kind: Kind) -> &'static str {
    match kind {
        Kind::Workflow => "workflow",
        Kind::Replay => "replay",
        Kind::ConditionalNode => "conditional",
        Kind::SemanticAspectCorrespondence => "correspondence",
        Kind::Reversal => "reversal",
        Kind::Lineage => "lineage",
        Kind::DependencyImpact => "impact",
        Kind::CounterContract => "counters",
    }
}
