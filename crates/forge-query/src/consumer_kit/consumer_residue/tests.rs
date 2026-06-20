use super::audit::query_owned_consumer_residue_root_authority;
use super::query_consumer_residue_audit;
use super::registry::{forge_query_consumer_residue_registry, ForgeQueryConsumerResidueClass};

static QUERY_OWNED_ROOT_COUNTER: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);

#[test]
fn consumer_residue_registry_covers_all_phase_nine_classes() {
    let classes = forge_query_consumer_residue_registry()
        .iter()
        .map(|row| row.class())
        .collect::<std::collections::BTreeSet<_>>();

    for required in [
        ForgeQueryConsumerResidueClass::RuntimeSchemaAdapter,
        ForgeQueryConsumerResidueClass::RuntimeSourceAdapter,
        ForgeQueryConsumerResidueClass::RuntimeWriteAuthorityAdapter,
        ForgeQueryConsumerResidueClass::RuntimeSignalSinkAdapter,
        ForgeQueryConsumerResidueClass::RuntimeSnapshotIdentityAdapter,
        ForgeQueryConsumerResidueClass::RuntimeSubscriptionActivationAdapter,
        ForgeQueryConsumerResidueClass::RuntimePreviewBasisAdapter,
        ForgeQueryConsumerResidueClass::RuntimeInspectorEvidenceAdapter,
        ForgeQueryConsumerResidueClass::RuntimeBridgeHandAssembly,
        ForgeQueryConsumerResidueClass::FabricatedMutationReceipt,
        ForgeQueryConsumerResidueClass::FabricatedBridgeMutationReceipt,
        ForgeQueryConsumerResidueClass::FabricatedWriteAuthorityReceipt,
        ForgeQueryConsumerResidueClass::LocalQueryReport,
        ForgeQueryConsumerResidueClass::LocalQueryProof,
        ForgeQueryConsumerResidueClass::RawSupportSnapshotRow,
        ForgeQueryConsumerResidueClass::SupportMatrixRowSearch,
        ForgeQueryConsumerResidueClass::DebugDerivedQueryProof,
        ForgeQueryConsumerResidueClass::DelimiterJoinedQueryProof,
        ForgeQueryConsumerResidueClass::DelimiterFormattedQueryProof,
    ] {
        assert!(
            classes.contains(&required),
            "missing consumer residue registry row for {required:?}"
        );
    }
}

#[test]
fn consumer_residue_registry_classes_are_unique() {
    let rows = forge_query_consumer_residue_registry();
    let classes = rows
        .iter()
        .map(|row| row.class())
        .collect::<std::collections::BTreeSet<_>>();

    assert_eq!(
        rows.len(),
        classes.len(),
        "consumer residue registry must not contain duplicate class rows"
    );
}

#[test]
fn query_owned_roots_can_be_allowed_only_inside_query_authority() {
    let unique = QUERY_OWNED_ROOT_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let root = std::env::temp_dir().join(format!(
        "forge-query-owned-consumer-residue-{}-{unique}",
        std::process::id(),
    ));
    std::fs::create_dir_all(&root).expect("query-owned fixture root should be creatable");
    std::fs::write(
        root.join("lib.rs"),
        r#"
struct LocalQueryReport;
struct LocalQueryProof;
struct ForgeQuerySupportSnapshotRow;
"#,
    )
    .expect("query-owned fixture source should be writable");

    let report = query_consumer_residue_audit("forge-query")
        .required_query_owned_implementation_root(
            &root,
            &query_owned_consumer_residue_root_authority(),
        )
        .evaluate()
        .expect("query-owned consumer residue fixture must parse");

    assert_eq!(report.finding_count(), 0, "{:?}", report.findings());
}
