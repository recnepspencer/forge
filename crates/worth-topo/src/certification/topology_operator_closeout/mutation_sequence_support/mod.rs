use schema::facade::platform::authority::{MutationOrigin, RawTopologyIntent};

use crate::topology_operators::application::TopologyDeclarationMutationPayload;
use crate::topology_operators::{
    TopologyDeclaredMutationSequence, TopologyMutationDigest, TopologyMutationFamily,
    TopologyMutationSequenceDigest, TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration,
    TopologySplitConnectedHalfEdgeSetToNewWireDeclaration,
};

#[derive(Clone)]
pub(super) enum TopologyCloseoutDeclaration {
    RehomeAllOwnedHalfEdgesToNewWire(TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration),
    SplitConnectedHalfEdgeSetToNewWire(TopologySplitConnectedHalfEdgeSetToNewWireDeclaration),
}

impl TopologyDeclarationMutationPayload for TopologyCloseoutDeclaration {
    const SEMANTIC_FAMILY_KEY: &'static str = "topology.closeout_declaration";

    fn into_mutation_sequence(self) -> TopologyDeclaredMutationSequence {
        match self {
            Self::RehomeAllOwnedHalfEdgesToNewWire(declaration) => {
                declaration.into_mutation_sequence()
            }
            Self::SplitConnectedHalfEdgeSetToNewWire(declaration) => {
                declaration.into_mutation_sequence()
            }
        }
    }
}

#[derive(Clone)]
pub(super) struct TopologyCloseoutMutationPlan {
    pub(super) raw_intent: RawTopologyIntent,
    pub(super) topology_mutation_digest: TopologyMutationDigest,
}

impl TopologyCloseoutMutationPlan {
    fn from_sequence_and_mutations(
        sequence: TopologyDeclaredMutationSequence,
        mutations: Vec<schema::facade::platform::authority::TopologyMutation>,
    ) -> Self {
        Self {
            raw_intent: RawTopologyIntent::new(mutations, MutationOrigin::BranchLocalApplication),
            topology_mutation_digest: sequence.topology_mutation_digest().clone(),
        }
    }
}

pub(super) fn closeout_mutation_plan_for_declaration<D>(
    declaration: D,
) -> TopologyCloseoutMutationPlan
where
    D: TopologyDeclarationMutationPayload,
{
    closeout_mutation_plan_for_declaration_and_mutations(
        declaration.clone(),
        lowered_mutations_for_declaration(&declaration),
    )
}

pub(super) fn closeout_mutation_plan_for_declaration_and_mutations<D>(
    declaration: D,
    mutations: Vec<schema::facade::platform::authority::TopologyMutation>,
) -> TopologyCloseoutMutationPlan
where
    D: TopologyDeclarationMutationPayload,
{
    closeout_mutation_plan_for_sequence_and_mutations(
        declaration.into_mutation_sequence(),
        mutations,
    )
}

pub(super) fn closeout_mutation_plan_for_sequence_and_mutations(
    sequence: TopologyDeclaredMutationSequence,
    mutations: Vec<schema::facade::platform::authority::TopologyMutation>,
) -> TopologyCloseoutMutationPlan {
    TopologyCloseoutMutationPlan::from_sequence_and_mutations(sequence, mutations)
}

pub(super) fn lowered_mutations_for_declaration<D>(
    declaration: &D,
) -> Vec<schema::facade::platform::authority::TopologyMutation>
where
    D: TopologyDeclarationMutationPayload,
{
    lowered_mutations_for_sequence(&declaration.clone().into_mutation_sequence())
}

pub(super) fn lowered_mutations_for_sequence(
    sequence: &TopologyDeclaredMutationSequence,
) -> Vec<schema::facade::platform::authority::TopologyMutation> {
    sequence
        .members()
        .flat_map(|contract| contract.lowered_mutations().iter().cloned())
        .collect()
}

pub(super) fn aggregate_topology_mutation_digest_for_plans(
    plans: impl IntoIterator<Item = TopologyCloseoutMutationPlan>,
) -> TopologyMutationDigest {
    let plans = plans.into_iter().collect::<Vec<_>>();
    digest_rows(
        plans
            .iter()
            .map(|plan| plan.topology_mutation_digest.digest.digest_hex.clone()),
    )
    .with_counts(
        plans
            .iter()
            .map(|plan| plan.topology_mutation_digest.mutation_record_count)
            .sum(),
        plans
            .iter()
            .map(|plan| plan.topology_mutation_digest.family_count)
            .sum(),
        plans
            .iter()
            .map(|plan| plan.topology_mutation_digest.changed_scope_count)
            .sum(),
        plans
            .iter()
            .map(|plan| plan.topology_mutation_digest.naming_scope_count)
            .sum(),
        plans
            .iter()
            .map(|plan| plan.topology_mutation_digest.derived_region_count)
            .sum(),
        plans
            .iter()
            .map(|plan| plan.topology_mutation_digest.fallback_policy_count)
            .sum(),
        plans
            .iter()
            .map(|plan| {
                plan.topology_mutation_digest
                    .fallback_rejection_policy_count
            })
            .sum(),
    )
}

pub(super) fn aggregate_topology_mutation_digest_for_declarations<D>(
    declarations: impl IntoIterator<Item = D>,
) -> TopologyMutationDigest
where
    D: TopologyDeclarationMutationPayload,
{
    aggregate_topology_mutation_digest_for_plans(
        declarations
            .into_iter()
            .map(closeout_mutation_plan_for_declaration),
    )
}

pub(super) fn topology_mutation_families_for_declarations<D>(
    declarations: impl IntoIterator<Item = D>,
) -> Vec<TopologyMutationFamily>
where
    D: TopologyDeclarationMutationPayload,
{
    declarations
        .into_iter()
        .flat_map(|declaration| declaration.semantic_families())
        .collect()
}

fn digest_rows(rows: impl IntoIterator<Item = String>) -> TopologyMutationSequenceDigest {
    let mut count = 0usize;
    let mut hash = 0xcbf29ce484222325u64;
    for row in rows {
        count += 1;
        for byte in row.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= u64::from(b'\n');
        hash = hash.wrapping_mul(0x100000001b3);
    }
    TopologyMutationSequenceDigest {
        algorithm: "fnv1a64".to_string(),
        digest_hex: format!("{hash:016x}"),
        row_count: count,
    }
}

trait WithCounts {
    fn with_counts(
        self,
        mutation_record_count: usize,
        family_count: usize,
        changed_scope_count: usize,
        naming_scope_count: usize,
        derived_region_count: usize,
        fallback_policy_count: usize,
        fallback_rejection_policy_count: usize,
    ) -> TopologyMutationDigest;
}

impl WithCounts for TopologyMutationSequenceDigest {
    fn with_counts(
        self,
        mutation_record_count: usize,
        family_count: usize,
        changed_scope_count: usize,
        naming_scope_count: usize,
        derived_region_count: usize,
        fallback_policy_count: usize,
        fallback_rejection_policy_count: usize,
    ) -> TopologyMutationDigest {
        TopologyMutationDigest {
            digest: self,
            mutation_record_count,
            family_count,
            changed_scope_count,
            naming_scope_count,
            derived_region_count,
            fallback_policy_count,
            fallback_rejection_policy_count,
        }
    }
}
