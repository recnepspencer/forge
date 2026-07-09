use crate::public_doc_coverage::WorthQueryPublicDocCoverageAudit;

const DOWNSTREAM_RUNTIME_INTEGRATION_DOC: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/foundations/downstream-runtime-integration.md"
));
const SUPPORT_MATRIX_DOC: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/foundations/support-matrix-and-admission.md"
));
const PUBLIC_DOC_COVERAGE_DOC: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/domain-capabilities/public-doc-coverage.md"
));

#[test]
fn runtime_backed_closure_docs_keep_downstream_support_and_coverage_story_aligned() {
    let audit = WorthQueryPublicDocCoverageAudit::current();

    assert!(audit.undocumented_public_surfaces().is_empty());
    assert!(audit.surfaces_missing_goldens().is_empty());
    assert!(audit.readme_discovery_gaps().is_empty());
    assert!(audit.journey_coverage_gaps().is_empty());

    assert!(DOWNSTREAM_RUNTIME_INTEGRATION_DOC
        .contains("workspace.public_downstream_delivery_contract()"));
    assert!(DOWNSTREAM_RUNTIME_INTEGRATION_DOC.contains("workspace.downstream_delivery(...)"));
    assert!(DOWNSTREAM_RUNTIME_INTEGRATION_DOC
        .contains("durable replay/restart resume is still deferred debt"));
    assert!(SUPPORT_MATRIX_DOC.contains("downstream-delivery-contract"));
    assert!(SUPPORT_MATRIX_DOC.contains("delivery/resume contract"));
    assert!(PUBLIC_DOC_COVERAGE_DOC.contains("public teaching ledger"));
}
