use super::super::locality_scale::{LocalityScaleTuple, SparseFanoutAxis};
use super::{FinancialLocalityDefinition, FinancialLocalityScenario, LocalityScope};

#[test]
fn sparse_generator_owns_every_exact_output_and_preserves_depth_sixteen_chain() {
    for axis in [
        SparseFanoutAxis::IndexDisjoint,
        SparseFanoutAxis::QueriedRejecting,
        SparseFanoutAxis::RejectedDescendants,
    ] {
        let definition = FinancialLocalityDefinition::generate(
            41,
            LocalityScaleTuple::SparseBookFanout {
                total_outputs: 64,
                axis,
            },
        );
        definition.validate_generator_invariants();
        assert_eq!(definition.seed(), 41);
        assert_eq!(
            definition.outputs()[15].dependencies[0].producer.ordinal(),
            14
        );
    }
}

#[test]
fn partition_generator_varies_regions_memberships_and_instruments_without_padding() {
    let definition = FinancialLocalityDefinition::generate(
        41,
        LocalityScaleTuple::PartitionedCurveUniverse {
            regions: 16,
            matching_memberships: 4,
            instruments_per_matching_region: 8,
        },
    );
    definition.validate_generator_invariants();
    assert_eq!(
        definition.scenario(),
        FinancialLocalityScenario::PartitionedCurveUniverse
    );
    assert_eq!(definition.outputs().len(), 1 + (1 + 8) + 3 + 2 * 15);
    assert_eq!(definition.mutation().scope.unwrap().region, 0);
    assert_eq!(
        definition
            .outputs()
            .iter()
            .filter(|output| output.expected_for_mutation)
            .count(),
        1 + 1 + 8
    );
}

#[test]
fn scheduled_partition_tuple_preserves_independent_r_m_and_i_axes() {
    let definition = FinancialLocalityDefinition::generate(
        41,
        LocalityScaleTuple::PartitionedCurveUniverse {
            regions: 1_024,
            matching_memberships: 256,
            instruments_per_matching_region: 32,
        },
    );

    definition.validate_generator_invariants();
    assert_eq!(definition.outputs().len(), 2 * 1_024 + 256 + 32 - 1);
    assert_eq!(
        definition
            .outputs()
            .iter()
            .filter(|output| output.expected_for_mutation)
            .count(),
        34
    );
}

#[test]
fn scheduled_sparse_rejection_contracts_never_wrap_into_the_queried_detail() {
    let definition = FinancialLocalityDefinition::generate(
        41,
        LocalityScaleTuple::SparseBookFanout {
            total_outputs: 100_000,
            axis: SparseFanoutAxis::QueriedRejecting,
        },
    );
    let queried = Some(LocalityScope::detail(0, 0));

    definition.validate_generator_invariants();
    assert!(definition.outputs()[16..].iter().all(|output| {
        output.dependencies[0].edge_scope == queried
            && output.dependencies[0].contract_scope != queried
    }));
}
