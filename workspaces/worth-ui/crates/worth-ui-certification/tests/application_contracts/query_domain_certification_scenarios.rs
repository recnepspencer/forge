use worth_query::facade::domain::{
    WorthQueryOperationLineageContract, WorthQueryOperationReplayContract,
    WorthQueryOperationResultState, WorthQueryOperationReversalContract,
    WorthQuerySupportRequirement,
};
use worth_query_certification::facade::{
    certify_provider_pair, WorthQueryCertificationCounter, WorthQueryCertificationCounters,
    WorthQueryCertificationObservation, WorthQueryCertificationProvider,
    WorthQueryCertificationScenario, WorthQueryCertificationScenarioKind as Kind,
    WorthQueryCertificationSuite,
};
use worth_ui_query_binding::certification::{
    WorthUiInstalledOperationCertificationFacts, WorthUiInstalledQueryTestFixture,
};

struct WorthUiSemanticProvider {
    identity: &'static str,
    facts: Option<WorthUiInstalledOperationCertificationFacts>,
    semantic_drift: Option<(Kind, &'static str)>,
}

impl WorthUiSemanticProvider {
    fn new(identity: &'static str) -> Self {
        Self {
            identity,
            facts: None,
            semantic_drift: None,
        }
    }

    fn with_semantic_drift(identity: &'static str, kind: Kind, observed: &'static str) -> Self {
        Self {
            identity,
            facts: None,
            semantic_drift: Some((kind, observed)),
        }
    }

    fn facts(&mut self) -> &WorthUiInstalledOperationCertificationFacts {
        self.facts.get_or_insert_with(|| {
            WorthUiInstalledQueryTestFixture::new(self.identity)
                .installed_operation_certification_facts()
        })
    }
}

impl WorthQueryCertificationProvider for WorthUiSemanticProvider {
    fn provider_identity(&self) -> &str {
        self.identity
    }

    fn execute(
        &mut self,
        scenario: &WorthQueryCertificationScenario,
    ) -> Result<WorthQueryCertificationObservation, String> {
        let value = match self.semantic_drift {
            Some((kind, observed)) if kind == scenario.kind() => observed.to_owned(),
            _ => provider_fact(scenario.kind(), self.facts()),
        };
        WorthQueryCertificationObservation::new(
            [(fact_key(scenario.kind()).to_owned(), value)],
            expected_provider_counters(),
        )
        .map_err(|denial| format!("Worth UI produced duplicate semantic facts: {denial:?}"))
    }
}

/// Worth UI contributes real installed-operation observations only. Query
/// certification owns the generic authority-impossibility and hostile matrix.
#[test]
fn worth_ui_registers_its_narrow_installed_operation_semantics() {
    let suite = complete_worth_ui_suite();
    let mut first = WorthUiSemanticProvider::new("worth-ui-fixture-a");
    let mut second = WorthUiSemanticProvider::new("worth-ui-fixture-b");
    let report = certify_provider_pair(&suite, &mut first, &mut second)
        .expect("two independent Worth UI worlds match the domain oracle");

    assert_eq!(report.scenarios().len(), Kind::ALL.len());
    assert_eq!(
        report.provider_identities(),
        &[
            "worth-ui-fixture-a".to_owned(),
            "worth-ui-fixture-b".to_owned()
        ]
    );
}

#[test]
fn worth_ui_domain_oracle_rejects_adapter_semantic_drift() {
    let suite = complete_worth_ui_suite();
    let mut first = WorthUiSemanticProvider::new("worth-ui-fixture-a");
    let mut second = WorthUiSemanticProvider::with_semantic_drift(
        "worth-ui-fixture-b",
        Kind::DependencyImpact,
        "required",
    );

    assert!(matches!(
        certify_provider_pair(&suite, &mut first, &mut second),
        Err(worth_query_certification::facade::WorthQueryCertificationFailure::OracleMismatch {
            provider,
            scenario,
            ..
        }) if provider == "worth-ui-fixture-b" && scenario == "allocation-dependency-impact"
    ));
}

fn complete_worth_ui_suite() -> WorthQueryCertificationSuite {
    let cases = [
        ("measurement-workflow", Kind::Workflow),
        ("measurement-replay", Kind::Replay),
        ("visibility-condition", Kind::ConditionalNode),
        (
            "measurement-allocation-correspondence",
            Kind::SemanticAspectCorrespondence,
        ),
        ("measurement-reversal", Kind::Reversal),
        ("measurement-lineage", Kind::Lineage),
        ("allocation-dependency-impact", Kind::DependencyImpact),
        ("measurement-counter-contract", Kind::CounterContract),
    ];
    WorthQueryCertificationSuite::complete(cases.map(|(identity, kind)| {
        WorthQueryCertificationScenario::with_oracle(
            identity,
            kind,
            [(fact_key(kind).to_owned(), oracle_fact(kind).to_owned())],
            expected_provider_counters(),
        )
        .expect("static domain scenario identities and independent oracles are canonical")
    }))
    .expect("Worth UI covers every domain-supplied semantic family")
}

fn fact_key(kind: Kind) -> &'static str {
    match kind {
        Kind::Workflow => "workflow-receipts",
        Kind::Replay => "replay-contract",
        Kind::ConditionalNode => "conditional-node-count",
        Kind::SemanticAspectCorrespondence => "semantic-read-count",
        Kind::Reversal => "reversal-contract",
        Kind::Lineage => "lineage-contract",
        Kind::DependencyImpact => "dependency-impact-support",
        Kind::CounterContract => "settled-execution",
    }
}

fn oracle_fact(kind: Kind) -> &'static str {
    match kind {
        Kind::Workflow => "stages=2;effects=1",
        Kind::Replay => "re-executable",
        Kind::ConditionalNode => "0",
        Kind::SemanticAspectCorrespondence => "2",
        Kind::Reversal => "irreversible",
        Kind::Lineage => "not-required",
        Kind::DependencyImpact => "not-required",
        Kind::CounterContract => "executor-contacts=1;state=ready",
    }
}

fn provider_fact(kind: Kind, facts: &WorthUiInstalledOperationCertificationFacts) -> String {
    match kind {
        Kind::Workflow => format!(
            "stages={};effects={}",
            facts.workflow_stage_receipts(),
            facts.workflow_effect_receipts()
        ),
        Kind::Replay => match facts.replay() {
            WorthQueryOperationReplayContract::ReExecutable => "re-executable".to_owned(),
            _ => "unexpected-replay-contract".to_owned(),
        },
        Kind::ConditionalNode => facts.conditional_node_count().to_string(),
        Kind::SemanticAspectCorrespondence => facts.semantic_read_count().to_string(),
        Kind::Reversal => match facts.reversal() {
            WorthQueryOperationReversalContract::Irreversible => "irreversible".to_owned(),
            _ => "unexpected-reversal-contract".to_owned(),
        },
        Kind::Lineage => match facts.lineage() {
            WorthQueryOperationLineageContract::NotRequired => "not-required".to_owned(),
            _ => "unexpected-lineage-contract".to_owned(),
        },
        Kind::DependencyImpact => match facts.dependency_impact() {
            WorthQuerySupportRequirement::NotRequired => "not-required".to_owned(),
            WorthQuerySupportRequirement::Required => "required".to_owned(),
        },
        Kind::CounterContract => format!(
            "executor-contacts={};state={}",
            facts.executor_contacts(),
            match facts.result_state() {
                WorthQueryOperationResultState::Ready => "ready",
                WorthQueryOperationResultState::Advisory => "advisory",
                WorthQueryOperationResultState::Pending => "pending",
                WorthQueryOperationResultState::Partial => "partial",
                WorthQueryOperationResultState::Violation => "violation",
            }
        ),
    }
}

fn expected_provider_counters() -> WorthQueryCertificationCounters {
    WorthQueryCertificationCounters::exact([(WorthQueryCertificationCounter::ProviderContacts, 1)])
        .expect("one provider contact is a unique counter")
}
