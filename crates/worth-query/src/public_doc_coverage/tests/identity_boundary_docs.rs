use crate::public_doc_coverage::WorthQueryPublicDocCoverageAudit;

const AI_README: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/AI_README.md"));
const WORKSPACE_OVERVIEW: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/foundations/workspace-overview.md"
));
const BRANCHES_AND_PREVIEWS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/foundations/branches-and-previews.md"
));
const DOWNSTREAM_RUNTIME_INTEGRATION: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/foundations/downstream-runtime-integration.md"
));
const INSPECTION_DOC: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/capabilities/inspection.md"
));
const ASPECTS_AND_AUTHORITY_LANES: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/modeling/aspects-and-authority-lanes.md"
));

#[test]
fn identity_boundary_docs_teach_canonical_identity_typed_stop_classes_and_typed_session_labels() {
    let audit = WorthQueryPublicDocCoverageAudit::current();

    assert!(audit.undocumented_public_surfaces().is_empty());
    assert!(audit.surfaces_missing_goldens().is_empty());
    assert!(audit.readme_discovery_gaps().is_empty());

    assert!(AI_README.contains("WorthQueryEvidenceIdentity::compose"));
    assert!(AI_README.contains("error.stop_class()"));
    assert!(AI_README.contains("messages are presentation"));
    assert!(AI_README.contains("WorthQuerySessionLabel"));
    assert!(WORKSPACE_OVERVIEW.contains("WorthQueryEvidenceIdentity::compose"));
    assert!(WORKSPACE_OVERVIEW.contains("error.stop_class()"));
    assert!(WORKSPACE_OVERVIEW.contains("messages are"));
    assert!(WORKSPACE_OVERVIEW.contains("WorthQuerySessionLabel"));
    assert!(BRANCHES_AND_PREVIEWS.contains("WorthQuerySessionLabel"));
    assert!(BRANCHES_AND_PREVIEWS.contains("typed session labels"));
    assert!(DOWNSTREAM_RUNTIME_INTEGRATION.contains("WorthQueryEvidenceIdentity::compose"));
    assert!(DOWNSTREAM_RUNTIME_INTEGRATION.contains("error.stop_class()"));
    assert!(DOWNSTREAM_RUNTIME_INTEGRATION.contains("messages are"));
    assert!(DOWNSTREAM_RUNTIME_INTEGRATION.contains("WorthQuerySessionLabel"));
    assert!(INSPECTION_DOC.contains("WorthQuerySessionLabel"));
    assert!(ASPECTS_AND_AUTHORITY_LANES.contains("WorthQuerySessionLabel"));
}
