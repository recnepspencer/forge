use super::super::*;
use super::materialization::ProjectedMaterializationBasis;

pub(super) struct StagingActionBasis {
    pub(super) actions: Vec<RecoveryStagingAction>,
    pub(super) allocated_targets: Vec<PhysicalRedoTargetIdentity>,
}

pub(super) fn derive(
    redo: &ImmutablePhysicalRedoPlan,
    materialization: &ProjectedMaterializationBasis,
    staging_generation: u64,
) -> Result<StagingActionBasis, ExecutionBasisDenial> {
    let mut grouped = BTreeMap::<
        PhysicalRedoTargetIdentity,
        (PhysicalRedoTarget, Vec<RecoveryStagingRedoStep>),
    >::new();
    for decision in redo.resolved_decisions() {
        if decision.kind() != PhysicalRedoDecisionKind::Apply {
            continue;
        }
        let target = decision.target();
        let step = step(decision)?;
        match grouped.get_mut(&target.identity()) {
            Some((retained, steps)) if *retained == *target => steps.push(step),
            Some(_) => return Err(ExecutionBasisDenial::Invalid),
            None => {
                grouped.insert(target.identity(), (target.clone(), vec![step]));
            }
        }
    }
    let allocated_targets = grouped.keys().copied().collect::<Vec<_>>();
    let actions = grouped
        .into_iter()
        .enumerate()
        .map(|(ordinal, (identity, (source, steps)))| {
            materialization
                .frames
                .contains_key(&identity)
                .then_some(RecoveryStagingAction {
                    ordinal: ordinal as u64,
                    steps: steps.into_boxed_slice(),
                    source,
                    destination_generation: staging_generation,
                })
                .ok_or(ExecutionBasisDenial::Invalid)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(StagingActionBasis {
        actions,
        allocated_targets,
    })
}

fn step(
    decision: worth_store_recovery_physics::PhysicalRedoDecisionView<'_>,
) -> Result<RecoveryStagingRedoStep, ExecutionBasisDenial> {
    Ok(RecoveryStagingRedoStep {
        operation: decision.operation(),
        record_index: decision.record_index(),
        target_index: decision.target_index(),
        record_lsn: decision.record().lsn().get(),
        prior: match decision.prior() {
            PhysicalRedoDecisionPrior::Page(prior) => prior,
            PhysicalRedoDecisionPrior::OperationFate(_) => {
                return Err(ExecutionBasisDenial::Invalid)
            }
        },
    })
}
