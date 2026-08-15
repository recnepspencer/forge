use std::collections::BTreeSet;

use super::locality_scale::{LocalityLane, LocalityScaleTuple};
use super::{FinancialAspect, FinancialLocalityScenario};

mod actions;
mod generation;
mod policy;
#[cfg(test)]
mod tests;
mod validation;
pub(in crate::tests::domains::fintech) use actions::{
    FinancialLocalityAction, FinancialLocalityActionTrace, FinancialLocalitySourceObligation,
    FinancialLocalityStagedWork, FinancialLocalityTopologyChange, FinancialLocalityTraceIdentity,
};
pub(in crate::tests::domains::fintech) use policy::{
    FinancialLocalityAdmissionPolicy, FinancialLocalityComparisonPolicy,
    FinancialLocalityExecutionPolicy, FinancialLocalityOutputPolicy,
};
use validation::{terminal_outputs, topological_release_waves};

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::tests::domains::fintech) enum LocalityMarketFactor {
    Quote,
    FxSpot,
    Curve,
    Volatility,
}

impl LocalityMarketFactor {
    const fn aspect(self) -> FinancialAspect {
        match self {
            Self::Quote | Self::FxSpot => FinancialAspect::Price,
            Self::Curve => FinancialAspect::Curve,
            Self::Volatility => FinancialAspect::Volatility,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct LocalityFactorPublication {
    primary: LocalityMarketFactor,
    secondary: Option<LocalityMarketFactor>,
}

impl LocalityFactorPublication {
    pub(super) const fn one(primary: LocalityMarketFactor) -> Self {
        Self {
            primary,
            secondary: None,
        }
    }

    pub(super) const fn two(
        primary: LocalityMarketFactor,
        secondary: LocalityMarketFactor,
    ) -> Self {
        Self {
            primary,
            secondary: Some(secondary),
        }
    }

    fn aspects(self) -> BTreeSet<FinancialAspect> {
        [Some(self.primary), self.secondary]
            .into_iter()
            .flatten()
            .map(LocalityMarketFactor::aspect)
            .collect()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) enum FinancialLocalityFormula {
    MarketSource {
        publication: LocalityFactorPublication,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::tests::domains::fintech) struct FinancialLocalitySubscription {
    pub(in crate::tests::domains::fintech) upstream: LocalitySemanticOutputId,
    pub(in crate::tests::domains::fintech) input_aspect: FinancialAspect,
    pub(in crate::tests::domains::fintech) edge_scope: Option<LocalityScope>,
    pub(in crate::tests::domains::fintech) eligibility_scope: Option<LocalityScope>,
}

impl FinancialLocalitySubscription {
    fn unscoped(upstream: LocalitySemanticOutputId, input_aspect: FinancialAspect) -> Self {
        Self {
            upstream,
            input_aspect,
            edge_scope: None,
            eligibility_scope: None,
        }
    }

    fn scoped(
        upstream: LocalitySemanticOutputId,
        input_aspect: FinancialAspect,
        edge_scope: LocalityScope,
        eligibility_scope: LocalityScope,
    ) -> Self {
        Self {
            upstream,
            input_aspect,
            edge_scope: Some(edge_scope),
            eligibility_scope: Some(eligibility_scope),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct FinancialLocalityOutput {
    pub(in crate::tests::domains::fintech) id: LocalitySemanticOutputId,
    pub(in crate::tests::domains::fintech) owner: LocalityEconomicOwner,
    pub(in crate::tests::domains::fintech) role: LocalityOutputRole,
    pub(in crate::tests::domains::fintech) formula: FinancialLocalityFormula,
    pub(in crate::tests::domains::fintech) subscriptions: Vec<FinancialLocalitySubscription>,
}

impl FinancialLocalityOutput {
    pub(in crate::tests::domains::fintech) fn produced_aspects(&self) -> BTreeSet<FinancialAspect> {
        match self.formula {
            FinancialLocalityFormula::MarketSource { publication, .. } => publication.aspects(),
            FinancialLocalityFormula::LinearDependency { .. }
            | FinancialLocalityFormula::StableControl { .. } => {
                BTreeSet::from([role_output_aspect(self.role)])
            }
        }
    }
}

const fn role_output_aspect(role: LocalityOutputRole) -> FinancialAspect {
    match role {
        LocalityOutputRole::MarketQuote | LocalityOutputRole::PositionValuation => {
            FinancialAspect::Price
        }
        LocalityOutputRole::PositionRisk
        | LocalityOutputRole::BookAggregate
        | LocalityOutputRole::DeskAggregate => FinancialAspect::Risk,
        LocalityOutputRole::AuditCheck | LocalityOutputRole::RegulatoryReport => {
            FinancialAspect::Alert
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct FinancialLocalityWorkload {
    observation_targets: BTreeSet<LocalitySemanticOutputId>,
    release_waves: Vec<BTreeSet<LocalitySemanticOutputId>>,
    execution_posture: LocalityLane,
    baseline_aspect_version: u64,
    mutation_aspect_version: u64,
    readiness_epoch: u64,
}

impl FinancialLocalityWorkload {
    pub(in crate::tests::domains::fintech) fn observation_targets(
        &self,
    ) -> &BTreeSet<LocalitySemanticOutputId> {
        &self.observation_targets
    }

    pub(in crate::tests::domains::fintech) const fn baseline_aspect_version(&self) -> u64 {
        self.baseline_aspect_version
    }

    pub(in crate::tests::domains::fintech) const fn mutation_aspect_version(&self) -> u64 {
        self.mutation_aspect_version
    }

    pub(in crate::tests::domains::fintech) const fn readiness_epoch(&self) -> u64 {
        self.readiness_epoch
    }

    pub(in crate::tests::domains::fintech) fn release_waves(
        &self,
    ) -> &[BTreeSet<LocalitySemanticOutputId>] {
        &self.release_waves
    }

    pub(in crate::tests::domains::fintech) const fn execution_posture(&self) -> LocalityLane {
        self.execution_posture
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct FinancialLocalityMutation {
    pub(in crate::tests::domains::fintech) producer: LocalitySemanticOutputId,
    pub(in crate::tests::domains::fintech) aspect: FinancialAspect,
    pub(in crate::tests::domains::fintech) scope: Option<LocalityScope>,
    pub(in crate::tests::domains::fintech) admission_generation: u64,
    pub(in crate::tests::domains::fintech) publication_order: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct FinancialStructuralMutation {
    pub(in crate::tests::domains::fintech) target: LocalitySemanticOutputId,
    pub(in crate::tests::domains::fintech) topology_mutation_ordinal: u64,
    pub(in crate::tests::domains::fintech) resulting_dependency_revision: u64,
}

pub(super) struct LocalityGenerationContract {
    action_traces: Vec<FinancialLocalityActionTrace>,
    execution_posture: LocalityLane,
}

impl LocalityGenerationContract {
    pub(super) fn direct(
        mutation: FinancialLocalityMutation,
        execution_posture: LocalityLane,
    ) -> Self {
        Self {
            action_traces: vec![FinancialLocalityActionTrace::new(
                FinancialLocalityTraceIdentity::PrimaryMutation,
                vec![FinancialLocalityAction::CommitFactor(mutation)],
            )],
            execution_posture,
        }
    }

    pub(super) fn traced(
        action_traces: Vec<FinancialLocalityActionTrace>,
        execution_posture: LocalityLane,
    ) -> Self {
        assert!(!action_traces.is_empty());
        Self {
            action_traces,
            execution_posture,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct FinancialLocalityDefinition {
    seed: u64,
    scale: LocalityScaleTuple,
    outputs: Vec<FinancialLocalityOutput>,
    mutations: Vec<FinancialLocalityMutation>,
    structural_mutations: Vec<FinancialStructuralMutation>,
    action_traces: Vec<FinancialLocalityActionTrace>,
    workload: FinancialLocalityWorkload,
}

impl FinancialLocalityDefinition {
    fn generated(
        seed: u64,
        scale: LocalityScaleTuple,
        outputs: Vec<FinancialLocalityOutput>,
        contract: LocalityGenerationContract,
    ) -> Self {
        let observation_targets = terminal_outputs(&outputs);
        let release_waves = topological_release_waves(&outputs);
        let mutations = contract.action_traces[0].committed_mutations();
        let structural_mutations = contract.action_traces[0].structural_mutations();
        let readiness_epoch = contract.action_traces[0].readiness_epoch();
        Self {
            seed,
            scale,
            outputs,
            mutations,
            structural_mutations,
            action_traces: contract.action_traces,
            workload: FinancialLocalityWorkload {
                observation_targets,
                release_waves,
                execution_posture: contract.execution_posture,
                baseline_aspect_version: 1,
                mutation_aspect_version: 2,
                readiness_epoch,
            },
        }
    }

    pub(in crate::tests::domains::fintech) const fn seed(&self) -> u64 {
        self.seed
    }

    pub(in crate::tests::domains::fintech) fn outputs(&self) -> &[FinancialLocalityOutput] {
        &self.outputs
    }

    pub(in crate::tests::domains::fintech) fn mutation(&self) -> FinancialLocalityMutation {
        self.mutations[0]
    }

    pub(in crate::tests::domains::fintech) fn mutations(&self) -> &[FinancialLocalityMutation] {
        &self.mutations
    }

    pub(in crate::tests::domains::fintech) fn action_traces(
        &self,
    ) -> &[FinancialLocalityActionTrace] {
        &self.action_traces
    }

    pub(in crate::tests::domains::fintech) const fn scale(&self) -> LocalityScaleTuple {
        self.scale
    }

    pub(in crate::tests::domains::fintech) const fn workload(&self) -> &FinancialLocalityWorkload {
        &self.workload
    }

    pub(in crate::tests::domains::fintech) const fn scenario(&self) -> FinancialLocalityScenario {
        self.scale.scenario()
    }
}
