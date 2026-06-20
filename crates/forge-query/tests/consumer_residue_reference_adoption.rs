use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use forge_query::facade::consumer_kit::{
    query_consumer_residue_audit, ForgeQueryConsumerResidueClass,
};
use forge_query::{ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope};

const AUDITED_DOWNSTREAM_ROOTS: &[&str] =
    &["worth-kernel/src/construction", "hadwiger-research/src"];
static WORKSPACE_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[test]
fn downstream_roots_adopt_generic_consumer_residue_audit_without_local_ceremony() {
    let workspace_crates_dir = workspace_crates_dir();
    let audit = AUDITED_DOWNSTREAM_ROOTS.iter().fold(
        query_consumer_residue_audit("downstream-query-consumers"),
        |audit, root| audit.required_root(workspace_crates_dir.join(root)),
    );

    let report = audit
        .evaluate()
        .expect("all audited downstream roots must exist and be readable");

    assert_eq!(report.audited_roots().len(), AUDITED_DOWNSTREAM_ROOTS.len());
    assert!(report.scanned_file_count() > 0);
    assert_eq!(
        report.audited_source_paths().len(),
        report.scanned_file_count()
    );
    assert!(report
        .audited_source_paths()
        .iter()
        .any(|path| path.ends_with("worth-kernel/src/construction/authoring.rs")));
    assert!(report
        .audited_source_paths()
        .iter()
        .any(|path| path.ends_with("hadwiger-research/src/lib.rs")));
    assert!(!report.source_inventory_digest().is_empty());
    assert_eq!(
        report.report_identity().scope(),
        ForgeQueryEvidenceScope::ConsumerResidueReport
    );
    assert!(
        report
            .finding_identities()
            .iter()
            .all(finding_identity_is_consumer_residue),
        "generic consumer residue findings must carry generic residue identity"
    );
    report.assert_clean();
}

#[test]
fn reference_consumer_adoption_path_detects_seeded_real_source_overlay() {
    let crates_dir = workspace_crates_dir();
    let source = fs::read_to_string(crates_dir.join("worth-kernel/src/construction/authoring.rs"))
        .expect("reference source should be readable");
    let seeded_root = seeded_reference_root(&format!(
        "{source}\nstruct LocalQueryReport;\nfn seeded(parts: Vec<String>) {{ let query_proof = parts.join(\"||\"); let _ = query_proof; }}\n"
    ));

    let clean = query_consumer_residue_audit("reference-overlay")
        .required_root(&seeded_root)
        .evaluate()
        .expect("seeded overlay should parse");

    assert!(
        clean
            .findings()
            .iter()
            .any(|finding| finding.residue_class()
                == ForgeQueryConsumerResidueClass::LocalQueryReport)
    );
    assert!(clean.findings().iter().any(|finding| {
        finding.residue_class() == ForgeQueryConsumerResidueClass::DelimiterJoinedQueryProof
    }));
    assert_eq!(clean.scanned_file_count(), 1);
    assert_eq!(clean.audited_source_paths().len(), 1);
}

fn finding_identity_is_consumer_residue(identity: &ForgeQueryEvidenceIdentity) -> bool {
    identity.scope() == ForgeQueryEvidenceScope::ConsumerResidueFinding
}

fn workspace_crates_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("forge-query crate should live under crates")
        .to_path_buf()
}

fn seeded_reference_root(source: &str) -> PathBuf {
    let unique = WORKSPACE_COUNTER.fetch_add(1, Ordering::SeqCst);
    let root = std::env::temp_dir()
        .join("forge-query-consumer-residue-reference-adoption")
        .join(format!("seeded-{}-{unique}", std::process::id()));
    fs::create_dir_all(&root).expect("seeded reference root should be creatable");
    fs::write(root.join("lib.rs"), source).expect("seeded reference source should be writable");
    root
}
