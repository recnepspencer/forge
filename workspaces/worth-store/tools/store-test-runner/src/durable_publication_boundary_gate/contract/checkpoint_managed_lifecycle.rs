use super::super::read_repository_document;

const RUNTIME_OWNER: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                             durability/checkpoint/runtime_owner.rs";
const HANDLE: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                      durability/checkpoint/handle.rs";
const LIFECYCLE: &str = "workspaces/worth-store/crates/worth-store/tests/\
                         physical_record_journeys/durability_admission/checkpoint_lifecycle.rs";
const OBSERVATION: &str =
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/\
                           durability_admission/checkpoint_lifecycle/observation.rs";

#[test]
fn checkpoint_worker_is_store_owned_finalized_and_drained() {
    inspect(&sources()).unwrap();
}

#[test]
fn checkpoint_lifecycle_gate_rejects_detachment_and_finalization_mutants() {
    let source = sources();

    let mut detached = source.clone();
    detached.runtime_owner = detached
        .runtime_owner
        .replace("lifecycle.worker = Some(worker);", "drop(worker);");
    assert!(inspect(&detached).is_err());

    let mut forgotten_current = source.clone();
    forgotten_current.runtime_owner = forgotten_current
        .runtime_owner
        .replace("lifecycle.current = Some(Arc::clone(&attempt));", "");
    assert!(inspect(&forgotten_current).is_err());

    let mut no_close_cancellation = source.clone();
    no_close_cancellation.runtime_owner = no_close_cancellation
        .runtime_owner
        .replace("attempt.mark_runtime_closing();", "");
    assert!(inspect(&no_close_cancellation).is_err());

    let mut no_close_join = source.clone();
    no_close_join.runtime_owner = no_close_join.runtime_owner.replace(
        "if let Some(worker) = worker {\n            let _ = worker.join();\n        }",
        "drop(worker);",
    );
    assert!(inspect(&no_close_join).is_err());

    let mut observer_wins = source.clone();
    observer_wins.runtime_owner = observer_wins.runtime_owner.replace(
        "owner.record_result(result, panicked);\n    attempt.complete(terminal);",
        "attempt.complete(terminal);\n    owner.record_result(result, panicked);",
    );
    assert!(inspect(&observer_wins).is_err());

    let mut unsafe_cancellation = source.clone();
    unsafe_cancellation.handle = unsafe_cancellation
        .handle
        .replace("if state.publication_started {", "if false {");
    assert!(inspect(&unsafe_cancellation).is_err());

    let mut cancellation_on_drop = source.clone();
    cancellation_on_drop
        .handle
        .push_str("\nimpl Drop for PhysicalCheckpointHandle { fn drop(&mut self) {} }\n");
    assert!(inspect(&cancellation_on_drop).is_err());

    let mut missing_disposal_evidence = source;
    missing_disposal_evidence.observation = missing_disposal_evidence.observation.replace(
        "pending_disposal_abandons_only_observation_and_close_drains_the_attempt",
        "pending_disposal_is_unproved",
    );
    assert!(inspect(&missing_disposal_evidence).is_err());
}

#[derive(Clone)]
struct LifecycleSources {
    runtime_owner: String,
    handle: String,
    lifecycle: String,
    observation: String,
}

fn sources() -> LifecycleSources {
    LifecycleSources {
        runtime_owner: read(RUNTIME_OWNER),
        handle: read(HANDLE),
        lifecycle: read(LIFECYCLE),
        observation: read(OBSERVATION),
    }
}

fn read(path: &str) -> String {
    read_repository_document(path)
        .unwrap_or_else(|error| panic!("{error}"))
        .replace("\r\n", "\n")
}

fn inspect(source: &LifecycleSources) -> Result<(), &'static str> {
    inspect_owner(&source.runtime_owner)?;
    inspect_handle(&source.handle)?;
    inspect_evidence(&source.lifecycle, &source.observation)
}

fn inspect_owner(source: &str) -> Result<(), &'static str> {
    let compact_source = compact(source);
    for required in [
        "current:Option<Arc<PhysicalCheckpointAttempt>>",
        "worker:Option<JoinHandle<()>>",
        "lifecycle.current=Some(Arc::clone(&attempt))",
        "lifecycle.worker=Some(worker)",
    ] {
        if !compact_source.contains(required) {
            return Err("checkpoint owner no longer retains the exact attempt and worker");
        }
    }
    if compact_source.contains("drop(worker)") || compact_source.contains("mem::forget(worker)") {
        return Err("checkpoint work can detach from Store lifecycle ownership");
    }

    let start = compact(function_body(source, "fn start(").ok_or("checkpoint start absent")?);
    if !contains_in_order(
        &start,
        &[
            "current.idempotency_key()==request.idempotency_key()",
            "PhysicalCheckpointHandle::new(current)",
            "!lifecycle.current_terminal",
            "PhysicalCheckpointStartDeferred::CaptureAlreadyActive",
            "lifecycle.worker.take()",
            "worker.join()",
        ],
    ) {
        return Err("checkpoint start no longer joins or reaps one serialized attempt");
    }

    let drain = compact(
        function_body(source, "fn stop_and_drain(").ok_or("checkpoint close drain absent")?,
    );
    if !contains_in_order(
        &drain,
        &[
            "lifecycle.accepting=false",
            "(lifecycle.current.clone(),lifecycle.worker.take())",
            "attempt.mark_runtime_closing()",
            "worker.join()",
            "letmutlifecycle=self.state()",
            "PhysicalCheckpointShutdown",
        ],
    ) {
        return Err("checkpoint close no longer enumerates cancels and joins owned work");
    }

    let worker = compact(
        function_body(source, "fn run_worker(").ok_or("checkpoint worker finalization absent")?,
    );
    if !contains_in_order(
        &worker,
        &[
            "letterminal=result.terminal()",
            "owner.record_result(result,panicked)",
            "attempt.complete(terminal)",
        ],
    ) {
        return Err("checkpoint caller can observe terminal fate before Store finalization");
    }
    Ok(())
}

fn inspect_handle(source: &str) -> Result<(), &'static str> {
    if compact(source).contains("implDropforPhysicalCheckpointHandle") {
        return Err("dropping a checkpoint handle can alter Store-owned work");
    }
    let cancellation = compact(
        last_function_body(source, "fn request_cancellation(")
            .ok_or("checkpoint cancellation absent")?,
    );
    if !contains_in_order(
        &cancellation,
        &[
            "state.terminal",
            "state.runtime_closing",
            "state.publication_started",
            "PhysicalCheckpointCancellationOutcome::PublicationAlreadyEffectful",
            "state.progress.request_cancellation()",
        ],
    ) {
        return Err("checkpoint cancellation can cross terminal close or publication boundaries");
    }
    let disposal =
        compact(last_function_body(source, "fn dispose(").ok_or("checkpoint disposal absent")?);
    for required in [
        "PhysicalCheckpointDisposal::ObservationAbandoned",
        "PhysicalCheckpointDisposal::Terminal",
    ] {
        if !disposal.contains(required) {
            return Err(
                "checkpoint disposal no longer distinguishes abandonment from terminal fate",
            );
        }
    }
    Ok(())
}

fn inspect_evidence(lifecycle: &str, observation: &str) -> Result<(), &'static str> {
    let evidence = compact(&format!("{lifecycle}\n{observation}"));
    for required in [
        "active_checkpoint_joins_same_key_and_defers_distinct_key_without_parallel_effects",
        "accepted_cancellation_reconciles_the_exact_candidate_through_c4_delete",
        "indeterminate_candidate_creation_requires_inspection_when_cleanup_cannot_be_proved",
        "close_drains_store_owned_checkpoint_after_the_caller_drops_its_handle",
        "pending_disposal_abandons_only_observation_and_close_drains_the_attempt",
        "terminal_poll_and_disposal_return_the_exact_store_finalized_outcome",
        "cancellation_after_publication_cutover_cannot_claim_no_effect",
    ] {
        if !evidence.contains(required) {
            return Err("managed checkpoint lifecycle evidence is incomplete");
        }
    }
    Ok(())
}

fn function_body<'a>(source: &'a str, signature: &str) -> Option<&'a str> {
    function_body_from(source, source.find(signature)?)
}

fn last_function_body<'a>(source: &'a str, signature: &str) -> Option<&'a str> {
    function_body_from(source, source.rfind(signature)?)
}

fn function_body_from(source: &str, start: usize) -> Option<&str> {
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
