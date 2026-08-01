use super::decode::read_record;
use super::watch::{classify_revision, AdmittedRevisionRelation};
use super::{
    PlatformPulseExecutorGatePosture, PlatformPulseIntentInputInstallation,
    PlatformPulseIntentInputOperability, PlatformPulseIntentInputWatchDenial,
};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_INSTALLATION: AtomicU64 = AtomicU64::new(1);

#[test]
fn intent_input_revision_admission_distinguishes_duplicate_stale_and_successor() {
    assert_eq!(classify_revision(7, 7), AdmittedRevisionRelation::Duplicate);
    assert_eq!(classify_revision(7, 6), AdmittedRevisionRelation::Stale);
    assert_eq!(classify_revision(7, 8), AdmittedRevisionRelation::Successor);
    assert_eq!(
        classify_revision(7, 70),
        AdmittedRevisionRelation::Successor
    );
}

#[test]
fn intent_samples_decode_to_distinct_typed_product_postures() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("intent_samples");
    let cases = [
        ("ready.json", PlatformPulseIntentInputOperability::Ready),
        (
            "confirmation-required.json",
            PlatformPulseIntentInputOperability::ConfirmationRequired,
        ),
        ("denied.json", PlatformPulseIntentInputOperability::Denied),
    ];
    for (file, posture) in cases {
        let record = read_record(&root.join(file)).expect("checked-in intent sample");
        assert_eq!(record.operability(), posture);
    }
}

#[test]
fn intent_input_rejects_unknown_fields_and_zero_revision() {
    let root = isolated_root();
    let target = root.join("platform-pulse-intent.json");
    std::fs::write(
        &target,
        br#"{"protocol":"worth-ui.platform-pulse.intent-source","schema_version":1,"revision":0,"operability":"ready","executor_gate":"held"}"#,
    )
    .expect("write invalid record");
    assert_eq!(
        read_record(&target),
        Err(PlatformPulseIntentInputWatchDenial::InvalidRevision)
    );
    std::fs::write(
        &target,
        br#"{"protocol":"worth-ui.platform-pulse.intent-source","schema_version":1,"revision":1,"operability":"ready","executor_gate":"held","forged":true}"#,
    )
    .expect("write hostile record");
    assert!(matches!(
        read_record(&target),
        Err(PlatformPulseIntentInputWatchDenial::Decode(_))
    ));
    std::fs::remove_dir_all(root).expect("remove intent input fixture");
}

#[test]
fn intent_installation_reads_once_before_starting_the_bounded_watch() {
    let root = isolated_root();
    std::fs::write(
        root.join("platform-pulse-intent.json"),
        include_bytes!("../../../intent_samples/ready.json"),
    )
    .expect("write initial intent input");
    let installation = PlatformPulseIntentInputInstallation::open(&root)
        .expect("open bounded intent input installation");
    let (initial, watch) = installation.into_parts();
    assert_eq!(initial.revision(), 1);
    assert_eq!(
        initial.executor_gate(),
        PlatformPulseExecutorGatePosture::Held
    );
    let shutdown = watch.shutdown().expect("shut down intent input watch");
    assert!(shutdown.worker_joined());
    assert_eq!(shutdown.pending_event_count(), 0);
    std::fs::remove_dir_all(root).expect("remove intent input fixture");
}

fn isolated_root() -> std::path::PathBuf {
    let ordinal = NEXT_INSTALLATION.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "worth-ui-platform-pulse-intent-input-{}-{ordinal}",
        std::process::id()
    ));
    std::fs::create_dir_all(&root).expect("create intent input fixture");
    root
}
