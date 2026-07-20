use crate::{
    WorthServerOperationRequestInput, WorthServerProductOperationDeclaration,
    WorthServerProductOperationInput,
};

pub(in crate::product_adapter) fn build_request_input(
    declaration: &WorthServerProductOperationDeclaration,
    input: &WorthServerProductOperationInput,
) -> WorthServerOperationRequestInput {
    let mut builder = WorthServerOperationRequestInput::builder()
        .with_operation_family(declaration.operation_family())
        .with_operation_name(declaration.operation_name())
        .with_payload_envelope(input.payload().envelope().clone());
    if let Some(basis_digest) = input.basis_digest() {
        builder = builder.with_basis_digest(basis_digest);
    }
    if let Some(idempotency_key) = input.idempotency_key() {
        builder = builder.with_idempotency_key(idempotency_key.value());
    }
    if let Some(product_session_identity) = input.product_session_identity() {
        builder = builder.with_product_session_identity(product_session_identity);
    }
    builder.build()
}
