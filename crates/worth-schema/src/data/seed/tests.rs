use forge_relational::facade::history::BranchId;
use forge_relational::facade::runtime::RelationalRuntimeApi;
use worth_math::predicates::orient2d;

use crate::data::aspects::{
    WorthAspect, WorthDiagnosticsAspect, WorthNamingAspect, WorthTopologyAspect,
};
use crate::data::authority::{
    CertifiedTopologyInterpretation, DerivedTopologyReadBasis, PersistedTopologyTruthBatch,
    RawWorthTopologyIntent, WorthMutationOrigin, WorthPrecisionFallbackRecord,
    WorthTopologyMutationBatch,
};
use crate::data::bootstrap::worth_bootstrap_schema_registry;
use crate::data::seed::{
    milestone_one_admitted_range_sweep_out_of_class_scenarios,
    milestone_one_admitted_range_sweep_scenarios, seed_minimal_topology,
};
use std::collections::BTreeSet;

#[test]
fn seed_minimal_topology_commits_a_readable_bootstrap_snapshot() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(
            worth_bootstrap_schema_registry().expect("worth bootstrap schema registry"),
        )
        .build();

    let seeded = seed_minimal_topology(&mut runtime, "test-seed").expect("seed worth topology");
    let read_view = runtime
        .read_truth()
        .read_snapshot(&seeded.snapshot)
        .expect("seeded worth snapshot");

    assert_eq!(read_view.entities().len(), 22);
    assert_eq!(read_view.relations().len(), 25);
    assert!(read_view.get_entity(seeded.face).is_some());
    assert!(read_view.get_entity(seeded.half_edge).is_some());
    assert_eq!(seeded.persistent_name_ids.len(), 11);
    assert_eq!(seeded.persisted_truth.snapshot, seeded.snapshot);
    assert_eq!(seeded.read_basis.snapshot(), &seeded.snapshot);
    assert_eq!(seeded.read_artifact.snapshot, seeded.snapshot);
    assert_eq!(
        seeded.certified_interpretation.read_basis.snapshot(),
        &seeded.snapshot
    );
    for name_id in &seeded.persistent_name_ids {
        assert!(read_view.get_entity(*name_id).is_some());
    }
}

#[test]
fn precision_fallback_record_threads_through_authority_flow() {
    let (_sign, escalation) =
        orient2d([0.0, 0.0], [1.0, 0.0], [0.5, 1e-30]).expect("predicate evaluation");
    let fallback = WorthPrecisionFallbackRecord::from(&escalation);

    let touched_aspects = BTreeSet::from([
        WorthAspect::Topology(WorthTopologyAspect::Structure),
        WorthAspect::Naming(WorthNamingAspect::PersistentName),
        WorthAspect::Diagnostics(WorthDiagnosticsAspect::Decisions),
    ]);

    let raw = RawWorthTopologyIntent::new(Vec::new(), WorthMutationOrigin::Seed)
        .with_precision_fallback(fallback.clone());
    let batch = WorthTopologyMutationBatch::from_raw_intent(raw, touched_aspects.clone());

    assert_eq!(batch.precision_fallbacks, vec![fallback.clone()]);
    assert!(batch.precision_budget_fallbacks.is_empty());

    let persisted = PersistedTopologyTruthBatch {
        batch,
        snapshot: forge_relational::facade::snapshots::SnapshotHandle::new(1, 1),
        branch_id: BranchId("main".to_string()),
        mutation_origin: WorthMutationOrigin::Seed,
    };
    let read_basis = DerivedTopologyReadBasis::from_persisted_truth(&persisted);
    let certified = CertifiedTopologyInterpretation::from_read_basis(read_basis.clone());

    assert_eq!(read_basis.precision_fallbacks, vec![fallback.clone()]);
    assert_eq!(certified.precision_fallbacks, vec![fallback]);
    assert_eq!(read_basis.touched_aspects(), &touched_aspects);
    assert_eq!(
        read_basis.authoritative_mutation_origin(),
        WorthMutationOrigin::Seed
    );
    assert_eq!(read_basis.derivation_origin(), WorthMutationOrigin::Seed);
    let replay_basis = read_basis.replay_of();
    assert_eq!(
        replay_basis.authoritative_mutation_origin(),
        WorthMutationOrigin::Seed
    );
    assert_eq!(
        replay_basis.derivation_origin(),
        WorthMutationOrigin::Replay
    );
    assert_eq!(
        read_basis
            .authority
            .truth_basis_identity
            .touched_aspect_count,
        3
    );
    assert!(!read_basis
        .authority
        .truth_basis_identity
        .mutation_batch_digest_hex
        .is_empty());
}

#[test]
fn admitted_range_sweep_generators_cover_the_declared_milestone_one_ranges() {
    let scenarios = milestone_one_admitted_range_sweep_scenarios();
    let out_of_class = milestone_one_admitted_range_sweep_out_of_class_scenarios();

    let unique_cases = scenarios.iter().fold(BTreeSet::new(), |mut acc, scenario| {
        acc.insert(format!("{}::{:?}", scenario.family, scenario.primitive));
        acc
    });

    assert_eq!(unique_cases.len(), scenarios.len());
    assert_eq!(
        scenarios
            .iter()
            .filter(|s| s.family == "WireOpen(n)")
            .count(),
        12
    );
    assert_eq!(
        scenarios
            .iter()
            .filter(|s| s.family == "WireClosed(n)")
            .count(),
        10
    );
    assert_eq!(
        scenarios
            .iter()
            .filter(|s| s.family == "WireBranch(k)")
            .count(),
        10
    );
    assert_eq!(
        scenarios
            .iter()
            .filter(|s| s.family == "SheetDisk(n)")
            .count(),
        10
    );
    assert_eq!(
        scenarios
            .iter()
            .filter(|s| s.family == "SheetPatch(f)")
            .count(),
        9
    );
    assert_eq!(
        scenarios
            .iter()
            .filter(|s| s.family == "SolidShell(f)")
            .count(),
        7
    );
    assert_eq!(
        scenarios
            .iter()
            .filter(|s| s.family == "NmtEdgeFan(k)")
            .count(),
        8
    );

    assert_eq!(out_of_class.len(), 7);
    assert!(out_of_class.iter().all(|scenario| {
        scenario.expected_outcome
            == crate::facade::WorthMilestoneOnePrimitiveExpectedOutcome::Reject
    }));
    assert!(out_of_class.iter().all(|scenario| {
        scenario.role == crate::facade::WorthMilestoneOnePrimitiveRole::OutOfClass
    }));
}
