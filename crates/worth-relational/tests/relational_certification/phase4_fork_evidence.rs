use super::world::supply_chain::{
    audit_supply_chain_baseline, compare, compile_supply_chain_baseline,
    CompiledSupplyChainProgram, ExpectedSupplyChainObservation, SupplyChainScale,
    SupplyChainWorldDefinition,
};
use worth_relational::facade::branch::{
    RelationalBranchIdentity, RelationalBranchReferenceState, RelationalForkSourceDescriptor,
    RelationalLegacyBranchBindingDenial,
};
use worth_relational::facade::history::{BranchId, CommitId, RelationalCommitIdentity};
use worth_relational::facade::runtime::{RelationalPhase4ReferenceCostCounters, RelationalRuntime};

/// Public owner observations and immutable catalog identity captured around a
/// Phase-4 fork attempt. This is evidence, not an authority carrier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Phase4ReferenceEvidence {
    pub(crate) source: Option<RelationalForkSourceDescriptor>,
    pub(crate) target: Option<RelationalForkSourceDescriptor>,
    pub(crate) source_identity:
        Result<RelationalBranchIdentity, RelationalLegacyBranchBindingDenial>,
    pub(crate) target_identity:
        Result<RelationalBranchIdentity, RelationalLegacyBranchBindingDenial>,
    pub(crate) source_state: Option<RelationalBranchReferenceState>,
    pub(crate) target_state: Option<RelationalBranchReferenceState>,
    pub(crate) catalog_count: usize,
    pub(crate) artifact_identity: Option<RelationalCommitIdentity>,
    pub(crate) artifact_parents: Option<Vec<CommitId>>,
    pub(crate) counters: RelationalPhase4ReferenceCostCounters,
}

pub(crate) fn certified_supply_chain_world(
    scale: SupplyChainScale,
) -> (
    super::world::supply_chain::ProductionSeededSupplyChainWorld,
    ExpectedSupplyChainObservation,
) {
    let program = CompiledSupplyChainProgram::compile(
        SupplyChainWorldDefinition::operating(scale).expect("Supply Chain definition is valid"),
    )
    .expect("Court Supply Chain program compiles");
    let certified = audit_supply_chain_baseline(
        compile_supply_chain_baseline(program).expect("baseline installs"),
    )
    .expect("baseline matches the independent Supply Chain oracle");
    (certified.world, certified.expected)
}

/// Compile the canonical empty Supply Chain installation through the same
/// schema compiler used by seeded worlds, but stop before the first commit so
/// the configured main cell remains an honest `Empty` source.
pub(crate) fn canonical_empty_supply_chain_runtime(scale: SupplyChainScale) -> RelationalRuntime {
    let definition = SupplyChainWorldDefinition::empty(empty_supply_chain_scale(scale));
    let program = CompiledSupplyChainProgram::compile(definition)
        .expect("empty Supply Chain definition compiles");
    let mut runtime = worth_relational::facade::runtime::RelationalRuntimeApi::builder().build();
    runtime
        .prepare_initial_schema_installation()
        .expect("empty Supply Chain schema installation prepares")
        .install(program.schema_registry().clone())
        .expect("empty Supply Chain schema installation succeeds");
    runtime
}

fn empty_supply_chain_scale(mut scale: SupplyChainScale) -> SupplyChainScale {
    scale.ports = 0;
    scale.terminals = 0;
    scale.berths = 0;
    scale.vessels = 0;
    scale.voyages = 0;
    scale.port_calls = 0;
    scale.cargo_lots = 0;
    scale.regions = 1;
    scale
}

pub(crate) fn assert_oracle_matches(
    world: &super::world::supply_chain::ProductionSeededSupplyChainWorld,
    expected: &ExpectedSupplyChainObservation,
) {
    let observed = super::world::supply_chain::observe_supply_chain(world)
        .expect("production Supply Chain observation remains available");
    compare(expected, &observed).expect("production observation matches the independent oracle");
}

pub(crate) fn capture_reference_evidence(
    runtime: &mut RelationalRuntime,
    source_branch: &BranchId,
    target_branch: &BranchId,
    commit_id: CommitId,
) -> Phase4ReferenceEvidence {
    Phase4ReferenceEvidence {
        source: observe_descriptor(runtime, source_branch),
        target: observe_descriptor(runtime, target_branch),
        source_identity: runtime.branch_identity(source_branch),
        target_identity: runtime.branch_identity(target_branch),
        source_state: runtime.branch_reference_state(source_branch),
        target_state: runtime.branch_reference_state(target_branch),
        catalog_count: runtime.history().immutable_commit_count(),
        artifact_identity: runtime.history().immutable_commit_identity(commit_id),
        artifact_parents: runtime
            .history()
            .immutable_commit_receipt(commit_id)
            .map(|receipt| receipt.parents),
        counters: runtime.phase4_reference_cost_counters(),
    }
}

pub(crate) fn assert_denial_left_no_reference_residue(
    before: &Phase4ReferenceEvidence,
    after: &Phase4ReferenceEvidence,
) {
    assert_eq!(
        after.source, before.source,
        "source reference moved on denial"
    );
    assert_eq!(
        after.target, before.target,
        "target reference moved on denial"
    );
    assert_eq!(
        after.source_identity, before.source_identity,
        "source identity changed on denial"
    );
    assert_eq!(
        after.target_identity, before.target_identity,
        "target identity changed on denial"
    );
    assert_eq!(
        after.source_state, before.source_state,
        "source branch-cell checkpoint changed on denial"
    );
    assert_eq!(
        after.target_state, before.target_state,
        "target branch-cell checkpoint changed on denial"
    );
    assert_eq!(
        after.catalog_count, before.catalog_count,
        "catalog length changed on denial"
    );
    assert_eq!(
        after.artifact_identity, before.artifact_identity,
        "canonical artifact identity changed on denial"
    );
    assert_eq!(
        after.artifact_parents, before.artifact_parents,
        "canonical ordered parentage changed on denial"
    );
    assert_eq!(
        after.counters.artifact_clones, before.counters.artifact_clones,
        "artifact materialization changed on denial"
    );
}

fn observe_descriptor(
    runtime: &mut RelationalRuntime,
    branch_id: &BranchId,
) -> Option<RelationalForkSourceDescriptor> {
    runtime
        .observe_fork_source(branch_id)
        .ok()
        .map(|(descriptor, _basis)| descriptor)
}
