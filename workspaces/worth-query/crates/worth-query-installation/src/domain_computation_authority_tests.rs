use crate::domain_computation_artifact_fixture::*;
use crate::facade::*;

fn build_index(runtime: WorthQueryInstallationRuntimeIdentity) -> WorthQueryInstalledPackageIndex {
    WorthQueryInstalledPackageIndex::build(
        runtime,
        WorthQueryInstallationGeneration::initial(),
        [admitted(valid_contract(
            false,
            WorthQueryArtifactLifecycleContract::Retained,
            domain_reproducibility(),
        ))],
    )
    .unwrap()
}

#[test]
fn installed_contract_authority_is_runtime_affine_and_rebuild_stable() {
    let runtime = WorthQueryInstallationRuntimeIdentity::fresh();
    let index = build_index(runtime.retained());
    let authority = index
        .artifact_contract(
            "worth.routing",
            CandidateArtifactFamily::SEMANTIC_FAMILY,
            WorthQueryArtifactSchemaVersion::new(2),
            WorthQueryArtifactProtocolVersion::new(1),
        )
        .unwrap();
    index.validate_artifact_contract(&authority).unwrap();
    assert_eq!(index.installed_artifact_contract_count(), 1);
    assert_eq!(index.counters().artifact_contract_rows_examined, 1);

    let rebuilt = index.rebuild();
    let rebuilt_authority = rebuilt
        .artifact_contract(
            "worth.routing",
            CandidateArtifactFamily::SEMANTIC_FAMILY,
            WorthQueryArtifactSchemaVersion::new(2),
            WorthQueryArtifactProtocolVersion::new(1),
        )
        .unwrap();
    assert_eq!(authority, rebuilt_authority);
    assert_eq!(index.identity(), rebuilt.identity());

    let foreign = build_index(WorthQueryInstallationRuntimeIdentity::fresh());
    assert_eq!(
        foreign
            .validate_artifact_contract(&authority)
            .unwrap_err()
            .kind(),
        WorthQueryInstalledPackageIndexDenialKind::ForeignRuntime
    );
    assert_eq!(
        index
            .successor_generation()
            .validate_artifact_contract(&authority)
            .unwrap_err()
            .kind(),
        WorthQueryInstalledPackageIndexDenialKind::StaleGeneration
    );
}

#[test]
fn copied_semantic_identity_only_resolves_through_the_installed_index() {
    let index = build_index(WorthQueryInstallationRuntimeIdentity::fresh());
    let declaration = valid_contract(
        false,
        WorthQueryArtifactLifecycleContract::Retained,
        domain_reproducibility(),
    );
    let copied_identity = declaration.identity().as_str().to_string();
    let authority = index
        .artifact_contract(
            "worth.routing",
            declaration.family().as_str(),
            declaration.schema_version(),
            declaration.protocol_version(),
        )
        .unwrap();

    assert_eq!(authority.contract().identity().as_str(), copied_identity);
    assert_eq!(
        authority.contract().content_identity(),
        declaration.content_identity()
    );
    assert_eq!(authority.contract().carriage(), declaration.carriage());
    assert_eq!(authority.contract().counters(), declaration.counters());
    index.validate_artifact_contract(&authority).unwrap();
}
