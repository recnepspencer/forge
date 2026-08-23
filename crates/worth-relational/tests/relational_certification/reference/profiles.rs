use super::world::supply_chain::assert_oracle_matches;
use super::world::supply_chain::{
    audit_supply_chain_baseline, compile_supply_chain_baseline_with_limits,
    CompiledSupplyChainProgram, ScaleName, SupplyChainScale, SupplyChainWorldDefinition,
};
use worth_relational::facade::history::BranchId;
use worth_relational::facade::runtime::RelationIntegrityScopeBudget;

#[test]
fn phase5_standard_profile_installs_and_forks_through_production_owner() {
    assert_profile_fork(SupplyChainScale::standard(), "standard");
}

#[test]
fn phase5_bounded_density_slope_is_explicitly_court_and_standard_not_true_scale() {
    let samples = [
        profile_fork_sample(SupplyChainScale::court(), "bounded-court"),
        profile_fork_sample(SupplyChainScale::standard(), "bounded-standard"),
    ];
    assert_eq!(
        samples.map(|sample| sample.profile),
        [ScaleName::Court, ScaleName::Standard]
    );
    assert!(samples
        .iter()
        .all(|sample| sample.materialized_entities == 0));
    assert!(samples
        .iter()
        .all(|sample| sample.materialized_relations == 0));
    assert!(samples.iter().all(|sample| sample.materialized_bytes == 0));
    assert_eq!(bounded_density_slope(&samples), 0.0);
    assert!(
        !samples
            .iter()
            .any(|sample| sample.profile == ScaleName::Scale),
        "bounded evidence must not relabel Court/Standard execution as the scheduled Scale lane"
    );
}

fn assert_profile_fork(scale: SupplyChainScale, label: &str) {
    let sample = profile_fork_sample(scale, label);
    assert_eq!(sample.materialized_entities, 0);
    assert_eq!(sample.materialized_relations, 0);
    assert_eq!(sample.materialized_bytes, 0);
}

#[derive(Clone, Copy)]
struct BoundedProfileSample {
    profile: ScaleName,
    log2_cargo_lots: f64,
    materialized_entities: u64,
    materialized_relations: u64,
    materialized_bytes: u64,
}

fn profile_fork_sample(scale: SupplyChainScale, label: &str) -> BoundedProfileSample {
    let program = CompiledSupplyChainProgram::compile(
        SupplyChainWorldDefinition::operating(scale).expect("profile definition is valid"),
    )
    .expect("profile program compiles");
    let world = compile_supply_chain_baseline_with_limits(
        program,
        200_000,
        RelationIntegrityScopeBudget {
            max_relation_kinds: 128,
            max_touched_entities: 131_072,
            max_deleted_entities: 131_072,
            max_scanned_relations: 131_072,
            max_planned_edges: 131_072,
        },
    )
    .unwrap_or_else(|error| panic!("{label} production profile failed: {error:?}"));
    let certified = audit_supply_chain_baseline(world)
        .unwrap_or_else(|error| panic!("{label} oracle audit failed: {error:?}"));
    let mut world = certified.world;
    assert_oracle_matches(&world, &certified.expected);

    let branch_id = BranchId(format!("phase5-{label}"));
    let (_, source_basis) = world
        .runtime
        .observe_fork_source(&BranchId("main".to_owned()))
        .expect("profile baseline has a forkable main root");
    world
        .runtime
        .fork_branch(branch_id.clone(), source_basis)
        .expect("profile fork remains metadata-only");
    let identities = [
        world.runtime.main_branch_identity(),
        world
            .runtime
            .branch_identity(&branch_id)
            .expect("profile fork identity"),
    ];
    let observation = world
        .runtime
        .inspect_branch_sharing(&identities)
        .expect("profile sharing inspection succeeds");
    assert_eq!(observation.unique_root_count(), 1);
    assert_eq!(observation.copied_truth_bytes(), 0);
    assert_eq!(observation.copied_commit_envelopes(), 0);
    assert_eq!(observation.fork_materialized_authoritative_bytes(), 0);
    assert_eq!(
        observation.logical_branch_partition_payload_bytes(),
        observation.unique_physical_partition_payload_bytes() * observation.branch_count(),
        "one unchanged profile fork accounts for two logical views of one physical root"
    );
    BoundedProfileSample {
        profile: scale.name,
        log2_cargo_lots: (scale.cargo_lots as f64).log2(),
        materialized_entities: observation.fork_materialized_entity_count(),
        materialized_relations: observation.fork_materialized_relation_count(),
        materialized_bytes: observation.fork_materialized_authoritative_bytes(),
    }
}

fn bounded_density_slope(samples: &[BoundedProfileSample]) -> f64 {
    let x_delta = samples[1].log2_cargo_lots - samples[0].log2_cargo_lots;
    let y_delta = samples[1].materialized_bytes as f64 - samples[0].materialized_bytes as f64;
    y_delta / x_delta
}
