use schema::facade::topology_authoring::{
    milestone_one_default_primitive_corpus, CanonicalTopologyMutationBatch,
    CertifiedTopologyInterpretation, DerivedTopologyReadBasis, DerivedTruthBasisIdentity,
    MilestoneOnePrimitiveScenario, PersistedTopologyTruthBatch, SeededTopologyCommit,
    TopologyReadArtifact,
};
use schema::facade::{
    bootstrap_schema_registry, QueryAspectFamily, QueryAspectPath, QueryCollection, QueryLiveField,
    QuerySchemaBasis, SCHEMA_ID, SCHEMA_VERSION_ID, SchemaBuildError, SchemaBuilder,
};
use schema::facade::platform::aspects::Aspect;
use schema::facade::platform::authority::{
    CreateKey, EntityReference, MutationOrigin, RawTopologyIntent, TopologyMutationBatch,
};
use schema::facade::platform::entities::EntityKind;
use schema::facade::platform::relations::RelationKind;

fn _query_vocab_contract() {
    let contract = QueryLiveSurfaceContract {
        surface: ".public.topology",
        family: QueryAspectFamily::Topology,
        collection: QueryCollection::TopologyEntity,
        basis: QuerySchemaBasis::TopologyEntityLiveView,
        aspect: QueryAspectPath::TOPOLOGY_STRUCTURE,
        field: QueryLiveField::IdentityId,
    };
    let _ = (
        contract.surface,
        contract.family,
        contract.collection,
        contract.basis,
        contract.aspect,
        contract.field,
    );
}

fn _truth_vocab_contract(
    aspect: Aspect,
    entity: EntityKind,
    relation: RelationKind,
    create_key: CreateKey,
    entity_reference: EntityReference,
    mutation_origin: MutationOrigin,
    intent: RawTopologyIntent,
    batch: TopologyMutationBatch,
) {
    let _ = (
        aspect,
        entity,
        relation,
        create_key,
        entity_reference,
        mutation_origin,
        intent,
        batch,
    );
}

fn _bootstrap_contract() {
    let _ = bootstrap_schema_registry();
    let _ = SchemaBuilder::new()
        .with_topology_kinds()
        .with_naming_kinds();
    let _ = SchemaBuildError::MissingTopologyKinds;
    let _ = (SCHEMA_ID, SCHEMA_VERSION_ID);
}

fn _topology_authoring_contract() -> Vec<MilestoneOnePrimitiveScenario> {
    milestone_one_default_primitive_corpus()
}

struct QueryLiveSurfaceContract {
    surface: &'static str,
    family: QueryAspectFamily,
    collection: QueryCollection,
    basis: QuerySchemaBasis,
    aspect: QueryAspectPath,
    field: QueryLiveField,
}

fn _topology_authoring_support_contract(
    seeded: SeededTopologyCommit,
    canonical: CanonicalTopologyMutationBatch,
    persisted: PersistedTopologyTruthBatch,
    read_basis: DerivedTopologyReadBasis,
    read_artifact: TopologyReadArtifact,
    certified: CertifiedTopologyInterpretation,
    truth_basis: DerivedTruthBasisIdentity,
) {
    let _ = (
        seeded.snapshot(),
        seeded.branch_id(),
        seeded.commits(),
        canonical,
        persisted,
        read_basis,
        read_artifact,
        certified,
        truth_basis,
    );
}

#[test]
fn schema_public_surface_stays_vocabulary_first() {
    let _ = _query_vocab_contract;
    let _ = _truth_vocab_contract;
    let _ = _bootstrap_contract;
    let _ = _topology_authoring_contract;
    let _ = _topology_authoring_support_contract;
}
