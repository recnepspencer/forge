use std::collections::BTreeSet;

use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::transactions::TransactionCommitError;

use crate::data::aspects::{WorthAspect, WorthDiagnosticsAspect, WorthNamingAspect, WorthTopologyAspect};
use crate::data::authority::{
    CertifiedTopologyInterpretation, DerivedTopologyReadBasis, PersistedTopologyTruthBatch,
    RawWorthTopologyIntent, WorthTopologyMutationBatch, WorthTopologyReadArtifact,
    WorthMutationOrigin,
};
use crate::data::entities::{WorthEntityKind, WorthTopologyEntityKind};
use crate::data::seed::labels::WorthMinimalTopologyLabels;
use crate::data::seed::lookup::find_seeded_entity;
use crate::data::seed::naming_creation::create_persistent_names;
use crate::data::seed::relation_creation::{create_bootstrap_entities, create_bootstrap_relations};
use crate::data::seed::types::WorthMinimalTopologySeed;

pub fn seed_minimal_topology(
    runtime: &mut RelationalRuntime,
    stem: &str,
) -> Result<WorthMinimalTopologySeed, TransactionCommitError> {
    let labels = WorthMinimalTopologyLabels::new(stem);
    let entity_commit = create_bootstrap_entities(runtime, &labels)?;
    let entity_read = runtime
        .read_truth()
        .read_snapshot(&entity_commit.snapshot)
        .expect("seeded entity snapshot should remain readable");

    let touched_aspects = BTreeSet::from([
        WorthAspect::Topology(WorthTopologyAspect::Structure),
        WorthAspect::Topology(WorthTopologyAspect::Ownership),
        WorthAspect::Topology(WorthTopologyAspect::Boundary),
        WorthAspect::Naming(WorthNamingAspect::PersistentName),
        WorthAspect::Diagnostics(WorthDiagnosticsAspect::Decisions),
    ]);

    let bootstrap_persisted_truth = PersistedTopologyTruthBatch {
        batch: WorthTopologyMutationBatch::from_raw_intent(
            RawWorthTopologyIntent::new(Vec::new(), WorthMutationOrigin::Seed),
            touched_aspects.clone(),
        ),
        snapshot: entity_commit.snapshot.clone(),
        mutation_origin: WorthMutationOrigin::Seed,
    };
    let bootstrap_read_basis = DerivedTopologyReadBasis::from_persisted_truth(&bootstrap_persisted_truth);

    let ids = WorthMinimalTopologySeed {
        snapshot: entity_commit.snapshot.clone(),
        model: find_seeded_entity(
            &entity_read,
            WorthEntityKind::Topology(WorthTopologyEntityKind::Model),
            &labels.model,
        ),
        body: find_seeded_entity(
            &entity_read,
            WorthEntityKind::Topology(WorthTopologyEntityKind::Body),
            &labels.body,
        ),
        lump: find_seeded_entity(
            &entity_read,
            WorthEntityKind::Topology(WorthTopologyEntityKind::Lump),
            &labels.lump,
        ),
        region: find_seeded_entity(
            &entity_read,
            WorthEntityKind::Topology(WorthTopologyEntityKind::Region),
            &labels.region,
        ),
        shell: find_seeded_entity(
            &entity_read,
            WorthEntityKind::Topology(WorthTopologyEntityKind::Shell),
            &labels.shell,
        ),
        face: find_seeded_entity(
            &entity_read,
            WorthEntityKind::Topology(WorthTopologyEntityKind::Face),
            &labels.face,
        ),
        outer_loop: find_seeded_entity(
            &entity_read,
            WorthEntityKind::Topology(WorthTopologyEntityKind::Loop),
            &labels.outer_loop,
        ),
        wire: find_seeded_entity(
            &entity_read,
            WorthEntityKind::Topology(WorthTopologyEntityKind::Wire),
            &labels.wire,
        ),
        half_edge: find_seeded_entity(
            &entity_read,
            WorthEntityKind::Topology(WorthTopologyEntityKind::HalfEdge),
            &labels.half_edge,
        ),
        edge: find_seeded_entity(
            &entity_read,
            WorthEntityKind::Topology(WorthTopologyEntityKind::Edge),
            &labels.edge,
        ),
        vertex: find_seeded_entity(
            &entity_read,
            WorthEntityKind::Topology(WorthTopologyEntityKind::Vertex),
            &labels.vertex,
        ),
        persistent_name_ids: Vec::new(),
        persisted_truth: bootstrap_persisted_truth,
        read_basis: bootstrap_read_basis.clone(),
        read_artifact: WorthTopologyReadArtifact::from_read_basis(&bootstrap_read_basis),
        certified_interpretation: CertifiedTopologyInterpretation::from_read_basis(
            bootstrap_read_basis,
        ),
    };

    let _relation_commit = create_bootstrap_relations(runtime, &ids, &labels)?;
    let naming_commit = create_persistent_names(runtime, &ids, &labels)?;
    let naming_read = runtime
        .read_truth()
        .read_snapshot(&naming_commit.snapshot)
        .expect("persistent naming snapshot should remain readable");
    let persistent_name_ids =
        crate::data::seed::naming_creation::collect_persistent_name_ids(&naming_read, &labels);

    let persisted_truth = PersistedTopologyTruthBatch {
        batch: WorthTopologyMutationBatch::from_raw_intent(
            RawWorthTopologyIntent::new(Vec::new(), WorthMutationOrigin::Seed),
            touched_aspects,
        ),
        snapshot: naming_commit.snapshot.clone(),
        mutation_origin: WorthMutationOrigin::Seed,
    };
    let read_basis = DerivedTopologyReadBasis::from_persisted_truth(&persisted_truth);
    let read_artifact = WorthTopologyReadArtifact::from_read_basis(&read_basis);
    let certified_interpretation = CertifiedTopologyInterpretation::from_read_basis(read_basis.clone());

    Ok(WorthMinimalTopologySeed {
        snapshot: naming_commit.snapshot.clone(),
        persistent_name_ids,
        persisted_truth,
        read_basis,
        read_artifact,
        certified_interpretation,
        ..ids
    })
}
