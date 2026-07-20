use crate::{
    WorthServerProductIdempotencyKey, WorthServerProductOperationPayload,
    WorthServerProductResultContract, WorthServerProductSnapshotPrecondition,
};

use super::{WorthServerDurableProductMutationContract, WorthServerProductAuthorityScope};

#[derive(Clone, Debug)]
pub struct WorthServerAdmittedDurableProductMutation {
    operation_name: String,
    tenant_id: String,
    workspace_id: String,
    durable_contract: WorthServerDurableProductMutationContract,
    expected_basis: WorthServerProductSnapshotPrecondition,
    idempotency_key: WorthServerProductIdempotencyKey,
    request_digest: String,
    payload: WorthServerProductOperationPayload,
    result_contract: WorthServerProductResultContract,
    lowered_plan_digest: String,
    canonical_digest: String,
}

impl WorthServerAdmittedDurableProductMutation {
    pub(crate) fn from_scheduled(
        scheduled: &crate::WorthServerScheduledProductOperation,
        durable_contract: &WorthServerDurableProductMutationContract,
    ) -> Result<Self, crate::WorthServerProductOperationSurfaceDenial> {
        let plan = scheduled.plan();
        let request = plan.operation_admission().operation_request();
        let request_context = request.resolved_request_context().request_context();
        let expected_basis_value = request.identity().basis_digest().ok_or_else(|| {
            missing_durable_attempt_contract(
                "durable product mutation requires a canonical expected basis",
            )
        })?;
        let expected_basis =
            crate::WorthServerProductOperationBaseDigest::new(expected_basis_value)
                .map(crate::WorthServerProductSnapshotPrecondition::at_base_digest)
                .map_err(missing_durable_attempt_contract)?;
        let idempotency_key = request
            .identity()
            .idempotency_key()
            .and_then(|key| WorthServerProductIdempotencyKey::new(key).ok())
            .ok_or_else(|| {
                missing_durable_attempt_contract(
                    "durable product mutation requires an admitted idempotency key",
                )
            })?;
        let tenant_id = request_context.workspace_target().tenant_id().to_string();
        let workspace_id = request_context
            .workspace_target()
            .workspace_id()
            .to_string();
        let request_digest = durable_request_digest(
            &tenant_id,
            &workspace_id,
            durable_contract,
            request,
            plan.payload(),
            plan.declaration(),
        );
        let canonical_digest = crate::canonical_digest::WorthServerCanonicalDigestBuilder::new(
            "worth-server-admitted-durable-product-mutation-v2",
        )
        .field("operation", plan.declaration().operation_name())
        .field("tenant", &tenant_id)
        .field("workspace", &workspace_id)
        .field("scope", durable_contract.authority_scope().value())
        .field("basis", expected_basis.base_digest().value())
        .field("key", idempotency_key.value())
        .field("request", &request_digest)
        .field("plan", plan.canonical_digest())
        .finish();
        Ok(Self {
            operation_name: plan.declaration().operation_name().to_string(),
            tenant_id,
            workspace_id,
            durable_contract: durable_contract.clone(),
            expected_basis,
            idempotency_key,
            request_digest,
            payload: plan.payload().clone(),
            result_contract: plan.declaration().result_contract().clone(),
            lowered_plan_digest: plan.canonical_digest().to_string(),
            canonical_digest,
        })
    }

    pub fn operation_name(&self) -> &str {
        &self.operation_name
    }

    pub fn tenant_id(&self) -> &str {
        &self.tenant_id
    }

    pub fn workspace_id(&self) -> &str {
        &self.workspace_id
    }

    pub fn authority_scope(&self) -> &WorthServerProductAuthorityScope {
        self.durable_contract.authority_scope()
    }

    pub fn durable_contract(&self) -> &WorthServerDurableProductMutationContract {
        &self.durable_contract
    }

    pub fn expected_basis(&self) -> &WorthServerProductSnapshotPrecondition {
        &self.expected_basis
    }

    pub fn idempotency_key(&self) -> &WorthServerProductIdempotencyKey {
        &self.idempotency_key
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn payload(&self) -> &WorthServerProductOperationPayload {
        &self.payload
    }

    pub fn result_contract(&self) -> &WorthServerProductResultContract {
        &self.result_contract
    }

    pub fn lowered_plan_digest(&self) -> &str {
        &self.lowered_plan_digest
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

fn durable_request_digest(
    tenant_id: &str,
    workspace_id: &str,
    durable_contract: &WorthServerDurableProductMutationContract,
    request: &crate::WorthServerOperationRequest,
    payload: &WorthServerProductOperationPayload,
    declaration: &crate::WorthServerProductOperationDeclaration,
) -> String {
    crate::canonical_digest::WorthServerCanonicalDigestBuilder::new(
        "worth-server-durable-product-request-v2",
    )
    .field("tenant", tenant_id)
    .field("workspace", workspace_id)
    .field("scope", durable_contract.authority_scope().value())
    .field("operation", request.identity().operation_name())
    .field("declaration", &declaration.canonical_digest())
    .field("basis", request.identity().basis_digest().unwrap_or("none"))
    .field(
        "key",
        request.identity().idempotency_key().unwrap_or("none"),
    )
    .field("payload", payload.envelope().canonical_digest())
    .field(
        "result_contract",
        declaration.result_contract().canonical_digest(),
    )
    .finish()
}

fn missing_durable_attempt_contract(
    detail: &str,
) -> crate::WorthServerProductOperationSurfaceDenial {
    crate::WorthServerProductOperationSurfaceDenial::new(
        crate::WorthServerProductOperationSurfaceDenialCode::InvalidDeclaration,
        detail.to_string(),
    )
}
