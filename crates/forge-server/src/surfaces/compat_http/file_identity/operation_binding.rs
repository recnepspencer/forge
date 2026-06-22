use crate::{
    ForgeServerCanonicalFilename, ForgeServerExternalRequestContract,
    ForgeServerQueryHandoffDenial, ForgeServerQueryHandoffDenialCode,
};

pub(crate) fn validate_operation_name_binding(
    request_contract: &ForgeServerExternalRequestContract,
    operation_name: &str,
    denial_code: ForgeServerQueryHandoffDenialCode,
    diagnostics_profile: forge_foundational::facade::DiagnosticRichnessProfile,
) -> Result<(), ForgeServerQueryHandoffDenial> {
    let expected_segment =
        expected_operation_segment(request_contract, diagnostics_profile, denial_code)?;
    let expected =
        ForgeServerCanonicalFilename::admit(&expected_segment, diagnostics_profile, denial_code)?;
    let claimed =
        ForgeServerCanonicalFilename::admit(operation_name, diagnostics_profile, denial_code)?;
    if expected.canonical() != claimed.canonical() {
        return Err(ForgeServerQueryHandoffDenial::new(
            denial_code,
            diagnostics_profile,
            format!(
                "compatibility operation name `{}` did not match the external request path identity `{}`",
                claimed.original(),
                expected.original(),
            ),
        ));
    }
    Ok(())
}

fn expected_operation_segment(
    request_contract: &ForgeServerExternalRequestContract,
    diagnostics_profile: forge_foundational::facade::DiagnosticRichnessProfile,
    denial_code: ForgeServerQueryHandoffDenialCode,
) -> Result<String, ForgeServerQueryHandoffDenial> {
    let route_stem = match request_contract.route_family() {
        crate::ForgeServerCompatHttpRouteFamily::Read => "/compat/reads/",
        crate::ForgeServerCompatHttpRouteFamily::Download => "/compat/downloads/",
        crate::ForgeServerCompatHttpRouteFamily::Streaming => "/compat/streams/",
        crate::ForgeServerCompatHttpRouteFamily::Upload => "/compat/uploads/",
        _ => {
            return Err(ForgeServerQueryHandoffDenial::new(
                denial_code,
                diagnostics_profile,
                "compatibility operation-name binding is only defined for read, streaming, upload, and download families",
            ));
        }
    };
    let Some(remainder) = request_contract.normalized_path().strip_prefix(route_stem) else {
        return Err(ForgeServerQueryHandoffDenial::new(
            denial_code,
            diagnostics_profile,
            format!(
                "compatibility path `{}` did not match the expected route family stem `{route_stem}`",
                request_contract.normalized_path()
            ),
        ));
    };
    if remainder.is_empty() || remainder.contains('/') {
        return Err(ForgeServerQueryHandoffDenial::new(
            denial_code,
            diagnostics_profile,
            format!(
                "compatibility path `{}` must carry exactly one operation segment after `{route_stem}`",
                request_contract.normalized_path()
            ),
        ));
    }
    Ok(remainder.to_string())
}
