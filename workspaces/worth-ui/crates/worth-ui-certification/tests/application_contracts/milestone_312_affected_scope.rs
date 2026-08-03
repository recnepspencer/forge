use std::collections::{BTreeMap, BTreeSet};

use worth_ui::facade::observation::UiChangeClassificationOutcome;
use worth_ui::facade::rebind::{
    UiAuthoredFactKind, UiAuthoredFactSelector, UiGraphFactConsumerKind, UiProducedFactFamily,
    UiResolvedAffectedScope,
};
use worth_ui::facade::source::WorthUiFilesystemSourceProvider;
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;

use super::filesystem_contract_workspace::FilesystemContractWorkspace;

#[test]
fn affected_scope_uses_both_generations_without_widening() {
    let label = "phase-312-dual-generation-scope";
    let scenario = FilesystemApplicationLifecycleScenario::new(label);
    let workspace = FilesystemContractWorkspace::new(label);
    workspace.write(
        "app/main.wui",
        &FilesystemApplicationLifecycleScenario::dual_generation_scope_initial_source_text(),
    );
    let provider = WorthUiFilesystemSourceProvider::new(workspace.root());
    let capabilities = scenario.capability_application();
    let initial = FilesystemApplicationLifecycleScenario::lower_snapshot(
        provider.read().expect("initial filesystem world reads"),
        capabilities.capabilities(),
    );
    let mut session = scenario
        .prepare_application(initial)
        .launch()
        .expect("initial filesystem world launches");
    let predecessor = session.generation_identity().clone();

    workspace.write(
        "app/main.wui",
        &FilesystemApplicationLifecycleScenario::dual_generation_scope_candidate_source_text(),
    );
    let candidate = FilesystemApplicationLifecycleScenario::lower_snapshot(
        provider.read().expect("candidate filesystem world reads"),
        session.capabilities(),
    );
    let mut turn = session.begin_observation_turn().unwrap();
    turn.admit_source(candidate)
        .expect("sealed candidate enters the authored owner");
    let admitted = turn.seal().unwrap();
    let changed = match session.classify_observations(admitted).unwrap() {
        UiChangeClassificationOutcome::Changed(changed) => changed,
        _ => panic!("create and retire must classify as semantic change"),
    };
    let scope = session
        .resolve_affected_scope(changed)
        .expect("both exact generation indexes resolve");

    assert_scope_truth(&scope, &predecessor);
    assert_eq!(
        session.generation_identity(),
        &predecessor,
        "scope resolution is pre-effect and cannot publish candidate truth"
    );
    let _ = session.shutdown();
    workspace.close();
}

fn assert_scope_truth(
    scope: &UiResolvedAffectedScope,
    predecessor: &worth_ui::facade::app::WorthUiPreparedApplicationGenerationIdentity,
) {
    let retired = FilesystemApplicationLifecycleScenario::current_component_declaration_identity();
    let created =
        FilesystemApplicationLifecycleScenario::candidate_component_declaration_identity();
    let preserved =
        FilesystemApplicationLifecycleScenario::imported_current_component_declaration_identity();
    assert_scope_basis_and_facts(scope, predecessor, &retired, &created);
    assert_consumer_scope(scope, &retired, &created, &preserved);
    assert_exact_cost(scope);
}

fn assert_scope_basis_and_facts(
    scope: &UiResolvedAffectedScope,
    predecessor: &worth_ui::facade::app::WorthUiPreparedApplicationGenerationIdentity,
    retired: &str,
    created: &str,
) {
    assert_eq!(
        scope.basis().classification().predecessor_generation(),
        predecessor
    );
    assert!(scope.basis().has_distinct_candidate_generation());
    assert_ne!(
        scope.basis().predecessor_graph(),
        scope.basis().candidate_graph()
    );
    assert!(scope
        .facts()
        .iter()
        .all(|fact| fact.family() == UiProducedFactFamily::AuthoredSource));
    let selectors = scope
        .facts()
        .iter()
        .map(|fact| {
            let authored = fact.authored_source().expect("authored family has payload");
            let UiAuthoredFactSelector::Node(identity) = authored.selector() else {
                panic!("node create/retire cannot become a module selector");
            };
            (identity.to_string(), authored.kind())
        })
        .collect::<BTreeMap<_, _>>();
    assert_eq!(
        selectors,
        BTreeMap::from([
            (created.to_owned(), UiAuthoredFactKind::Created),
            (retired.to_owned(), UiAuthoredFactKind::Retired),
        ])
    );
    for lookup in scope.lookups() {
        let authored = scope.facts()[lookup.fact_ordinal()]
            .authored_source()
            .unwrap();
        match authored.kind() {
            UiAuthoredFactKind::Created => {
                assert!(lookup.predecessor().entries().is_empty());
                assert_eq!(lookup.candidate().entries().len(), 2);
            }
            UiAuthoredFactKind::Retired => {
                assert_eq!(lookup.predecessor().entries().len(), 2);
                assert!(lookup.candidate().entries().is_empty());
            }
            other => panic!("unexpected authored fact in scoped world: {other:?}"),
        }
    }
}

fn assert_consumer_scope(
    scope: &UiResolvedAffectedScope,
    retired: &str,
    created: &str,
    preserved: &str,
) {
    let observed = scope
        .consumers()
        .iter()
        .map(|consumer| {
            (
                consumer.key().authored_identity().to_owned(),
                consumer.key().kind(),
                consumer.predecessor().is_some(),
                consumer.candidate().is_some(),
            )
        })
        .collect::<BTreeSet<_>>();
    let expected = BTreeSet::from([
        (
            retired.to_owned(),
            UiGraphFactConsumerKind::GraphNode,
            true,
            false,
        ),
        (
            retired.to_owned(),
            UiGraphFactConsumerKind::MountEligibilitySlot,
            true,
            false,
        ),
        (
            created.to_owned(),
            UiGraphFactConsumerKind::GraphNode,
            false,
            true,
        ),
        (
            created.to_owned(),
            UiGraphFactConsumerKind::MountEligibilitySlot,
            false,
            true,
        ),
    ]);
    assert_eq!(observed, expected);
    assert!(scope
        .consumers()
        .iter()
        .all(|consumer| consumer.key().authored_identity() != preserved));
    assert!(scope.affected_aspects().is_empty());
}

fn assert_exact_cost(scope: &UiResolvedAffectedScope) {
    let cost = scope.cost();
    assert_eq!(cost.observations(), 1);
    assert_eq!(cost.changed_facts(), 2);
    assert_eq!(cost.affected_aspects(), 0);
    assert_eq!(cost.indexed_consumers(), 4);
    assert_eq!(cost.lookup_receipts(), 4);
    assert_eq!(cost.index_probes(), 4);
    assert_eq!(cost.contract_checks(), 4);
    assert_eq!(cost.graph_and_mounted_entries(), 4);
}
