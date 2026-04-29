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

impl<T> ForgeQueryRuntimeStateTarget for &ForgeQueryLiveView<T> {
    fn into_state_snapshot(
        self,
        runtime: &ForgeQueryRuntime,
    ) -> Result<ForgeQueryRuntimeStateSnapshot, ForgeQueryRuntimeError> {
        let installation = runtime.inspect_live_view(self)?;
        Ok(ForgeQueryRuntimeStateSnapshot::ready(
            installation.basis_binding_digest(),
            installation.view_shape_digest(),
            installation.authority_lane(),
            format!(
                "sync runtime-backed live view `{}` is ready through retained subscription evidence",
                installation.view_name()
            ),
        ))
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
            ForgeQueryRuntimeError::UnsupportedFacadeFamily(ForgeQueryRuntimeSupportDenial::new(
                self,
                "runtime public API contract does not declare this facade family",
            ))
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
