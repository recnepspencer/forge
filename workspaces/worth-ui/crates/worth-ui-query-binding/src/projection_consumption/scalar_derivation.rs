use crate::application_binding::{
    WorthUiConsumedScalarTextProjection, WorthUiExecutedScalarTextConsumer,
    WorthUiPreparedScalarTextConsumer, WorthUiPublishedScalarTextConsumer,
    WorthUiScalarTextConsumptionOutcome, WorthUiScalarTextExecutionOutcome,
    WorthUiScalarTextPublicationOutcome, WorthUiScalarTextSettlementOutcome,
    WorthUiSettledScalarTextProjection,
};

pub(crate) struct UiDerivedScalarProjection {
    pub(crate) value: super::UiNativeTextValue,
    pub(crate) counters: super::UiScalarProjectionWorkCounters,
}

pub(crate) struct UiScalarProjectionDerivationStop {
    pub(crate) kind: super::UiProjectionFactStopKind,
    pub(crate) summary: String,
    pub(crate) counters: super::UiScalarProjectionWorkCounters,
}

pub(crate) fn derive_scalar_projection(
    prepared: WorthUiPreparedScalarTextConsumer,
    workspace: &mut worth_query::facade::runtime::WorthQueryWorkspace,
    budget: super::UiProjectionConsumptionBudget,
) -> Result<UiDerivedScalarProjection, UiScalarProjectionDerivationStop> {
    let executed = execute(prepared, workspace)?;
    let published = publish(executed)?;
    let consumed = consume(published)?;
    let settled = settle(consumed)?;
    derive_native_value(settled, budget)
}

fn execute(
    prepared: WorthUiPreparedScalarTextConsumer,
    workspace: &mut worth_query::facade::runtime::WorthQueryWorkspace,
) -> Result<WorthUiExecutedScalarTextConsumer, UiScalarProjectionDerivationStop> {
    let executed = match prepared.execute(workspace) {
        WorthUiScalarTextExecutionOutcome::Executed(executed) => *executed,
        WorthUiScalarTextExecutionOutcome::Deferred(_deferred) => {
            return Err(stop(
                super::UiProjectionFactStopKind::Unsupported,
                "Query deferred scalar projection execution",
            ));
        }
        WorthUiScalarTextExecutionOutcome::ResourceAdmission(resource) => {
            return Err(resource_stop(*resource));
        }
        WorthUiScalarTextExecutionOutcome::Denied(denial)
        | WorthUiScalarTextExecutionOutcome::Failed(denial) => {
            return Err(stop(
                super::UiProjectionFactStopKind::Unsupported,
                format!(
                    "Query denied scalar projection execution: {}",
                    denial.detail()
                ),
            ));
        }
        WorthUiScalarTextExecutionOutcome::Stale(denial)
        | WorthUiScalarTextExecutionOutcome::RebindRequired(denial) => {
            return Err(stop(
                super::UiProjectionFactStopKind::StaleBindingGeneration,
                format!(
                    "Query requires scalar projection rebind: {}",
                    denial.detail()
                ),
            ));
        }
    };
    Ok(executed)
}

fn publish(
    executed: WorthUiExecutedScalarTextConsumer,
) -> Result<WorthUiPublishedScalarTextConsumer, UiScalarProjectionDerivationStop> {
    let published = match executed.publish() {
        WorthUiScalarTextPublicationOutcome::Published(published) => *published,
        WorthUiScalarTextPublicationOutcome::Denied(denial)
        | WorthUiScalarTextPublicationOutcome::Failed(denial) => {
            return Err(stop(
                super::UiProjectionFactStopKind::PayloadShapeMismatch,
                format!("Query denied scalar projection publication: {denial:?}"),
            ));
        }
        WorthUiScalarTextPublicationOutcome::Stale(denial)
        | WorthUiScalarTextPublicationOutcome::RebindRequired(denial) => {
            return Err(stop(
                super::UiProjectionFactStopKind::StaleBindingGeneration,
                format!("Query publication requires rebind: {denial:?}"),
            ));
        }
    };
    Ok(published)
}

fn consume(
    published: WorthUiPublishedScalarTextConsumer,
) -> Result<WorthUiConsumedScalarTextProjection, UiScalarProjectionDerivationStop> {
    let consumed = match published.consume() {
        WorthUiScalarTextConsumptionOutcome::Consumed(consumed) => *consumed,
        WorthUiScalarTextConsumptionOutcome::Denied(denial)
        | WorthUiScalarTextConsumptionOutcome::Deferred(denial)
        | WorthUiScalarTextConsumptionOutcome::Failed(denial) => {
            return Err(progression_stop(*denial));
        }
        WorthUiScalarTextConsumptionOutcome::Stale(denial)
        | WorthUiScalarTextConsumptionOutcome::RebindRequired(denial) => {
            return Err(stale_progression_stop(*denial));
        }
    };
    Ok(consumed)
}

fn settle(
    consumed: WorthUiConsumedScalarTextProjection,
) -> Result<WorthUiSettledScalarTextProjection, UiScalarProjectionDerivationStop> {
    let settled = match consumed.settle() {
        WorthUiScalarTextSettlementOutcome::Settled(settled) => *settled,
        WorthUiScalarTextSettlementOutcome::Denied(denial)
        | WorthUiScalarTextSettlementOutcome::Failed(denial) => {
            return Err(progression_stop(*denial));
        }
        WorthUiScalarTextSettlementOutcome::Stale(denial)
        | WorthUiScalarTextSettlementOutcome::RebindRequired(denial) => {
            return Err(stale_progression_stop(*denial));
        }
    };
    Ok(settled)
}

fn derive_native_value(
    settled: WorthUiSettledScalarTextProjection,
    budget: super::UiProjectionConsumptionBudget,
) -> Result<UiDerivedScalarProjection, UiScalarProjectionDerivationStop> {
    let derived = settled
        .derive_native_text(budget)
        .map_err(|stop| derivation_stop(*stop))?;
    if !derived.installation_is_current() {
        return Err(stop(
            super::UiProjectionFactStopKind::StaleBindingGeneration,
            "Query scalar installation changed before native value admission",
        ));
    }
    let resolution = derived.resolution_counters();
    let access = derived.access_counters();
    Ok(UiDerivedScalarProjection {
        counters: super::UiScalarProjectionWorkCounters::query_native(resolution, access),
        value: derived.into_value(),
    })
}

fn resource_stop(
    resource: worth_query::facade::installed::transition::WorthQueryResourceAdmissionStop,
) -> UiScalarProjectionDerivationStop {
    use worth_query::facade::installed::transition::WorthQueryResourceAdmissionStop as Stop;
    match resource {
        Stop::Stale(denial) | Stop::RebindRequired(denial) => stop(
            super::UiProjectionFactStopKind::StaleBindingGeneration,
            format!("Query resource admission requires rebind: {denial:?}"),
        ),
        Stop::Denied(denial) | Stop::Deferred(denial) | Stop::Failed(denial) => stop(
            super::UiProjectionFactStopKind::Unsupported,
            format!("Query denied scalar resource admission: {denial:?}"),
        ),
    }
}

fn progression_stop(
    denial: worth_query::facade::installed::operation::WorthQueryProgressionDenial,
) -> UiScalarProjectionDerivationStop {
    use worth_query::facade::installed::operation::WorthQueryProgressionDenial as Denial;
    match denial {
        Denial::StaleInstallationGeneration => stale_progression_stop(denial),
        Denial::ConsumerContractMismatch | Denial::DependencyCompilation(_) => stop(
            super::UiProjectionFactStopKind::SchemaMismatch,
            format!("Query scalar consumer contract mismatch: {denial:?}"),
        ),
        Denial::NativeAccess(denial) => native_access_stop(denial),
        Denial::Projection(_) => stop(
            super::UiProjectionFactStopKind::PayloadShapeMismatch,
            format!("Query scalar projection payload mismatch: {denial:?}"),
        ),
    }
}

fn stale_progression_stop(
    denial: worth_query::facade::installed::operation::WorthQueryProgressionDenial,
) -> UiScalarProjectionDerivationStop {
    stop(
        super::UiProjectionFactStopKind::StaleBindingGeneration,
        format!("Query scalar progression requires rebind: {denial:?}"),
    )
}

fn native_access_stop(
    denial: worth_query::facade::installed::operation::WorthQueryNativeAccessDenial,
) -> UiScalarProjectionDerivationStop {
    stop(
        native_access_kind(denial.kind()),
        format!("Query native access denied: {denial:?}"),
    )
}

fn native_access_kind(
    denial: worth_query::facade::installed::operation::WorthQueryNativeAccessDenialKind,
) -> super::UiProjectionFactStopKind {
    use worth_query::facade::installed::operation::WorthQueryNativeAccessDenialKind as Kind;
    match denial {
        Kind::RuntimeMismatch => super::UiProjectionFactStopKind::WrongWorld,
        Kind::StaleInstallationGeneration | Kind::AccessKeyInstallationGenerationMismatch => {
            super::UiProjectionFactStopKind::StaleBindingGeneration
        }
        Kind::CapabilityMismatch | Kind::LayoutMismatch => {
            super::UiProjectionFactStopKind::NativeFamilyMismatch
        }
        Kind::RowOutOfBounds => super::UiProjectionFactStopKind::PayloadShapeMismatch,
    }
}

fn derivation_stop(
    denial: crate::application_binding::WorthUiScalarTextDerivationStop,
) -> UiScalarProjectionDerivationStop {
    use crate::application_binding::WorthUiScalarTextDerivationStop as Stop;
    match denial {
        Stop::NativeAccess {
            denial,
            resolution_counters,
        } => {
            let access_counters = denial.counters();
            stop_with_counters(
                native_access_kind(denial.kind()),
                format!("Query native access denied: {denial:?}"),
                resolution_counters,
                access_counters,
            )
        }
        Stop::NativeRefinement {
            denial,
            resolution_counters,
            access_counters,
        } => stop_with_counters(
            super::UiProjectionFactStopKind::NativeFamilyMismatch,
            format!("Query native value was not text: {denial:?}"),
            resolution_counters,
            access_counters,
        ),
        Stop::SymbolicText {
            resolution_counters,
            access_counters,
        } => stop_with_counters(
            super::UiProjectionFactStopKind::NativeFamilyMismatch,
            "symbolic text is not direct authored presentation text",
            resolution_counters,
            access_counters,
        ),
        Stop::BudgetExceeded {
            byte_len,
            limit,
            resolution_counters,
            access_counters,
        } => stop_with_counters(
            super::UiProjectionFactStopKind::BudgetExceeded,
            format!("native text used {byte_len} bytes but the limit is {limit}"),
            resolution_counters,
            access_counters,
        ),
    }
}

fn stop(
    kind: super::UiProjectionFactStopKind,
    summary: impl Into<String>,
) -> UiScalarProjectionDerivationStop {
    UiScalarProjectionDerivationStop {
        kind,
        summary: summary.into(),
        counters: super::UiScalarProjectionWorkCounters::default(),
    }
}

fn stop_with_counters(
    kind: super::UiProjectionFactStopKind,
    summary: impl Into<String>,
    resolution: worth_query::facade::domain::WorthQueryNativeKeyResolutionCounters,
    access: worth_query::facade::installed::operation::WorthQueryNativeAccessCounters,
) -> UiScalarProjectionDerivationStop {
    UiScalarProjectionDerivationStop {
        kind,
        summary: summary.into(),
        counters: super::UiScalarProjectionWorkCounters::query_native(resolution, access),
    }
}
