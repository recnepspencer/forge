use std::{collections::BTreeMap, path::Path};

use crate::facade::WorthUi;
use crate::runtime::candidate::{
    file_authored_replacement_candidate, rust_authored_replacement_candidate,
};
use crate::runtime::{
    WorthUiCandidateArtifactBundle, WorthUiCandidateDependencyMetadata,
    WorthUiCandidateLoweringBasis, WorthUiQuerySupportReceipt, WorthUiQuerySupportStatus,
    WorthUiReplacementCandidate, WorthUiReplacementCandidateDenial, WorthUiReplacementCause,
};
use crate::source::{
    WorthUiArtifact, WorthUiArtifactDigestor, WorthUiArtifactEquivalenceBasis,
    WorthUiArtifactHandle, WorthUiArtifactIdentitySeed, WorthUiArtifactImportHandle,
    WorthUiArtifactImportNode, WorthUiArtifactInputReference, WorthUiArtifactModule,
    WorthUiArtifactNode, WorthUiBindingSemanticsLowerer, WorthUiCanonicalArtifactAssembler,
    WorthUiDurableStateEligibility, WorthUiDurableStateIneligibilityReason,
    WorthUiIdentitySeedLowerer, WorthUiParsedSourceToArtifactInputLowerer,
    WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule,
    WorthUiRustAuthoredToArtifactInputLowerer, WorthUiSourceModuleId, WorthUiSourcePackageLoader,
    WorthUiSourceParser, WorthUiStructuralLegalityLowerer,
};

#[test]
fn equivalent_file_and_rust_candidates_with_same_artifact_share_candidate_basis() {
    let file_artifact = file_authored_import_artifact("app/panels/inspector.wui");
    let rust_artifact = rust_authored_import_artifact("app/panels/inspector.wui");
    let file_candidate = file_authored_replacement_candidate(
        file_artifact,
        default_snapshot_digest(),
        WorthUiReplacementCause::file_source_change(module_id("app/main.wui"), 10),
    )
    .expect("file-authored candidate seals");
    let rust_candidate = rust_authored_replacement_candidate(
        rust_artifact,
        default_snapshot_digest(),
        WorthUiReplacementCause::rust_authored_input_change(99),
    )
    .expect("rust-authored candidate seals");

    assert_eq!(file_candidate.basis(), rust_candidate.basis());
    assert_ne!(
        file_candidate.provenance_handle(),
        rust_candidate.provenance_handle()
    );
    assert_ne!(
        file_candidate.authoring_lane(),
        rust_candidate.authoring_lane()
    );
    assert_eq!(file_candidate.cause().kind_name(), "file-source-changed");
    assert_eq!(
        rust_candidate.cause().kind_name(),
        "rust-authored-input-changed"
    );
}

#[test]
fn candidate_without_artifact_digest_or_dependency_metadata_rejected() {
    let artifact = import_artifact(["app/panels/inspector.wui"]);
    let artifact_digest =
        WorthUiArtifactDigestor::digest(&artifact, WorthUiArtifactEquivalenceBasis::semantic());

    assert_eq!(
        WorthUiCandidateArtifactBundle::from_optional_parts_for_test(
            artifact.clone(),
            None,
            Some(WorthUiCandidateDependencyMetadata::derive_for_artifact(
                &artifact
            )),
            Some(test_lowering_basis(1))
        ),
        Err(WorthUiReplacementCandidateDenial::MissingArtifactDigest)
    );
    assert_eq!(
        WorthUiCandidateArtifactBundle::from_optional_parts_for_test(
            artifact,
            Some(artifact_digest),
            None,
            Some(test_lowering_basis(1))
        ),
        Err(WorthUiReplacementCandidateDenial::MissingDependencyMetadata)
    );
}

#[test]
fn candidate_cause_does_not_change_artifact_equivalence() {
    let artifact = import_artifact(["app/panels/inspector.wui"]);
    let source_candidate = candidate_from_bundle(
        artifact.clone(),
        WorthUiReplacementCause::file_source_change(module_id("app/main.wui"), 1),
    );
    let refresh_candidate =
        candidate_from_bundle(artifact, WorthUiReplacementCause::manual_refresh(2));

    assert_eq!(source_candidate.basis(), refresh_candidate.basis());
    assert_ne!(
        source_candidate.provenance_handle(),
        refresh_candidate.provenance_handle()
    );
    assert_ne!(
        source_candidate.cause().kind_name(),
        refresh_candidate.cause().kind_name()
    );
}

#[test]
fn candidate_with_stale_dependency_metadata_rejected_even_when_digest_matches() {
    let stale_source = import_artifact(["app/panels/inspector.wui"]);
    let candidate_artifact = import_artifact(["app/panels/settings.wui"]);
    let candidate_digest = WorthUiArtifactDigestor::digest(
        &candidate_artifact,
        WorthUiArtifactEquivalenceBasis::semantic(),
    );
    let stale_metadata = WorthUiCandidateDependencyMetadata::derive_for_artifact(&stale_source)
        .with_artifact_digest_for_test(candidate_digest);

    assert_eq!(
        WorthUiCandidateArtifactBundle::seal(
            candidate_artifact,
            stale_metadata,
            default_snapshot_digest()
        ),
        Err(WorthUiReplacementCandidateDenial::StaleDependencyMetadata)
    );
}

#[test]
fn dependency_metadata_digest_participates_in_candidate_basis() {
    let imported = import_artifact(["app/panels/inspector.wui"]);
    let multi_import = import_artifact(["app/panels/inspector.wui", "app/panels/settings.wui"]);
    let imported_candidate = candidate_from_bundle(
        imported,
        WorthUiReplacementCause::rust_authored_input_change(1),
    );
    let multi_import_candidate = candidate_from_bundle(
        multi_import,
        WorthUiReplacementCause::rust_authored_input_change(1),
    );

    assert_ne!(imported_candidate.basis(), multi_import_candidate.basis());
    assert_eq!(
        imported_candidate
            .artifact_bundle()
            .artifact()
            .module_ids()
            .len(),
        1
    );
    assert_eq!(
        imported_candidate.artifact_bundle().artifact_digest(),
        imported_candidate.basis().artifact_digest()
    );
    assert_eq!(
        imported_candidate
            .artifact_bundle()
            .artifact_digest_report()
            .basis(),
        imported_candidate.basis().artifact_equivalence_basis()
    );
    assert_eq!(
        imported_candidate
            .artifact_bundle()
            .dependency_metadata()
            .dependency_report()
            .basis(),
        imported_candidate
            .artifact_bundle()
            .dependency_metadata()
            .invalidation_basis()
    );
    assert_ne!(
        imported_candidate
            .artifact_bundle()
            .dependency_metadata()
            .dependency_metadata_digest(),
        multi_import_candidate
            .artifact_bundle()
            .dependency_metadata()
            .dependency_metadata_digest()
    );
    assert_ne!(
        imported_candidate.basis().dependency_metadata_digest(),
        multi_import_candidate.basis().dependency_metadata_digest()
    );
}

fn candidate_from_bundle(
    artifact: WorthUiArtifact,
    cause: WorthUiReplacementCause,
) -> WorthUiReplacementCandidate {
    rust_authored_replacement_candidate(artifact, default_snapshot_digest(), cause)
        .expect("candidate seals")
}

fn default_snapshot_digest() -> crate::capability::CapabilitySnapshotDigest {
    WorthUi::app().freeze().capabilities().digest()
}

fn test_lowering_basis(snapshot_digest: u64) -> WorthUiCandidateLoweringBasis {
    WorthUiCandidateLoweringBasis::from_raw_parts_for_test(
        snapshot_digest,
        WorthUiQuerySupportReceipt::for_test(
            WorthUiQuerySupportStatus::Supported,
            "replacement-candidate",
        ),
    )
}

fn import_artifact<const N: usize>(targets: [&str; N]) -> WorthUiArtifact {
    let module_id = module_id("app/main.wui");
    let nodes = targets
        .into_iter()
        .enumerate()
        .map(|(node_index, target)| import_node(&module_id, node_index, target))
        .collect::<Vec<_>>();
    let module = WorthUiArtifactModule::new(module_id.clone(), nodes);
    let mut modules = BTreeMap::new();
    modules.insert(module_id.clone(), module);

    WorthUiArtifact::new(modules, vec![module_id])
}

fn import_node(
    module_id: &WorthUiSourceModuleId,
    node_index: usize,
    target: &str,
) -> WorthUiArtifactNode {
    WorthUiArtifactNode::Import(WorthUiArtifactImportNode::new(
        WorthUiArtifactHandle::Import(WorthUiArtifactImportHandle::new(
            module_id.clone(),
            node_index,
        )),
        WorthUiArtifactInputReference::new(target),
        0,
        WorthUiArtifactIdentitySeed::structural_fallback(format!(
            "module:{}|import:{}",
            module_id.as_str(),
            target
        )),
        WorthUiDurableStateEligibility::Ineligible {
            reason: WorthUiDurableStateIneligibilityReason::NoDurableStateSurface,
        },
    ))
}

fn module_id(path: &str) -> WorthUiSourceModuleId {
    WorthUiSourceModuleId::from_relative_path(Path::new(path)).expect("valid module id")
}

fn file_authored_import_artifact(target_module_path: &str) -> WorthUiArtifact {
    let source_package = WorthUiSourcePackageLoader::from_workspace_root(r"C:\workspace")
        .register_module_with_source("app/main.wui", format!(r#"import "{target_module_path}";"#))
        .register_module_with_source(target_module_path, "")
        .compile()
        .expect("file-authored package compiles");
    let parsed_source_package =
        WorthUiSourceParser::parse_package(&source_package).expect("source package parses");
    let artifact_input = WorthUiParsedSourceToArtifactInputLowerer::lower(&parsed_source_package);
    canonical_artifact_from_input(artifact_input)
}

fn rust_authored_import_artifact(target_module_path: &str) -> WorthUiArtifact {
    let artifact_input = WorthUiRustAuthoredToArtifactInputLowerer::lower(
        &WorthUiRustAuthoredArtifactInput::from_modules([
            WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
                .with_import(target_module_path),
            WorthUiRustAuthoredArtifactInputModule::new(target_module_path),
        ]),
    );
    canonical_artifact_from_input(artifact_input)
}

fn canonical_artifact_from_input(
    artifact_input: crate::source::WorthUiArtifactInput,
) -> WorthUiArtifact {
    let app = WorthUi::app().freeze();
    let snapshot = app.capabilities();
    let resolved = crate::source::WorthUiArtifactInputResolver::resolve(&artifact_input, snapshot)
        .expect("artifact input resolves");
    let structured =
        WorthUiStructuralLegalityLowerer::lower(&resolved, snapshot).expect("structure lowers");
    let bound = WorthUiBindingSemanticsLowerer::lower(&structured, snapshot)
        .expect("binding semantics lower");
    let identity_seeded = WorthUiIdentitySeedLowerer::lower(&bound)
        .expect("identity seeds lower")
        .0;
    WorthUiCanonicalArtifactAssembler::assemble(&identity_seeded)
        .expect("canonical artifact assembles")
}
