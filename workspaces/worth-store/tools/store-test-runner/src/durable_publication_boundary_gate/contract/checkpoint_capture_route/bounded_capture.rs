use super::{compact, contains_in_order, function_body, CheckpointRouteSources};

pub(super) fn inspect(source: &CheckpointRouteSources) -> Result<(), &'static str> {
    inspect_exact_source(&source.capture, &source.capture_execution)?;
    inspect_bounded_progression(
        &source.capture,
        &source.capture_execution,
        &source.capture_streaming,
    )?;
    inspect_allocation_observation(&source.handle, &source.progress)?;
    inspect_pressure_evidence(&source.pressure)
}

fn inspect_exact_source(capture: &str, execution: &str) -> Result<(), &'static str> {
    let admission =
        compact(function_body(capture, "fn admit(").ok_or("checkpoint source admission absent")?);
    if !contains_in_order(
        &admission,
        &[
            "letroot=publication.current_root()",
            "letwal=self.wal.checkpoint_source_range()",
            "letsession=self.frames.begin_checkpoint_capture()",
            "PhysicalCheckpointSource::concurrent(identity,wal,CheckpointRootBasis::new(root.generation(),root.tree_identity()),session.frontier().get(),)",
        ],
    ) {
        return Err("checkpoint source no longer freezes the exact root, WAL, and dirty frontier");
    }
    let completion_match = compact(
        function_body(execution, "fn capture_completion_matches_basis(")
            .ok_or("checkpoint source completion predicate absent")?,
    );
    if !completion_match.contains("completion.store_identity()==self.store")
        || !completion_match
            .contains("completion.frontier().get()==basis.source().dirty_generation_frontier()")
    {
        return Err("checkpoint completion no longer validates the admitted dirty frontier");
    }
    Ok(())
}

fn inspect_bounded_progression(
    capture: &str,
    execution: &str,
    streaming: &str,
) -> Result<(), &'static str> {
    let capture_domain = compact(&format!("{capture}\n{execution}\n{streaming}"));
    let progression = compact(
        function_body(streaming, "fn capture_dirty_generation(")
            .ok_or("checkpoint streaming owner absent")?,
    );
    for required in [
        "loop{",
        "self.next_capture_step(session)",
        "split_capture_step(step)",
        "emit_slice(&mutcandidate,&slice,attempt)",
        "DirtyCaptureContinuation::More(next)",
        "DirtyCaptureContinuation::Complete(completion)",
    ] {
        if !progression.contains(required) {
            return Err("checkpoint streaming lost bounded immediate slice progression");
        }
    }
    let next_slice = compact(
        function_body(streaming, "fn next_capture_step(")
            .ok_or("checkpoint slice allocation owner absent")?,
    );
    for required in [
        "checkpoint_capture_allocation(self.checkpoint_policy.memory_limit().get())",
        "capture_checkpoint_slice(session,allocation)",
    ] {
        if !next_slice.contains(required) {
            return Err("checkpoint streaming lost bounded slice allocation");
        }
    }
    let emission = compact(
        function_body(streaming, "fn emit_slice(")
            .ok_or("checkpoint immediate slice emission absent")?,
    );
    for required in [
        "attempt.begin_capture_allocation(slice.metadata_bytes())",
        "append_slice(candidate,slice)",
        "attempt.record_capture(candidate.dirty_records(),candidate.encoded_bytes())",
        "drop(capture_allocation)",
    ] {
        if !emission.contains(required) {
            return Err("checkpoint slice emission lost exact allocation or progress accounting");
        }
    }
    if emission.matches("append_slice(candidate,slice)").count() != 1 {
        return Err("checkpoint slice emission has competing append paths");
    }
    for forbidden in [
        "Vec<",
        "Vec::",
        "vec![",
        "VecDeque",
        "LinkedList",
        "HashMap",
        "BTreeMap",
        "HashSet",
        "BTreeSet",
        "SmallVec",
        "ArrayVec",
        ".collect",
        ".push(",
        ".extend(",
        "FromIterator",
        "from_iter(",
    ] {
        if capture_domain.contains(forbidden) {
            return Err("checkpoint capture materializes the dirty source");
        }
    }
    Ok(())
}

fn inspect_allocation_observation(handle: &str, progress: &str) -> Result<(), &'static str> {
    let begin = compact(
        function_body(handle, "fn begin_capture_allocation(")
            .ok_or("checkpoint allocation guard absent")?,
    );
    let drop = compact(
        function_body(handle, "fn drop(").ok_or("checkpoint allocation guard cleanup absent")?,
    );
    for required in [
        "state.progress.begin_capture_allocation(bytes)",
        "PhysicalCheckpointCaptureAllocation{attempt:self}",
    ] {
        if !begin.contains(required) {
            return Err("checkpoint handle no longer begins exact allocation observation");
        }
    }
    if !drop.contains("state.progress.end_capture_allocation()") {
        return Err("checkpoint allocation observation is not cleared by scope exit");
    }
    let progress = compact(progress);
    for required in [
        "current_capture_bytes:u64",
        "peak_capture_bytes:u64",
        "self.peak_capture_bytes=self.peak_capture_bytes.max(bytes)",
        "self.current_capture_bytes=0",
    ] {
        if !progress.contains(required) {
            return Err("checkpoint progress lost current or peak bounded resource use");
        }
    }
    Ok(())
}

fn inspect_pressure_evidence(pressure: &str) -> Result<(), &'static str> {
    let pressure = compact(pressure);
    for required in [
        "resident_dirty_frames*32",
        "final_foreground_lsn>frozen_source.wal().covered_end_lsn_exclusive()",
        "during.current_capture_bytes()",
        "during.peak_capture_bytes()",
        "completed.basis().source(),frozen_source",
        "CheckpointStreamDecoder::begin",
        "assert_checkpoint_io(",
    ] {
        if !pressure.contains(required) {
            return Err("whole-Store checkpoint pressure evidence is incomplete");
        }
    }
    Ok(())
}

#[test]
fn bounded_capture_gate_rejects_materialization_and_accounting_mutants() {
    let source = super::sources();

    let mut unbounded = source.clone();
    unbounded.capture_streaming = unbounded
        .capture_streaming
        .replace("self.checkpoint_policy.memory_limit().get()", "u64::MAX");
    assert!(inspect(&unbounded).is_err());

    let mut false_accounting = source.clone();
    false_accounting.capture_streaming = false_accounting
        .capture_streaming
        .replace("slice.metadata_bytes()", "0");
    assert!(inspect(&false_accounting).is_err());

    let mut leaked_accounting = source.clone();
    leaked_accounting.handle = leaked_accounting
        .handle
        .replace("state.progress.end_capture_allocation();", "");
    assert!(inspect(&leaked_accounting).is_err());

    let mut substituted_frontier = source.clone();
    substituted_frontier.capture_execution = substituted_frontier.capture_execution.replace(
        "basis.source().dirty_generation_frontier()",
        "completion.frontier().get()",
    );
    assert!(inspect(&substituted_frontier).is_err());

    let mut materialized = source.clone();
    materialized
        .capture_streaming
        .push_str("\nfn materialize() { source.collect::<Vec<_>>(); }\n");
    assert!(inspect(&materialized).is_err());

    let mut hidden_accumulation = source.clone();
    hidden_accumulation
        .capture
        .push_str("\nfn accumulate(frame: Frame) { let mut all = Vec::new(); all.push(frame); }\n");
    assert!(inspect(&hidden_accumulation).is_err());

    let mut deferred_slice = source.clone();
    deferred_slice.capture_streaming =
        deferred_slice
            .capture_streaming
            .replacen("append_slice(candidate, slice)", "Ok(())", 1);
    assert!(inspect(&deferred_slice).is_err());

    let mut weak_pressure = source;
    weak_pressure.pressure = weak_pressure
        .pressure
        .replace("resident_dirty_frames * 32", "resident_dirty_frames");
    assert!(inspect(&weak_pressure).is_err());
}
