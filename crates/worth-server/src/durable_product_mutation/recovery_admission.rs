use crate::{
    WorthServerAdmission, WorthServerOperationRegistry, WorthServerProductOperationDeclaration,
    WorthServerProductOperationSurfaceDenial, WorthServerProductOperationSurfaceDenialCode,
};

pub(crate) fn admit_durable_product_recovery(
    operation_registry: &WorthServerOperationRegistry,
    admission: &WorthServerAdmission,
    declaration: &WorthServerProductOperationDeclaration,
) -> Result<(), WorthServerProductOperationSurfaceDenial> {
    let family = declaration.operation_family();
    let surface = admission.resolved_request_context().surface_family();
    operation_registry
        .admit(surface, family)
        .map_err(|denial| {
            recovery_denial(format!("recovery surface admission denied: {denial:?}"))
        })?;
    operation_registry
        .admit_operation_name(family, declaration.operation_name())
        .map_err(|denial| {
            recovery_denial(format!("recovery operation admission denied: {denial:?}"))
        })?;
    crate::operation_admission::authorize_operation(
        family,
        admission.query_handoff_intent().kind(),
        operation_registry.authorization_policy_for(family),
        admission,
    )
    .map_err(recovery_denial)?;

    Ok(())
}

fn recovery_denial(detail: impl Into<String>) -> WorthServerProductOperationSurfaceDenial {
    WorthServerProductOperationSurfaceDenial::new(
        WorthServerProductOperationSurfaceDenialCode::AdmissionDenied,
        detail.into(),
    )
}
