use std::path::PathBuf;

use worth_query::facade::consumer_kit::{
    downstream_authority_adoption, WorthQueryConsumerResidueClass,
};

#[test]
fn adoption_report_localizes_every_authority_reconstruction_family() {
    let fixture = temporary_fixture(
        r#"
fn old_attempt(value: ProjectionFactConsumptionAttempt) { let _ = value; }
fn completed_parts(value: CompletedProjectionFactConsumption) { let _ = value; }
struct ConsumerMeasurementConsumptionIdentity;
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
