use crate::application::{
    declaration_publication::declaration_publication_for_tier,
    route_scoped_declaration_aspect_contract, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationAspectCoverage, ForgeQueryDeclarationAspectFit,
    ForgeQueryDeclarationAspectPublication,
    ForgeQueryDeclarationEntryOrchestrationMaterializationTier,
};

pub(super) fn route_aspect_contract(
    declaration_contract: &ForgeQueryDeclarationAspectContract,
) -> ForgeQueryDeclarationAspectContract {
    route_scoped_declaration_aspect_contract(declaration_contract)
}

pub(super) fn route_aspect_fit(
    coverage: &ForgeQueryDeclarationAspectCoverage,
    route_contract: &ForgeQueryDeclarationAspectContract,
) -> ForgeQueryDeclarationAspectFit {
    coverage.fit_against(route_contract)
}

pub(super) fn route_aspect_publication(
    route_contract: &ForgeQueryDeclarationAspectContract,
    coverage: &ForgeQueryDeclarationAspectCoverage,
) -> ForgeQueryDeclarationAspectPublication {
    declaration_publication_for_tier(
        route_contract,
        coverage,
        ForgeQueryDeclarationEntryOrchestrationMaterializationTier::SupportReady,
    )
}

pub(super) fn route_aspect_publication_summary(
    publication: &ForgeQueryDeclarationAspectPublication,
) -> String {
    format!(
        "present:{}|widened:{}|elided:{}|masked:{}",
        publication.present().join(","),
        publication.widened().join(","),
        publication.elided().join(","),
        publication.masked().join(",")
    )
}
