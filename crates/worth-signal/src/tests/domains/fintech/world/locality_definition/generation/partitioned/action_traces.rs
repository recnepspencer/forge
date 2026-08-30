use super::{PartitionMutationWidth, PartitionScale};
use crate::tests::domains::fintech::world::locality_definition::{
    FinancialAspect, FinancialLocalityAction, FinancialLocalityActionTrace,
    FinancialLocalityMutation, FinancialLocalityTraceIdentity, LocalityScope,
    LocalitySemanticOutputId,
};

pub(super) fn generate(
    source: LocalitySemanticOutputId,
    correlated_source: LocalitySemanticOutputId,
    dimensions: &PartitionScale,
    mutation_width: PartitionMutationWidth,
) -> Vec<FinancialLocalityActionTrace> {
    let mutation_width = match mutation_width {
        PartitionMutationWidth::FixedDetail => 1,
        PartitionMutationWidth::ApproximatelyOnePercent => {
            approximately_one_percent_width(dimensions)
        }
    };
    vec![
        factor_trace(
            FinancialLocalityTraceIdentity::PrimaryMutation,
            scoped_curve_mutations(source, mutation_width, false),
        ),
        factor_trace(
            FinancialLocalityTraceIdentity::PartitionWholeRegion,
            scoped_curve_mutations(source, mutation_width, true),
        ),
        factor_trace(FinancialLocalityTraceIdentity::PartitionCorrelatedScopes, {
            let mut mutations = scoped_curve_mutations(source, mutation_width, false);
            mutations.extend([
                mutation(MutationDeclaration {
                    producer: correlated_source,
                    aspect: FinancialAspect::Price,
                    scope: LocalityScope::detail(500, 1),
                    // Direct invalidation generations are scoped to the source,
                    // so this correlated source starts its own successor sequence.
                    admission_generation: 2,
                    publication_order: u32::from(mutation_width),
                }),
                mutation(MutationDeclaration {
                    producer: correlated_source,
                    aspect: FinancialAspect::Volatility,
                    scope: LocalityScope::detail(501, 2),
                    admission_generation: 3,
                    publication_order: u32::from(mutation_width) + 1,
                }),
            ]);
            mutations
        }),
    ]
}

fn approximately_one_percent_width(dimensions: &PartitionScale) -> u16 {
    let total_outputs = 1usize
        + 1
        + usize::from(dimensions.instruments_per_matching_region)
        + usize::from(dimensions.matching_memberships.saturating_sub(1))
        + 2 * usize::from(dimensions.regions.saturating_sub(1))
        + 3;
    total_outputs.div_ceil(100) as u16
}

fn scoped_curve_mutations(
    source: LocalitySemanticOutputId,
    width: u16,
    whole_partition: bool,
) -> Vec<FinancialLocalityMutation> {
    (0..width)
        .map(|index| {
            let scope = if whole_partition {
                LocalityScope::partition(index)
            } else {
                LocalityScope::detail(index, 0)
            };
            mutation(MutationDeclaration {
                producer: source,
                aspect: FinancialAspect::Curve,
                scope,
                // One canonical source/aspect batch admission covers every
                // changed scope, so all regions share the same generation.
                admission_generation: 2,
                publication_order: u32::from(index),
            })
        })
        .collect()
}

fn factor_trace(
    identity: FinancialLocalityTraceIdentity,
    mutations: Vec<FinancialLocalityMutation>,
) -> FinancialLocalityActionTrace {
    FinancialLocalityActionTrace::new(
        identity,
        mutations
            .into_iter()
            .map(FinancialLocalityAction::CommitFactor)
            .collect(),
    )
}

struct MutationDeclaration {
    producer: LocalitySemanticOutputId,
    aspect: FinancialAspect,
    scope: LocalityScope,
    admission_generation: u64,
    publication_order: u32,
}

fn mutation(declaration: MutationDeclaration) -> FinancialLocalityMutation {
    FinancialLocalityMutation {
        producer: declaration.producer,
        aspect: declaration.aspect,
        scope: Some(declaration.scope),
        admission_generation: declaration.admission_generation,
        publication_order: declaration.publication_order,
    }
}
