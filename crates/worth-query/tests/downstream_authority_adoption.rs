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
    assert_eq!(proof.manifest().prohibited_class_count(), 7);
    assert!(proof.manifest().audited_source_count() > 100);
    assert!(!proof.manifest().source_inventory_digest().is_empty());
    assert_eq!(
        proof.manifest().report_identity(),
        proof.residue_report().report_identity()
    );
    let receipt = proof
        .deletion_receipt()
        .expect("zero-residue adoption must seal a deletion receipt");
    assert_eq!(receipt.rows().len(), 4);
    assert!(receipt.rows().iter().all(|row| row.finding_count() == 0));
    assert_eq!(
        receipt.source_inventory_digest(),
        proof.manifest().source_inventory_digest()
    );
    assert_eq!(
        receipt.report_identity(),
        proof.manifest().report_identity()
    );
}

#[test]
fn adoption_report_localizes_every_authority_reconstruction_family() {
    let fixture = temporary_fixture(
        r#"
fn old_attempt(value: ProjectionFactConsumptionAttempt) { let _ = value; }
fn completed_parts(value: CompletedProjectionFactConsumption) { let _ = value; }
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
    assert_eq!(classes.len(), 7);
    for class in [
        WorthQueryConsumerResidueClass::DecomposedProjectionConsumptionAttempt,
        WorthQueryConsumerResidueClass::IndependentlyPairableProjectionConsumptionParts,
        WorthQueryConsumerResidueClass::LocalQueryMeasurementConsumptionIdentity,
        WorthQueryConsumerResidueClass::LocalProjectionContractBinding,
        WorthQueryConsumerResidueClass::LocalQueryBasisDigestCompatibility,
        WorthQueryConsumerResidueClass::LegacyProjectionPrerequisiteAssembly,
        WorthQueryConsumerResidueClass::DirectInternalQueryImport,
    ] {
        assert!(classes.contains(&class), "missing hostile class {class:?}");
    }
    assert!(proof.deletion_receipt().is_none());
}

#[test]
fn deletion_receipt_rows_equal_the_closure_contract_obligations() {
    let fixture = temporary_fixture("fn facade_only_consumer() {}\n");
    let proof = downstream_authority_adoption("clean-consumer")
        .required_root(&fixture)
        .evaluate()
        .expect("clean fixture must parse");
    let receipt = proof.deletion_receipt().expect("clean audit seals receipt");
    let actual = receipt
        .rows()
        .iter()
        .map(|row| row.obligation())
        .collect::<Vec<_>>();
    let expected = worth_query::facade::foundation::downstream_authority_closure_contract()
        .deletion_obligations()
        .collect::<Vec<_>>();
    assert_eq!(actual, expected);
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("worth-query crate must live under repository crates/")
        .to_path_buf()
}

fn temporary_fixture(source: &str) -> PathBuf {
    static NEXT_FIXTURE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let fixture = NEXT_FIXTURE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "worth-query-downstream-authority-adoption-{}-{fixture}",
        std::process::id(),
    ));
    std::fs::create_dir_all(&root).expect("fixture root must be creatable");
    std::fs::write(root.join("lib.rs"), source).expect("fixture source must be writable");
    root
}
