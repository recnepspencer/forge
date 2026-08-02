use super::super::super::read_repository_document;
use super::{contains_in_order, function_body};

const PORT: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                    durability/grouping/wal_barrier/port.rs";
const DECLARATION: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                           durability/grouping/wal_barrier/declaration.rs";
const SETTLEMENT: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                          durability/grouping/wal_barrier/settlement.rs";
const MEMBER_SETTLEMENT: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                                 durability/grouping/member_settlement.rs";
const WAL_DURABLE: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                           durability/mutation/progression/wal_durable.rs";
const EXECUTOR: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                        instance/executor/wal_barrier.rs";

#[test]
fn matching_scheduled_barrier_is_the_only_wal_durable_promotion() {
    inspect(&sources()).unwrap();
}

#[test]
fn barrier_gate_rejects_route_binding_and_raw_settlement_mutants() {
    let source = sources();

    let mut scheduler_bypass = source.clone();
    scheduler_bypass.port = scheduler_bypass
        .port
        .replace("PhysicalWorkScheduler::admit", "bypass_scheduler");
    assert!(inspect(&scheduler_bypass).is_err());

    let mut substituted_work = source.clone();
    substituted_work.settlement = substituted_work
        .settlement
        .replace("work != expected_work", "false");
    assert!(inspect(&substituted_work).is_err());

    let mut raw_settlement = source.clone();
    raw_settlement.member_settlement = raw_settlement.member_settlement.replace(
        "barrier: CompletionBoundPhysicalWalGroupBarrierSettlement",
        "barrier: PhysicalWalGroupBarrierSettlement",
    );
    assert!(inspect(&raw_settlement).is_err());

    let mut collapsed_members = source.clone();
    collapsed_members.member_settlement = collapsed_members.member_settlement.replace(
        ".into_members().map(|member|",
        ".into_members().into_vec().into_iter().take(1).map(|member|",
    );
    assert!(inspect(&collapsed_members).is_err());

    let mut direct_backend = source;
    direct_backend.executor = direct_backend
        .executor
        .replace("synchronize_scheduled_file", "direct_file_sync");
    assert!(inspect(&direct_backend).is_err());
}

#[derive(Clone)]
struct BarrierSources {
    port: String,
    declaration: String,
    settlement: String,
    member_settlement: String,
    wal_durable: String,
    executor: String,
}

fn sources() -> BarrierSources {
    BarrierSources {
        port: read_repository_document(PORT).expect("read WAL barrier port"),
        declaration: read_repository_document(DECLARATION).expect("read WAL barrier declaration"),
        settlement: read_repository_document(SETTLEMENT).expect("read WAL barrier settlement"),
        member_settlement: read_repository_document(MEMBER_SETTLEMENT)
            .expect("read WAL member settlement"),
        wal_durable: read_repository_document(WAL_DURABLE).expect("read WAL durable progression"),
        executor: read_repository_document(EXECUTOR).expect("read WAL barrier executor"),
    }
}

fn inspect(source: &BarrierSources) -> Result<(), &'static str> {
    inspect_route(source)?;
    inspect_declaration(&source.declaration)?;
    inspect_settlement(&source.settlement)?;
    inspect_promotion(&source.port, &source.member_settlement, &source.wal_durable)?;
    Ok(())
}

fn inspect_route(source: &BarrierSources) -> Result<(), &'static str> {
    let entry = function_body(&source.port, "fn synchronize_appended_group(")
        .ok_or("ordinary WAL barrier entry is absent")?;
    if !contains_in_order(
        entry,
        &[
            "PhysicalWalGroupBarrierDeclaration::for_appended_group",
            "self.prepare_command",
            "self.execute",
        ],
    ) {
        return Err("ordinary barrier entry bypasses declaration or execution");
    }

    let preparation = function_body(&source.port, "fn prepare_command(")
        .ok_or("WAL barrier command preparation is absent")?;
    if !contains_in_order(
        preparation,
        &[
            ".submit(request)",
            "PhysicalWorkAdmission::admit",
            ".request(admitted)",
            "admit_record_queue_policy",
            "PhysicalWorkScheduler::admit",
            "PhysicalExecutorCommand::wal_barrier",
        ],
    ) {
        return Err("WAL barrier bypasses Signal scheduler or command admission");
    }

    let execution =
        function_body(&source.port, "fn execute(").ok_or("WAL barrier execution is absent")?;
    if !contains_in_order(
        execution,
        &[
            "execute_physical_work(command)",
            "PhysicalWalGroupBarrierSettlement::bind_completed",
            "WalDurablePhysicalMutationMembers::derive",
        ],
    ) {
        return Err("WAL durable promotion does not follow exact executor settlement");
    }
    if !source.executor.contains("synchronize_scheduled_file")
        || !source
            .executor
            .contains("PhysicalExecutorOutcome::WalBarrierCompleted")
    {
        return Err("WAL barrier executor lost the C4 synchronization effect");
    }
    Ok(())
}

fn inspect_declaration(source: &str) -> Result<(), &'static str> {
    let source = compact(source);
    for required in [
        "GROUP_BARRIER_BINDING_DOMAIN",
        "basis.identity().bytes()",
        "basis.membership_digest()",
        "basis.member_count().get()",
        "binding.member_identity().bytes()",
        "binding.ordinal().get()",
        "mutation.mutation_identity().operation_identity()",
        "mutation.settlement().payload_digest()",
        "durability.policy_identity().bytes()",
        "durability.admission_basis_identity().bytes()",
        "required_barrier",
        "barrier_tag(barrier)",
        "PhysicalWalBarrierScope::new(",
    ] {
        if !source.contains(&compact(required)) {
            return Err("WAL barrier declaration lost an exact binding component");
        }
    }
    Ok(())
}

fn inspect_settlement(source: &str) -> Result<(), &'static str> {
    let binding = function_body(source, "fn bind_completed(")
        .ok_or("completion-bound WAL settlement is absent")?;
    for predicate in [
        "work != expected_work",
        "declaration.binding_digest() != expected_binding_digest",
        "QueueExecutionOutcome::Executed",
        "physical.artifact() != declaration.artifact()",
        "ArtifactTreePublicationEffect::FileSynchronization",
        "CompletionBoundPhysicalWalGroupBarrierSettlement(Self",
    ] {
        if !binding.contains(predicate) {
            return Err("WAL barrier settlement lost an exact completion predicate");
        }
    }
    if !source.contains(
        "pub(in crate::physical_runtime) struct CompletionBoundPhysicalWalGroupBarrierSettlement(",
    ) {
        return Err("WAL barrier completion witness escaped its authority boundary");
    }
    Ok(())
}

fn inspect_promotion(
    port: &str,
    member_settlement: &str,
    durable: &str,
) -> Result<(), &'static str> {
    if port
        .matches("WalDurablePhysicalMutationMembers::derive(")
        .count()
        != 1
    {
        return Err("ordinary group WAL durable derivation is not singular");
    }
    let member_settlement = compact(member_settlement);
    for required in [
        "barrier: CompletionBoundPhysicalWalGroupBarrierSettlement",
        "shared.group_identity() != basis.identity()",
        "shared.membership_digest() != basis.membership_digest()",
        "shared.member_count() != basis.member_count().get()",
        ".into_members().map(|member|",
        "WalDurablePhysicalMutation::new(member, settlement)",
    ] {
        if !member_settlement.contains(&compact(required)) {
            return Err("group barrier lost exact all-member durability derivation");
        }
    }
    if !durable.contains("settlement: CompletionBoundPhysicalWalBarrierSettlement")
        || !durable.contains("pub(in crate::physical_runtime) fn new(")
    {
        return Err("raw barrier settlement can construct WAL-durable authority");
    }
    Ok(())
}

fn compact(source: &str) -> String {
    source
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect()
}
