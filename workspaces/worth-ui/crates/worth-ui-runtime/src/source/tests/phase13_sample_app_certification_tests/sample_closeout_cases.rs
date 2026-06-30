use super::sample_app_sources::sample_file_source_package;
use super::sample_certification_pipeline::{
    certify_file_source_package, source_module_id, WorthUiSampleAuthoringEvidence,
    WorthUiSampleCertificate,
};
use super::sample_snapshot_support::{SAMPLE_SNAPSHOT_FAMILY_COUNT, SAMPLE_SNAPSHOT_TOTAL_WIDTH};
use crate::source::WorthUiArtifactEquivalenceBasis;

#[test]
fn multi_file_source_package_lowers_to_one_canonical_artifact() {
    let certificate = certify_file_source_package(sample_file_source_package())
        .expect("sample app should certify through the full pipeline");

    assert_eq!(certificate.artifact().module_ids().len(), 3);
    assert_eq!(certificate.handles().len(), 9);
    assert_eq!(certificate.inspection().handles().len(), 9);
    assert_file_authoring_evidence_is_carried(&certificate);
    assert_snapshot_authority_evidence_is_carried(&certificate);
    assert_eq!(
        certificate.semantic_digest().basis(),
        WorthUiArtifactEquivalenceBasis::semantic()
    );
    assert_ne!(certificate.semantic_digest().raw(), 0);
    assert_dependency_metadata_is_narrow_and_explained(&certificate);
    assert_provenance_explains_every_canonical_node(&certificate);
    assert_query_binding_is_inspectable(&certificate);
}

fn assert_file_authoring_evidence_is_carried(certificate: &WorthUiSampleCertificate) {
    match certificate.authoring_evidence() {
        WorthUiSampleAuthoringEvidence::FileSourcePackage {
            package_digest,
            module_count,
        } => {
            assert_eq!(module_count, 3);
            assert_ne!(package_digest.raw(), 0);
        }
        WorthUiSampleAuthoringEvidence::RustComposition { .. } => {
            panic!("file certification should carry source-package evidence")
        }
    }
}

fn assert_snapshot_authority_evidence_is_carried(certificate: &WorthUiSampleCertificate) {
    assert_ne!(certificate.snapshot_digest().as_u64(), 0);
    assert_eq!(
        certificate.snapshot_metrics().registered_family_count(),
        SAMPLE_SNAPSHOT_FAMILY_COUNT
    );
    assert_eq!(
        certificate.snapshot_metrics().total_width(),
        SAMPLE_SNAPSHOT_TOTAL_WIDTH
    );
}

fn assert_dependency_metadata_is_narrow_and_explained(certificate: &WorthUiSampleCertificate) {
    let graph = certificate.dependency_basis().dependency_graph();
    let metrics = certificate.dependency_metrics();
    let handle_count = certificate.handles().len();

    assert_eq!(metrics.nodes_indexed(), handle_count);
    assert_eq!(metrics.subtree_digests_recorded(), handle_count);
    assert!(metrics.dependency_edges_recorded() >= 5);
    assert!(metrics.runtime_hooks_recorded() >= 1);

    let main = source_module_id("app/main.wui");
    let inspector = source_module_id("app/panels/inspector.wui");
    let theme = source_module_id("app/theme/tokens.wui");
    assert_eq!(
        graph.module_dependencies().get(&main),
        Some(&vec![inspector.clone(), theme.clone()])
    );

    let theme_impact = certificate
        .dependency_basis()
        .impact_metadata()
        .impact_for_module(&theme);
    assert!(theme_impact.requires_less_than_full_artifact_scan());
    assert_eq!(theme_impact.lookup_count(), 1);
    assert!(theme_impact.impacted_handles().len() < theme_impact.full_artifact_handle_count());
}

fn assert_provenance_explains_every_canonical_node(certificate: &WorthUiSampleCertificate) {
    for handle in certificate.handles() {
        assert!(
            certificate
                .inspection()
                .provenance_map()
                .source_origin(&handle)
                .is_some(),
            "missing provenance for {handle:?}"
        );
    }
}

fn assert_query_binding_is_inspectable(certificate: &WorthUiSampleCertificate) {
    assert!(certificate.handles().iter().any(|handle| {
        certificate
            .inspection()
            .node(handle)
            .is_some_and(|node| !node.query_inspection_links().is_empty())
    }));
}
