use std::sync::Arc;

use worth_ui::facade::graph::{
    UiRepeatedInstanceBasisKind, UiRuntimeDataInstanceKeyToken, UiRuntimeInstanceBasisAdmission,
};
use worth_ui::facade::source::WorthUiFilesystemSourceProvider;
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;
use worth_ui_test_support::WorthUiApplicationGraphCertificationExt;

use crate::filesystem_contract_workspace::FilesystemContractWorkspace;
use crate::milestone_312_identity_lifecycle::{
    assert_closed_transition_model, assert_real_scope_decisions, assert_unaffected_control,
    real_dual_generation_lifecycle,
};

#[test]
fn repeated_instance_order_and_nested_containment_match_the_independent_lattice() {
    assert_closed_transition_model();

    let scenario = FilesystemApplicationLifecycleScenario::new("phase-312-tt04");
    let workspace = FilesystemContractWorkspace::new("phase-312-tt04");
    workspace.write(
        "app/main.wui",
        &FilesystemApplicationLifecycleScenario::current_source_text(),
    );
    let provider = WorthUiFilesystemSourceProvider::new(workspace.root());
    let capabilities = scenario.capability_application();
    let reference =
        scenario.prepare_application(FilesystemApplicationLifecycleScenario::lower_snapshot(
            provider.read().expect("reference source reads"),
            capabilities.capabilities(),
        ));
    let declaration = repeated_declaration(&reference);
    let forward = repeated_instance_rows(
        &scenario,
        &provider,
        capabilities.capabilities(),
        &declaration,
        ["row:alpha", "row:beta"],
    );
    let reversed = repeated_instance_rows(
        &scenario,
        &provider,
        capabilities.capabilities(),
        &declaration,
        ["row:beta", "row:alpha"],
    );
    assert_eq!(
        forward.world_identity, reversed.world_identity,
        "the admission-order comparison must remain in one graph world"
    );
    assert_eq!(
        forward.identities, reversed.identities,
        "runtime-data identity cannot depend on admission position"
    );
    assert_eq!(forward.identities.len(), 2);
    assert_ne!(forward.identities[0].0, forward.identities[1].0);
    assert_ne!(forward.identities[0].1, forward.identities[1].1);

    let lifecycle = real_dual_generation_lifecycle();
    assert_real_scope_decisions(&lifecycle);
    assert_unaffected_control(&lifecycle);
    drop(reference);
    drop(capabilities);
    workspace.close();
}

struct RepeatedInstanceWorld {
    world_identity: u64,
    identities: Vec<(u64, u64)>,
}

fn repeated_declaration(
    reference: &worth_ui::facade::app::WorthUiApp,
) -> worth_ui::facade::declaration::UiDeclarationIdentity {
    let component_name =
        FilesystemApplicationLifecycleScenario::current_component_declaration_identity();
    reference
        .declaration_artifacts()
        .iter()
        .find(|artifact| artifact.identity().authored_semantic_name() == component_name)
        .expect("source component declaration exists")
        .identity()
        .clone()
}

fn repeated_instance_rows(
    scenario: &FilesystemApplicationLifecycleScenario,
    provider: &WorthUiFilesystemSourceProvider,
    capabilities: &worth_ui::facade::diagnostics::CapabilitySnapshot,
    declaration: &worth_ui::facade::declaration::UiDeclarationIdentity,
    keys: [&'static str; 2],
) -> RepeatedInstanceWorld {
    let admissions = keys.map(|key| {
        UiRuntimeInstanceBasisAdmission::admit_runtime_data_keyed(
            declaration,
            UiRuntimeDataInstanceKeyToken::new(Arc::<str>::from(key)),
        )
        .expect("stable non-positional runtime key admits")
    });
    let repeated_submission = FilesystemApplicationLifecycleScenario::lower_snapshot(
        provider.read().expect("repeated-instance source reads"),
        capabilities,
    );
    let app =
        scenario.prepare_application_with_runtime_instance_bases(repeated_submission, admissions);
    let identity_rows = app.repeated_instance_identity_rows(declaration);
    assert_eq!(identity_rows.len(), 2);
    let identities = identity_rows
        .iter()
        .map(|row| {
            assert_eq!(
                row.repeated_instance_basis_kind(),
                UiRepeatedInstanceBasisKind::RuntimeDataKeyed
            );
            (
                row.graph_node_identity_digest(),
                row.repeated_instance_basis_digest(),
            )
        })
        .collect::<Vec<_>>();
    RepeatedInstanceWorld {
        world_identity: app.graph_world_identity_digest(),
        identities,
    }
}
