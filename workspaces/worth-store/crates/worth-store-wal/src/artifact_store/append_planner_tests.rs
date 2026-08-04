use std::io::{Seek, SeekFrom, Write};

use super::{WalAppendPlanner, WalAppendPlannerDenial, WalArtifactStoreDenial, WalFrameAppendPlan};

#[test]
fn reused_planner_scans_each_durable_frame_once_instead_of_each_history() {
    let directory = tempfile::tempdir().expect("temp directory");
    let planner = WalAppendPlanner::open(directory.path(), 7, 3).expect("open planner");
    assert_eq!(planner.opening_prefix_bytes_scanned(), 0);
    assert_eq!(planner.scan_buffer_bytes(), 64 * 1024);

    let mut total_frame_bytes = 0u64;
    let mut total_prefix_bytes_scanned = 0u64;
    for index in 0..128 {
        let plan = planner
            .prepare_append(
                index + 10,
                index + 11,
                &format!("frame-{index}"),
                b"payload",
            )
            .expect("prepare append");
        if index == 0 {
            assert_eq!(plan.prefix_bytes_scanned(), 0);
        } else {
            assert_eq!(
                plan.prefix_bytes_scanned(),
                plan.encoded_frame().len() as u64
            );
        }
        total_prefix_bytes_scanned += plan.prefix_bytes_scanned();
        total_frame_bytes += plan.encoded_frame().len() as u64;
        persist(directory.path(), &plan);
    }

    let final_plan = planner
        .prepare_append(138, 139, "final", b"payload")
        .expect("observe final durable frame");
    total_prefix_bytes_scanned += final_plan.prefix_bytes_scanned();
    assert_eq!(total_prefix_bytes_scanned, total_frame_bytes);
    assert_eq!(
        planner
            .prepare_append(138, 139, "same-tail", b"payload")
            .expect("unchanged tail")
            .prefix_bytes_scanned(),
        0
    );

    let reopened = WalAppendPlanner::open(directory.path(), 7, 3).expect("reopen planner");
    assert_eq!(reopened.opening_prefix_bytes_scanned(), total_frame_bytes);
    assert_eq!(
        reopened
            .prepare_append(138, 139, "reopened", b"payload")
            .expect("prepare after reopen")
            .prefix_bytes_scanned(),
        0
    );
}

#[test]
fn torn_suffix_is_trimmed_by_the_append_contract_not_accepted_as_history() {
    let directory = tempfile::tempdir().expect("temp directory");
    let planner = WalAppendPlanner::open(directory.path(), 7, 3).expect("open planner");
    let first = planner
        .prepare_append(10, 11, "first", b"payload")
        .expect("first plan");
    persist(directory.path(), &first);
    let path = directory.path().join(first.relative_path());
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(&path)
        .expect("open WAL");
    file.write_all(b"TORN").expect("write torn suffix");
    file.sync_all().expect("sync torn suffix");

    let repair = planner
        .prepare_append(11, 12, "second", b"next")
        .expect("plan from durable prefix");
    assert_eq!(
        repair.valid_prefix_bytes(),
        first.encoded_frame().len() as u64
    );
    assert_eq!(
        repair.observed_file_bytes(),
        first.encoded_frame().len() as u64 + 4
    );
    persist(directory.path(), &repair);

    let reopened = WalAppendPlanner::open(directory.path(), 7, 3).expect("reopen repaired WAL");
    assert_eq!(
        reopened.opening_prefix_bytes_scanned(),
        first.encoded_frame().len() as u64 + repair.encoded_frame().len() as u64
    );
}

#[test]
fn checksum_corruption_denies_reopen_instead_of_becoming_an_append_prefix() {
    let directory = tempfile::tempdir().expect("temp directory");
    let planner = WalAppendPlanner::open(directory.path(), 7, 3).expect("open planner");
    let first = planner
        .prepare_append(10, 11, "first", b"payload")
        .expect("first plan");
    persist(directory.path(), &first);
    let path = directory.path().join(first.relative_path());
    let mut bytes = std::fs::read(&path).expect("read WAL");
    bytes[116] ^= 0x80;
    std::fs::write(path, bytes).expect("corrupt WAL");

    assert!(matches!(
        WalAppendPlanner::open(directory.path(), 7, 3),
        Err(WalAppendPlannerDenial::Artifact(
            WalArtifactStoreDenial::DigestMismatch
        ))
    ));
}

fn persist(root: &std::path::Path, plan: &WalFrameAppendPlan) {
    let path = root.join(plan.relative_path());
    std::fs::create_dir_all(path.parent().expect("WAL parent")).expect("create WAL parent");
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(path)
        .expect("open WAL");
    assert_eq!(
        file.metadata().expect("WAL metadata").len(),
        plan.observed_file_bytes()
    );
    file.set_len(plan.valid_prefix_bytes())
        .expect("truncate invalid tail");
    file.seek(SeekFrom::Start(plan.valid_prefix_bytes()))
        .expect("seek durable prefix");
    file.write_all(plan.encoded_frame()).expect("append frame");
    file.sync_all().expect("sync WAL");
}
