mod owner_bodies;
mod parsed_owner;
mod source_files;

use owner_bodies::{planner_owner_bodies, RUNTIME_OWNER_BODIES};
use parsed_owner::{
    assert_owner_body_changed, assert_source_file_changed, owner_body_digest, source_file_digest,
};
use source_files::SOURCE_FILES;

#[test]
fn phase_1_inventory_freezes_complete_writer_and_publication_owners() {
    assert_eq!(owner_bodies().count(), 51);
    assert_eq!(
        owner_bodies()
            .map(|owner| (owner.responsibility, owner_body_digest(owner, owner.source)))
            .collect::<Vec<_>>(),
        owner_bodies()
            .map(|owner| (owner.responsibility, owner.expected_digest.to_owned()))
            .collect::<Vec<_>>()
    );
}

#[test]
fn phase_1_inventory_freezes_complete_critical_source_files() {
    assert_eq!(SOURCE_FILES.len(), 38);
    assert_eq!(
        SOURCE_FILES
            .iter()
            .map(|source| (
                source.source_path,
                source_file_digest(source, source.source)
            ))
            .collect::<Vec<_>>(),
        SOURCE_FILES
            .iter()
            .map(|source| (source.source_path, source.expected_digest.to_owned()))
            .collect::<Vec<_>>()
    );
}

#[test]
fn phase_1_inventory_rejects_comment_decoy_and_writer_misrouting() {
    let owner = owner("routing counter projection");
    let mutated = owner.source.replace(
        "telemetry.invalidation.frontier_seed_count += counters.frontier_seed_count;",
        "telemetry.invalidation.frontier_group_count += counters.frontier_seed_count;\n/* telemetry.invalidation.frontier_seed_count += counters.frontier_seed_count; */",
    );
    assert_owner_body_changed(owner, &mutated);
}

#[test]
fn phase_1_inventory_rejects_dead_branch_publication_bypass() {
    let owner = owner("atomic output publication");
    let statement = "self.publish_direct_output_causes(direct_causes)";
    let mutated = owner.source.replace(
        statement,
        &format!("if false {{ {statement} }} else {{ Ok(()) }}"),
    );
    assert_owner_body_changed(owner, &mutated);
}

#[test]
fn phase_1_inventory_rejects_ledger_key_and_argument_misrouting() {
    let owner = owner("output commit ledger publication");
    let mutated = owner.source.replace(
        ".insert(delta.output_commit_ordinal.0, delta);",
        ".insert(delta.output_commit_ordinal.0.saturating_add(1), delta);",
    );
    assert_owner_body_changed(owner, &mutated);
}

#[test]
fn phase_1_inventory_rejects_cause_writer_deletion() {
    let owner = owner("canonical cause slot write");
    let mutated = owner.source.replace(
        "let previous = std::mem::replace(&mut self.sets[index], causes);",
        "let previous = causes;",
    );
    assert_owner_body_changed(owner, &mutated);
}

#[test]
fn phase_1_inventory_rejects_leaf_cache_or_lifecycle_writer_deletion() {
    let cause_application = source_file("data/graph/storage/invalidation_causes/application.rs");
    let without_cache_rebuild = cause_application.source.replacen(
        "self.rebuild_dirty_caches_from_pending_causes(node)?;",
        "",
        1,
    );
    assert_source_file_changed(cause_application, &without_cache_rebuild);

    let effect = source_file("data/graph/runtime/effect.rs");
    let without_clean = effect
        .source
        .replace("self.transition_node_clean(node)?;", "");
    assert_source_file_changed(effect, &without_clean);
}

#[test]
fn phase_1_inventory_rejects_serial_or_parallel_promotion_bypass() {
    let serial = owner("serial output promotion entry");
    let serial_mutation = serial.source.replace(
        "self.publish_output_commit_packet(packet)",
        "unreachable!()",
    );
    assert_owner_body_changed(serial, &serial_mutation);

    let parallel = owner("parallel output promotion entry");
    let parallel_mutation = parallel.source.replace(
        "self.publish_output_commit_packet(packet)",
        "unreachable!()",
    );
    assert_owner_body_changed(parallel, &parallel_mutation);

    let parallel_caller = owner("parallel task output publication");
    let normalized = parallel_caller.source.replace("\r\n", "\n");
    let call = "graph\n        .publish_prepared_parallel_apply_commit_packet(commit_packet, comparator_resolver)";
    assert!(normalized.contains(call));
    let caller_mutation = normalized.replace(
        call,
        &format!("(if false {{ {call} }} else {{ unreachable!() }})"),
    );
    assert_owner_body_changed(parallel_caller, &caller_mutation);
}

fn owner(responsibility: &str) -> &'static owner_bodies::OwnerBody {
    owner_bodies()
        .find(|owner| owner.responsibility == responsibility)
        .expect("named Phase 1 owner must be inventoried")
}

fn owner_bodies() -> impl Iterator<Item = &'static owner_bodies::OwnerBody> {
    RUNTIME_OWNER_BODIES
        .iter()
        .chain(planner_owner_bodies().iter())
}

fn source_file(path: &str) -> &'static source_files::SourceFile {
    SOURCE_FILES
        .iter()
        .find(|source| source.source_path == path)
        .expect("named Phase 1 source file must be inventoried")
}
