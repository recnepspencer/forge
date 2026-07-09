use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    product_operation_contract::{
        admit_replay, build_storage_key, record_replay, WorthServerProductIdempotencyBinding,
        WorthServerStoredProductOperation,
    },
    WorthServerAdmission, WorthServerCompatibilityPreparedRequest,
    WorthServerOperationAdmissionFacade, WorthServerOperationRegistry,
    WorthServerOperationRequestFacade, WorthServerProductSessionRegistry,
    WorthServerQueryHandoffConfig,
};

use super::read_batch_execution::execute_shared_read_batch_from_worth_native;
use super::runtime_support::{
    build_early_envelope, build_envelope, build_request_input, close_product_operation_readiness,
    declaration_metadata, stale_basis_denial, validate_payload_schema,
};
use super::{
    WorthServerCompletedProductOperation, WorthServerExecutedProductReadBatch,
    WorthServerLoweredProductOperationPlan, WorthServerProductAdapterRegistry,
    WorthServerProductOperationExecutionBoundary, WorthServerProductOperationInput,
    WorthServerProductOperationOutcome, WorthServerProductOperationSurfaceDenial,
    WorthServerProductOperationSurfaceDenialCode, WorthServerProductOperationSurfaceDenialFacts,
    WorthServerScheduledProductOperation,
};

#[derive(Clone, Debug)]
pub struct WorthServerProductOperationRuntime {
    operation_registry: WorthServerOperationRegistry,
    adapter_registry: WorthServerProductAdapterRegistry,
    query_handoff_config: WorthServerQueryHandoffConfig,
    product_session_registry: WorthServerProductSessionRegistry,
    replay_store: Arc<Mutex<HashMap<String, WorthServerStoredProductOperation>>>,
}

impl WorthServerProductOperationRuntime {
    pub(crate) fn new(
        operation_registry: WorthServerOperationRegistry,
        adapter_registry: WorthServerProductAdapterRegistry,
        query_handoff_config: WorthServerQueryHandoffConfig,
        product_session_registry: WorthServerProductSessionRegistry,
        replay_store: Arc<Mutex<HashMap<String, WorthServerStoredProductOperation>>>,
    ) -> Self {
        Self {
            operation_registry,
            adapter_registry,
            query_handoff_config,
            product_session_registry,
            replay_store,
        }
    }

    pub fn execute_from_worth_native(
        &self,
        admission: &WorthServerAdmission,
        input: WorthServerProductOperationInput,
    ) -> Result<WorthServerCompletedProductOperation, WorthServerProductOperationSurfaceDenial>
    {
        let (adapter, declaration) = self.resolve_declaration(input.operation_name())?;
        let request = WorthServerOperationRequestFacade::new(self.operation_registry.clone())
            .admit_from_worth_native_admission(admission, build_request_input(declaration, &input))
            .map_err(WorthServerProductOperationSurfaceDenial::from_request_denial)?;
        self.execute_resolved(
            admission,
            adapter.as_ref(),
            declaration,
            request,
            input.into_payload(),
        )
    }

    pub fn execute_from_compat_http(
        &self,
        prepared_request: &WorthServerCompatibilityPreparedRequest,
        input: WorthServerProductOperationInput,
    ) -> Result<WorthServerCompletedProductOperation, WorthServerProductOperationSurfaceDenial>
    {
        let (adapter, declaration) = self.resolve_declaration(input.operation_name())?;
        let request = WorthServerOperationRequestFacade::new(self.operation_registry.clone())
            .admit_from_compat_http_with_request_input(
                prepared_request,
                build_request_input(declaration, &input),
            )
            .map_err(WorthServerProductOperationSurfaceDenial::from_request_denial)?;
        self.execute_resolved(
            prepared_request.admission(),
            adapter.as_ref(),
            declaration,
            request,
            input.into_payload(),
        )
    }

    pub fn execute_shared_read_batch_from_worth_native(
        &self,
        admission: &WorthServerAdmission,
        inputs: Vec<WorthServerProductOperationInput>,
    ) -> Result<WorthServerExecutedProductReadBatch, WorthServerProductOperationSurfaceDenial> {
        execute_shared_read_batch_from_worth_native(
            &self.operation_registry,
            &self.adapter_registry,
            &self.query_handoff_config,
            admission,
            inputs,
        )
    }

    fn resolve_declaration(
        &self,
        operation_name: &str,
    ) -> Result<
        (
            &std::sync::Arc<dyn super::WorthServerProductApplicationAdapter>,
            &super::WorthServerProductOperationDeclaration,
        ),
        WorthServerProductOperationSurfaceDenial,
    > {
        self.adapter_registry
            .resolve(operation_name)
            .ok_or_else(|| {
                WorthServerProductOperationSurfaceDenial::new(
                    WorthServerProductOperationSurfaceDenialCode::UnknownOperationName,
                    format!("no registered product adapter owns `{operation_name}`"),
                )
            })
    }

    fn execute_resolved(
        &self,
        admission: &WorthServerAdmission,
        adapter: &dyn super::WorthServerProductApplicationAdapter,
        declaration: &super::WorthServerProductOperationDeclaration,
        request: crate::WorthServerOperationRequest,
        payload: super::WorthServerProductOperationPayload,
    ) -> Result<WorthServerCompletedProductOperation, WorthServerProductOperationSurfaceDenial>
    {
        if let Err(denial) = validate_payload_schema(declaration, &payload) {
            let outcome = WorthServerProductOperationOutcome::Denied(denial);
            let envelope = build_early_envelope(declaration.operation_name(), &request, &outcome);
            return Ok(WorthServerCompletedProductOperation::new(outcome, envelope));
        }
        let admitted_session = request
            .identity()
            .product_session_identity()
            .and_then(|identity| {
                self.product_session_registry
                    .lookup(identity, request.resolved_request_context())
            });
        crate::product_session::admit_session_for_product_operation(
            admitted_session.as_ref(),
            &request,
            declaration.basis_kind(),
        )
        .map_err(|denial| {
            self.product_session_registry.record_denial(denial.code());
            WorthServerProductOperationSurfaceDenial::new(
                WorthServerProductOperationSurfaceDenialCode::AdmissionDenied,
                denial.detail().to_string(),
            )
            .with_facts(
                WorthServerProductOperationSurfaceDenialFacts::default()
                .with_session_denial_code(denial.code())
                .with_execution_boundary(
                    WorthServerProductOperationExecutionBoundary::RejectedBeforeAdapterExecution,
                ),
            )
        })?;
        if request.identity().operation_family()
            == crate::WorthServerOperationFamily::ProductApplicationMutation
            && request.identity().basis_digest().is_none()
        {
            return Err(
                WorthServerProductOperationSurfaceDenial::new(
                    WorthServerProductOperationSurfaceDenialCode::PreconditionDenied,
                    "product mutation operations require an explicit snapshot precondition basis digest".to_string(),
                )
                .with_facts(
                    WorthServerProductOperationSurfaceDenialFacts::default()
                        .with_execution_boundary(
                            WorthServerProductOperationExecutionBoundary::RejectedBeforeAdapterExecution,
                        ),
                ),
            );
        }
        if let Some(expected_basis_digest) = request.identity().basis_digest() {
            if let Some(observed_basis_digest) = admitted_session
                .as_ref()
                .and_then(|session| session.basis_digest())
            {
                if expected_basis_digest != observed_basis_digest {
                    return Err(stale_basis_denial(
                        format!(
                            "product snapshot precondition `{expected_basis_digest}` did not match the admitted session basis `{observed_basis_digest}`"
                        ),
                        expected_basis_digest,
                        observed_basis_digest,
                    ));
                }
            }
        }
        let replay_binding = request
            .identity()
            .idempotency_key()
            .map(|_| WorthServerProductIdempotencyBinding::derive(&request, &payload));
        if let (Some(idempotency_key), Some(binding)) = (
            request.identity().idempotency_key(),
            replay_binding.as_ref(),
        ) {
            let admitted_idempotency_key =
                crate::WorthServerProductIdempotencyKey::new(idempotency_key)
                    .expect("admitted operation request should preserve idempotency key validity");
            if let Some(replayed) = admit_replay(
                &self.replay_store,
                &build_storage_key(binding),
                &admitted_idempotency_key,
                binding.request_digest(),
            )? {
                return Ok(replayed);
            }
        }
        let admission = WorthServerOperationAdmissionFacade::with_operation_registry(
            self.operation_registry.clone(),
        )
        .admit(
            admission,
            &request,
            declaration_metadata(declaration, &request)?,
        )
        .map_err(WorthServerProductOperationSurfaceDenial::from_admission_denial)?;
        let readiness = close_product_operation_readiness(
            &self.operation_registry,
            &self.query_handoff_config,
            &admission,
            declaration,
            request.resolved_request_context(),
        )?;
        let plan = WorthServerLoweredProductOperationPlan::new(
            admission,
            declaration.clone(),
            payload,
            readiness.support_posture().clone(),
            readiness.precondition_posture().clone(),
            readiness.concurrency_class(),
        );
        let scheduled = WorthServerScheduledProductOperation::admit(plan)?;
        let outcome = match adapter.execute(&scheduled) {
            Ok(success) => WorthServerProductOperationOutcome::Success(success),
            Err(error) => declaration.error_map().map_error(error),
        };
        let envelope = build_envelope(&scheduled, &outcome);
        let mut completed = WorthServerCompletedProductOperation::new(outcome, envelope)
            .with_scheduled_operation(&scheduled);
        if let (Some(idempotency_key), Some(binding)) = (
            request.identity().idempotency_key(),
            replay_binding.as_ref(),
        ) {
            completed = completed.with_replay_receipt(
                crate::WorthServerProductOperationReplayReceipt::authoritative(
                    idempotency_key,
                    binding.request_digest(),
                ),
            );
            record_replay(
                &self.replay_store,
                build_storage_key(binding),
                binding.request_digest().to_string(),
                completed.clone(),
            );
        }
        Ok(completed)
    }
}
