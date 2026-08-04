use super::super::read_repository_document;

const REGISTRY: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                        durability/mutation/idempotency/registry.rs";
const GROUP_PORT: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                          durability/wal/port/group.rs";
const OBSERVATION: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                           durability/grouping/observation.rs";
const JOURNEY: &str = "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/\
                       durability_admission/group_commit.rs";

#[test]
fn group_commit_preserves_terminal_fate_atomic_sealing_and_stage_exact_amplification() {
    inspect(&sources()).unwrap();
}

#[test]
fn group_commit_gate_rejects_seal_cancellation_and_amplification_mutants() {
    let source = sources();

    let mut seal_bypass = source.clone();
    seal_bypass.group_port = seal_bypass
        .group_port
        .replace("self.idempotency.seal_group(&sealing_bindings)", "Ok(())");
    assert!(inspect(&seal_bypass).is_err());

    let mut partial_validation = source.clone();
    partial_validation.registry = partial_validation.registry.replacen(
        "for expected in expected {",
        "for expected in expected.iter().take(1) {",
        1,
    );
    assert!(inspect(&partial_validation).is_err());

    let mut cancellation_after_seal = source.clone();
    cancellation_after_seal.registry = cancellation_after_seal.registry.replace(
        "Err(PhysicalMutationPreSealCancellationDenial::GroupSealed)",
        "Ok(terminal)",
    );
    assert!(inspect(&cancellation_after_seal).is_err());

    let mut collapsed_root_plan = source;
    collapsed_root_plan.observation = collapsed_root_plan
        .observation
        .replace("PhysicalGroupRootPublicationPlan::Shared(_) => 1", "_ => 0");
    assert!(inspect(&collapsed_root_plan).is_err());
}

#[derive(Clone)]
struct GroupCommitSources {
    registry: String,
    group_port: String,
    observation: String,
    journey: String,
}

fn sources() -> GroupCommitSources {
    GroupCommitSources {
        registry: read_repository_document(REGISTRY).expect("read idempotency registry"),
        group_port: read_repository_document(GROUP_PORT).expect("read group WAL port"),
        observation: read_repository_document(OBSERVATION)
            .expect("read group amplification observation"),
        journey: read_repository_document(JOURNEY).expect("read group commit journey"),
    }
}

fn inspect(source: &GroupCommitSources) -> Result<(), &'static str> {
    inspect_registry(&source.registry)?;
    inspect_append_order(&source.group_port)?;
    inspect_amplification(&source.observation)?;
    inspect_journey(&source.journey)?;
    Ok(())
}

fn inspect_registry(source: &str) -> Result<(), &'static str> {
    let cancellation = compact(
        function_body(source, "fn cancel_before_group_seal(")
            .ok_or("pre-seal cancellation transition is absent")?,
    );
    for required in [
        "PhysicalMutationIdempotencyBindingState::Unsealed(basis)ifbasis.observation()==expected",
        "*state=PhysicalMutationIdempotencyBindingState::Terminal{basis:basis.clone(),fate:PersistedPhysicalMutationFate::proven_no_effect(terminal),last_compacted:None,}",
        "PhysicalMutationIdempotencyBindingState::GroupSealed{basis,..}ifbasis.observation()==expected",
        "Err(PhysicalMutationPreSealCancellationDenial::GroupSealed)",
        "PhysicalMutationIdempotencyBindingState::RebuiltUnresolved{basis,..}ifbasis.observation()==expected",
        "Err(PhysicalMutationPreSealCancellationDenial::ReopenedUnresolved)",
        "PhysicalMutationIdempotencyBindingState::WalBound{basis,..}ifbasis.observation()==expected",
        "PhysicalMutationIdempotencyBindingState::Terminal{basis,fate,..}ifbasis.observation()==expected",
        "fate.as_proven_no_effect().ok_or(PhysicalMutationPreSealCancellationDenial::GroupSealed)",
    ] {
        if !cancellation.contains(required) {
            return Err("pre-seal cancellation lost exact terminal or sealed-state behavior");
        }
    }

    let sealing =
        compact(function_body(source, "fn seal_group(").ok_or("atomic group sealing is absent")?);
    if sealing.matches("forexpectedinexpected{").count() != 2 {
        return Err("group sealing must validate every member before mutating every member");
    }
    for required in [
        "PhysicalMutationIdempotencyBindingState::Unsealed(basis)",
        "basis.observation()==observation",
        "PhysicalMutationIdempotencyBindingState::GroupSealed{basis,group}",
        "*group==expected.group()",
        "PhysicalMutationIdempotencyGroupSealDenial::AlreadyGroupSealed",
        "PhysicalMutationIdempotencyBindingState::RebuiltUnresolved",
        "PhysicalMutationIdempotencyGroupSealDenial::ReopenedUnresolved",
        "PhysicalMutationIdempotencyBindingState::Terminal",
        "PhysicalMutationIdempotencyGroupSealDenial::ProvenNoEffect",
        ".get_mut(&observation.key())",
        "*state=PhysicalMutationIdempotencyBindingState::GroupSealed{basis:basis.clone(),group:expected.group(),}",
    ] {
        if !sealing.contains(required) {
            return Err("group sealing lost all-member validation or terminal fencing");
        }
    }
    Ok(())
}

fn inspect_append_order(source: &str) -> Result<(), &'static str> {
    let append = compact(
        function_body(source, "fn append_prepared_group(").ok_or("group append entry is absent")?,
    );
    if !contains_in_order(
        &append,
        &[
            ".grouping.admit(",
            "letsealing_bindings=admitted.idempotency_sealing_bindings()",
            "self.idempotency.seal_group(&sealing_bindings)",
            "let(basis,members)=admitted.into_parts()",
            "self.reserve_and_append_group(",
        ],
    ) {
        return Err("group append can reserve or append WAL before atomic idempotency sealing");
    }
    Ok(())
}

fn inspect_amplification(source: &str) -> Result<(), &'static str> {
    let source = compact(source);
    for required in [
        "members:NonZeroU32",
        "wal_bytes:NonZeroU64",
        "root_plan:PhysicalGroupRootPublicationPlan",
        "pubconstfnroot_publications_planned(self)->u32{1}",
        "PhysicalGroupRootPublicationPlan::Shared(_)=>1",
        "pubconstfnwal_barrier_executions(self)->u32{1}",
        "self.append.members_per_group()",
    ] {
        if !source.contains(required) {
            return Err("group amplification observation lost stage-exact cardinality or cost");
        }
    }
    for premature in ["data_writes_executed", "acknowledgments_completed"] {
        if source.contains(premature) {
            return Err("Phase 5 amplification claims a future-stage effect");
        }
    }
    Ok(())
}

fn inspect_journey(source: &str) -> Result<(), &'static str> {
    for required in [
        "a terminal duplicate must invalidate every older prepared value",
        "PhysicalDurabilityGroupAdmissionDenial::IdempotencyProvenNoEffect",
        "append_amplification.shared_root_publications_planned()",
        "sync_after - sync_before, 1",
        "barrier_amplification.members_proven_wal_durable(), 3",
    ] {
        if !source.contains(required) {
            return Err("identity-blender journey lost a Phase 5 adversarial assertion");
        }
    }
    if source.matches("appended_frames(), 0").count() < 3 {
        return Err("identity-blender cancellation no longer proves every pre-WAL seam");
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
