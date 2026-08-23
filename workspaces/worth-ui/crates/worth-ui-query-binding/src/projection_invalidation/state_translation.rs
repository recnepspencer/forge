use worth_query::facade::runtime::{
    WorthQueryAsyncResultTransitionBatch, WorthQueryRuntimeAsyncResultState,
    WorthQueryRuntimeAsyncResultStateKind,
};

use crate::{
    UiPresentProjection, UiProjectionAvailability, UiProjectionConsumptionBudget,
    UiProjectionFactStopKind, UiProjectionRetainedActivityKind,
    UiProjectionRetainedActivityReceipt, UiProjectionUnavailableKind,
    UiProjectionUnavailableReceipt, UiScalarProjectionFactReceipt, UiScalarProjectionWorkCounters,
};

pub(super) struct ScalarStateContext<'a> {
    pub(super) binding: &'a mut crate::UiScalarProjectionBinding,
    pub(super) workspace: &'a mut worth_query::facade::runtime::WorthQueryWorkspace,
    pub(super) batch: &'a WorthQueryAsyncResultTransitionBatch,
    pub(super) state: &'a WorthQueryRuntimeAsyncResultState,
    pub(super) budget: UiProjectionConsumptionBudget,
}

pub(super) struct StateTransition {
    pub(super) fact: UiScalarProjectionFactReceipt,
    pub(super) retained_predecessor: Option<UiScalarProjectionFactReceipt>,
    pub(super) work: UiScalarProjectionWorkCounters,
}

pub(super) fn translate_state(
    context: ScalarStateContext<'_>,
    predecessor: Option<UiScalarProjectionFactReceipt>,
) -> StateTransition {
    if let Some((kind, identity)) = unavailable_override(&context) {
        return unavailable_with_identity(context, predecessor, kind, identity, false);
    }
    use WorthQueryRuntimeAsyncResultStateKind as Kind;
    match context.state.kind() {
        Kind::Unresolved => unavailable(
            context,
            predecessor,
            UiProjectionUnavailableKind::Unresolved,
            false,
        ),
        Kind::Current => current(context, predecessor),
        Kind::Stale => retained(context, predecessor, UiProjectionRetainedActivityKind::Idle),
        Kind::Revalidating => retained(
            context,
            predecessor,
            UiProjectionRetainedActivityKind::Revalidating,
        ),
        Kind::Pending => pending(context, predecessor),
        Kind::Failed => unavailable(
            context,
            predecessor,
            UiProjectionUnavailableKind::Failed,
            false,
        ),
        Kind::Cancelled => unavailable(
            context,
            predecessor,
            UiProjectionUnavailableKind::Cancelled,
            false,
        ),
        Kind::Retried => unavailable(
            context,
            predecessor,
            UiProjectionUnavailableKind::Retried,
            false,
        ),
        Kind::Superseded => unavailable(
            context,
            predecessor,
            UiProjectionUnavailableKind::Superseded,
            false,
        ),
        Kind::Denied => unavailable(
            context,
            predecessor,
            UiProjectionUnavailableKind::Denied,
            false,
        ),
    }
}

fn pending(
    context: ScalarStateContext<'_>,
    predecessor: Option<UiScalarProjectionFactReceipt>,
) -> StateTransition {
    context.binding.discard_prepared_after_pending();
    unavailable(
        context,
        predecessor,
        UiProjectionUnavailableKind::Pending,
        true,
    )
}

fn unavailable_override(
    context: &ScalarStateContext<'_>,
) -> Option<(
    UiProjectionUnavailableKind,
    worth_query::facade::runtime::WorthQueryEvidenceIdentity,
)> {
    if let Some(remask) = context.batch.remask_posture() {
        use worth_query::facade::runtime::WorthQueryRuntimeRemaskDispositionKind as Disposition;
        let kind = match remask.disposition_kind() {
            Disposition::Remasked => UiProjectionUnavailableKind::Remasked,
            Disposition::Denied => UiProjectionUnavailableKind::Denied,
        };
        return Some((kind, remask.remask_identity().clone()));
    }
    if context.state.basis_identity() != context.batch.expected_basis_identity() {
        return Some((
            UiProjectionUnavailableKind::BasisDrift,
            context.state.result_state_identity().clone(),
        ));
    }
    if context.state.checkpoint_identity() != context.batch.expected_checkpoint_identity() {
        return Some((
            UiProjectionUnavailableKind::GenerationDrift,
            context.state.result_state_identity().clone(),
        ));
    }
    None
}

fn current(
    context: ScalarStateContext<'_>,
    predecessor: Option<UiScalarProjectionFactReceipt>,
) -> StateTransition {
    let prepared = match prepared_for_current(context.binding, context.workspace) {
        Ok(prepared) => prepared,
        Err((kind, summary)) => return stopped(context, predecessor, kind, summary),
    };
    match crate::projection_consumption::derive_scalar_projection(
        prepared,
        context.workspace,
        context.budget,
    ) {
        Ok(derived) => StateTransition {
            fact: state_fact(
                &context,
                UiProjectionAvailability::Present(UiPresentProjection::Current(derived.value)),
            ),
            retained_predecessor: None,
            work: derived.counters,
        },
        Err(stop) => {
            stopped_with_work(context, predecessor, stop.kind, stop.summary, stop.counters)
        }
    }
}

fn prepared_for_current(
    binding: &mut crate::UiScalarProjectionBinding,
    workspace: &worth_query::facade::runtime::WorthQueryWorkspace,
) -> Result<
    crate::application_binding::WorthUiPreparedScalarTextConsumer,
    (UiProjectionFactStopKind, String),
> {
    if let Some(prepared) = binding.take_prepared() {
        return Ok(prepared);
    }
    let selected_field = binding.requirement().selected_field().clone();
    let gateway = binding
        .reference()
        .enter_attempt(workspace)
        .map_err(|denial| {
            (
                UiProjectionFactStopKind::StaleBindingGeneration,
                format!("Query denied scalar operating-world entry: {denial:?}"),
            )
        })?;
    gateway
        .prepare_consumer(&selected_field)
        .map_err(preparation_stop)
}

fn preparation_stop(
    denial: crate::application_binding::WorthUiScalarTextConsumerPreparationDenial,
) -> (UiProjectionFactStopKind, String) {
    use crate::application_binding::WorthUiScalarTextConsumerPreparationDenial as Denial;
    match denial {
        Denial::Binding(denial) => (
            UiProjectionFactStopKind::StaleBindingGeneration,
            denial.detail().to_owned(),
        ),
        Denial::ConsumerContract(denial) => {
            let _ = denial;
            (
                UiProjectionFactStopKind::SchemaMismatch,
                "Query consumer support no longer satisfies the scalar lifecycle".to_owned(),
            )
        }
        Denial::NativeRequest(denial) => native_request_stop(denial),
    }
}

fn native_request_stop(
    denial: crate::application_binding::WorthUiScalarTextNativeRequestDenial,
) -> (UiProjectionFactStopKind, String) {
    use crate::application_binding::WorthUiScalarTextNativeRequestDenial as Denial;
    use worth_query::facade::installed::operation::WorthQueryNativeProjectionRequestDenialKind;
    match denial {
        Denial::ProjectionRequest(denial) => {
            use WorthQueryNativeProjectionRequestDenialKind as Kind;
            let stop = match denial.kind() {
                Kind::WholeAspectNotProjected
                | Kind::UnknownField
                | Kind::FieldNotProjected
                | Kind::ConflictingDeclaration => UiProjectionFactStopKind::SchemaMismatch,
                Kind::FieldRequiresStruct | Kind::UnsupportedAspectShape | Kind::NoNativeFacts => {
                    UiProjectionFactStopKind::NativeFamilyMismatch
                }
            };
            (
                stop,
                format!(
                    "Query rejected the contract-derived native text request: {:?}",
                    denial.kind()
                ),
            )
        }
        Denial::SelectionMismatch(denial) => (
            UiProjectionFactStopKind::SchemaMismatch,
            format!(
                "Query native text selection no longer matches its declaration: {:?}",
                denial.kind()
            ),
        ),
    }
}

fn retained(
    context: ScalarStateContext<'_>,
    predecessor: Option<UiScalarProjectionFactReceipt>,
    activity: UiProjectionRetainedActivityKind,
) -> StateTransition {
    let Some(predecessor) = predecessor else {
        return stopped(
            context,
            None,
            UiProjectionFactStopKind::BasisMismatch,
            "Query stale/revalidating posture requires one predecessor value",
        );
    };
    let (predecessor_core, availability) = predecessor.into_parts();
    let value = match availability {
        UiProjectionAvailability::Present(UiPresentProjection::Current(value))
        | UiProjectionAvailability::Present(UiPresentProjection::RetainedStale { value, .. }) => {
            value
        }
        availability => {
            let predecessor =
                UiScalarProjectionFactReceipt::admitted(predecessor_core, availability);
            return stopped(
                context,
                Some(predecessor),
                UiProjectionFactStopKind::BasisMismatch,
                "Query stale/revalidating posture cannot retain an absent predecessor value",
            );
        }
    };
    let activity = UiProjectionRetainedActivityReceipt::query_issued(
        activity,
        context.state.result_state_identity().clone(),
    );
    StateTransition {
        fact: state_fact(
            &context,
            UiProjectionAvailability::Present(UiPresentProjection::RetainedStale {
                value,
                activity,
            }),
        ),
        retained_predecessor: None,
        work: UiScalarProjectionWorkCounters::default(),
    }
}

fn unavailable(
    context: ScalarStateContext<'_>,
    predecessor: Option<UiScalarProjectionFactReceipt>,
    kind: UiProjectionUnavailableKind,
    predecessor_must_be_absent: bool,
) -> StateTransition {
    let identity = context.state.result_state_identity().clone();
    unavailable_with_identity(
        context,
        predecessor,
        kind,
        identity,
        predecessor_must_be_absent,
    )
}

fn unavailable_with_identity(
    context: ScalarStateContext<'_>,
    predecessor: Option<UiScalarProjectionFactReceipt>,
    kind: UiProjectionUnavailableKind,
    identity: worth_query::facade::runtime::WorthQueryEvidenceIdentity,
    predecessor_must_be_absent: bool,
) -> StateTransition {
    if predecessor_must_be_absent && predecessor.is_some() {
        return stopped(
            context,
            predecessor,
            UiProjectionFactStopKind::BasisMismatch,
            "Query pending posture cannot replace an existing projection value",
        );
    }
    let receipt = UiProjectionUnavailableReceipt::query_issued(kind, identity);
    StateTransition {
        fact: state_fact(&context, UiProjectionAvailability::Unavailable(receipt)),
        retained_predecessor: predecessor,
        work: UiScalarProjectionWorkCounters::default(),
    }
}

fn stopped(
    context: ScalarStateContext<'_>,
    predecessor: Option<UiScalarProjectionFactReceipt>,
    kind: UiProjectionFactStopKind,
    summary: impl Into<String>,
) -> StateTransition {
    stopped_with_work(
        context,
        predecessor,
        kind,
        summary,
        UiScalarProjectionWorkCounters::default(),
    )
}

fn stopped_with_work(
    context: ScalarStateContext<'_>,
    predecessor: Option<UiScalarProjectionFactReceipt>,
    kind: UiProjectionFactStopKind,
    summary: impl Into<String>,
    work: UiScalarProjectionWorkCounters,
) -> StateTransition {
    let fact = super::fact_construction::state_stop(
        fact_context(&context),
        predecessor.as_ref(),
        kind,
        summary,
    );
    StateTransition {
        fact,
        retained_predecessor: predecessor,
        work,
    }
}

fn state_fact(
    context: &ScalarStateContext<'_>,
    availability: UiProjectionAvailability<crate::UiNativeTextValue>,
) -> UiScalarProjectionFactReceipt {
    super::fact_construction::state_fact(fact_context(context), availability)
}

fn fact_context<'a>(
    context: &'a ScalarStateContext<'a>,
) -> super::fact_construction::StateFactContext<'a> {
    super::fact_construction::StateFactContext {
        binding: context.binding,
        batch: context.batch,
        state: context.state,
    }
}
