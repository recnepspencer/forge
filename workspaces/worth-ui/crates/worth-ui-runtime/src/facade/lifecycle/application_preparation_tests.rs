use std::sync::Arc;

use crate::facade::WorthUiRustAuthoredDeclarationFixture;
use worth_ui_dsl::{
    UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey, UiDslSourceProvenance,
    UiDslStructuralToken,
};

use crate::declaration::UiDeclarationIdentity;
use crate::facade::WorthUi;
use crate::graph::{
    UiGraphInstantiationDenial, UiRepeatedInstanceBasisDenial, UiRuntimeDataInstanceKeyToken,
    UiRuntimeInstanceBasisAdmission,
};

use super::{WorthUiApplicationPreparationDenial, WorthUiApplicationPreparationPhase};

#[test]
fn foreign_runtime_basis_denies_in_graph_admission_before_authority_seal() {
    let foreign_identity = control_identity(
        "worth-ui.runtime.preparation.foreign-basis",
        "workspace.control.foreign",
        "app/preparation_foreign.wui",
    );
    let foreign_basis = runtime_basis(&foreign_identity, "row:foreign");

    let denial = freeze_denial(
        control_package(
            "worth-ui.runtime.preparation.target",
            "workspace.control.target",
            "app/preparation_target.wui",
        ),
        [foreign_basis],
    );

    assert_eq!(
        denial.phase(),
        WorthUiApplicationPreparationPhase::GraphAdmission
    );
    assert_eq!(
        denial,
        WorthUiApplicationPreparationDenial::GraphAdmission(
            UiGraphInstantiationDenial::RuntimeBasisTargetsUnknownDeclaration {
                declaration_identity: foreign_identity,
            },
        )
    );
}

#[test]
fn contradictory_runtime_basis_replay_returns_equivalent_graph_admission_denial() {
    let package_name = "worth-ui.runtime.preparation.contradictory-basis";
    let semantic_key = "workspace.control.repeated";
    let module_path = "app/preparation_repeated.wui";
    let declaration_identity = control_identity(package_name, semantic_key, module_path);
    let duplicate_basis = runtime_basis(&declaration_identity, "row:duplicate");
    let admissions = [duplicate_basis.clone(), duplicate_basis];

    let first = freeze_denial(
        control_package(package_name, semantic_key, module_path),
        admissions.clone(),
    );
    let second = freeze_denial(
        control_package(package_name, semantic_key, module_path),
        admissions,
    );

    assert_eq!(first, second);
    assert_eq!(
        first,
        WorthUiApplicationPreparationDenial::GraphAdmission(
            UiGraphInstantiationDenial::RuntimeBasisDenied {
                declaration_identity,
                denial: UiRepeatedInstanceBasisDenial::ContradictoryBasis,
            },
        )
    );
}

fn control_identity(
    package_name: &'static str,
    semantic_key: &'static str,
    module_path: &'static str,
) -> UiDeclarationIdentity {
    WorthUi::app()
        .bind_certification_host()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(control_package(
            package_name,
            semantic_key,
            module_path,
        ))
        .freeze()
        .expect("reference declaration should prepare")
        .declaration_artifacts()
        .iter()
        .find(|artifact| artifact.provenance().source_provenance().module_path() == module_path)
        .expect("reference control declaration should exist")
        .identity()
        .clone()
}

fn freeze_denial<const N: usize>(
    package: WorthUiRustAuthoredDeclarationFixture,
    admissions: [UiRuntimeInstanceBasisAdmission; N],
) -> WorthUiApplicationPreparationDenial {
    match WorthUi::app()
        .bind_certification_host()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(package)
        .with_runtime_instance_basis_admissions(admissions)
        .freeze()
    {
        Ok(_) => panic!("invalid runtime basis must deny application preparation"),
        Err(denial) => denial,
    }
}

fn runtime_basis(
    declaration_identity: &UiDeclarationIdentity,
    key: &'static str,
) -> UiRuntimeInstanceBasisAdmission {
    UiRuntimeInstanceBasisAdmission::admit_runtime_data_keyed(
        declaration_identity,
        UiRuntimeDataInstanceKeyToken::new(Arc::<str>::from(key)),
    )
    .expect("stable runtime data key should admit")
}

fn control_package(
    package_name: &'static str,
    semantic_key: &'static str,
    module_path: &'static str,
) -> WorthUiRustAuthoredDeclarationFixture {
    WorthUiRustAuthoredDeclarationFixture::named(package_name).with_semantic_artifact_spec(
        UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new(semantic_key),
            UiDslSemanticFamily::Control,
            UiDslSourceProvenance::file_authored(module_path, 0),
        )
        .with_structural_token(UiDslStructuralToken::new("control:prepared")),
    )
}
