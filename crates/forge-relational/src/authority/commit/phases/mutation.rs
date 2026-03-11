use serde_json::json;

use crate::capabilities::DiagnosticsSink;
use crate::authority::mutation::{apply_plan_to_draft, MutationEffect};
use crate::diagnostics::data::{DiagnosticCode, DiagnosticsScope};
use crate::identity::data::VersionId;
use crate::transactions::data::{
    AuthoritativeApplyPlan, MergedCommitPlan, TransactionCommitError,
};
use crate::transactions::logic::RelationalTransaction;

use super::invariants::run_mutation_sensitive_invariants;
use super::prepare::record_mutation_counters;

pub(crate) struct MutationPhaseOutput {
    pub(crate) version_id: VersionId,
    pub(crate) effect: MutationEffect,
}

pub(crate) fn run_authoritative_mutation(
    transaction: &mut RelationalTransaction<'_>,
    draft: &mut crate::logic::runtime::RelationalDraft,
    merged_plan: &MergedCommitPlan,
) -> Result<MutationPhaseOutput, TransactionCommitError> {
    let version_id = VersionId(transaction.runtime.history.next_version_id);
    let apply_plan = AuthoritativeApplyPlan {
        transaction_id: transaction.transaction_id,
        version_id,
        merged_intents: merged_plan.merged_intents.clone(),
    };
    let mutation_config = transaction.runtime.mutation_config();
    let effect = apply_plan_to_draft(
        draft,
        &apply_plan,
        &mutation_config,
        &transaction.runtime.config.schema_registry,
        &mut transaction.runtime.symbols,
    )
    .map_err(TransactionCommitError::Conflict)?;
    record_mutation_counters(transaction.runtime, draft);

    {
        let overlay_state = transaction.runtime.overlay_state_view(draft);
        if let Err(error) = run_mutation_sensitive_invariants(
            transaction.runtime,
            &overlay_state,
            version_id,
            merged_plan,
        ) {
            transaction.runtime.emit_diagnostic_entry(
                DiagnosticsScope::Invariant,
                DiagnosticCode::InvariantViolation,
                error.detail.clone(),
                json!({ "execution_point": "mutation_sensitive" }),
            );
            return Err(TransactionCommitError::Conflict(error));
        }
    }

    Ok(MutationPhaseOutput { version_id, effect })
}
