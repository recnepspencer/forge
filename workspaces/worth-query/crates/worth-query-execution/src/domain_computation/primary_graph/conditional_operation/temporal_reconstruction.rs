use std::{collections::BTreeMap, num::NonZeroUsize, sync::Arc};

use worth_query_installation::facade::{
    ApplicationFieldRef, ApplicationFieldUnit, ApplicationSchema, EqualityPredicate,
    TypedApplicationIdentityValue, TypedApplicationReadableValue, TypedApplicationValue,
    WorthQueryHostConditionalPredicateProvider, WorthQueryInstalledTemporalConditionalOperation,
    WorthQueryNamedClock, WorthQueryNamedClockSource, WorthQueryTemporalIntentLifecycle,
    WorthQueryTemporalIntentProjector, WritePosture,
};
use worth_runtime_bridge::facade::{
    BridgeManagedClockBinding, BridgeManagedTemporalIntentIdentity,
    BridgeManagedTemporalIntentLifecycle, BridgeManagedTemporalIntentReconciliation,
    BridgeManagedTemporalIntentReconciliationParts, BridgeOwnedSignalRuntime,
};

use super::installation::{
    WorthQueryConditionalRuntimeInstallationDenial,
    WorthQueryConditionalRuntimeInstallationDenialKind,
};
use super::reconstruction_authority::{
    WorthQueryTemporalPrincipalSource, WorthQueryTemporalReconstructionAccess,
};
mod refresh;
mod source_record_binding;
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationProjection, WorthQueryApplicationQueryAccessContext,
    WorthQueryApplicationQueryControls, WorthQueryPrimaryGraphApplicationRuntime,
    WorthQueryPrincipalResolutionMode,
};
pub(super) use refresh::reconcile_refreshed_temporal_intents;
use source_record_binding::bind_source_records;
pub(super) use source_record_binding::WorthQueryReconstructedTemporalIntent;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(in crate::domain_computation::primary_graph) struct WorthQueryTemporalReconstructionWork {
    pub(super) examined_candidates: usize,
    pub(super) projected_records: usize,
    pub(super) projected_fields: usize,
    pub(super) total_work_units: usize,
}

pub(super) struct WorthQueryTemporalReconstruction<Clock, Input> {
    pub(super) intents: BTreeMap<String, WorthQueryReconstructedTemporalIntent<Clock, Input>>,
    pub(super) work: WorthQueryTemporalReconstructionWork,
}

pub(super) fn reconstruct_temporal_intents<
    Schema,
    ApplicationOperation,
    Input,
    D,
    O,
    F,
    Node,
    Provider,
    Clock,
    Source,
    Query,
    Parameters,
    QueryResult,
    Scope,
    Projector,
    PrincipalBinding,
    PrincipalMapping,
    Principal,
    PrincipalIdentity,
    ScopeAspect,
    ScopeField,
    ScopeValue,
    ScopeWrite,
    ScopeUnit,
    PrincipalSource,
    QueryAuthorization,
    IntentEntity,
    IdentityAspect,
    IdentityField,
    IdentityValue,
    IdentityWrite,
    IdentityUnit,
>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    binding: &WorthQueryInstalledTemporalConditionalOperation<
        Schema,
        ApplicationOperation,
        Input,
        D,
        O,
        F,
        Node,
        Provider,
        Clock,
        Source,
        Query,
        Parameters,
        QueryResult,
        Scope,
        Projector,
    >,
    access: &WorthQueryTemporalReconstructionAccess<
        Schema,
        PrincipalBinding,
        PrincipalMapping,
        Principal,
        PrincipalIdentity,
        Scope,
        ScopeAspect,
        ScopeField,
        ScopeValue,
        ScopeWrite,
        ScopeUnit,
        PrincipalSource,
        QueryAuthorization,
    >,
    identity_field: ApplicationFieldRef<
        Schema,
        IntentEntity,
        IdentityAspect,
        IdentityField,
        IdentityValue,
        IdentityWrite,
        EqualityPredicate,
        IdentityUnit,
    >,
) -> Result<
    WorthQueryTemporalReconstruction<Clock, Input>,
    WorthQueryConditionalRuntimeInstallationDenial,
>
where
    Schema: ApplicationSchema,
    Provider: WorthQueryHostConditionalPredicateProvider<Node>,
    Clock: WorthQueryNamedClock,
    Source: WorthQueryNamedClockSource<Clock>,
    QueryResult: WorthQueryApplicationProjection<Schema, Query>,
    Projector: WorthQueryTemporalIntentProjector<Node, Clock, QueryResult, Input>,
    PrincipalIdentity: TypedApplicationIdentityValue,
    ScopeValue: TypedApplicationValue + Clone,
    ScopeWrite: WritePosture,
    ScopeUnit: ApplicationFieldUnit,
    PrincipalSource: WorthQueryTemporalPrincipalSource<Schema>,
    QueryAuthorization: super::WorthQueryTemporalQueryAuthorization<
        Schema,
        Query,
        Parameters,
        QueryResult,
        Principal,
        PrincipalIdentity,
        Scope,
    >,
    IdentityValue: TypedApplicationReadableValue,
    IdentityWrite: WritePosture,
    IdentityUnit: ApplicationFieldUnit,
{
    let admission = isolate_principal_source(access)?;
    let (external, request) = admission.into_parts();
    let principal = runtime
        .resolve_authenticated_principal(
            &access.principal_binding,
            external,
            &request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .map_err(|denial| {
            reconstruction_denial(
                WorthQueryConditionalRuntimeInstallationDenialKind::ReconstructionPrincipal,
                format!("{:?}: {}", denial.kind(), denial.binding()),
            )
        })?;
    let scope = runtime
        .resolve_entity(
            access.scope_field,
            access.scope_value.clone(),
            &request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .map_err(|denial| {
            reconstruction_denial(
                WorthQueryConditionalRuntimeInstallationDenialKind::ReconstructionScope,
                format!("{:?}: {}", denial.kind(), denial.subject()),
            )
        })?;
    let query_access = WorthQueryApplicationQueryAccessContext::new(&principal, &scope);
    let bounds = binding.bounds();
    let controls = WorthQueryApplicationQueryControls::current_one_shot(
        NonZeroUsize::new(bounds.maximum_reconstruction_rows())
            .expect("installed temporal bounds are non-zero"),
        NonZeroUsize::new(bounds.maximum_query_work())
            .expect("installed temporal bounds are non-zero"),
        &request,
    );
    let plan = access
        .query_authorization
        .admit(
            runtime,
            binding.query(),
            &query_access,
            binding.parameters().clone(),
            controls,
        )
        .map_err(|denial| {
            reconstruction_denial(
                WorthQueryConditionalRuntimeInstallationDenialKind::ReconstructionQuery,
                denial,
            )
        })?;
    let result = runtime
        .execute_application_query_one_shot(plan)
        .map_err(|denial| {
            reconstruction_denial(
                WorthQueryConditionalRuntimeInstallationDenialKind::ReconstructionQuery,
                format!("{:?}: {}", denial.kind(), denial.subject()),
            )
        })?;
    let receipt = result.receipt();
    let work = WorthQueryTemporalReconstructionWork {
        examined_candidates: receipt.examined_candidate_count(),
        projected_records: receipt.projected_record_count(),
        projected_fields: receipt.projected_field_count(),
        total_work_units: receipt.total_work_units(),
    };
    let candidates =
        super::temporal_intent_projection::project_unique_candidates(binding, result.into_rows())?;
    let intents = bind_source_records(runtime, candidates, identity_field, &request)?;
    Ok(WorthQueryTemporalReconstruction { intents, work })
}

fn isolate_principal_source<
    Schema,
    Binding,
    Mapping,
    Principal,
    PrincipalIdentity,
    Scope,
    ScopeAspect,
    ScopeField,
    ScopeValue,
    ScopeWrite,
    ScopeUnit,
    PrincipalSource,
    QueryAuthorization,
>(
    access: &WorthQueryTemporalReconstructionAccess<
        Schema,
        Binding,
        Mapping,
        Principal,
        PrincipalIdentity,
        Scope,
        ScopeAspect,
        ScopeField,
        ScopeValue,
        ScopeWrite,
        ScopeUnit,
        PrincipalSource,
        QueryAuthorization,
    >,
) -> Result<
    super::WorthQueryTemporalPrincipalAdmission<Schema>,
    WorthQueryConditionalRuntimeInstallationDenial,
>
where
    PrincipalIdentity: TypedApplicationIdentityValue,
    ScopeValue: TypedApplicationValue,
    ScopeWrite: WritePosture,
    ScopeUnit: ApplicationFieldUnit,
    PrincipalSource: WorthQueryTemporalPrincipalSource<Schema>,
{
    match access.fresh_admission() {
        Ok(admission) => Ok(admission),
        Err(failure) => Err(reconstruction_denial(
            WorthQueryConditionalRuntimeInstallationDenialKind::ReconstructionPrincipal,
            format!("{:?}: {}", failure.kind(), failure.detail()),
        )),
    }
}

pub(super) fn reconcile_temporal_intents<Clock, Input>(
    bridge: &mut BridgeOwnedSignalRuntime,
    clock: &BridgeManagedClockBinding,
    candidates: &mut BTreeMap<String, WorthQueryReconstructedTemporalIntent<Clock, Input>>,
) -> Result<(), WorthQueryConditionalRuntimeInstallationDenial> {
    for reconstructed in candidates.values() {
        let candidate = reconstructed.candidate();
        let identity = BridgeManagedTemporalIntentIdentity::declare(Arc::<str>::from(
            candidate.identity().as_str(),
        ))
        .map_err(|denial| bridge_reconstruction_denial(denial.detail()))?;
        let lifecycle = match candidate.lifecycle() {
            WorthQueryTemporalIntentLifecycle::Active => {
                BridgeManagedTemporalIntentLifecycle::Active
            }
            WorthQueryTemporalIntentLifecycle::Cancelled => {
                BridgeManagedTemporalIntentLifecycle::Cancelled
            }
            WorthQueryTemporalIntentLifecycle::Completed => {
                BridgeManagedTemporalIntentLifecycle::Completed
            }
        };
        let outcome = bridge
            .reconcile_managed_temporal_intent(BridgeManagedTemporalIntentReconciliationParts {
                binding: clock,
                identity,
                revision: candidate.revision(),
                due_coordinate: candidate.due().nanoseconds(),
                idempotency_identity: Arc::from(candidate.idempotency().as_str()),
                source_record_identity: reconstructed.source_record(),
                lifecycle,
            })
            .map_err(|denial| bridge_reconstruction_denial(denial.detail()))?;
        let expected = matches!(
            (candidate.lifecycle(), outcome),
            (
                WorthQueryTemporalIntentLifecycle::Active,
                BridgeManagedTemporalIntentReconciliation::Installed
            ) | (
                WorthQueryTemporalIntentLifecycle::Cancelled
                    | WorthQueryTemporalIntentLifecycle::Completed,
                BridgeManagedTemporalIntentReconciliation::TerminalNoop
            )
        );
        if !expected {
            return Err(bridge_reconstruction_denial(
                "fresh conditional publication observed non-fresh temporal intent state",
            ));
        }
    }
    candidates.retain(|_, intent| {
        intent.candidate().lifecycle() == WorthQueryTemporalIntentLifecycle::Active
    });
    Ok(())
}

pub(super) fn bridge_reconstruction_denial(
    detail: impl Into<String>,
) -> WorthQueryConditionalRuntimeInstallationDenial {
    reconstruction_denial(
        WorthQueryConditionalRuntimeInstallationDenialKind::ReconstructionIntent,
        detail,
    )
}

fn reconstruction_denial(
    kind: WorthQueryConditionalRuntimeInstallationDenialKind,
    detail: impl Into<String>,
) -> WorthQueryConditionalRuntimeInstallationDenial {
    WorthQueryConditionalRuntimeInstallationDenial::new(kind, detail)
}
