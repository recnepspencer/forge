use super::boundary_validation::BoundaryValidatedCommitExecution;

mod phase;

use phase::{run_authoritative_mutation_phase, MutationPhaseInput};

pub(super) struct MutatedCommitExecution {
    validated: BoundaryValidatedCommitExecution,
    version_id: crate::identity::data::VersionId,
    effect: crate::authority::mutation::MutationEffect,
    created_entities: crate::transactions::data::CommitCreatedEntityBindings,
}

impl MutatedCommitExecution {
    pub(super) fn validated_mut(&mut self) -> &mut BoundaryValidatedCommitExecution {
        &mut self.validated
    }

    pub(super) fn version_id(&self) -> crate::identity::data::VersionId {
        self.version_id
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        BoundaryValidatedCommitExecution,
        crate::identity::data::VersionId,
        crate::authority::mutation::MutationEffect,
        crate::transactions::data::CommitCreatedEntityBindings,
    ) {
        (
            self.validated,
            self.version_id,
            self.effect,
            self.created_entities,
        )
    }
}

pub(super) fn mutate_commit_execution(
    runtime: &mut crate::logic::runtime::RelationalRuntime,
    mut validated: BoundaryValidatedCommitExecution,
) -> Result<MutatedCommitExecution, crate::transactions::data::TransactionCommitError> {
    let (admitted, working_state) = validated.prepared_mut().mutation_parts();
    let (transaction_id, options, merged_plan, _, commit_log, phase_timing) =
        admitted.phase_view().into_parts();
    let mutation = run_authoritative_mutation_phase(
        runtime,
        MutationPhaseInput {
            commit_log,
            phase_timing,
            transaction_id,
            working_state,
            merged_plan,
            target_branch: options.target_branch.as_ref(),
        },
    )?;
    let (version_id, effect, invariant_results, created_entities) = mutation.into_parts();
    validated.push_invariant(invariant_results);
    Ok(MutatedCommitExecution {
        validated,
        version_id,
        effect,
        created_entities,
    })
}
