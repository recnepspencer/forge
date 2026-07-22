use std::io::Write;
use std::path::Path;

use worth_store::physical_runtime::{
    ExternalPhysicalRecordLocator, ManifestEntryCapacity, PageFillPercent,
    PhysicalRecordPlacementPolicy, RecordAppendBatch, RecordAppendDenial, RecordAppendError,
    RecordByteLimit, RecordReadLimits, SegmentPageCount,
};
use worth_store_physical_backend::MediaOperationRole;

use super::{
    allocation_probe::allocations_during, configuration, serving_from_initialization,
    serving_from_open,
};

const CHILD_TEST: &str = "child_process::c5_child_role";
const ROLE_ENV: &str = "WORTH_STORE_C5_CHILD_ROLE";
const ROOT_ENV: &str = "WORTH_STORE_C5_CHILD_ROOT";
const LOCATOR_ENV: &str = "WORTH_STORE_C5_LOCATOR";
const ORACLE_ENV: &str = "WORTH_STORE_C5_ORACLE";
const DEATH_CASE_ENV: &str = "WORTH_STORE_C5_DEATH_CASE";

#[path = "child_process/locator_codec.rs"]
mod locator_codec;
pub(super) use locator_codec::{decode_locator, hex, unhex};

pub(super) fn run_child(role: &str, root: &Path, locator: Option<&str>) -> String {
    let mut command = std::process::Command::new(std::env::current_exe().unwrap());
    command
        .args(["--exact", CHILD_TEST, "--nocapture"])
        .env(ROLE_ENV, role)
        .env(ROOT_ENV, root);
    if let Some(locator) = locator {
        command.env(LOCATOR_ENV, locator);
    }
    let output = command.output().unwrap();
    let causal_marker = match role {
        "publication_reopener" => "C5_PREDICATE:independent-decision-path ",
        "allocation_writer" | "allocation_reader" => "C5_PREDICATE:transfer-allocation-slope ",
        _ => "",
    };
    assert!(
        output.status.success(),
        "{causal_marker}child {role} failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

pub(super) fn run_courtroom_writer(root: &Path, locators: &Path, oracle: &Path) -> String {
    run_courtroom_child("courtroom_writer", root, locators, Some(oracle))
}

pub(super) fn run_courtroom_reopener(root: &Path, locators: &Path) -> String {
    run_courtroom_child("courtroom_reopener", root, locators, None)
}

fn run_courtroom_child(role: &str, root: &Path, locators: &Path, oracle: Option<&Path>) -> String {
    let mut command = std::process::Command::new(std::env::current_exe().unwrap());
    command
        .args(["--exact", CHILD_TEST, "--nocapture"])
        .env(ROLE_ENV, role)
        .env(ROOT_ENV, root)
        .env(LOCATOR_ENV, locators);
    if let Some(oracle) = oracle {
        command.env(ORACLE_ENV, oracle);
    }
    let output = command.output().unwrap();
    assert!(
        output.status.success(),
        "child {role} failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn c5_child_role() {
    let Ok(role) = std::env::var(ROLE_ENV) else {
        return;
    };
    let root = std::path::PathBuf::from(std::env::var_os(ROOT_ENV).unwrap());
    match role.as_str() {
        "writer" => child_writer(&root),
        "reader" => child_reader(&root),
        "segment_reader" => child_segment_reader(&root),
        "extent_reader" => {
            super::extent_child::extent_reader(&root, &std::env::var(LOCATOR_ENV).unwrap())
        }
        "allocation_writer" => {
            super::extent_child::allocation_writer(&root, &std::env::var(LOCATOR_ENV).unwrap())
        }
        "allocation_reader" => {
            super::extent_child::allocation_reader(&root, &std::env::var(LOCATOR_ENV).unwrap())
        }
        "scale_allocation_reader" => super::extent_child::scale_allocation_reader(
            &root,
            &std::env::var(LOCATOR_ENV).unwrap(),
        ),
        "batch_admission_probe" => child_batch_admission_probe(),
        "geometry_admission_probe" => child_geometry_admission_probe(&root),
        "second_owner" => child_second_owner(&root),
        "courtroom_writer" => super::courtroom_child::writer(
            &root,
            std::path::PathBuf::from(std::env::var_os(LOCATOR_ENV).unwrap()),
            std::path::PathBuf::from(std::env::var_os(ORACLE_ENV).unwrap()),
        ),
        "courtroom_reopener" => super::courtroom_child::reopener(
            &root,
            std::path::PathBuf::from(std::env::var_os(LOCATOR_ENV).unwrap()),
        ),
        "publication_death_writer" => child_publication_death_writer(&root),
        "publication_reopener" => child_publication_reopener(&root),
        "c6_pressure_writer" => {
            super::c6_preparation::pressure_writer(&root, &std::env::var(LOCATOR_ENV).unwrap())
        }
        "c6_pressure_reader" => {
            super::c6_preparation::pressure_reader(&root, &std::env::var(LOCATOR_ENV).unwrap())
        }
        "c6_writeback_writer" => super::writeback_courtroom::writer(&root),
        "c6_writeback_observer" => {
            super::writeback_courtroom::observer(&root, &std::env::var(LOCATOR_ENV).unwrap())
        }
        "c6_writeback_reopener" => super::writeback_courtroom::reopener(&root),
        _ => panic!("unknown child role"),
    }
}

fn child_publication_reopener(root: &Path) {
    let serving = serving_from_open(root);
    let scan = super::scan_journeys::collect_scan(&serving, 17, 64_000);
    super::scenario_evidence::emit_process("fresh-reopener", &serving);
    println!(
        "C5_PUBLICATION_REOPEN {} {} {}",
        serving
            .observer()
            .acquisition_snapshot()
            .unwrap()
            .root_generation(),
        scan.len(),
        serving.observed_non_authoritative_residue(),
    );
    std::io::stdout().flush().unwrap();
    serving.close();
}

fn child_publication_death_writer(root: &Path) {
    use worth_proof::TransitionOutcome;
    use worth_store::physical_runtime::{
        FilesystemMediaAdmission, PhysicalRecordOpen, PhysicalRuntimeAdmission, PhysicalStore,
    };
    use worth_store_physical_backend::{
        FilesystemAccessPosture, MediaFaultDirective, MediaOperationRole,
    };

    let case = std::env::var(DEATH_CASE_ENV).unwrap();
    let fields = case.split(',').collect::<Vec<_>>();
    assert_eq!(fields.len(), 4);
    let role = match fields[0] {
        "positioned-write" => MediaOperationRole::PositionedWrite,
        "file-sync" => MediaOperationRole::SynchronizeFileState,
        "atomic-replace" => MediaOperationRole::AtomicReplace,
        "directory-sync" => MediaOperationRole::SynchronizeDirectoryPublication,
        _ => panic!("unknown death role"),
    };
    let ordinal = fields[1].parse().unwrap();
    let payload_bytes = fields[3].parse().unwrap();
    let admission =
        FilesystemMediaAdmission::production(FilesystemAccessPosture::CoordinatedServiceAccount);
    let authority = admission.fault_schedule_authority();
    let gate = authority.pause_gate();
    let directive = match fields[2] {
        "prefix" => MediaFaultDirective::AllowPrefixThenPause {
            bytes: 1,
            gate: gate.clone(),
        },
        "before" => MediaFaultDirective::PauseBefore(gate.clone()),
        "after" => MediaFaultDirective::PauseAfter(gate.clone()),
        _ => panic!("unknown death directive"),
    };
    let schedule = authority
        .schedule(vec![authority.rule(role, ordinal, directive)])
        .unwrap();
    let runtime = PhysicalStore::admit(PhysicalRuntimeAdmission::new(root).unwrap()).unwrap();
    let media = match runtime
        .try_admit_filesystem_media(admission.with_fault_schedule(schedule))
        .into_raw()
    {
        TransitionOutcome::Success(media) => media,
        _ => panic!("death writer must acquire media"),
    };
    let (format, placement, access) = super::configuration();
    let mut serving =
        super::success(media.open_record_store(PhysicalRecordOpen::new(format, access)));
    std::thread::spawn(move || {
        gate.wait_until_reached();
        let context = gate.reached_context().unwrap();
        println!(
            "C5_PUBLICATION_DEATH {} {} {}",
            context.role().metric_name(),
            context.role_ordinal(),
            context.requested_bytes()
        );
        std::io::stdout().flush().unwrap();
        std::process::exit(86);
    });
    let payload = vec![0x5a; payload_bytes];
    super::scenario_evidence::emit_process("faulting-writer", &serving);
    std::io::stdout().flush().unwrap();
    let _blocked = serving.records_mut().append_batch(
        RecordAppendBatch::try_from_iter([payload.as_slice()]).unwrap(),
        placement,
    );
    panic!("publication unexpectedly crossed the death gate");
}

fn child_second_owner(root: &Path) {
    use worth_proof::TransitionOutcome;
    use worth_store::physical_runtime::{
        FilesystemMediaAdmission, PhysicalRuntimeAdmission, PhysicalStore,
    };
    use worth_store_physical_backend::FilesystemAccessPosture;

    let admitted = PhysicalRuntimeAdmission::new(root)
        .ok()
        .and_then(|request| PhysicalStore::admit(request).ok());
    let admitted = admitted.is_some_and(|runtime| {
        matches!(
            runtime
                .try_admit_filesystem_media(FilesystemMediaAdmission::production(
                    FilesystemAccessPosture::CoordinatedServiceAccount,
                ))
                .into_raw(),
            TransitionOutcome::Success(_)
        )
    });
    println!(
        "C5_SECOND_OWNER {}",
        if admitted { "admitted" } else { "denied" }
    );
    std::io::stdout().flush().unwrap();
}

fn child_batch_admission_probe() {
    let builder = (0..u16::MAX).fold(RecordAppendBatch::builder(), |builder, _| {
        builder.push_owned(Vec::new())
    });
    let payload = vec![0x5a; 1024 * 1024];
    let (builder, allocations) = allocations_during(|| builder.push_bytes(&payload));
    let denied = matches!(
        builder.build(),
        Err(RecordAppendDenial::BatchRecordLimitExceeded)
    );
    println!(
        "C5_BATCH_ADMISSION {} {} {denied}",
        allocations.allocations, allocations.bytes_allocated,
    );
    std::io::stdout().flush().unwrap();
}

fn child_geometry_admission_probe(root: &Path) {
    let (format, _, _) = configuration();
    let placement = PhysicalRecordPlacementPolicy::builder()
        .segment_pages(SegmentPageCount::new(4).unwrap())
        .extent_threshold(RecordByteLimit::new(8_000).unwrap())
        .page_fill(PageFillPercent::new(1).unwrap())
        .manifest_capacity(ManifestEntryCapacity::new(64).unwrap())
        .admit(format)
        .unwrap();
    let mut serving = serving_from_initialization(root);
    let batch = RecordAppendBatch::builder()
        .push_source(super::stream_fixture::PatternSource::exact(200))
        .build()
        .unwrap();
    let writes_before = serving
        .media_counters()
        .attempts_for(MediaOperationRole::PositionedWrite);
    let (result, allocations) =
        allocations_during(|| serving.records_mut().append_batch(batch, placement));
    let writes_after = serving
        .media_counters()
        .attempts_for(MediaOperationRole::PositionedWrite);
    println!(
        "C5_GEOMETRY_ADMISSION {} {} {} {}",
        allocations.allocations,
        allocations.bytes_allocated,
        writes_after - writes_before,
        matches!(
            result,
            Err(RecordAppendError::Denied(
                RecordAppendDenial::InlinePageFull
            ))
        )
    );
    std::io::stdout().flush().unwrap();
    serving.close();
}

fn child_segment_reader(root: &Path) {
    let serving = serving_from_open(root);
    for requested in std::env::var(LOCATOR_ENV).unwrap().split(';') {
        let (index, encoded) = requested.split_once(':').unwrap();
        let locator = ExternalPhysicalRecordLocator::decode(unhex(encoded)).unwrap();
        let mut record = serving
            .records()
            .open_external(
                locator,
                RecordReadLimits::new(RecordByteLimit::new(4_000).unwrap()),
            )
            .expect("C5_PREDICATE:identity-placement-seam");
        let mut bytes = vec![0_u8; 3_000];
        let mut completed = 0;
        while completed < bytes.len() {
            let count = record.read_next(&mut bytes[completed..]).unwrap();
            assert!(count > 0);
            completed += count;
        }
        assert_eq!(record.read_next(&mut [0_u8; 1]).unwrap(), 0);
        assert!(
            bytes.iter().all(|byte| *byte == bytes[0]),
            "C5_PREDICATE:identity-placement-seam"
        );
        let observation = record.observation();
        println!(
            "C5_SEGMENT {index} {} {} {} {}",
            bytes[0],
            completed,
            observation.touched_segments(),
            observation.touched_pages(),
        );
    }
    std::io::stdout().flush().unwrap();
    serving.close();
}

fn child_writer(root: &Path) {
    let (_, placement, _) = configuration();
    let mut serving = serving_from_initialization(root);
    let published = serving
        .records_mut()
        .append_batch(
            RecordAppendBatch::try_from_iter([b"alpha".as_slice()]).unwrap(),
            placement,
        )
        .unwrap();
    let first = ExternalPhysicalRecordLocator::new(
        serving.store_identity(),
        published.record_id(0).unwrap(),
    );
    let successor = serving
        .records_mut()
        .append_batch(
            RecordAppendBatch::try_from_iter([b"beta".as_slice()]).unwrap(),
            placement,
        )
        .unwrap();
    let second = ExternalPhysicalRecordLocator::new(
        serving.store_identity(),
        successor.record_id(0).unwrap(),
    );
    println!("C5_LOCATOR {}", hex(&first.encode()));
    println!("C5_LOCATOR_2 {}", hex(&second.encode()));
    std::io::stdout().flush().unwrap();
    std::process::exit(0);
}

fn child_reader(root: &Path) {
    let serving = serving_from_open(root);
    let locators = std::env::var(LOCATOR_ENV).unwrap();
    for (index, encoded) in locators.split(',').enumerate() {
        let locator = ExternalPhysicalRecordLocator::decode(unhex(encoded)).unwrap();
        let mut record = serving
            .records()
            .open_external(
                locator,
                RecordReadLimits::new(RecordByteLimit::new(1024).unwrap()),
            )
            .unwrap();
        let label = if index == 0 {
            "C5_PAYLOAD"
        } else {
            "C5_PAYLOAD_2"
        };
        let mut bytes = vec![0_u8; 5_usize.saturating_sub(index)];
        let mut completed = 0;
        while completed < bytes.len() {
            let count = record.read_next(&mut bytes[completed..]).unwrap();
            assert!(count > 0);
            completed += count;
        }
        assert_eq!(record.read_next(&mut [0_u8; 1]).unwrap(), 0);
        println!("{label} {}", hex(&bytes));
    }
    serving.close();
}
