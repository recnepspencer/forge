use std::{io::Write, path::Path};

use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    FilesystemMediaAdmission, PhysicalRecordOpen, PhysicalRuntimeAdmission, PhysicalStore,
    RecordAppendBatch,
};
use worth_store_physical_backend::{
    FilesystemAccessPosture, MediaFaultDirective, MediaOperationRole,
};

use super::DEATH_CASE_ENV;

pub(super) fn run(root: &Path) {
    let case = std::env::var(DEATH_CASE_ENV).unwrap();
    let fields = case.split(',').collect::<Vec<_>>();
    assert_eq!(fields.len(), 4);
    let role = death_role(fields[0]);
    let ordinal = fields[1].parse().unwrap();
    let payload_bytes = fields[3].parse().unwrap();
    let admission =
        FilesystemMediaAdmission::production(FilesystemAccessPosture::CoordinatedServiceAccount);
    let authority = admission.fault_schedule_authority();
    let gate = authority.pause_gate();
    let schedule = authority
        .schedule(vec![authority
            .rule(role, ordinal, death_directive(fields[2], &gate))
            .for_identified_operation_ordinal()])
        .unwrap();
    let runtime = PhysicalStore::admit(PhysicalRuntimeAdmission::new(root).unwrap()).unwrap();
    let media = match runtime
        .try_admit_filesystem_media(admission.with_fault_schedule(schedule))
        .into_raw()
    {
        TransitionOutcome::Success(media) => media,
        _ => panic!("death writer must acquire media"),
    };
    let (format, placement, access) = super::super::configuration();
    let serving = super::super::success(open_record_store!(media, |durability| {
        PhysicalRecordOpen::new(format, access, durability)
    }));
    spawn_death_observer(gate);
    let payload = vec![0x5a; payload_bytes];
    super::super::scenario_evidence::emit_process("faulting-writer", &serving);
    std::io::stdout().flush().unwrap();
    let _blocked = serving.record_submission().append_batch(
        RecordAppendBatch::try_from_iter([payload.as_slice()]).unwrap(),
        placement,
    );
    panic!("publication unexpectedly crossed the death gate");
}

fn death_role(role: &str) -> MediaOperationRole {
    match role {
        "positioned-write" => MediaOperationRole::PositionedWrite,
        "file-sync" => MediaOperationRole::SynchronizeFileState,
        "atomic-replace" => MediaOperationRole::AtomicReplace,
        "directory-sync" => MediaOperationRole::SynchronizeDirectoryPublication,
        _ => panic!("unknown death role"),
    }
}

fn death_directive(
    directive: &str,
    gate: &worth_store_physical_backend::MediaPauseGate,
) -> MediaFaultDirective {
    match directive {
        "prefix" => MediaFaultDirective::AllowPrefixThenPause {
            bytes: 1,
            gate: gate.clone(),
        },
        "before" => MediaFaultDirective::PauseBefore(gate.clone()),
        "after" => MediaFaultDirective::PauseAfter(gate.clone()),
        _ => panic!("unknown death directive"),
    }
}

fn spawn_death_observer(gate: worth_store_physical_backend::MediaPauseGate) {
    std::thread::spawn(move || {
        gate.wait_until_reached();
        let context = gate.reached_context().unwrap();
        println!(
            "C5_PUBLICATION_DEATH {} {} {} {}",
            context.role().metric_name(),
            context.role_ordinal(),
            context
                .identified_operation_ordinal()
                .expect("identified-operation fault must retain its ordinal"),
            context.requested_bytes()
        );
        std::io::stdout().flush().unwrap();
        std::process::exit(86);
    });
}
