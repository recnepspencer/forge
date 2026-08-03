use super::{
    BridgeConditionalDenial, BridgeConditionalDenialKind, BridgeConditionalInstallationRequest,
};

pub(super) fn validate_declaration_pairing(
    request: &BridgeConditionalInstallationRequest,
) -> Result<(), BridgeConditionalDenial> {
    if !request.location.is_valid()
        || !request.contract.is_valid()
        || request.location.node_identity() != request.contract.identity()
    {
        return Err(denial(
            BridgeConditionalDenialKind::DeclarationLocationMismatch,
            "conditional location does not name the retained Bridge contract",
        ));
    }
    if request.registrations.len() != request.contract.dependency_count() {
        return Err(denial(
            BridgeConditionalDenialKind::DeclarationCorrespondenceMismatch,
            "conditional correspondence count differs from the declared dependency count",
        ));
    }
    for ordinal in 0..request.contract.dependency_count() {
        let registration = request
            .registrations
            .iter()
            .find(|item| item.dependency().dependency_ordinal() == ordinal)
            .ok_or_else(|| {
                denial(
                    BridgeConditionalDenialKind::DeclarationCorrespondenceMismatch,
                    "conditional correspondence omitted or duplicated a dependency ordinal",
                )
            })?;
        let dependency = registration.dependency();
        if dependency.source_node_identity() != request.location.node_identity()
            || dependency.source_stage_identity.as_deref() != request.location.stage_identity()
        {
            return Err(denial(
                BridgeConditionalDenialKind::DeclarationCorrespondenceMismatch,
                "conditional correspondence belongs to another source declaration",
            ));
        }
    }
    Ok(())
}

fn denial(kind: BridgeConditionalDenialKind, detail: &'static str) -> BridgeConditionalDenial {
    BridgeConditionalDenial::new(kind, detail)
}
