use forge_relational::facade::runtime::RelationalRuntimeApi;
use forge_relational::facade::history::BranchId;
use worth_math::predicates::orient2d;

use crate::data::bootstrap::worth_bootstrap_schema_registry;
use crate::data::aspects::{
    WorthAspect, WorthDiagnosticsAspect, WorthNamingAspect, WorthTopologyAspect,
};
use crate::data::authority::{
    CertifiedTopologyInterpretation, DerivedTopologyReadBasis, PersistedTopologyTruthBatch,
    RawWorthTopologyIntent, WorthMutationOrigin, WorthPrecisionFallbackRecord,
    WorthTopologyMutationBatch,
};
use crate::data::seed::seed_minimal_topology;
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
    assert_eq!(seeded.read_basis.snapshot, seeded.snapshot);
    assert_eq!(seeded.read_artifact.snapshot, seeded.snapshot);
    assert_eq!(seeded.certified_interpretation.read_basis.snapshot, seeded.snapshot);
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
    assert_eq!(read_basis.touched_aspects, touched_aspects);
}
