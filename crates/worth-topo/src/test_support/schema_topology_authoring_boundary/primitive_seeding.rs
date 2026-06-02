use forge_relational::facade::history::BranchId;
use forge_relational::facade::identity::EntityId;
use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::snapshots::SnapshotHandle;
use schema::facade::platform::authority::{MutationOrigin, TopologyMutation};
use schema::facade::topology_authoring::{
    seed_milestone_one_primitive as seed_schema_primitive,
    seed_milestone_one_primitive_on_branch as seed_schema_primitive_on_branch,
    seed_minimal_topology as seed_schema_minimal_topology, DerivedTopologyReadBasis,
    MilestoneOnePrimitiveAuthoringError as SchemaPrimitiveAuthoringFailure,
    MilestoneOnePrimitiveCase, MinimalTopologySeed as SchemaMinimalTopologySeed,
    TopologyIntentCommitError,
};

use crate::certification::support::commit_certification_input::TopologyCommitCertificationInput;

use super::branch_execution::open_schema_topology_authoring_branch;

#[derive(Debug)]
pub enum SchemaPrimitiveAuthoringError {
    InvalidParameter {
        family: &'static str,
        parameter: usize,
        requirement: &'static str,
    },
    Authority(TopologyIntentCommitError),
}

impl std::fmt::Display for SchemaPrimitiveAuthoringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidParameter {
                family,
                parameter,
                requirement,
            } => write!(
                f,
                "invalid `{family}` parameter `{parameter}`; expected {requirement}"
            ),
            Self::Authority(error) => write!(f, "{error:?}"),
        }
    }
}

impl std::error::Error for SchemaPrimitiveAuthoringError {}

impl From<SchemaPrimitiveAuthoringFailure> for SchemaPrimitiveAuthoringError {
    fn from(value: SchemaPrimitiveAuthoringFailure) -> Self {
        match value {
            SchemaPrimitiveAuthoringFailure::InvalidParameter {
                family,
                parameter,
                requirement,
            } => Self::InvalidParameter {
                family,
                parameter,
                requirement,
            },
            SchemaPrimitiveAuthoringFailure::Authority(error) => Self::Authority(error),
        }
    }
}

#[derive(Debug)]
pub(crate) enum SchemaBranchPrimitiveAuthoringError {
    BranchSetup(String),
    PrimitiveAuthoring(SchemaPrimitiveAuthoringError),
}

impl std::fmt::Display for SchemaBranchPrimitiveAuthoringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BranchSetup(error) => write!(f, "{error}"),
            Self::PrimitiveAuthoring(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for SchemaBranchPrimitiveAuthoringError {}

#[cfg_attr(not(test), allow(dead_code))]
#[derive(Debug, Clone)]
pub(crate) struct SchemaMinimalTopologySeedWitness {
    pub(crate) snapshot: SnapshotHandle,
    pub(crate) model: EntityId,
    pub(crate) body: EntityId,
    pub(crate) lump: EntityId,
    pub(crate) region: EntityId,
    pub(crate) shell: EntityId,
    pub(crate) face: EntityId,
    pub(crate) outer_loop: EntityId,
    pub(crate) wire: EntityId,
    pub(crate) half_edge: EntityId,
    pub(crate) edge: EntityId,
    pub(crate) vertex: EntityId,
    authority_mutations: Vec<TopologyMutation>,
    read_basis: DerivedTopologyReadBasis,
}

impl SchemaMinimalTopologySeedWitness {
    fn from_schema_seed(seed: SchemaMinimalTopologySeed) -> Self {
        let read_basis = seed.read_basis().clone();
        let authority_mutations = seed
            .persisted_truth()
            .committed_mutation_set
            .raw_intent()
            .mutations
            .clone();
        Self {
            snapshot: seed.snapshot,
            model: seed.model,
            body: seed.body,
            lump: seed.lump,
            region: seed.region,
            shell: seed.shell,
            face: seed.face,
            outer_loop: seed.outer_loop,
            wire: seed.wire,
            half_edge: seed.half_edge,
            edge: seed.edge,
            vertex: seed.vertex,
            authority_mutations,
            read_basis,
        }
    }

    pub(crate) fn authority_mutations(&self) -> &[TopologyMutation] {
        &self.authority_mutations
    }

    pub(crate) fn read_basis(&self) -> &DerivedTopologyReadBasis {
        &self.read_basis
    }
}

pub(crate) fn seed_milestone_one_primitive_through_schema_execution(
    runtime: &mut RelationalRuntime,
    stem: &str,
    primitive: &MilestoneOnePrimitiveCase,
) -> Result<TopologyCommitCertificationInput, SchemaPrimitiveAuthoringError> {
    let seeded = seed_schema_primitive(runtime, stem, primitive)?;
    Ok(TopologyCommitCertificationInput::from_seeded_commit(seeded))
}

fn seed_milestone_one_primitive_on_branch_through_schema_execution(
    runtime: &mut RelationalRuntime,
    stem: &str,
    primitive: &MilestoneOnePrimitiveCase,
    branch_id: BranchId,
    mutation_origin: MutationOrigin,
) -> Result<TopologyCommitCertificationInput, SchemaPrimitiveAuthoringError> {
    let seeded =
        seed_schema_primitive_on_branch(runtime, stem, primitive, branch_id, mutation_origin)?;
    Ok(TopologyCommitCertificationInput::from_seeded_commit(seeded))
}

pub(crate) fn seed_milestone_one_primitive_in_new_branch_through_schema_execution(
    runtime: &mut RelationalRuntime,
    stem: &str,
    primitive: &MilestoneOnePrimitiveCase,
    branch_label: impl Into<String>,
    mutation_origin: MutationOrigin,
) -> Result<TopologyCommitCertificationInput, SchemaBranchPrimitiveAuthoringError> {
    let branch = open_schema_topology_authoring_branch(runtime, branch_label)
        .map_err(SchemaBranchPrimitiveAuthoringError::BranchSetup)?;
    seed_milestone_one_primitive_on_branch_through_schema_execution(
        runtime,
        stem,
        primitive,
        branch.branch_id().clone(),
        mutation_origin,
    )
    .map_err(SchemaBranchPrimitiveAuthoringError::PrimitiveAuthoring)
}

pub(crate) fn seed_minimal_topology_through_schema_execution(
    runtime: &mut RelationalRuntime,
    stem: &str,
) -> Result<
    SchemaMinimalTopologySeedWitness,
    forge_relational::facade::transactions::TransactionCommitError,
> {
    seed_schema_minimal_topology(runtime, stem)
        .map(SchemaMinimalTopologySeedWitness::from_schema_seed)
}
