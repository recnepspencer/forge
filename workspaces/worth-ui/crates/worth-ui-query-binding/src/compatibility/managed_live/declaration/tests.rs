use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};
use worth_query::facade::read::WorthQueryProjectionOutcome;
use worth_query::facade::{domain, runtime};
use worth_relational::facade::runtime::InvariantCatalog;

use super::WorthUiManagedLiveDeclarationExt;
use crate::{worth_ui_domain_package, WorthUiDomainEntry};

#[test]
fn worth_ui_executes_installed_contribution_and_invariant_journeys() {
    let workspace = measurement_workspace("worth-ui-installed-workflow");
    let handle = workspace
        .domain(WorthUiDomainEntry)
        .expect("Worth UI domain should be installed");
    let installation = workspace
        .domain_installation_receipt(WorthUiDomainEntry)
        .expect("Worth UI installation receipt should be retained");
    assert_eq!(installation.definition_counts().graph_read_operations(), 1);
    assert_eq!(installation.definition_counts().graph_obligations(), 1);
    assert_eq!(
        installation
            .construction_counters()
            .graph_obligation_index_entries(),
        1
    );

    let intent = runtime::WorthQueryIntentDeclaration::strategy_commit(
        "record-ui-measurement",
        "worth-ui",
        "1.0",
        "worth-ui.measurement.v1",
        runtime::WorthQueryIntentInput::null(),
    );
    let invariant = handle
        .contributions_in(&workspace)
        .expect("contribution authority should be runtime-affine")
        .for_intent(&intent)
        .expect("the intent target should retain installed authority")
        .register_invariant_catalog(
            "measurement.allocation.integrity",
            InvariantCatalog::default(),
        )
        .because("allocation measurements must retain the installed UI invariant catalog")
        .materialize()
        .expect("Worth UI invariant contribution should materialize");
    assert!(invariant.semantic_code().starts_with("WORTH.ui.runtime."));
    assert!(!invariant.materialization_digest().is_empty());
}

#[test]
fn worth_ui_installed_live_handle_owns_activation_and_disposal() {
    let mut workspace = measurement_workspace("worth-ui-installed-live");
    let handle = workspace
        .domain(WorthUiDomainEntry)
        .expect("Worth UI domain should be installed");
    let live = match handle
        .live_measurements("worth-ui.measurements.test")
        .expect("Worth UI installed live declaration should admit")
        .using(domain::current())
        .open(&mut workspace)
        .expect("the installed handle should match its workspace")
    {
        domain::WorthQueryInstalledDomainLiveOpenOutcome::Opened(handle) => handle,
        domain::WorthQueryInstalledDomainLiveOpenOutcome::Stopped(stop) => {
            panic!("Worth UI installed live open stopped: {:?}", stop.stop())
        }
    };

    let read = match live.read(&mut workspace) {
        Ok(read) => read,
        Err(_) => panic!("Worth UI installed live read should succeed"),
    };
    let projection = live.project(&read, domain::project_facts().entity_identities());
    assert!(matches!(
        projection.outcome(),
        WorthQueryProjectionOutcome::Completed(_)
    ));
    assert_eq!(
        projection
            .receipt()
            .installed_authority()
            .package_identity(),
        handle.package_identity()
    );

    let observation = match live.observe(&mut workspace) {
        Ok(observation) => observation,
        Err(_) => panic!("Worth UI live observation should succeed"),
    };
    assert_eq!(observation.activation_work().declaration_count(), 1);
    assert!(matches!(
        live.close(&mut workspace),
        domain::WorthQueryInstalledDomainLiveCloseOutcome::Closed(_)
    ));
}

fn measurement_workspace(name: &str) -> worth_query::facade::runtime::WorthQueryWorkspace {
    let schema = WorthQueryTestBackendSchema::single_collection("WorthUiMeasurement")
        .aspect_contracts(crate::worth_ui_native_aspect_contracts())
        .expect("Worth UI native aspect contracts should build")
        .aspect("identity.id", "identity.id")
        .expect("identity aspect should build")
        .aspect("measurement.value", "measurement.value")
        .expect("measurement aspect should build");
    crate::install_worth_ui_test_operation_executors(
        in_memory_test_runtime()
            .with_schema(schema)
            .domain_package(worth_ui_domain_package()),
    )
    .workspace(name)
    .expect("Worth UI installed-domain workspace should build")
}
