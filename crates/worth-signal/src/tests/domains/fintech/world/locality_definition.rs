use std::collections::BTreeSet;

use super::locality_scale::LocalityScaleTuple;
use super::{FinancialAspect, FinancialLocalityScenario};

mod generation;
#[cfg(test)]
mod tests;

const RELEVANT_CHAIN_OUTPUTS: u32 = 16;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::tests::domains::fintech) struct LocalitySemanticOutputId(u32);

impl LocalitySemanticOutputId {
    pub(in crate::tests::domains::fintech) const fn new(ordinal: u32) -> Self {
        Self(ordinal)
    }

    pub(in crate::tests::domains::fintech) const fn ordinal(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::tests::domains::fintech) enum LocalityEconomicOwner {
    MarketDataFeed(u32),
    Position(u32),
    BookRisk(u16),
    DeskRisk(u16),
    AuditControl(u32),
    RegulatoryReport(u32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::tests::domains::fintech) enum LocalityOutputRole {
    MarketQuote,
    PositionValuation,
    PositionRisk,
    BookAggregate,
    DeskAggregate,
    AuditCheck,
    RegulatoryReport,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) enum FinancialLocalityFormula {
    MarketSource {
        baseline_value: i64,
        mutation_delta: i64,
    },
    LinearDependency {
        multiplier_micros: i64,
        basis_value: i64,
    },
    StableControl {
        retained_value: i64,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::tests::domains::fintech) struct LocalityScope {
    pub(in crate::tests::domains::fintech) region: u16,
    pub(in crate::tests::domains::fintech) detail: Option<u16>,
}

impl LocalityScope {
    pub(in crate::tests::domains::fintech) const fn partition(region: u16) -> Self {
        Self {
            region,
            detail: None,
        }
    }

    pub(in crate::tests::domains::fintech) const fn detail(region: u16, detail: u16) -> Self {
        Self {
            region,
            detail: Some(detail),
        }
    }

    pub(super) fn partition_label(self) -> String {
        format!("curve-region-{}", self.region)
    }

    pub(super) fn detail_label(self) -> Option<String> {
        self.detail.map(|detail| format!("bucket-{detail}"))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct FinancialLocalityDependency {
    pub(in crate::tests::domains::fintech) producer: LocalitySemanticOutputId,
    pub(in crate::tests::domains::fintech) aspect: FinancialAspect,
    pub(in crate::tests::domains::fintech) edge_scope: Option<LocalityScope>,
    pub(in crate::tests::domains::fintech) contract_scope: Option<LocalityScope>,
}

impl FinancialLocalityDependency {
    fn unscoped(producer: LocalitySemanticOutputId, aspect: FinancialAspect) -> Self {
        Self {
            producer,
            aspect,
            edge_scope: None,
            contract_scope: None,
        }
    }

    fn scoped(
        producer: LocalitySemanticOutputId,
        aspect: FinancialAspect,
        edge_scope: LocalityScope,
        contract_scope: LocalityScope,
    ) -> Self {
        Self {
            producer,
            aspect,
            edge_scope: Some(edge_scope),
            contract_scope: Some(contract_scope),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct FinancialLocalityOutput {
    pub(in crate::tests::domains::fintech) id: LocalitySemanticOutputId,
    pub(in crate::tests::domains::fintech) owner: LocalityEconomicOwner,
    pub(in crate::tests::domains::fintech) role: LocalityOutputRole,
    pub(in crate::tests::domains::fintech) formula: FinancialLocalityFormula,
    pub(in crate::tests::domains::fintech) produced_aspects: BTreeSet<FinancialAspect>,
    pub(in crate::tests::domains::fintech) dependencies: Vec<FinancialLocalityDependency>,
    pub(in crate::tests::domains::fintech) expected_for_mutation: bool,
    pub(in crate::tests::domains::fintech) unchanged_output_stop: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct FinancialLocalityMutation {
    pub(in crate::tests::domains::fintech) producer: LocalitySemanticOutputId,
    pub(in crate::tests::domains::fintech) aspect: FinancialAspect,
    pub(in crate::tests::domains::fintech) scope: Option<LocalityScope>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct FinancialLocalityDefinition {
    seed: u64,
    scale: LocalityScaleTuple,
    outputs: Vec<FinancialLocalityOutput>,
    mutation: FinancialLocalityMutation,
}

impl FinancialLocalityDefinition {
    pub(in crate::tests::domains::fintech) const fn seed(&self) -> u64 {
        self.seed
    }

    pub(in crate::tests::domains::fintech) fn outputs(&self) -> &[FinancialLocalityOutput] {
        &self.outputs
    }

    pub(in crate::tests::domains::fintech) const fn mutation(&self) -> FinancialLocalityMutation {
        self.mutation
    }

    pub(in crate::tests::domains::fintech) fn validate_generator_invariants(&self) {
        match self.scale {
            LocalityScaleTuple::SparseBookFanout { total_outputs, .. } => {
                assert_eq!(self.outputs.len(), total_outputs as usize);
            }
            LocalityScaleTuple::PartitionedCurveUniverse {
                regions,
                matching_memberships,
                instruments_per_matching_region,
            } => self.validate_partition_axes(
                regions,
                matching_memberships,
                instruments_per_matching_region,
            ),
            _ => {}
        }
        let mut owners = BTreeSet::new();
        for (ordinal, output) in self.outputs.iter().enumerate() {
            assert_eq!(output.id.ordinal(), ordinal as u32);
            assert!(
                owners.insert(output.owner),
                "economic owners must be unique"
            );
            assert!(!output.produced_aspects.is_empty());
            if ordinal > 0 {
                assert!(!output.dependencies.is_empty());
            }
            for dependency in &output.dependencies {
                assert!(dependency.producer.ordinal() < output.id.ordinal());
            }
        }
    }

    fn validate_partition_axes(
        &self,
        regions: u16,
        matching_memberships: u16,
        instruments_per_matching_region: u16,
    ) {
        let mutation = self.mutation;
        let source_dependencies = self
            .outputs
            .iter()
            .flat_map(|output| &output.dependencies)
            .filter(|dependency| dependency.producer == mutation.producer)
            .collect::<Vec<_>>();
        let queried_memberships = source_dependencies
            .iter()
            .filter(|dependency| dependency.edge_scope == mutation.scope)
            .count();
        let admitted_memberships = source_dependencies
            .iter()
            .filter(|dependency| {
                dependency.edge_scope == mutation.scope
                    && dependency.contract_scope == mutation.scope
            })
            .count();
        assert_eq!(
            source_dependencies.len(),
            usize::from(regions + matching_memberships - 1)
        );
        assert_eq!(queried_memberships, usize::from(matching_memberships));
        assert_eq!(admitted_memberships, 1);
        assert_eq!(
            self.outputs
                .iter()
                .filter(|output| output.expected_for_mutation)
                .count(),
            usize::from(instruments_per_matching_region) + 2
        );
        assert_eq!(
            self.outputs.len(),
            usize::from(2 * regions + matching_memberships + instruments_per_matching_region - 1)
        );
    }

    pub(in crate::tests::domains::fintech) const fn scenario(&self) -> FinancialLocalityScenario {
        self.scale.scenario()
    }
}
