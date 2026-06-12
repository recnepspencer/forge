use super::ordinary_runtime_posture::project_live_subscription_ordinary_runtime_posture;
use super::{
    ForgeQueryAuthorityLane, ForgeQueryBatchWriteReceipt, ForgeQueryDerivedViewHandle,
    ForgeQueryLiveView, ForgeQueryRuntime, ForgeQueryRuntimeError, ForgeQueryRuntimeFacadeFamily,
    ForgeQueryRuntimeFamilySupportStatus, ForgeQueryRuntimeStateKind,
    ForgeQueryRuntimeStateSnapshot, ForgeQueryRuntimeSupportDenial, ForgeQueryWriteReceipt,
};

pub trait ForgeQueryRuntimeStateTarget {
    fn into_state_snapshot(
        self,
        runtime: &ForgeQueryRuntime,
    ) -> Result<ForgeQueryRuntimeStateSnapshot, ForgeQueryRuntimeError>;
}

pub(crate) fn snapshot_live_view_name(
    runtime: &ForgeQueryRuntime,
    view_name: &str,
) -> Result<ForgeQueryRuntimeStateSnapshot, ForgeQueryRuntimeError> {
    let state = runtime
        .live_subscriptions
        .get(view_name)
        .ok_or_else(|| ForgeQueryRuntimeError::MissingLiveSubscription(view_name.to_string()))?;
    let installation = &state.installation;
    let mixed_cause_delivery = state
        .last_delivery
        .as_ref()
        .map(|delivery| delivery.mixed_cause_delivery());
    let (result_shape_digest, explanation) = match (
        state.last_delivery.as_ref(),
        state.async_result_state.as_ref(),
        mixed_cause_delivery,
    ) {
        (Some(delivery), Some(async_result_state), Some(mixed_cause_delivery)) => (
            installation.view_shape_digest().to_string(),
            format!(
                "sync runtime-backed live view `{}` is ready through retained subscription evidence; last delivery cause is `{}` with evidence `{}` at sequence {} and relational_patch={}; mixed-cause delivery is `{}` over ordered members `{}` with {} suppressed and {} denied causes; async result state is `{}` with causality `{}` over basis `{}` and generation `{}`",
                installation.view_name(),
                delivery.delivery_cause_kind().as_str(),
                delivery.delivery_cause_digest(),
                delivery.sequence(),
                delivery.has_relational_patch(),
                mixed_cause_delivery.coalescing_kind().as_public_str(),
                mixed_cause_delivery
                    .ordered_member_kinds()
                    .iter()
                    .map(|kind| kind.as_public_str())
                    .collect::<Vec<_>>()
                    .join(","),
                mixed_cause_delivery.suppressed_cause_digests().len(),
                mixed_cause_delivery.denied_cause_digests().len(),
                async_result_state.kind().as_str(),
                async_result_state.causality_digest(),
                async_result_state.basis_digest(),
                async_result_state.generation_digest()
            ),
        ),
        (Some(delivery), None, Some(mixed_cause_delivery)) => (
            installation.view_shape_digest().to_string(),
            format!(
                "sync runtime-backed live view `{}` is ready through retained subscription evidence; last delivery cause is `{}` with evidence `{}` at sequence {} and relational_patch={}; mixed-cause delivery is `{}` over ordered members `{}` with {} suppressed and {} denied causes",
                installation.view_name(),
                delivery.delivery_cause_kind().as_str(),
                delivery.delivery_cause_digest(),
                delivery.sequence(),
                delivery.has_relational_patch(),
                mixed_cause_delivery.coalescing_kind().as_public_str(),
                mixed_cause_delivery
                    .ordered_member_kinds()
                    .iter()
                    .map(|kind| kind.as_public_str())
                    .collect::<Vec<_>>()
                    .join(","),
                mixed_cause_delivery.suppressed_cause_digests().len(),
                mixed_cause_delivery.denied_cause_digests().len()
            ),
        ),
        (None, Some(async_result_state), None) => (
            installation.view_shape_digest().to_string(),
            format!(
                "sync runtime-backed live view `{}` is ready through retained subscription evidence; async result state is `{}` with causality `{}` over basis `{}` and generation `{}`",
                installation.view_name(),
                async_result_state.kind().as_str(),
                async_result_state.causality_digest(),
                async_result_state.basis_digest(),
                async_result_state.generation_digest()
            ),
        ),
        (None, None, None) => (
            installation.view_shape_digest().to_string(),
            format!(
                "sync runtime-backed live view `{}` is ready through retained subscription evidence",
                installation.view_name()
            ),
        ),
        _ => (
            installation.view_shape_digest().to_string(),
            format!(
                "sync runtime-backed live view `{}` is ready through retained subscription evidence",
                installation.view_name()
            ),
        ),
    };
    let mut snapshot = ForgeQueryRuntimeStateSnapshot::ready(
        installation.basis_binding_digest(),
        result_shape_digest,
        installation.authority_lane(),
        explanation,
    )
    .with_ordinary_runtime_posture(project_live_subscription_ordinary_runtime_posture(state));
    if let Some(async_result_state) = state.async_result_state.clone() {
        snapshot = snapshot.with_async_result_state(async_result_state);
    }
    if let Some(remask_posture) = state.remask_posture.clone() {
        snapshot = ForgeQueryRuntimeStateSnapshot::deferred(
            remask_posture.disposition_kind().state_kind(),
            installation.basis_binding_digest(),
            installation.view_shape_digest().to_string(),
            installation.authority_lane(),
            format!(
                "sync runtime-backed live view `{}` is {} through retained remask posture `{}` over basis `{}`; policy `{}`, tenant truth `{}`, tenant schema `{}`, relationship proof `{}`, schema context `{}`",
                installation.view_name(),
                remask_posture.disposition_kind().as_str(),
                remask_posture.reason_kind().as_str(),
                remask_posture.basis_digest(),
                remask_posture.policy_digest(),
                remask_posture.tenant_truth_digest(),
                remask_posture.tenant_schema_digest(),
                remask_posture.relationship_proof_digest(),
                remask_posture.schema_context_digest()
            ),
        )
        .with_ordinary_runtime_posture(project_live_subscription_ordinary_runtime_posture(state));
        if let Some(async_result_state) = state.async_result_state.clone() {
            snapshot = snapshot.with_async_result_state(async_result_state);
        }
        snapshot = snapshot.with_remask_posture(remask_posture);
    }
    Ok(snapshot)
}

impl<T> ForgeQueryRuntimeStateTarget for &ForgeQueryLiveView<T> {
    fn into_state_snapshot(
        self,
        runtime: &ForgeQueryRuntime,
    ) -> Result<ForgeQueryRuntimeStateSnapshot, ForgeQueryRuntimeError> {
        snapshot_live_view_name(runtime, self.name())
    }
}

impl<T> ForgeQueryRuntimeStateTarget for &ForgeQueryDerivedViewHandle<T> {
    fn into_state_snapshot(
        self,
        runtime: &ForgeQueryRuntime,
    ) -> Result<ForgeQueryRuntimeStateSnapshot, ForgeQueryRuntimeError> {
        let inspection = runtime.inspect_derived_view(self)?;
        Ok(ForgeQueryRuntimeStateSnapshot::ready(
            inspection.dependency_digest(),
            inspection.materialization_digest(),
            inspection.authority_lane(),
            format!(
                "sync runtime-backed computed view `{}` is ready through retained materialization evidence",
                inspection.name()
            ),
        ))
    }
}

impl ForgeQueryRuntimeStateTarget for ForgeQueryRuntimeFacadeFamily {
    fn into_state_snapshot(
        self,
        runtime: &ForgeQueryRuntime,
    ) -> Result<ForgeQueryRuntimeStateSnapshot, ForgeQueryRuntimeError> {
        let contract = runtime.public_api_contract();
        let row = contract.family(self).ok_or_else(|| {
            ForgeQueryRuntimeError::UnsupportedFacadeFamily(
                ForgeQueryRuntimeSupportDenial::unsupported(
                    self,
                    "runtime public API contract does not declare this facade family",
                ),
            )
        })?;
        let explanation = row.reason().unwrap_or_else(|| match row.status() {
            ForgeQueryRuntimeFamilySupportStatus::Supported => {
                "runtime-backed facade family is currently supported"
            }
            ForgeQueryRuntimeFamilySupportStatus::DeferredDebt => {
                "runtime-backed facade family is deferred to its owning future milestone"
            }
            ForgeQueryRuntimeFamilySupportStatus::Unsupported => {
                "runtime-backed facade family is unsupported by this runtime"
            }
        });
        let result_shape_digest = format!("facade-family:{}", self.as_str());
        match row.status() {
            ForgeQueryRuntimeFamilySupportStatus::Supported => {
                Ok(ForgeQueryRuntimeStateSnapshot::ready(
                    row.contract_digest(),
                    result_shape_digest,
                    row.authority_lanes()
                        .first()
                        .copied()
                        .unwrap_or(ForgeQueryAuthorityLane::AuthoritativeTruth),
                    explanation,
                ))
            }
            ForgeQueryRuntimeFamilySupportStatus::DeferredDebt => {
                Ok(ForgeQueryRuntimeStateSnapshot::deferred(
                    ForgeQueryRuntimeStateKind::Pending,
                    row.contract_digest(),
                    result_shape_digest,
                    deferred_authority_lane(self),
                    explanation,
                ))
            }
            ForgeQueryRuntimeFamilySupportStatus::Unsupported => {
                Ok(ForgeQueryRuntimeStateSnapshot::deferred(
                    ForgeQueryRuntimeStateKind::Unsupported,
                    row.contract_digest(),
                    result_shape_digest,
                    deferred_authority_lane(self),
                    explanation,
                ))
            }
        }
    }
}

impl ForgeQueryRuntimeStateTarget for &ForgeQueryWriteReceipt {
    fn into_state_snapshot(
        self,
        _runtime: &ForgeQueryRuntime,
    ) -> Result<ForgeQueryRuntimeStateSnapshot, ForgeQueryRuntimeError> {
        let result_shape_digest = format!(
            "mutation-receipt:{}:{}:{}",
            self.mutation_family(),
            self.declared_collection().unwrap_or(""),
            self.declared_entity_identity().unwrap_or("")
        );
        Ok(ForgeQueryRuntimeStateSnapshot::ready(
            self.commit_identity(),
            result_shape_digest,
            self.authority_lane(),
            format!(
                "mutation receipt `{}` is ready with `{}` family evidence over `{}` basis lane",
                self.commit_identity(),
                self.mutation_family(),
                self.basis_lane()
            ),
        ))
    }
}

impl ForgeQueryRuntimeStateTarget for &ForgeQueryBatchWriteReceipt {
    fn into_state_snapshot(
        self,
        _runtime: &ForgeQueryRuntime,
    ) -> Result<ForgeQueryRuntimeStateSnapshot, ForgeQueryRuntimeError> {
        Ok(ForgeQueryRuntimeStateSnapshot::ready(
            self.batch_digest(),
            format!("batch-write-receipt:{}", self.write_count()),
            self.authority_lane(),
            format!(
                "batch write receipt `{}` is ready with {} component writes over `{}` basis lane",
                self.batch_digest(),
                self.write_count(),
                self.basis_lane()
            ),
        ))
    }
}

fn deferred_authority_lane(family: ForgeQueryRuntimeFacadeFamily) -> ForgeQueryAuthorityLane {
    match family {
        ForgeQueryRuntimeFacadeFamily::Temporal => ForgeQueryAuthorityLane::TemporalExecutionState,
        ForgeQueryRuntimeFacadeFamily::AsyncResource => ForgeQueryAuthorityLane::AsyncResourceState,
        ForgeQueryRuntimeFacadeFamily::MixedCauseDelivery
        | ForgeQueryRuntimeFacadeFamily::StoreBackedExecution
        | ForgeQueryRuntimeFacadeFamily::DurableArtifacts => {
            ForgeQueryAuthorityLane::BridgeExternalState
        }
        ForgeQueryRuntimeFacadeFamily::Computed => ForgeQueryAuthorityLane::DerivedRuntimeState,
        ForgeQueryRuntimeFacadeFamily::Effect => ForgeQueryAuthorityLane::EffectDeliveryState,
        ForgeQueryRuntimeFacadeFamily::Intent => ForgeQueryAuthorityLane::PendingWriteIntent,
        ForgeQueryRuntimeFacadeFamily::BranchPreview => ForgeQueryAuthorityLane::PreviewTruth,
        ForgeQueryRuntimeFacadeFamily::Read
        | ForgeQueryRuntimeFacadeFamily::Live
        | ForgeQueryRuntimeFacadeFamily::Write
        | ForgeQueryRuntimeFacadeFamily::Inspect => ForgeQueryAuthorityLane::AuthoritativeTruth,
    }
}
