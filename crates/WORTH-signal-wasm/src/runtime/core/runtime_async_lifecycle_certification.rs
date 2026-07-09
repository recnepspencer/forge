use worth_signal::facade::{
    AsyncNodeCapabilityDeclaration, AsyncNodePayloadContract, AsyncNodePayloadContractId,
    RawCompletionEnvelope, ResourceAttemptId,
};

use crate::boundary::errors::WORTHSignalJsError;
use crate::runtime::summaries::RuntimeAsyncLifecycleCertification;

use super::RuntimeCore;

impl RuntimeCore {
    pub(crate) fn certify_runtime_async_lifecycle(
        &mut self,
        id: &str,
        payload_contract_id: u64,
        payload_byte_len: u64,
    ) -> Result<RuntimeAsyncLifecycleCertification, WORTHSignalJsError> {
        let node = self.node_for_id(id)?;
        let payload_contract =
            AsyncNodePayloadContract::new(AsyncNodePayloadContractId::new(payload_contract_id))
                .with_max_payload_bytes(payload_byte_len);
        let async_capable_node = self
            .runtime
            .attach_async_capability(AsyncNodeCapabilityDeclaration::new(node, payload_contract))?;
        let request_admission = self
            .runtime
            .admit_async_node_request(async_capable_node.request_intent())?;
        let admitted_request = request_admission
            .resource_admission()
            .ok_or_else(|| {
                WORTHSignalJsError::internal(
                    "runtime async certification expected admitted request",
                )
            })?
            .admitted_request();
        let request_handle = admitted_request.handle();
        let raw_completion = RawCompletionEnvelope::new(
            request_handle.request_id(),
            request_handle.generation(),
            request_handle.branch_epoch(),
            ResourceAttemptId::new(admitted_request.attempt().get()),
            async_capable_node.payload_contract_digest().clone(),
            payload_byte_len,
        );
        let admitted_completion = self
            .runtime
            .admit_resource_completion(raw_completion)
            .admitted_completion()
            .ok_or_else(|| {
                WORTHSignalJsError::internal(
                    "runtime async certification expected admitted completion",
                )
            })?;
        let staged_completion = self
            .runtime
            .stage_admitted_resource_completion(admitted_completion)?;
        self.runtime
            .commit_staged_resource_completion(staged_completion.staged_effect())?;

        Ok(RuntimeAsyncLifecycleCertification {
            node_id: id.to_owned(),
            payload_contract_id,
            payload_byte_len,
            request_admitted: true,
            completion_committed: true,
            resource_runtime_digest: canonical_resource_digest(
                &self.runtime.resource_runtime_summary(),
            )?,
            resource_replay_digest: canonical_resource_digest(
                &self.runtime.reconstruct_resource_replay_summary(),
            )?,
        })
    }
}

fn canonical_resource_digest<T: serde::Serialize>(value: &T) -> Result<String, WORTHSignalJsError> {
    crate::runtime::core::certification_digest::canonical_certification_digest(value)
        .map_err(|err| WORTHSignalJsError::internal(format!("resource digest failed: {:?}", err)))
}
