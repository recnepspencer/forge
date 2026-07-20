use crate::{
    WorthServerProductOperationDeclaration, WorthServerProductOperationDenial,
    WorthServerProductOperationDenialCode, WorthServerProductOperationPayload,
};

pub(in crate::product_adapter) fn validate_payload_schema(
    declaration: &WorthServerProductOperationDeclaration,
    payload: &WorthServerProductOperationPayload,
) -> Result<(), WorthServerProductOperationDenial> {
    if payload.envelope().declared_schema_identity() != Some(declaration.payload_schema_identity())
    {
        return Err(WorthServerProductOperationDenial::new(
            "invalid_payload_schema",
            format!(
                "payload schema `{}` did not match declared schema `{}`",
                payload
                    .envelope()
                    .declared_schema_identity()
                    .unwrap_or("none"),
                declaration.payload_schema_identity()
            ),
        )
        .with_code(WorthServerProductOperationDenialCode::PayloadSchemaMismatch));
    }
    if let Some(validator) = declaration.payload_validator() {
        validator.validate(payload).map_err(|denial| {
            denial.with_code(WorthServerProductOperationDenialCode::DeclaredPayloadValidator)
        })?;
    }
    Ok(())
}
