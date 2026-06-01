use forge_foundational::facade::{
    evaluate_boundary_role_claim_legality, evaluate_boundary_surface_disposition_legality,
};

use crate::application::{
    ForgeQueryDeclarationFamilySupportReport, ForgeQueryDeclarationInput,
    ForgeQueryDomainEntryMarker,
};
use crate::identity::hash_parts;

use super::{
    ForgeQueryDeclarationLegalityChecked, ForgeQueryDeclarationLegalityClass,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationLegalityDenial,
    ForgeQueryDeclarationLegalityEvidence, ForgeQueryDeclarationLegalityInput,
};

pub(crate) fn review_declaration_legality<D, I>(
    expected_handle_identity_digest: &str,
    input: ForgeQueryDeclarationLegalityInput<D, I>,
) -> ForgeQueryDeclarationLegalityChecked<D, I>
where
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
{
    let actual_handle_identity_digest = input.handle_identity_digest().to_string();
    if actual_handle_identity_digest != expected_handle_identity_digest {
        let (declaration, support_report, legality_contract, operating_context_identity_digest) =
            input.into_parts();
        return ForgeQueryDeclarationLegalityChecked::Illegal(
            ForgeQueryDeclarationLegalityDenial::WrongAdmittedWorld {
                declaration,
                expected_handle_identity_digest: expected_handle_identity_digest.to_string(),
                actual_handle_identity_digest,
                operating_context_identity_digest,
                support_report,
                legality_contract,
            },
        );
    }

    let (declaration, support_report, legality_contract, operating_context_identity_digest) =
        input.into_parts();

    match legality_contract.legality_class() {
        ForgeQueryDeclarationLegalityClass::DeferredBoundary => {
            return ForgeQueryDeclarationLegalityChecked::Illegal(
                ForgeQueryDeclarationLegalityDenial::DeferredByLegalityBoundary {
                    declaration,
                    operating_context_identity_digest,
                    support_report,
                    legality_contract,
                },
            );
        }
        ForgeQueryDeclarationLegalityClass::UnsupportedBoundary => {
            return ForgeQueryDeclarationLegalityChecked::Illegal(
                ForgeQueryDeclarationLegalityDenial::UnsupportedLegalityClass {
                    declaration,
                    operating_context_identity_digest,
                    support_report,
                    legality_contract,
                },
            );
        }
        _ => {}
    }

    if let Err(denial) = evaluate_boundary_role_claim_legality(
        legality_contract.category(),
        legality_contract.role(),
    ) {
        return ForgeQueryDeclarationLegalityChecked::Illegal(
            ForgeQueryDeclarationLegalityDenial::IllegalRoleClaim {
                declaration,
                denial,
                operating_context_identity_digest,
                support_report,
                legality_contract,
            },
        );
    }

    let surface_disposition = match evaluate_boundary_surface_disposition_legality(
        legality_contract.delivery_class(),
        legality_contract.availability(),
    ) {
        Ok(legality) => legality,
        Err(denial) => {
            return ForgeQueryDeclarationLegalityChecked::Illegal(
                ForgeQueryDeclarationLegalityDenial::IllegalSurfaceDisposition {
                    declaration,
                    denial,
                    operating_context_identity_digest,
                    support_report,
                    legality_contract,
                },
            );
        }
    };
    let legality_digest = derive_legality_digest(
        &declaration,
        &support_report,
        legality_contract,
        surface_disposition,
    );
    let reviewed_aspect_coverage = support_report.aspect_coverage().clone();
    ForgeQueryDeclarationLegalityChecked::Legal(ForgeQueryDeclarationLegalityEvidence::new(
        declaration,
        support_report,
        legality_contract,
        reviewed_aspect_coverage,
        operating_context_identity_digest,
        legality_contract.category(),
        legality_contract.role(),
        surface_disposition,
        legality_digest,
    ))
}

fn derive_legality_digest<D, I>(
    declaration: &crate::application::ForgeQueryCanonicalDeclarationArtifact<D, I>,
    support_report: &ForgeQueryDeclarationFamilySupportReport<D, I::Family>,
    legality_contract: ForgeQueryDeclarationLegalityContract,
    surface_disposition: forge_foundational::facade::FoundationalBoundarySurfaceDispositionLegality,
) -> String
where
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
{
    hash_parts(&[
        format!("handle:{}", declaration.handle_identity_digest()),
        format!("declaration:{:?}", declaration.declaration_digest()),
        format!("family:{}", declaration.declaration_family_key()),
        format!("taxonomy:{:?}", declaration.declaration_taxonomy()),
        format!("reviewed_aspects:{:?}", support_report.aspect_coverage()),
        format!("support:{}", support_report.support_digest()),
        format!("contract:{legality_contract:?}"),
        format!("surface:{surface_disposition:?}"),
    ])
}
