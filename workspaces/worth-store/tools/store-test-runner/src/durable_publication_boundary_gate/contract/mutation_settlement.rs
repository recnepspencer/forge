use super::super::read_repository_document;

const OWNER: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/lifecycle/managed_work.rs";
const HANDLE: &str =
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/mutation/handle.rs";
const COMPLETED: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/mutation/progression/completed.rs";
const ACKNOWLEDGMENT: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/settlement/acknowledgment.rs";
const PROVEN_NO_EFFECT: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/settlement/proven_no_effect.rs";
const INDETERMINATE: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/settlement/indeterminate.rs";
const JOURNEY: &str = "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/durability_admission/managed_mutation.rs";
const DROP_MATRIX: &str = "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/durability_admission/managed_mutation/drop_boundaries.rs";
const CANCELLATION_MATRIX: &str = "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/durability_admission/managed_mutation/cancellation_boundaries.rs";
const UI_HARNESS: &str =
    "workspaces/worth-store/crates/worth-store/tests/physical_runtime_authority_ui.rs";

#[test]
fn mutation_settlement_is_store_owned_completion_only_and_observation_safe() {
    inspect(&sources()).unwrap();
}

#[test]
fn mutation_settlement_gate_rejects_lifecycle_acknowledgment_and_evidence_mutants() {
    let source = sources();

    let mut detached = source.clone();
    detached.owner = detached
        .owner
        .replace("let _ = worker.join();", "drop(worker);");
    assert!(inspect(&detached).is_err());

    let mut notified_early = source.clone();
    notified_early.owner = notified_early.owner.replace(
        "state.counters.record_terminal(terminal_class, panicked);",
        "attempt.publish_terminal();\n        state.counters.record_terminal(terminal_class, panicked);",
    );
    assert!(inspect(&notified_early).is_err());

    let mut drop_cancels = source.clone();
    drop_cancels.handle = drop_cancels.handle.replace(
        "self.attempt.release_observer(self.observed_terminal.get());",
        "let _ = self.request_cancellation();",
    );
    assert!(inspect(&drop_cancels).is_err());

    let mut public_acknowledgment_mint = source.clone();
    public_acknowledgment_mint.acknowledgment = public_acknowledgment_mint.acknowledgment.replace(
        "pub(in crate::physical_runtime) fn from_completed",
        "pub fn from_completed",
    );
    assert!(inspect(&public_acknowledgment_mint).is_err());

    let mut missing_drop_matrix = source.clone();
    missing_drop_matrix.drop_matrix = missing_drop_matrix.drop_matrix.replace(
        "dropping_the_only_handle_never_cancels_at_any_effect_boundary",
        "drop_matrix_removed",
    );
    assert!(inspect(&missing_drop_matrix).is_err());

    let mut missing_reverse_attack = source;
    missing_reverse_attack.ui_harness = missing_reverse_attack.ui_harness.replace(
        "mutation_evidence_cannot_reenter_authority",
        "reverse_attack_removed",
    );
    assert!(inspect(&missing_reverse_attack).is_err());
}

#[derive(Clone)]
struct MutationSettlementSources {
    owner: String,
    handle: String,
    completed: String,
    acknowledgment: String,
    proven_no_effect: String,
    indeterminate: String,
    evidence: String,
    drop_matrix: String,
    cancellation_matrix: String,
    ui_harness: String,
}

fn sources() -> MutationSettlementSources {
    MutationSettlementSources {
        owner: read(OWNER),
        handle: read(HANDLE),
        completed: read(COMPLETED),
        acknowledgment: read(ACKNOWLEDGMENT),
        proven_no_effect: read(PROVEN_NO_EFFECT),
        indeterminate: read(INDETERMINATE),
        evidence: read(JOURNEY),
        drop_matrix: read(DROP_MATRIX),
        cancellation_matrix: read(CANCELLATION_MATRIX),
        ui_harness: read(UI_HARNESS),
    }
}

fn read(path: &str) -> String {
    read_repository_document(path).unwrap_or_else(|error| panic!("{error}"))
}

fn inspect(source: &MutationSettlementSources) -> Result<(), &'static str> {
    let owner = compact(&source.owner);
    for required in [
        "state.accepting=false",
        "attempt.mark_runtime_closing()",
        "worker.join()",
    ] {
        if !owner.contains(required) {
            return Err("mutation close can abandon Store-owned work");
        }
    }
    let finalization = source
        .owner
        .split_once("fn record_terminal(")
        .and_then(|(_, body)| body.split_once("fn settle_without_worker("))
        .map(|(body, _)| compact(body))
        .ok_or("mutation terminal finalization owner absent")?;
    if !contains_in_order(
        &finalization,
        &[
            ".persist_mutation_terminal(&terminal)",
            "attempt.install_terminal(terminal)",
            "state.counters.record_terminal(terminal_class,panicked)",
            "attempt.publish_terminal()",
        ],
    ) || finalization.find("attempt.publish_terminal()")
        < finalization.find("state.counters.record_terminal(terminal_class,panicked)")
    {
        return Err("waiters can observe mutation fate before Store finalization");
    }

    let drop = source
        .handle
        .split_once("impl Drop for PhysicalMutationHandle")
        .and_then(|(_, body)| body.split_once("impl PhysicalMutationAttempt"))
        .map(|(body, _)| compact(body))
        .ok_or("mutation handle drop contract absent")?;
    if !drop.contains("release_observer(self.observed_terminal.get())")
        || drop.contains("request_cancellation")
    {
        return Err("dropping a mutation handle changes physical fate");
    }

    if !source.completed.contains("pub fn into_acknowledgment")
        || source.proven_no_effect.contains("into_acknowledgment")
        || source.indeterminate.contains("into_acknowledgment")
        || !source
            .acknowledgment
            .contains("pub(in crate::physical_runtime) fn from_completed")
    {
        return Err("physical acknowledgment is not completion-only");
    }

    let evidence = compact(&format!(
        "{}\n{}\n{}\n{}",
        source.evidence, source.drop_matrix, source.cancellation_matrix, source.ui_harness
    ));
    for required in [
        "managed_mutation_completion_is_the_only_acknowledgment_source",
        "dropping_the_only_handle_never_cancels_at_any_effect_boundary",
        "close_drains_dropped_handle_work_at_every_effect_boundary",
        "cancellation_after_each_possible_effect_boundary_remains_effectful",
        "deadline_elapsed_after_group_seal_cannot_rewrite_effectful_fate",
        "mutation_evidence_cannot_reenter_authority",
        "noncompleted_mutation_cannot_acknowledge",
        "ordinary_mutation_phase_driving_is_absent",
        "ordinary_mutation_outcomes_are_supported",
    ] {
        if !evidence.contains(required) {
            return Err("Phase 8 mutation settlement evidence is incomplete");
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

fn contains_in_order(source: &str, expected: &[&str]) -> bool {
    let mut remainder = source;
    for item in expected {
        let Some(position) = remainder.find(item) else {
            return false;
        };
        remainder = &remainder[position + item.len()..];
    }
    true
}
