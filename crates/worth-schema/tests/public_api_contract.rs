use schema::facade::platform::aspects::Aspect;
use schema::facade::platform::authority::{
    CreateKey, EntityReference, MutationOrigin, RawTopologyIntent,
};
use schema::facade::platform::entities::EntityKind;
use schema::facade::platform::relations::RelationKind;
use schema::facade::topology_authoring::{
    commit_topology_intent, commit_topology_intent_on_branch, commit_topology_mutation_set,
    commit_topology_mutation_set_on_branch, milestone_one_default_primitive_corpus,
    CertifiedTopologyInterpretation, DerivedTopologyReadBasis, DerivedTruthBasisIdentity,
    MilestoneOnePrimitiveScenario, PersistedTopologyTruth, SeededTopologyCommit,
    TopologyCommittedMutationSet, TopologyIntentCommitError, TopologyMutationSetCommitError,
    TopologyReadArtifact,
};
use schema::facade::{
    bootstrap_schema_registry, QueryAspectFamily, QueryAspectPath, QueryCollection, QueryLiveField,
    QuerySchemaBasis, SchemaBuildError, SchemaBuilder, SCHEMA_ID, SCHEMA_VERSION_ID,
};

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
    committed_mutation_set: TopologyCommittedMutationSet,
) {
    let _ = (
        aspect,
        entity,
        relation,
        create_key,
        entity_reference,
        mutation_origin,
        intent,
        committed_mutation_set,
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
    committed_mutation_set: TopologyCommittedMutationSet,
    persisted: PersistedTopologyTruth,
    read_basis: DerivedTopologyReadBasis,
    read_artifact: TopologyReadArtifact,
    certified: CertifiedTopologyInterpretation,
    truth_basis: DerivedTruthBasisIdentity,
) {
    let _: fn(
        &mut forge_relational::facade::runtime::RelationalRuntime,
        &'static str,
        Vec<forge_relational::facade::transactions::MutationIntent>,
    ) -> Result<
        forge_relational::facade::transactions::CommitResult,
        TopologyMutationSetCommitError,
    > = commit_topology_mutation_set;
    let _: fn(
        &mut forge_relational::facade::runtime::RelationalRuntime,
        forge_relational::facade::history::BranchId,
        &'static str,
        Vec<forge_relational::facade::transactions::MutationIntent>,
    ) -> Result<
        forge_relational::facade::transactions::CommitResult,
        TopologyMutationSetCommitError,
    > = commit_topology_mutation_set_on_branch;
    let _ = (
        commit_topology_intent,
        commit_topology_intent_on_branch,
        seeded.snapshot(),
        seeded.branch_id(),
        seeded.commits(),
        committed_mutation_set,
        persisted,
        read_basis,
        read_artifact,
        certified,
        truth_basis,
        std::mem::size_of::<TopologyIntentCommitError>(),
        std::mem::size_of::<TopologyMutationSetCommitError>(),
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
