use super::super::read_repository_document;

mod bounded_capture;

const BASIS: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                     record_serving/work_semantics/durability/checkpoint_capture_basis.rs";
const WORK_PORT: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                         durability/checkpoint/work_port.rs";
const CAPTURE: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                       durability/checkpoint/capture.rs";
const CAPTURE_EXECUTION: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                                 durability/checkpoint/capture/execution.rs";
const CAPTURE_STREAMING: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                                 durability/checkpoint/capture/streaming.rs";
const HANDLE: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                      durability/checkpoint/handle.rs";
const PROGRESS: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                        durability/checkpoint/progress.rs";
const PUBLICATION: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                           durability/checkpoint/publication.rs";
const SCHEDULER: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                         instance/scheduler_admission/checkpoint.rs";
const DEMAND: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                      work/scheduler_demand.rs";
const RECORD_POLICY: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                             record_serving/record_queue_policy.rs";
const QUEUE_POLICY: &str = "workspaces/worth-store/crates/worth-store-io-scheduler/src/\
                            queue_execution/admission/policy_receipt.rs";
const EXECUTOR: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                        instance/executor/checkpoint.rs";
const JOURNEY: &str = "workspaces/worth-store/crates/worth-store/tests/\
                       physical_record_journeys/durability_admission/checkpoint_capture.rs";
const OWNERSHIP: &str = "workspaces/worth-store/crates/worth-store/tests/\
                         physical_record_journeys/durability_admission/wal_ownership_shape.rs";
const PRESSURE: &str = "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/\
                        durability_admission/checkpoint_pressure.rs";

#[test]
fn checkpoint_capture_has_one_exact_background_signal_and_media_route() {
    inspect(&sources()).unwrap();
}

#[test]
fn checkpoint_capture_gate_rejects_authority_and_route_bypass_mutants() {
    let source = sources();

    let mut projection_basis = source.clone();
    projection_basis.basis = projection_basis.basis.replace(
        "PhysicalWorkSemanticBasis::mutation",
        "PhysicalWorkSemanticBasis::projection",
    );
    assert!(inspect(&projection_basis).is_err());

    let mut wrong_signal = source.clone();
    wrong_signal.basis = wrong_signal.basis.replace(
        "PhysicalWorkSignalFamily::CheckpointCapture",
        "PhysicalWorkSignalFamily::WalAppend",
    );
    assert!(inspect(&wrong_signal).is_err());

    let mut foreground_route = source.clone();
    foreground_route.work_port = foreground_route.work_port.replace(
        "PhysicalSchedulerDemand::checkpoint_background",
        "PhysicalSchedulerDemand::foreground",
    );
    assert!(inspect(&foreground_route).is_err());

    let mut raw_signal_bypass = source.clone();
    raw_signal_bypass.work_port = raw_signal_bypass.work_port.replace(
        "runtime\n            .signal\n            .request(admitted)",
        "runtime.submission.request(admitted)",
    );
    assert!(inspect(&raw_signal_bypass).is_err());

    let mut wrong_pressure = source.clone();
    wrong_pressure.scheduler = wrong_pressure.scheduler.replace(
        "filesystem_admitted_checkpoint_flush",
        "filesystem_admitted_compaction_flush",
    );
    assert!(inspect(&wrong_pressure).is_err());

    let mut planning_policy = source.clone();
    planning_policy.record_policy = planning_policy.record_policy.replace(
        "FoundationalPerformanceWorkClass::AuthoritativeMutation,\n        FoundationalPerformanceAccessPatternPosture::RebuildCapable",
        "FoundationalPerformanceWorkClass::ValidationPlanning,\n        FoundationalPerformanceAccessPatternPosture::RebuildCapable",
    );
    assert!(inspect(&planning_policy).is_err());

    let mut receipt_substitution = source.clone();
    receipt_substitution.work_port = receipt_substitution.work_port.replace(
        "PhysicalWorkScheduler::admit(demand, &backend, policy)",
        "PhysicalWorkScheduler::admit(demand, &backend, policy.clone())",
    );
    assert!(inspect(&receipt_substitution).is_err());

    let mut wrong_operation = source.clone();
    wrong_operation.demand = wrong_operation.demand.replace(
        "operation != PhysicalWorkOperationFamily::CheckpointCapture",
        "operation != PhysicalWorkOperationFamily::WalAppend",
    );
    assert!(inspect(&wrong_operation).is_err());

    let mut direct_media = source.clone();
    direct_media
        .capture
        .push_str("\nfn bypass() { media.artifact_tree(); }\n");
    assert!(inspect(&direct_media).is_err());

    let mut unscheduled_publish = source;
    unscheduled_publish.executor = unscheduled_publish
        .executor
        .replace(".replace_scheduled(", ".replace(");
    assert!(inspect(&unscheduled_publish).is_err());

    let mut missing_independent_evidence = sources();
    missing_independent_evidence.journey = missing_independent_evidence
        .journey
        .replace(".checkpoints()", ".checkpoint_bypass()");
    assert!(inspect(&missing_independent_evidence).is_err());
}

#[derive(Clone)]
struct CheckpointRouteSources {
    basis: String,
    work_port: String,
    capture: String,
    capture_execution: String,
    capture_streaming: String,
    handle: String,
    progress: String,
    publication: String,
    scheduler: String,
    demand: String,
    record_policy: String,
    queue_policy: String,
    executor: String,
    journey: String,
    ownership: String,
    pressure: String,
}

fn sources() -> CheckpointRouteSources {
    CheckpointRouteSources {
        basis: read(BASIS),
        work_port: read(WORK_PORT),
        capture: read(CAPTURE),
        capture_execution: read(CAPTURE_EXECUTION),
        capture_streaming: read(CAPTURE_STREAMING),
        handle: read(HANDLE),
        progress: read(PROGRESS),
        publication: read(PUBLICATION),
        scheduler: read(SCHEDULER),
        demand: read(DEMAND),
        record_policy: read(RECORD_POLICY),
        queue_policy: read(QUEUE_POLICY),
        executor: read(EXECUTOR),
        journey: read(JOURNEY),
        ownership: read(OWNERSHIP),
        pressure: read(PRESSURE),
    }
}

fn read(path: &str) -> String {
    read_repository_document(path)
        .unwrap_or_else(|error| panic!("{error}"))
        .replace("\r\n", "\n")
}

fn inspect(source: &CheckpointRouteSources) -> Result<(), &'static str> {
    inspect_basis(&source.basis)?;
    bounded_capture::inspect(source)?;
    inspect_progress(&source.work_port)?;
    inspect_scheduler(&source.scheduler, &source.record_policy)?;
    inspect_demand(&source.demand, &source.queue_policy)?;
    inspect_executor(
        &source.executor,
        [
            &source.capture,
            &source.capture_execution,
            &source.capture_streaming,
            &source.work_port,
            &source.publication,
        ],
    )?;
    inspect_evidence(&source.journey, &source.ownership)?;
    Ok(())
}

fn inspect_basis(source: &str) -> Result<(), &'static str> {
    let install = compact(function_body(source, "fn install(").ok_or("checkpoint basis absent")?);
    for required in [
        "PhysicalWorkSemanticBasis::mutation(",
        "dependency_and_output_declaration(",
        "PhysicalWorkSignalFamily::CheckpointCapture",
    ] {
        if !install.contains(required) {
            return Err("checkpoint capture lost its dedicated mutation Signal basis");
        }
    }
    Ok(())
}

fn inspect_progress(source: &str) -> Result<(), &'static str> {
    let progress = compact(
        function_body(source, "fn prepare_command(").ok_or("checkpoint progression absent")?,
    );
    if !contains_in_order(
        &progress,
        &[
            "PhysicalMutationWorkRequest::checkpoint_capture(",
            "self.record.checkpoint_capture_basis()",
            "runtime.signal.request(admitted)",
            "self.scheduler.checkpoint_background(",
            "letlease=require_complete_lease(pacing)?",
            "PhysicalSchedulerDemand::checkpoint_background(ready,lease)",
            "PhysicalWorkScheduler::admit(demand,&backend,policy)",
            "PhysicalExecutorCommand::checkpoint(work,payload)",
        ],
    ) {
        return Err("checkpoint progression bypasses Signal background admission or exact receipt");
    }
    Ok(())
}

fn inspect_scheduler(scheduler: &str, policy: &str) -> Result<(), &'static str> {
    let scheduler = compact(
        function_body(scheduler, "fn checkpoint_background(")
            .ok_or("checkpoint scheduler owner absent")?,
    );
    for required in [
        "ForegroundLaneDeclaration::filesystem_admitted_wal_barrier()",
        "BackgroundIoPressureShape::filesystem_admitted_checkpoint_flush()",
        "admit_checkpoint_background_policy(budget)",
        "BackgroundCapacityAdmissionRequest::new(pressure,&foreground_receipt,&self.fsync,policy.clone(),)",
        "admit_background_pacing(",
    ] {
        if !scheduler.contains(required) {
            return Err("checkpoint scheduler lost foreground preservation or background authority");
        }
    }
    let policy = compact(
        function_body(policy, "fn admit_checkpoint_background_policy(")
            .ok_or("checkpoint Foundational policy absent")?,
    );
    for required in [
        "FoundationalPerformanceWorkClass::AuthoritativeMutation",
        "FoundationalPerformanceAccessPatternPosture::RebuildCapable",
        "FoundationalPerformanceExecutionTemperature::ColdPath",
    ] {
        if !policy.contains(required) {
            return Err("checkpoint policy is not authoritative reconstructive cold work");
        }
    }
    Ok(())
}

fn inspect_demand(demand: &str, queue_policy: &str) -> Result<(), &'static str> {
    let demand = compact(
        function_body(demand, "fn checkpoint_background(")
            .ok_or("checkpoint background demand absent")?,
    );
    for required in [
        "operation!=PhysicalWorkOperationFamily::CheckpointCapture",
        "PhysicalWorkPressureClass::BackgroundCheckpoint",
        "work:lower_background_queue_lease(lease)",
    ] {
        if !demand.contains(required) {
            return Err("checkpoint demand can enter a foreign operation or foreground lane");
        }
    }
    let policy = compact(queue_policy);
    if !policy.contains(
        "BackgroundIoPressureClass::CheckpointFlush=>{FoundationalPerformanceWorkClass::AuthoritativeMutation}",
    ) {
        return Err("queue policy no longer requires authoritative checkpoint mutation evidence");
    }
    Ok(())
}

fn inspect_executor(executor: &str, checkpoint_domain: [&str; 5]) -> Result<(), &'static str> {
    if !compact(executor).contains("fndispatch_checkpoint(") {
        return Err("sole checkpoint executor entry is absent");
    }
    for effect in [
        ".write_scheduled_new_exact(",
        ".append_scheduled_artifact_exact_at(",
        ".synchronize_scheduled_file(",
        ".remove_scheduled_file_durably(",
        ".replace_scheduled(",
        ".synchronize_scheduled_directory(",
    ] {
        if executor.matches(effect).count() != 1 {
            return Err("checkpoint executor lost one exact scheduled C4 effect");
        }
    }
    if checkpoint_domain
        .iter()
        .any(|source| source.contains("artifact_tree()"))
    {
        return Err("checkpoint domain bypasses the Store executor for direct media");
    }
    Ok(())
}

fn inspect_evidence(journey: &str, ownership: &str) -> Result<(), &'static str> {
    let journey = compact(journey);
    for required in [
        "serving.checkpoints().start(",
        "CheckpointStreamDecoder::begin",
        "begin_binding_compaction(",
        "compaction.finish(",
    ] {
        if !journey.contains(required) {
            return Err("ordinary checkpoint journey lost independent publication evidence");
        }
    }
    for required in [
        "fn dispatch_checkpoint(",
        "instance/executor/checkpoint.rs",
        ".append_scheduled_artifact_exact_at(",
    ] {
        if !ownership.contains(required) {
            return Err("checkpoint executor ownership proof is absent");
        }
    }
    Ok(())
}

fn function_body<'a>(source: &'a str, signature: &str) -> Option<&'a str> {
    let start = source.find(signature)?;
    let open = source[start..].find('{')? + start;
    let mut depth = 0_u32;
    for (offset, character) in source[open..].char_indices() {
        match character {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&source[open + 1..open + offset]);
                }
            }
            _ => {}
        }
    }
    None
}

fn compact(source: &str) -> String {
    source
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect()
}

fn contains_in_order(source: &str, required: &[&str]) -> bool {
    let mut offset = 0;
    required.iter().all(|needle| {
        let Some(found) = source[offset..].find(needle) else {
            return false;
        };
        offset += found + needle.len();
        true
    })
}
