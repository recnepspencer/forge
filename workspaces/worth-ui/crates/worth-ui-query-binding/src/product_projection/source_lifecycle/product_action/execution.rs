use worth_query::facade::runtime;
use worth_runtime_bridge::facade::{
    AdmittedBridgeAsyncRequestIdentity, BridgeMixedCauseOrderingInput,
    BridgeMixedCauseOrderingLaneKind, BridgeMixedCauseOrderingRequest, RuntimeBridge,
};

use crate::{
    UiProjectionConsumptionBudget, UiScalarProjectionBatchOutcome, UiScalarProjectionBinding,
    UiScalarProjectionFactReceipt,
};

use super::{
    WorthUiScalarProjectionActionAdvance, WorthUiScalarProjectionActionEvidence,
    WorthUiScalarProjectionActionExecution, WorthUiScalarProjectionActionIndeterminate,
    WorthUiScalarProjectionActionOutcome, WorthUiScalarProjectionActionRequest,
};
use crate::product_projection::source_lifecycle::{
    admitted_completion, issue_advance, revalidate, ScalarLiveView,
    WorthUiScalarProjectionLiveOwner, WorthUiScalarProjectionUnpublishedOwner,
};

struct ActionOwnerParts {
    workspace: runtime::WorthQueryWorkspace,
    bridge: RuntimeBridge,
    source: crate::product_projection::SharedSourceState,
    view: ScalarLiveView,
    binding: UiScalarProjectionBinding,
    request: AdmittedBridgeAsyncRequestIdentity,
    predecessor: Option<UiScalarProjectionFactReceipt>,
    revision: u64,
    basis_revision: u64,
}

pub(super) fn execute_query_action(
    owner: WorthUiScalarProjectionLiveOwner,
    request: WorthUiScalarProjectionActionRequest,
) -> WorthUiScalarProjectionActionOutcome {
    let mut parts = ActionOwnerParts::from(owner);
    let declaration = product_action_declaration(&request);
    let receipt = match parts.workspace.intent(declaration) {
        Ok(receipt) => receipt,
        Err(error) => {
            return indeterminate(parts, format!("Query intent outcome unknown: {error}"))
        }
    };
    let evidence = WorthUiScalarProjectionActionEvidence {
        source_revision: request.source_revision,
        status: request.status.clone(),
        query_receipt_digest: receipt.receipt_digest().to_string(),
        affected_live_view_ids: receipt.terminal_affected_live_view_ids_projection(),
    };
    let next_basis_revision = parts.basis_revision.saturating_add(1);
    let predecessor = parts
        .predecessor
        .take()
        .expect("a live product action owner retains one predecessor fact");
    match revalidate(
        &parts.bridge,
        &mut parts.workspace,
        &parts.view,
        &mut parts.binding,
        &parts.request,
        predecessor,
        next_basis_revision,
    ) {
        Ok((request, predecessor)) => {
            parts.request = request;
            parts.predecessor = Some(predecessor);
        }
        Err(stop) => {
            let (error, request, predecessor) = stop.into_parts();
            parts.request = request;
            parts.predecessor = Some(predecessor);
            return indeterminate(
                parts,
                format!("Query intent committed but projection revalidation failed: {error:?}"),
            );
        }
    }
    finish_action_projection(parts, evidence, next_basis_revision)
}

fn finish_action_projection(
    mut parts: ActionOwnerParts,
    evidence: WorthUiScalarProjectionActionEvidence,
    next_basis_revision: u64,
) -> WorthUiScalarProjectionActionOutcome {
    let payload_bytes = evidence.status.len().saturating_add(8) as u64;
    let completion = match admitted_completion(&parts.bridge, &parts.request, payload_bytes) {
        Ok(completion) => completion,
        Err(error) => {
            return indeterminate(
                parts,
                format!("Query intent committed but projection completion failed: {error:?}"),
            );
        }
    };
    let ordering = parts
        .bridge
        .order_mixed_causes(&BridgeMixedCauseOrderingRequest::new(
            BridgeMixedCauseOrderingLaneKind::Authoritative,
            vec![BridgeMixedCauseOrderingInput::AsyncCompletion(completion)],
        ));
    let batch = match parts
        .workspace
        .admit_bridge_async_result_transitions(&parts.view, &ordering)
    {
        Ok(batch) => batch,
        Err(error) => {
            return indeterminate(
                parts,
                format!("Query intent committed but projection delivery failed: {error:?}"),
            );
        }
    };
    let predecessor = parts
        .predecessor
        .take()
        .expect("action projection completion retains its revalidating predecessor");
    let transition = match parts.binding.consume_async_result_batch(
        &mut parts.workspace,
        batch,
        Some(predecessor),
        UiProjectionConsumptionBudget::platform_pulse(),
    ) {
        UiScalarProjectionBatchOutcome::Advanced(transition) => transition,
        UiScalarProjectionBatchOutcome::Unchanged(unchanged) => {
            parts.predecessor = unchanged.into_predecessor();
            return indeterminate(
                parts,
                "Query intent committed but projection completion was unexpectedly unchanged",
            );
        }
    };
    let (fact, retained) = transition.into_fact_and_predecessor();
    parts.basis_revision = next_basis_revision;
    let advance = issue_advance(parts.into_unpublished_owner(), fact, retained);
    WorthUiScalarProjectionActionOutcome::Executed(WorthUiScalarProjectionActionExecution {
        evidence,
        advance: WorthUiScalarProjectionActionAdvance::new(advance),
    })
}

fn product_action_declaration(
    request: &WorthUiScalarProjectionActionRequest,
) -> runtime::WorthQueryIntentDeclaration {
    runtime::WorthQueryIntentDeclaration::strategy_commit(
        crate::product_projection::action_contract::PRODUCT_ACTION_NAME,
        crate::product_projection::action_contract::PRODUCT_ACTION_STRATEGY,
        crate::product_projection::action_contract::PRODUCT_ACTION_STRATEGY_VERSION,
        crate::product_projection::action_contract::PRODUCT_ACTION_INPUT_CONTRACT,
        runtime::WorthQueryIntentInput::object([
            (
                "source_revision",
                runtime::WorthQueryIntentInput::string(request.source_revision.to_string()),
            ),
            (
                "status",
                runtime::WorthQueryIntentInput::string(request.status.clone()),
            ),
        ]),
    )
}

fn indeterminate(
    parts: ActionOwnerParts,
    detail: impl Into<String>,
) -> WorthUiScalarProjectionActionOutcome {
    WorthUiScalarProjectionActionOutcome::Indeterminate(
        WorthUiScalarProjectionActionIndeterminate {
            owner: parts.into_live_owner(),
            detail: detail.into(),
        },
    )
}

impl From<WorthUiScalarProjectionLiveOwner> for ActionOwnerParts {
    fn from(owner: WorthUiScalarProjectionLiveOwner) -> Self {
        Self {
            workspace: owner.workspace,
            bridge: owner.bridge,
            source: owner.source,
            view: owner.view,
            binding: owner.binding,
            request: owner.request,
            predecessor: Some(owner.predecessor),
            revision: owner.revision,
            basis_revision: owner.basis_revision,
        }
    }
}

impl ActionOwnerParts {
    fn into_live_owner(mut self) -> WorthUiScalarProjectionLiveOwner {
        WorthUiScalarProjectionLiveOwner {
            workspace: self.workspace,
            bridge: self.bridge,
            source: self.source,
            view: self.view,
            binding: self.binding,
            request: self.request,
            predecessor: self
                .predecessor
                .take()
                .expect("an indeterminate product action retains its predecessor"),
            revision: self.revision,
            basis_revision: self.basis_revision,
        }
    }

    fn into_unpublished_owner(self) -> WorthUiScalarProjectionUnpublishedOwner {
        debug_assert!(self.predecessor.is_none());
        WorthUiScalarProjectionUnpublishedOwner {
            workspace: self.workspace,
            bridge: self.bridge,
            source: self.source,
            view: self.view,
            binding: self.binding,
            request: self.request,
            revision: self.revision,
            basis_revision: self.basis_revision,
        }
    }
}
