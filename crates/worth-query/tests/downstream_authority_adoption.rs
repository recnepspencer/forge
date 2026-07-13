use std::path::PathBuf;

use worth_query::facade::consumer_kit::{
    downstream_authority_adoption, WorthQueryConsumerResidueClass,
};

#[test]
fn worth_ui_sources_certify_sealed_query_authority_adoption() {
    let root = repository_root();
    let proof = downstream_authority_adoption("worth-ui")
        .required_root(root.join("workspaces/worth-ui/crates/worth-ui-query-binding/src"))
        .required_root(root.join("workspaces/worth-ui/crates/worth-ui-runtime/src"))
        .evaluate()
        .expect("Worth UI source inventory must be auditable");

    proof.assert_adopted();
    assert!(proof.manifest().adopted());
    assert_eq!(proof.manifest().finding_count(), 0);
    assert_eq!(proof.manifest().prohibited_class_count(), 6);
    assert!(proof.manifest().audited_source_count() > 100);
    assert!(!proof.manifest().source_inventory_digest().is_empty());
    assert_eq!(
        proof.manifest().report_identity(),
        proof.residue_report().report_identity()
    );
}

#[test]
fn adoption_report_localizes_every_authority_reconstruction_family() {
    let fixture = temporary_fixture(
        r#"
fn old_attempt(value: ProjectionFactConsumptionAttempt) { let _ = value; }
struct WorthUiQueryMeasurementConsumptionIdentity;
fn bind_projection_contract() {}
fn digest_pair(contract: Contract) { let _ = contract.basis_digest() != Some("basis"); }
fn with_query_prerequisites_from_projection_consumption() {}
use worth_query::projection_consumption::ProjectionConsumptionReceipt;
"#,
    );
    let proof = downstream_authority_adoption("hostile-consumer")
        .required_root(&fixture)
        .evaluate()
        .expect("hostile adoption fixture must parse");
    let classes = proof
        .residue_report()
        .findings()
        .iter()
        .map(|finding| finding.residue_class())
        .collect::<std::collections::BTreeSet<_>>();

    assert!(!proof.manifest().adopted());
    assert_eq!(classes.len(), 6);
    for class in [
        WorthQueryConsumerResidueClass::DecomposedProjectionConsumptionAttempt,
        WorthQueryConsumerResidueClass::LocalQueryMeasurementConsumptionIdentity,
        WorthQueryConsumerResidueClass::LocalProjectionContractBinding,
        WorthQueryConsumerResidueClass::LocalQueryBasisDigestCompatibility,
        WorthQueryConsumerResidueClass::LegacyProjectionPrerequisiteAssembly,
        WorthQueryConsumerResidueClass::DirectInternalQueryImport,
    ] {
        assert!(classes.contains(&class), "missing hostile class {class:?}");
    }
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("worth-query crate must live under repository crates/")
        .to_path_buf()
}

fn temporary_fixture(source: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "worth-query-downstream-authority-adoption-{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&root).expect("fixture root must be creatable");
    std::fs::write(root.join("lib.rs"), source).expect("fixture source must be writable");
    root
}
