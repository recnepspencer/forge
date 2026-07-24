use super::{
    WorthQueryExecutionResourceRequest, WorthQueryResourceDimension, WorthQuerySemanticScaleAxis,
};

pub(super) fn validate_resource_request(
    request: &WorthQueryExecutionResourceRequest,
) -> Result<(), &'static str> {
    if WorthQuerySemanticScaleAxis::ALL
        .iter()
        .any(|axis| request.scale().get(*axis).is_none())
    {
        return Err("incomplete-semantic-scale-request");
    }
    if WorthQueryResourceDimension::ALL
        .iter()
        .any(|dimension| request.limits().get(*dimension).is_none())
    {
        return Err("incomplete-resource-limit-request");
    }
    if request.modes().is_empty() {
        return Err("empty-execution-mode-set");
    }
    Ok(())
}
