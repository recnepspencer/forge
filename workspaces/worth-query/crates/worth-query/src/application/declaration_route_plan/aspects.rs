use crate::application::{
    declaration_publication::declaration_publication_for_tier,
    route_scoped_declaration_aspect_contract, WorthQueryDeclarationAspectContract,
    WorthQueryDeclarationAspectCoverage, WorthQueryDeclarationAspectFit,
    WorthQueryDeclarationAspectPublication,
    WorthQueryDeclarationEntryOrchestrationMaterializationTier,
};

pub(super) fn route_aspect_contract(
    declaration_contract: &WorthQueryDeclarationAspectContract,
) -> WorthQueryDeclarationAspectContract {
    route_scoped_declaration_aspect_contract(declaration_contract)
}

pub(super) fn route_aspect_fit(
    coverage: &WorthQueryDeclarationAspectCoverage,
    route_contract: &WorthQueryDeclarationAspectContract,
) -> WorthQueryDeclarationAspectFit {
    coverage.fit_against(route_contract)
}

pub(super) fn route_aspect_publication(
    route_contract: &WorthQueryDeclarationAspectContract,
    coverage: &WorthQueryDeclarationAspectCoverage,
) -> WorthQueryDeclarationAspectPublication {
    declaration_publication_for_tier(
        route_contract,
        coverage,
        WorthQueryDeclarationEntryOrchestrationMaterializationTier::SupportReady,
    )
}

pub(super) fn route_aspect_publication_summary(
    publication: &WorthQueryDeclarationAspectPublication,
) -> String {
    format!(
        "present:{}|widened:{}|elided:{}|masked:{}",
        publication
            .terminal_present_projections_for_boundary()
            .join(","),
        publication
            .terminal_widened_projections_for_boundary()
            .join(","),
        publication
            .terminal_elided_projections_for_boundary()
            .join(","),
        publication
            .terminal_masked_projections_for_boundary()
            .join(",")
    )
}
