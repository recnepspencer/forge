use std::collections::BTreeSet;

use super::{ExpectedTrace, FinancialLocalityDefinition, FinancialLocalitySourceObligation};
use crate::tests::domains::fintech::certification::invalidation::locality_expectation::{
    ExpectedActionCheckpoint, ExpectedActionCheckpointKind, ExpectedCanonicalWork,
    ExpectedDependencyCause, ExpectedDirectSourceBasis, ExpectedGraphBinding,
};

impl ExpectedTrace {
    pub(in crate::tests::domains::fintech::certification::invalidation::locality_expectation) fn current_source_bases(
        &self,
        definition: &FinancialLocalityDefinition,
        graph_instance: u64,
    ) -> BTreeSet<ExpectedDirectSourceBasis> {
        source_bases(
            definition,
            graph_instance,
            self.final_readiness_epoch,
            &self.current_source_bases,
        )
    }

    pub(in crate::tests::domains::fintech::certification::invalidation::locality_expectation) fn action_checkpoints(
        &self,
        definition: &FinancialLocalityDefinition,
        graph_instance: u64,
        causes: &BTreeSet<ExpectedDependencyCause>,
        final_work: &ExpectedCanonicalWork,
    ) -> Vec<ExpectedActionCheckpoint> {
        let pre_restore_work = rebind_work(final_work, 1);
        self.checkpoints
            .iter()
            .map(|checkpoint| {
                let (canonical_causes, persisted_causes, canonical_work) = match checkpoint.kind {
                    ExpectedActionCheckpointKind::BranchCaptured
                    | ExpectedActionCheckpointKind::CheckpointCaptured => {
                        (causes.clone(), causes.clone(), pre_restore_work.clone())
                    }
                    ExpectedActionCheckpointKind::DerivedStateDestroyed => (
                        BTreeSet::new(),
                        causes.clone(),
                        ExpectedCanonicalWork::new(),
                    ),
                    ExpectedActionCheckpointKind::CausesReadmitted => {
                        (causes.clone(), causes.clone(), ExpectedCanonicalWork::new())
                    }
                    ExpectedActionCheckpointKind::ReadyWorkReconstructed
                    | ExpectedActionCheckpointKind::DeterministicRerun => {
                        (causes.clone(), causes.clone(), final_work.clone())
                    }
                    ExpectedActionCheckpointKind::PreRewireStaged(_)
                    | ExpectedActionCheckpointKind::TopologyAccepted(_)
                    | ExpectedActionCheckpointKind::StaleWorkDenied { .. }
                    | ExpectedActionCheckpointKind::CycleRejected { .. } => (
                        BTreeSet::new(),
                        BTreeSet::new(),
                        ExpectedCanonicalWork::new(),
                    ),
                };
                ExpectedActionCheckpoint {
                    action_ordinal: checkpoint.action_ordinal,
                    kind: checkpoint.kind,
                    canonical_causes,
                    persisted_causes,
                    canonical_work,
                    current_source_bases: source_bases(
                        definition,
                        graph_instance,
                        checkpoint.runtime_epoch,
                        &checkpoint.current_source_bases,
                    ),
                    persisted_source_bases: source_bases(
                        definition,
                        graph_instance,
                        checkpoint.persisted_runtime_epoch,
                        &checkpoint.persisted_source_bases,
                    ),
                }
            })
            .collect()
    }
}

fn rebind_work(work: &ExpectedCanonicalWork, readiness_epoch: u64) -> ExpectedCanonicalWork {
    work.iter()
        .map(|(identity, origins)| {
            let mut identity = identity.clone();
            identity.readiness_epoch = readiness_epoch;
            (identity, origins.clone())
        })
        .collect()
}

fn source_bases(
    definition: &FinancialLocalityDefinition,
    graph_instance: u64,
    runtime_epoch: u64,
    obligations: &[FinancialLocalitySourceObligation],
) -> BTreeSet<ExpectedDirectSourceBasis> {
    let graph = ExpectedGraphBinding {
        graph_instance,
        seed: definition.seed(),
        scale: definition.scale(),
    };
    obligations
        .iter()
        .map(|obligation| ExpectedDirectSourceBasis {
            graph,
            source: obligation.source,
            aspect: obligation.aspect,
            scope: obligation.scope,
            admission_generation: obligation.admission_generation,
            dependency_revision: obligation.dependency_revision,
            runtime_epoch,
        })
        .collect()
}
