use super::super::super::read_repository_document;
use super::{compact, function_body};

const TRANSITION: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                          record_serving/publication/durable_preparation/\
                          manifest_capacity_transition.rs";
const FACADE: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                      record_serving/publication/director/submission.rs";
const PREPARATION: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                           record_serving/publication/director/durable_preparation.rs";
const SCOPE: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                     record_serving/publication/durable_preparation/scope.rs";
const PREPARED: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                        record_serving/publication/durable_preparation/prepared.rs";
const WAL_PLANNING: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                            record_serving/publication/director/wal_data_planning.rs";
const RETRY: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                     durability/mutation/progression/wal_reserved.rs";
const ROOT_PROJECTION: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                              record_serving/planning/prepared_root_projection.rs";
const SETTLED_MERGE: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                            record_serving/planning/settled_root_projection.rs";
const ROOT_PREPARATION: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                               record_serving/publication/director/root_preparation.rs";
const SCALE_POLICY: &str = "workspaces/worth-store/crates/worth-store/tests/c5/\
                            scale_policy_evolution.rs";

#[test]
fn manifest_capacity_transition_is_effect_relevant_and_linearly_carried() {
    inspect(&sources()).unwrap();
}

#[test]
fn manifest_capacity_transition_gate_rejects_omission_and_hardcoding_mutants() {
    let mut omitted_identity = sources();
    omitted_identity.scope = omitted_identity.scope.replace(
        "write_field(&mut digest, &[manifest_capacity_transition.identity_code()]);",
        "let _ = manifest_capacity_transition;",
    );
    assert!(inspect(&omitted_identity).is_err());

    let mut hardcoded_facade = sources();
    hardcoded_facade.facade = hardcoded_facade.facade.replace(
        "director.prepare_durable_append(batch, placement, manifest_capacity_transition, request)",
        "director.prepare_durable_append(\n            batch,\n            placement,\n            PhysicalManifestCapacityTransition::PreserveCurrent,\n            request,\n        )",
    );
    assert!(inspect(&hardcoded_facade).is_err());

    let mut dropped_retry = sources();
    dropped_retry.retry = dropped_retry.retry.replace(
        "manifest_capacity_transition,",
        "manifest_capacity_transition: PhysicalManifestCapacityTransition::PreserveCurrent,",
    );
    assert!(inspect(&dropped_retry).is_err());

    let mut mixed_group = sources();
    mixed_group.settled_merge = mixed_group.settled_merge.replace(
        "return Err(SettledRootProjectionMergeDenial::ManifestCapacityTransitionMismatch);",
        "continue;",
    );
    assert!(inspect(&mixed_group).is_err());

    let mut hardcoded_rebase = sources();
    hardcoded_rebase.root_preparation = hardcoded_rebase.root_preparation.replace(
        "capacity_transition,",
        "capacity_transition: PhysicalManifestCapacityTransition::PreserveCurrent,",
    );
    assert!(inspect(&hardcoded_rebase).is_err());
}

#[derive(Clone)]
struct TransitionSources {
    transition: String,
    facade: String,
    preparation: String,
    scope: String,
    prepared: String,
    wal_planning: String,
    retry: String,
    root_projection: String,
    settled_merge: String,
    root_preparation: String,
    scale_policy: String,
}

fn sources() -> TransitionSources {
    TransitionSources {
        transition: read(TRANSITION),
        facade: read(FACADE),
        preparation: read(PREPARATION),
        scope: read(SCOPE),
        prepared: read(PREPARED),
        wal_planning: read(WAL_PLANNING),
        retry: read(RETRY),
        root_projection: read(ROOT_PROJECTION),
        settled_merge: read(SETTLED_MERGE),
        root_preparation: read(ROOT_PREPARATION),
        scale_policy: read(SCALE_POLICY),
    }
}

fn read(path: &str) -> String {
    read_repository_document(path).unwrap_or_else(|error| panic!("cannot read {path}: {error}"))
}

fn inspect(source: &TransitionSources) -> Result<(), &'static str> {
    inspect_vocabulary(&source.transition)?;
    inspect_facade(&source.facade)?;
    inspect_preparation(&source.preparation, &source.scope)?;
    inspect_carriage(source)?;
    inspect_scale_evidence(&source.scale_policy)?;
    Ok(())
}

fn inspect_vocabulary(source: &str) -> Result<(), &'static str> {
    for required in [
        "pub enum PhysicalManifestCapacityTransition",
        "PreserveCurrent",
        "ReconstructToRequested",
        "Self::PreserveCurrent => 1",
        "Self::ReconstructToRequested => 2",
    ] {
        if !source.contains(required) {
            return Err("manifest capacity transition vocabulary is incomplete");
        }
    }
    Ok(())
}

fn inspect_facade(source: &str) -> Result<(), &'static str> {
    let ordinary = function_body(source, "pub fn prepare_durable_append(")
        .ok_or("ordinary durable preparation facade is absent")?;
    if !ordinary.contains("PhysicalManifestCapacityTransition::PreserveCurrent") {
        return Err("ordinary preparation does not select preserve-current explicitly");
    }
    let reconstructive = function_body(
        source,
        "pub fn prepare_durable_append_with_manifest_capacity_transition(",
    )
    .ok_or("typed manifest capacity preparation facade is absent")?;
    if !compact(reconstructive).contains(
        "director.prepare_durable_append(batch,placement,manifest_capacity_transition,request)",
    ) {
        return Err("typed preparation facade hardcodes or drops the requested transition");
    }
    Ok(())
}

fn inspect_preparation(owner: &str, scope: &str) -> Result<(), &'static str> {
    for required in [
        "self.preflight_durable_append(&batch, placement, manifest_capacity_transition)",
        "record_append_scope_identity(self.format, placement, manifest_capacity_transition)",
        "manifest_capacity_transition,",
    ] {
        if !owner.contains(required) {
            return Err("preparation omitted transition validation, identity, or carriage");
        }
    }
    let scope = compact(scope);
    let capacity = scope
        .find("placement.manifest_capacity().get().to_le_bytes()")
        .ok_or("manifest capacity is absent from append identity")?;
    let transition = scope
        .find("manifest_capacity_transition.identity_code()")
        .ok_or("manifest capacity transition is absent from append identity")?;
    if transition <= capacity {
        return Err("transition identity is not encoded after placement identity");
    }
    Ok(())
}

fn inspect_carriage(source: &TransitionSources) -> Result<(), &'static str> {
    if !source
        .prepared
        .contains("manifest_capacity_transition: context.manifest_capacity_transition")
        || !source
            .prepared
            .contains("manifest_capacity_transition: parts.context.manifest_capacity_transition")
    {
        return Err("prepared mutation does not carry the transition through state changes");
    }
    if !source
        .wal_planning
        .contains("prepared.manifest_capacity_transition()")
        || !source
            .root_projection
            .contains("manifest_capacity_transition: self.manifest_capacity_transition")
    {
        return Err("WAL planning or root projection dropped the transition");
    }
    if !source
        .retry
        .contains("let manifest_capacity_transition = self.root.manifest_capacity_transition();")
        || !source.retry.contains("manifest_capacity_transition,")
    {
        return Err("proven-no-effect retry reconstruction hardcodes the transition");
    }
    if !source
        .settled_merge
        .contains("projection.manifest_capacity_transition != first.manifest_capacity_transition")
        || !source.settled_merge.contains(
            "return Err(SettledRootProjectionMergeDenial::ManifestCapacityTransitionMismatch);",
        )
    {
        return Err("settled group merge admits conflicting capacity transitions");
    }
    if !source
        .root_preparation
        .contains("let capacity_transition = prepared.manifest_capacity_transition;")
        || !source.root_preparation.contains("capacity_transition,")
    {
        return Err("root rebase hardcodes or drops the settled transition");
    }
    Ok(())
}

fn inspect_scale_evidence(source: &str) -> Result<(), &'static str> {
    if source.contains("append_batch_reconstructing_manifest_capacity")
        || source
            .matches("publish_single_with_manifest_capacity_transition(")
            .count()
            != 2
        || !source.contains("PhysicalManifestCapacityTransition::PreserveCurrent")
        || !source.contains("RecordAppendDenial::ManifestCapacityMigrationRequired")
    {
        return Err("scale policy evolution does not prove the canonical typed transition lane");
    }
    Ok(())
}
