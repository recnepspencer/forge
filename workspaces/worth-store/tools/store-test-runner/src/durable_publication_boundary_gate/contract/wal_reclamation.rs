use super::super::read_repository_document;

const ELIGIBILITY: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                           durability/wal/reclamation/eligibility.rs";
const AUTHORITY: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                         durability/wal/reclamation/authority.rs";
const EXECUTION: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                         durability/wal/reclamation/execution.rs";
const TRANSITION: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                          durability/wal/reclamation/inventory_transition.rs";
const WORK_PORT: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                         durability/wal/reclamation/work_port.rs";
const PUBLICATION: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                           durability/checkpoint/capture/publication_cutover.rs";
const TYPESTATE: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                         durability/checkpoint/publication.rs";
const EXECUTOR: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                        instance/executor/wal_reclamation.rs";
const SCHEDULER: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                         instance/scheduler_admission/reclamation.rs";
const SEMANTICS: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                         record_serving/work_semantics/durability/wal_reclamation_basis.rs";
const RECOVERY: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                        work/recovery/locator/codec.rs";
const REOPEN: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                      durability/wal/inventory/reopen.rs";
const JOURNEY: &str = "workspaces/worth-store/crates/worth-store/tests/\
                       physical_record_journeys/durability_admission/\
                       checkpoint_wal_reclamation.rs";

#[test]
fn wal_reclamation_requires_the_complete_durable_authority_join() {
    inspect(&sources()).unwrap();
}

#[test]
fn wal_reclamation_gate_rejects_partial_authority_and_effect_mutants() {
    let source = sources();
    reject_authority_mutants(&source);
    reject_effect_mutants(&source);
    reject_failure_and_reopen_mutants(source);
}

fn reject_authority_mutants(source: &WalReclamationSources) {
    let mut checkpoint_only = source.clone();
    checkpoint_only.eligibility = mutate_once(
        &checkpoint_only.eligibility,
        "if tail.checkpoint_identity() != checkpoint\
         || compaction.checkpoint_identity() != checkpoint",
        "if false",
    );
    assert!(inspect(&checkpoint_only).is_err());

    let mut cutoff_only = source.clone();
    cutoff_only.eligibility = mutate_once(
        &cutoff_only.eligibility,
        "require_retained_suffix(entries, retained_index, tail.segments())?;",
        "",
    );
    assert!(inspect(&cutoff_only).is_err());

    let mut tail_only = source.clone();
    tail_only.eligibility = mutate_once(
        &tail_only.eligibility,
        "if compaction.wal_cutoff_lsn_exclusive() < checkpoint_boundary.get()",
        "if false",
    );
    assert!(inspect(&tail_only).is_err());

    let mut crossing = source.clone();
    crossing.eligibility = mutate_once(
        &crossing.eligibility,
        ".any(|entry| entry.lsn_range().end_exclusive() > checkpoint_boundary)",
        ".any(|_| false)",
    );
    assert!(inspect(&crossing).is_err());
}

fn reject_effect_mutants(source: &WalReclamationSources) {
    let mut direct_inventory = source.clone();
    direct_inventory.transition = mutate_once(
        &direct_inventory.transition,
        "if completed.segment() != expected.identity()",
        "if false",
    );
    assert!(inspect(&direct_inventory).is_err());

    let mut unordered_transition = source.clone();
    unordered_transition.transition = mutate_once(
        &unordered_transition.transition,
        "state.segments.consume_reclaimed_head(expected)",
        "state.segments.consume_any(expected)",
    );
    assert!(inspect(&unordered_transition).is_err());

    let mut unscheduled_delete = source.clone();
    unscheduled_delete.executor = mutate_once(
        &unscheduled_delete.executor,
        ".remove_scheduled_file_durably(",
        ".remove_file(",
    );
    assert!(inspect(&unscheduled_delete).is_err());

    let mut unbound_recovery = source.clone();
    unbound_recovery.executor = mutate_once(
        &unbound_recovery.executor,
        "PhysicalWorkRecoveryTarget::WalSegmentReclamation",
        "PhysicalWorkRecoveryTarget::RecordNamespaceSynchronization",
    );
    assert!(inspect(&unbound_recovery).is_err());

    let mut skipped_completion = source.clone();
    skipped_completion.publication = mutate_once(
        &skipped_completion.publication,
        "publication.with_wal_reclamation(observation)",
        "publication.complete_without_reclamation()",
    );
    assert!(inspect(&skipped_completion).is_err());
}

fn reject_failure_and_reopen_mutants(source: WalReclamationSources) {
    let mut possible_effect_is_retryable = source.clone();
    possible_effect_is_retryable.execution = mutate_once(
        &possible_effect_is_retryable.execution,
        "Self::EffectRequiresInspection | Self::StaleOrForeignSettlement",
        "Self::StaleOrForeignSettlement",
    );
    assert!(inspect(&possible_effect_is_retryable).is_err());

    let mut denied_consumes_truth = source.clone();
    denied_consumes_truth.execution = mutate_once(
        &denied_consumes_truth.execution,
        "PhysicalWalReclamationObservation::DeferredBeforeEffect",
        "PhysicalWalReclamationObservation::Reclaimed",
    );
    assert!(inspect(&denied_consumes_truth).is_err());
}

#[derive(Clone)]
struct WalReclamationSources {
    authority: String,
    eligibility: String,
    execution: String,
    transition: String,
    work_port: String,
    publication: String,
    typestate: String,
    executor: String,
    scheduler: String,
    semantics: String,
    recovery: String,
    reopen: String,
    journey: String,
}

fn sources() -> WalReclamationSources {
    WalReclamationSources {
        authority: read(AUTHORITY),
        eligibility: read(ELIGIBILITY),
        execution: read(EXECUTION),
        transition: read(TRANSITION),
        work_port: read(WORK_PORT),
        publication: read(PUBLICATION),
        typestate: read(TYPESTATE),
        executor: read(EXECUTOR),
        scheduler: read(SCHEDULER),
        semantics: read(SEMANTICS),
        recovery: read(RECOVERY),
        reopen: read(REOPEN),
        journey: read(JOURNEY),
    }
}

fn read(path: &str) -> String {
    read_repository_document(path).unwrap_or_else(|error| panic!("{error}"))
}

fn inspect(source: &WalReclamationSources) -> Result<(), &'static str> {
    require_all(
        &source.authority,
        &[
            "struct ProvenNoLiveBindingLastCopyObligation",
            "last_copy: ProvenNoLiveBindingLastCopyObligation",
            "segments: NonEmpty<EligiblePhysicalWalSegmentReclamation>",
            "last_copy: ProvenNoLiveBindingLastCopyObligation {",
        ],
        "eligible deletion no longer carries a per-segment last-copy classification",
    )?;
    require_all(
        &source.eligibility,
        &[
            "publication: &NamespaceDurableCheckpointPublication",
            "tail.checkpoint_identity() != checkpoint",
            "compaction.checkpoint_identity() != checkpoint",
            "compaction.wal_cutoff_lsn_exclusive() < checkpoint_boundary.get()",
            "entry.identity() == first_retained.artifact()",
            "entry.lsn_range() == first_retained.observed_lsn_range()",
            "entry.byte_count() == first_retained.physical_bytes()",
            "require_retained_suffix(entries, retained_index, tail.segments())?",
            "let candidates = &entries[..retained_index]",
            ".any(|entry| entry.lsn_range().end_exclusive() > checkpoint_boundary)",
            "EligiblePhysicalWalReclamation::new(",
        ],
        "reclamation eligibility lost part of the durable authority join",
    )?;
    require_all(
        &source.publication,
        &[
            "replaced.synchronize_namespace(tail, binding_cutover)",
            "let plan = cutover.reclamation_plan(&publication)",
            "drop(cutover)",
            "reclamation.execute(plan)",
            "publication.with_wal_reclamation(observation)",
        ],
        "checkpoint completion can bypass or predate reclamation observation",
    )?;
    require_all(
        &source.typestate,
        &[
            "struct NamespaceDurableCheckpointPublication",
            "fn with_wal_reclamation(",
            ") -> PhysicalCheckpointPublication",
        ],
        "checkpoint typestate no longer requires reclamation settlement",
    )?;
    inspect_effect_path(source)?;
    inspect_failure_path(source)?;
    inspect_route_and_recovery(source)?;
    inspect_reopen_and_evidence(source)
}

fn inspect_effect_path(source: &WalReclamationSources) -> Result<(), &'static str> {
    require_all(
        &source.execution,
        &[
            "match self.execute_segment(eligible)",
            "Ok(completed) if self.wal.complete_reclamation(expected, &completed)",
            "PhysicalWalReclamationObservation::DeferredBeforeEffect",
            "PhysicalWalReclamationObservation::InspectionRequired",
            "Self::EffectRequiresInspection | Self::StaleOrForeignSettlement",
        ],
        "inventory truth can move without an exact completed effect",
    )?;
    require_all(
        &source.transition,
        &[
            "completed: &CompletedPhysicalWalReclamationAction",
            "completed.segment() != expected.identity()",
            "completed.lsn_range() != expected.lsn_range()",
            "completed.byte_count() != expected.byte_count()",
            "state.segments.consume_reclaimed_head(expected)",
            "state.sealed = true",
        ],
        "reclamation completion no longer performs an exact-head transition",
    )?;
    require_all(
        &source.executor,
        &[
            "PhysicalWorkRecoveryTarget::WalSegmentReclamation",
            "ArtifactTreeDirectory::families()",
            ".child(\"wal\")",
            ".file(&scope.segment().file_name())",
            ".remove_scheduled_file_durably(",
            "ScheduledArtifactTreePublicationEffectOutcome::Indeterminate",
            "PhysicalEffectRecoveryObligation::Retained",
        ],
        "WAL deletion escaped the scheduled executor or exact recovery target",
    )
}

fn inspect_failure_path(source: &WalReclamationSources) -> Result<(), &'static str> {
    require_all(
        &source.journey,
        &[
            "reclaimed_wal_prefix_remains_absent_after_fresh_process_reopen",
            "denied_reclamation_preserves_the_exact_live_inventory_for_retry",
            "second_delete_denial_preserves_the_exact_partially_reclaimed_prefix",
            "indeterminate_reclamation_seals_without_consuming_inventory_truth",
            "MediaFaultDirective::FailBefore",
            "MediaFaultDirective::IndeterminateAfterEffect",
            "first_delete_gate.wait_until_reached()",
            "fail_activation.arm().unwrap()",
            "first_delete_gate.release()",
            "inspect_wal_inventory(&store_root)",
            "assert_eq!(report.planned_segments(), 2)",
            "assert_eq!(report.reclaimed_segments(), 1)",
            "assert_eq!(wal.active_segment_count(), 3)",
            "PhysicalMutationAdmissionDisposition::DuplicateUnresolved",
            "assert_eq!(replay.mutation_identity(), base_identity)",
        ],
        "reclamation lacks fresh-process and terminal-fate evidence",
    )
}

fn inspect_route_and_recovery(source: &WalReclamationSources) -> Result<(), &'static str> {
    require_all(
        &source.work_port,
        &[
            "PhysicalMutationWorkRequest::wal_reclamation(",
            "PhysicalWorkReadiness::Ready(ready)",
            ".wal_reclamation_background(",
            "PhysicalSchedulerDemand::wal_reclamation_background(ready, lease)",
            "PhysicalExecutorCommand::wal_reclamation(work)",
        ],
        "WAL reclamation bypasses the ordinary work route",
    )?;
    require_all(
        &source.scheduler,
        &[
            "admit_wal_reclamation_background_policy(",
            "admit_background_capacity(",
            "admit_background_pacing(",
            "with_foreground_pressure_events(foreground_pressure_events)",
        ],
        "reclamation lost background pacing or foreground preservation",
    )?;
    require_all(
        &source.semantics,
        &[
            "store.physical.durability.wal-reclamation-basis",
            "wal-reclamation-work-admitted",
            "PhysicalWorkSignalFamily::WalReclamation",
            "dependency_and_output_declaration(",
        ],
        "reclamation lost its dedicated semantic signal",
    )?;
    require_all(
        &source.recovery,
        &[
            "const TARGET_WAL_RECLAMATION: u8 = 8",
            "PhysicalWorkOperationFamily::WalReclamation => 8",
            "PhysicalWorkRecoveryTarget::WalSegmentReclamation",
            "segment: first",
            "generation: second",
        ],
        "reclamation recovery cannot identify the exact segment artifact",
    )
}

fn inspect_reopen_and_evidence(source: &WalReclamationSources) -> Result<(), &'static str> {
    super::wal_reopen_origin::inspect(&source.reopen)
        .map_err(|_| "WAL reopen lost its canonical-origin classification")?;
    require_all(
        &source.reopen,
        &[
            "require_checkpoint_cutoff_within_retained_wal(cutoff, &segment_inventory, active_lsn_end)?",
            "if cutoff < first || cutoff > active_lsn_end",
        ],
        "reopen can trust a reclaimed inventory without checkpoint cutoff authority",
    )
}

fn require_all(source: &str, required: &[&str], denial: &'static str) -> Result<(), &'static str> {
    let source = compact(source);
    required
        .iter()
        .all(|needle| source.contains(&compact(needle)))
        .then_some(())
        .ok_or(denial)
}

fn compact(source: &str) -> String {
    source
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect()
}

fn mutate_once(source: &str, from: &str, to: &str) -> String {
    let source = compact(source);
    let from = compact(from);
    assert_eq!(source.matches(&from).count(), 1, "mutant anchor: {from}");
    source.replacen(&from, &compact(to), 1)
}
