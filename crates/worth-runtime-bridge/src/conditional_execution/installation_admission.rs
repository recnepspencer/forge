use worth_query_installation::facade::WorthQueryPortableConditionalNodeDeclaration;

use super::{
    BridgeConditionalDenial, BridgeConditionalDenialKind, BridgeConditionalInstallationRequest,
};

pub(super) fn validate_declaration_pairing(
    request: &BridgeConditionalInstallationRequest,
) -> Result<(), BridgeConditionalDenial> {
    if request.location.node_identity() != request.declaration.identity() {
        return Err(denial(
            BridgeConditionalDenialKind::DeclarationLocationMismatch,
            "conditional location does not name the retained declaration",
        ));
    }
    if request.registrations.len() != request.declaration.dependencies().len() {
        return Err(denial(
            BridgeConditionalDenialKind::DeclarationCorrespondenceMismatch,
            "conditional correspondence count differs from the declared dependency count",
        ));
    }
    for (ordinal, dependency) in request.declaration.dependencies().iter().enumerate() {
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
        if registration.dependency().conditional_node_location() != &request.location
            || !registration
                .dependency()
                .matches_declared_dependency(dependency)
        {
            return Err(denial(
                BridgeConditionalDenialKind::DeclarationCorrespondenceMismatch,
                "conditional correspondence belongs to another declaration or semantic dependency",
            ));
        }
    }
    Ok(())
}

pub(super) fn validate_supported_postures(
    declaration: &WorthQueryPortableConditionalNodeDeclaration,
) -> Result<(), BridgeConditionalDenial> {
    use worth_query_installation::facade::{
        WorthQueryArtifactPosture, WorthQueryMaintenancePosture,
    };
    if matches!(
        declaration.maintenance(),
        WorthQueryMaintenancePosture::EagerOnEligibleInvalidation
    ) {
        return Err(denial(
            BridgeConditionalDenialKind::UnsupportedMaintenancePosture,
            "eager invalidation requires a scheduler-owned execution lane that is not installed",
        ));
    }
    if matches!(declaration.artifact(), WorthQueryArtifactPosture::Durable) {
        return Err(denial(
            BridgeConditionalDenialKind::UnsupportedArtifactPosture,
            "durable conditional artifacts require an installed persistence authority",
        ));
    }
    Ok(())
}

fn denial(kind: BridgeConditionalDenialKind, detail: &'static str) -> BridgeConditionalDenial {
    BridgeConditionalDenial::new(kind, detail)
}
