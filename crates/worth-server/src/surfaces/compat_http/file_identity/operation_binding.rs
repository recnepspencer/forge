use crate::{
    WorthServerCanonicalFilename, WorthServerExternalRequestContract,
    WorthServerQueryHandoffDenial, WorthServerQueryHandoffDenialCode,
};

pub(crate) fn validate_operation_name_binding(
    request_contract: &WorthServerExternalRequestContract,
    operation_name: &str,
    denial_code: WorthServerQueryHandoffDenialCode,
    diagnostics_profile: worth_foundational::facade::DiagnosticRichnessProfile,
) -> Result<(), WorthServerQueryHandoffDenial> {
    let expected_segment =
        expected_operation_segment(request_contract, diagnostics_profile, denial_code)?;
    let expected =
        WorthServerCanonicalFilename::admit(&expected_segment, diagnostics_profile, denial_code)?;
    let claimed =
        WorthServerCanonicalFilename::admit(operation_name, diagnostics_profile, denial_code)?;
    if expected.canonical() != claimed.canonical() {
        return Err(WorthServerQueryHandoffDenial::new(
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
    request_contract: &WorthServerExternalRequestContract,
    diagnostics_profile: worth_foundational::facade::DiagnosticRichnessProfile,
    denial_code: WorthServerQueryHandoffDenialCode,
) -> Result<String, WorthServerQueryHandoffDenial> {
    let route_stem = match request_contract.route_family() {
        crate::WorthServerCompatHttpRouteFamily::Read => "/compat/reads/",
        crate::WorthServerCompatHttpRouteFamily::Download => "/compat/downloads/",
        crate::WorthServerCompatHttpRouteFamily::Streaming => "/compat/streams/",
        crate::WorthServerCompatHttpRouteFamily::Upload => "/compat/uploads/",
        _ => {
            return Err(WorthServerQueryHandoffDenial::new(
                denial_code,
                diagnostics_profile,
                "compatibility operation-name binding is only defined for read, streaming, upload, and download families",
            ));
        }
    };
    let Some(remainder) = request_contract.normalized_path().strip_prefix(route_stem) else {
        return Err(WorthServerQueryHandoffDenial::new(
            denial_code,
            diagnostics_profile,
            format!(
                "compatibility path `{}` did not match the expected route family stem `{route_stem}`",
                request_contract.normalized_path()
            ),
        ));
    };
    if remainder.is_empty() || remainder.contains('/') {
        return Err(WorthServerQueryHandoffDenial::new(
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
