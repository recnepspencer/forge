use super::super::read_repository_document;

const TAIL: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/\
                    checkpoint/retained_wal_tail.rs";
const SNAPSHOT: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                        durability/wal/inventory/snapshot.rs";
const CUTOVER: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                       durability/wal/checkpoint_cutover.rs";
const EXECUTION: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                         durability/checkpoint/capture/publication_cutover.rs";
const OUTCOME: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                       durability/checkpoint/outcome.rs";
const JOURNEY: &str = "workspaces/worth-store/crates/worth-store/tests/\
                       physical_record_journeys/durability_admission/\
                       checkpoint_retained_wal_tail.rs";
const ORACLE: &str = "workspaces/worth-store/crates/worth-store/tests/\
                      physical_record_journeys/durability_admission/\
                      independent_wal_oracle/segment_inventory.rs";

#[test]
fn checkpoint_retained_tail_is_nonempty_canonical_artifact_exact_and_bounded() {
    inspect(&sources()).unwrap();
}

#[test]
fn checkpoint_retained_tail_gate_rejects_topology_authority_and_bound_mutants() {
    let source = sources();
    reject_topology_mutants(&source);
    reject_authority_mutants(&source);
    reject_evidence_mutants(source);
}

fn reject_topology_mutants(source: &RetainedTailSources) {
    let mut empty = source.clone();
    empty.tail = mutate_once(
        &empty.tail,
        "NonEmpty::try_from_vec(segments)",
        "Ok(segments)",
    );
    assert!(inspect(&empty).is_err());

    let mut reordered = source.clone();
    reordered.tail = mutate_once(
        &reordered.tail,
        "CanonicalVec::try_from_sorted",
        "CanonicalVec::new",
    );
    assert!(inspect(&reordered).is_err());

    let mut artifact_gap = source.clone();
    artifact_gap.tail = mutate_once(&artifact_gap.tail, ".checked_add(1)", ".checked_add(2)");
    assert!(inspect(&artifact_gap).is_err());

    let mut generation_substitution = source.clone();
    generation_substitution.tail = mutate_once(
        &generation_substitution.tail,
        "successor.artifact.generation()!=generation",
        "successor.artifact.generation()==generation",
    );
    assert!(inspect(&generation_substitution).is_err());

    let mut lsn_gap = source.clone();
    lsn_gap.tail = mutate_once(
        &lsn_gap.tail,
        ".is_contiguous_with(successor.observed_lsn_range)",
        ".overlaps(successor.observed_lsn_range)",
    );
    assert!(inspect(&lsn_gap).is_err());

    let mut overlap = source.clone();
    overlap.tail = mutate_once(
        &overlap.tail,
        "current.observed_lsn_range.overlaps(successor.observed_lsn_range)",
        "false",
    );
    assert!(inspect(&overlap).is_err());

    let mut boundary = source.clone();
    boundary.tail = mutate_once(
        &boundary.tail,
        "require_boundary(ordered[0],checkpoint_boundary,durable_end)?;",
        "",
    );
    assert!(inspect(&boundary).is_err());

    let mut truncated = source.clone();
    truncated.tail = mutate_once(
        &truncated.tail,
        "require_final_coverage(ordered[ordered.len()-1],checkpoint_boundary,durable_end)?;",
        "",
    );
    assert!(inspect(&truncated).is_err());
}

fn reject_authority_mutants(source: &RetainedTailSources) {
    let mut copied_range = source.clone();
    copied_range.tail.push_str(
        "\nimpl RetainedWalSegment { pub fn new(artifact: WalSegmentArtifactIdentity, \
         observed_lsn_range: WalLsnRange, physical_bytes: u64) -> Self { \
         Self { artifact, observed_lsn_range, physical_bytes } } }\n",
    );
    assert!(inspect(&copied_range).is_err());

    let mut unbounded = source.clone();
    unbounded.tail = mutate_once(
        &unbounded.tail,
        "retained_physical_bytes>limit.get().get()",
        "retained_physical_bytes<limit.get().get()",
    );
    assert!(inspect(&unbounded).is_err());

    let mut detached_outcome = source.clone();
    detached_outcome.outcome = mutate_once(
        &detached_outcome.outcome,
        "dirty_records:u64,retained_wal_tail:Arc<super::ContiguousRetainedWalTail>,binding_compaction:crate::physical_runtime::PhysicalMutationBindingCompaction,wal_reclamation:crate::physical_runtime::PhysicalWalReclamationObservation,}#[derive",
        "dirty_records:u64,binding_compaction:crate::physical_runtime::PhysicalMutationBindingCompaction,wal_reclamation:crate::physical_runtime::PhysicalWalReclamationObservation,}#[derive",
    );
    assert!(inspect(&detached_outcome).is_err());

    let mut released_fence = source.clone();
    released_fence.execution = mutate_once(
        &released_fence.execution,
        "publish_under_cutover(durable,context,cutover,binding_cutover,tail,&self.reclamation,)",
        "{drop(cutover);remove_without_tail(durable,context)}",
    );
    assert!(inspect(&released_fence).is_err());

    let mut unguarded_cutover = source.clone();
    unguarded_cutover.cutover = mutate_once(
        &unguarded_cutover.cutover,
        "MutexGuard<'owner,PhysicalWalRuntimeState>",
        "PhysicalWalRuntimeState",
    );
    assert!(inspect(&unguarded_cutover).is_err());
}

fn reject_evidence_mutants(source: RetainedTailSources) {
    let mut copied_oracle = source;
    copied_oracle.journey = mutate_once(
        &copied_oracle.journey,
        "inspect_wal_inventory(store_root)",
        "submission.wal_observation()",
    );
    copied_oracle.journey = mutate_once(&copied_oracle.journey, "expected.identity()", "(2,1)");
    assert!(inspect(&copied_oracle).is_err());
}

#[derive(Clone)]
struct RetainedTailSources {
    tail: String,
    snapshot: String,
    cutover: String,
    execution: String,
    outcome: String,
    journey: String,
    oracle: String,
}

fn sources() -> RetainedTailSources {
    RetainedTailSources {
        tail: read(TAIL),
        snapshot: read(SNAPSHOT),
        cutover: read(CUTOVER),
        execution: read(EXECUTION),
        outcome: read(OUTCOME),
        journey: read(JOURNEY),
        oracle: read(ORACLE),
    }
}

fn read(path: &str) -> String {
    read_repository_document(path).unwrap_or_else(|error| panic!("{error}"))
}

fn inspect(source: &RetainedTailSources) -> Result<(), &'static str> {
    inspect_authority(&source.tail)?;
    inspect_inventory_source(&source.snapshot, &source.tail)?;
    inspect_checkpoint_join(&source.cutover, &source.execution, &source.outcome)?;
    inspect_independent_evidence(&source.journey, &source.oracle)
}

fn inspect_authority(source: &str) -> Result<(), &'static str> {
    let compact_source = compact(source);
    for forbidden in [
        "pubfnnew(artifact:WalSegmentArtifactIdentity",
        "pubfntry_from_range",
        "pubfnfrom_copied",
        "Delete",
        "Recycle",
    ] {
        if compact_source.contains(forbidden) {
            return Err("retained-tail surface exposes construction or deletion authority");
        }
    }
    for required in [
        "NonEmpty::try_from_vec(segments)",
        "NonZeroUsize::new(segments.len())",
        "CanonicalVec::try_from_sorted(segments.into_vec())",
        "require_boundary(ordered[0],checkpoint_boundary,durable_end)",
        "require_final_coverage(ordered[ordered.len()-1],checkpoint_boundary,durable_end)",
        "current.artifact.segment().get().checked_add(1)",
        "successor.artifact.generation()!=generation",
        "current.observed_lsn_range.overlaps(successor.observed_lsn_range)",
        ".is_contiguous_with(successor.observed_lsn_range)",
        "total.checked_add(segment.physical_bytes)",
        "retained_physical_bytes>limit.get().get()",
    ] {
        if !compact_source.contains(required) {
            return Err("retained-tail admission lost a required structural proof");
        }
    }
    Ok(())
}

fn inspect_inventory_source(snapshot: &str, tail: &str) -> Result<(), &'static str> {
    let snapshot = compact(snapshot);
    let tail = compact(tail);
    for required in [
        "NonEmpty::try_from_vec(self.entries.clone())",
        "durable_lsn_end",
        "segments:NonEmpty<PhysicalWalSegmentInventoryEntry>",
    ] {
        if !snapshot.contains(required) {
            return Err("WAL owner no longer supplies one immutable exact inventory snapshot");
        }
    }
    if !tail.contains("RetainedWalSegment::from_inventory")
        || !tail.contains("inventory.segments().as_slice()")
    {
        return Err("checkpoint tail can be assembled without WAL inventory facts");
    }
    Ok(())
}

fn inspect_checkpoint_join(
    cutover: &str,
    execution: &str,
    outcome: &str,
) -> Result<(), &'static str> {
    let cutover = compact(cutover);
    for required in [
        "state:MutexGuard<'owner,PhysicalWalRuntimeState>",
        "ifstate.sealed||state.durable_lsn_end.is_none()",
        "Some(PhysicalWalCheckpointCutover{state})",
        "self.state.segments.snapshot",
    ] {
        if !cutover.contains(required) {
            return Err("checkpoint cutover no longer holds the exact WAL owner fence");
        }
    }
    let admission = compact(
        function_body(execution, "fn finalize_and_publish_candidate(")
            .ok_or("checkpoint publication-cutover entry is absent")?,
    );
    if !contains_in_order(
        &admission,
        &[
            "letcutover=matchself.wal.checkpoint_cutover()",
            "self.admit_retained_tail(context.basis,&cutover)",
            "self.binding_compaction.begin_binding_compaction(",
            "if!context.attempt.begin_publication()",
            "publish_under_cutover(durable,context,cutover,binding_cutover,tail,&self.reclamation,)",
        ],
    ) {
        return Err("checkpoint publication can detach or bypass retained-tail authority");
    }
    let tail_admission = compact(
        function_body(execution, "fn admit_retained_tail(")
            .ok_or("retained-tail admission join is absent")?,
    );
    for required in [
        "ContiguousRetainedWalTail::from_inventory",
        "basis.source()",
        "&cutover.inventory_snapshot()",
        "self.checkpoint_policy.retained_wal_tail_limit()",
    ] {
        if !tail_admission.contains(required) {
            return Err("retained tail can detach from cutover inventory or exact policy");
        }
    }
    let publication = compact(
        function_body(execution, "fn publish_under_cutover(")
            .ok_or("checkpoint fenced publication owner is absent")?,
    );
    if !contains_in_order(
        &publication,
        &[
            "letreplaced=matchdurable.publish()",
            "replaced.synchronize_namespace(tail,binding_cutover)",
            "drop(cutover)",
        ],
    ) {
        return Err("WAL cutover fence does not span replacement and namespace durability");
    }
    let outcome = compact(outcome);
    for required in [
        "pubstructCompletedPhysicalCheckpoint{basis:PhysicalCheckpointCaptureBasis,footer:CheckpointStreamFooter,encoded_bytes:u64,dirty_records:u64,retained_wal_tail:Arc<super::ContiguousRetainedWalTail>,binding_compaction:crate::physical_runtime::PhysicalMutationBindingCompaction,wal_reclamation:crate::physical_runtime::PhysicalWalReclamationObservation,}",
        "pubfnretained_wal_tail(&self)->&super::ContiguousRetainedWalTail",
    ] {
        if !outcome.contains(required) {
            return Err("completed checkpoint truth no longer owns the retained tail");
        }
    }
    Ok(())
}

fn inspect_independent_evidence(journey: &str, oracle: &str) -> Result<(), &'static str> {
    let journey = compact(journey);
    for required in [
        "published_checkpoint_carries_the_exact_original_multi_rotation_wal_tail",
        "inspect_wal_inventory(store_root)",
        "independent.segment_facts()",
        "expected.identity()",
        "expected.lsn_range()",
        "expected.byte_count()",
        "tail.retained_physical_bytes()",
        "foreground_wal_cannot_cross_the_checkpoint_publication_cutover",
        "PhysicalCheckpointProgressPhase::PublicationReplacement",
        "assert!(matches!(completion.try_recv(),Err(TryRecvError::Empty)))",
    ] {
        if !journey.contains(required) {
            return Err("multi-rotation retained-tail evidence is not independently exact");
        }
    }
    let oracle = compact(oracle);
    for required in [
        "std::fs::read_dir(directory)",
        "parse_artifact_name(&name)",
        "inspect_segment(&bytes,segment,generation,last_lsn_end)",
        "Sha256::digest(payload)",
        "segment_facts.push(IndependentWalSegment",
    ] {
        if !oracle.contains(required) {
            return Err("retained-tail oracle depends on production inventory projection");
        }
    }
    Ok(())
}

fn compact(source: &str) -> String {
    source
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect()
}

fn mutate_once(source: &str, from: &str, to: &str) -> String {
    let source = compact(source);
    assert_eq!(
        source.matches(from).count(),
        1,
        "controlled mutant anchor must match exactly once: {from}"
    );
    source.replacen(from, to, 1)
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
