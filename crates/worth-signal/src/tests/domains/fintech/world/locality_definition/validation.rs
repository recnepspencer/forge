use std::collections::BTreeSet;

use super::super::locality_scale::{LocalityScaleTuple, RestorePosture};
use super::{
    FinancialLocalityAction, FinancialLocalityDefinition, FinancialLocalityFormula,
    FinancialLocalityOutput, FinancialLocalityTraceIdentity, LocalitySemanticOutputId,
};

impl FinancialLocalityDefinition {
    pub(in crate::tests::domains::fintech) fn validate_generator_invariants(&self) {
        self.validate_scenario_shape();
        self.validate_output_ownership_and_topology();
        self.validate_workload_contract();
        self.validate_origin_contracts();
        self.validate_action_trace_contract();
    }

    fn validate_scenario_shape(&self) {
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
            LocalityScaleTuple::ConvergentFactorBatch { .. } => {
                assert_eq!(self.outputs.len(), 5);
                assert_eq!(self.mutations.len(), 4);
                assert_eq!(self.workload.release_waves[0].len(), 4);
            }
            LocalityScaleTuple::DenseMarketClose { total_outputs, .. } => {
                assert_eq!(self.outputs.len(), total_outputs as usize);
                assert_eq!(self.mutations.len(), 1);
            }
            LocalityScaleTuple::PortfolioDependencyChurn { rounds, .. } => {
                assert_eq!(self.outputs.len(), 4);
                assert_eq!(self.structural_mutations.len(), usize::from(rounds) * 3);
            }
            LocalityScaleTuple::BranchRestoreLocalityReplay {
                posture,
                total_outputs,
                ..
            } => self.validate_restore_shape(posture, total_outputs),
        }
    }

    fn validate_action_trace_contract(&self) {
        match self.scale {
            LocalityScaleTuple::SparseBookFanout { .. }
            | LocalityScaleTuple::DenseMarketClose { .. } => {
                assert_eq!(self.action_traces.len(), 1);
            }
            LocalityScaleTuple::PartitionedCurveUniverse { .. } => {
                assert_eq!(
                    self.action_traces
                        .iter()
                        .map(|trace| trace.identity())
                        .collect::<BTreeSet<_>>(),
                    [
                        FinancialLocalityTraceIdentity::PrimaryMutation,
                        FinancialLocalityTraceIdentity::PartitionWholeRegion,
                        FinancialLocalityTraceIdentity::PartitionCorrelatedScopes,
                    ]
                    .into()
                );
            }
            LocalityScaleTuple::ConvergentFactorBatch {
                duplicate_admissions,
                ..
            } => {
                assert_eq!(self.action_traces.len(), 24);
                assert!(self.action_traces.iter().all(|trace| {
                    trace.committed_mutations().len() == 4
                        && trace.retry_count() == usize::from(duplicate_admissions)
                }));
            }
            LocalityScaleTuple::PortfolioDependencyChurn { rounds, .. } => {
                let trace = &self.action_traces[0];
                assert_eq!(
                    trace.identity(),
                    FinancialLocalityTraceIdentity::PortfolioChurn
                );
                assert_eq!(trace.actions().len(), usize::from(rounds) * 8);
                assert_eq!(trace.committed_mutations().len(), usize::from(rounds) * 2);
                assert_eq!(
                    self.structural_mutations
                        .last()
                        .unwrap()
                        .resulting_dependency_revision,
                    1 + u64::from(rounds) * 3
                );
            }
            LocalityScaleTuple::BranchRestoreLocalityReplay { .. } => {
                let trace = &self.action_traces[0];
                assert_eq!(
                    trace.identity(),
                    FinancialLocalityTraceIdentity::BranchRestoreReplay
                );
                assert!(trace.actions().iter().any(|action| matches!(
                    action,
                    FinancialLocalityAction::ReadmitFreshRuntime { runtime_epoch: 2 }
                )));
                assert_eq!(trace.readiness_epoch(), 2);
            }
        }
    }

    fn validate_restore_shape(&self, posture: RestorePosture, total_outputs: u32) {
        match posture {
            RestorePosture::Narrow => assert_eq!(self.outputs.len(), 2),
            RestorePosture::Convergent => assert_eq!(self.outputs.len(), 5),
            RestorePosture::DenseFourInFive => {
                assert_eq!(self.outputs.len(), total_outputs as usize)
            }
        }
    }

    fn validate_output_ownership_and_topology(&self) {
        let mut owners = BTreeSet::new();
        for (ordinal, output) in self.outputs.iter().enumerate() {
            assert_eq!(output.id.ordinal(), ordinal as u32);
            assert!(
                owners.insert(output.owner),
                "economic owners must be unique"
            );
            assert!(!output.produced_aspects().is_empty());
            if !matches!(
                output.formula,
                FinancialLocalityFormula::MarketSource { .. }
            ) {
                assert!(!output.subscriptions.is_empty());
            }
            for subscription in &output.subscriptions {
                assert!(subscription.upstream.ordinal() < output.id.ordinal());
                assert!(self.outputs[subscription.upstream.ordinal() as usize]
                    .produced_aspects()
                    .contains(&subscription.input_aspect));
            }
        }
    }

    fn validate_workload_contract(&self) {
        assert_eq!(
            self.workload.observation_targets,
            terminal_outputs(&self.outputs),
            "the declared workload must observe every real terminal output"
        );
        assert!(!self.workload.observation_targets.is_empty());
        assert_eq!(
            self.workload.release_waves,
            topological_release_waves(&self.outputs),
            "the hostile release schedule must be declared from compiler topology"
        );
    }

    fn validate_origin_contracts(&self) {
        assert!(!self.mutations.is_empty());
        let mut producer_generations = BTreeSet::new();
        for mutation in &self.mutations {
            assert!(
                producer_generations.insert((mutation.producer, mutation.admission_generation,))
            );
            assert!(matches!(
                self.outputs[mutation.producer.ordinal() as usize].formula,
                FinancialLocalityFormula::MarketSource { .. }
            ));
        }
        let mut topology_ordinals = BTreeSet::new();
        for structural in &self.structural_mutations {
            assert!(structural.target.ordinal() < self.outputs.len() as u32);
            assert!(topology_ordinals.insert(structural.topology_mutation_ordinal));
            assert!(structural.resulting_dependency_revision > 0);
        }
    }

    fn validate_partition_axes(
        &self,
        regions: u16,
        matching_memberships: u16,
        instruments_per_matching_region: u16,
    ) {
        let mutation = self.mutation();
        let source_dependencies = self
            .outputs
            .iter()
            .flat_map(|output| &output.subscriptions)
            .filter(|subscription| subscription.upstream == mutation.producer)
            .collect::<Vec<_>>();
        let queried_memberships = source_dependencies
            .iter()
            .filter(|subscription| subscription.edge_scope == mutation.scope)
            .count();
        let admitted_memberships = source_dependencies
            .iter()
            .filter(|subscription| {
                subscription.edge_scope == mutation.scope
                    && subscription.eligibility_scope == mutation.scope
            })
            .count();
        assert_eq!(
            source_dependencies.len(),
            usize::from(regions + matching_memberships)
        );
        assert_eq!(queried_memberships, usize::from(matching_memberships));
        assert_eq!(admitted_memberships, 1);
        let admitted_membership = self.admitted_partition_membership(mutation);
        assert_eq!(
            self.outputs
                .iter()
                .filter(|output| {
                    output
                        .subscriptions
                        .iter()
                        .any(|subscription| subscription.upstream == admitted_membership)
                })
                .count(),
            usize::from(instruments_per_matching_region)
        );
        assert_eq!(
            self.outputs.len(),
            usize::from(2 * regions + matching_memberships + instruments_per_matching_region + 3)
        );
    }

    fn admitted_partition_membership(
        &self,
        mutation: super::FinancialLocalityMutation,
    ) -> LocalitySemanticOutputId {
        self.outputs
            .iter()
            .find(|output| {
                output.subscriptions.iter().any(|subscription| {
                    subscription.upstream == mutation.producer
                        && subscription.edge_scope == mutation.scope
                        && subscription.eligibility_scope == mutation.scope
                })
            })
            .expect("one matching membership is required")
            .id
    }
}

pub(super) fn terminal_outputs(
    outputs: &[FinancialLocalityOutput],
) -> BTreeSet<LocalitySemanticOutputId> {
    let producers = outputs
        .iter()
        .flat_map(|output| output.subscriptions.iter())
        .map(|subscription| subscription.upstream)
        .collect::<BTreeSet<_>>();
    outputs
        .iter()
        .filter_map(|output| (!producers.contains(&output.id)).then_some(output.id))
        .collect()
}

pub(super) fn topological_release_waves(
    outputs: &[FinancialLocalityOutput],
) -> Vec<BTreeSet<LocalitySemanticOutputId>> {
    let mut stages = Vec::<u32>::with_capacity(outputs.len());
    let mut waves = Vec::<BTreeSet<LocalitySemanticOutputId>>::new();
    for output in outputs {
        let stage = output
            .subscriptions
            .iter()
            .map(|subscription| stages[subscription.upstream.ordinal() as usize] + 1)
            .max()
            .unwrap_or(0);
        stages.push(stage);
        if waves.len() <= stage as usize {
            waves.resize_with(stage as usize + 1, BTreeSet::new);
        }
        waves[stage as usize].insert(output.id);
    }
    waves
}
