use tempfile::tempdir;
use worth_proof::TransitionOutcome;

use super::super::fixture::{serving_from_initialization_with_work_profile, work_fixture};

#[test]
fn reopen_cannot_consume_serialized_signal_state() {
    reopen_ignores_external_state(
        "worth-c5-serialized-signal-reopen",
        b"serialized Signal state",
        "serialized-signal-reopen",
    );
}

#[test]
fn ordinary_physical_work_cannot_add_an_internal_json_carrier() {
    reopen_ignores_external_state(
        "worth-c5-internal-physical-work-json",
        br#"{"route":"forged"}"#,
        "internal-json-carrier",
    );
}

fn reopen_ignores_external_state(prefix: &str, bytes: &[u8], predicate: &str) {
    let probe = std::env::temp_dir().join(format!("{prefix}-{}", std::process::id()));
    let _ = std::fs::remove_file(&probe);
    let root = tempdir().unwrap();
    let (profile, _, _) = work_fixture();
    serving_from_initialization_with_work_profile(root.path(), profile.clone()).close();
    std::fs::write(&probe, bytes).unwrap();

    let (format, _, access) = super::super::configuration();
    let reopened = open_record_store!(super::super::media(root.path()), |durability| {
        worth_store::physical_runtime::PhysicalRecordOpen::new(format, access, durability)
            .with_physical_work_profile(profile)
    },)
    .into_raw();
    let _ = std::fs::remove_file(&probe);
    let serving = match reopened {
        TransitionOutcome::Success(serving) => serving,
        _ => panic!(
            "C5_PREDICATE:{predicate}: fresh reopen consumed external derived state instead of physical truth"
        ),
    };
    serving.close();
}
