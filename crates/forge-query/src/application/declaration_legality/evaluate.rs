use forge_foundational::facade::{
    evaluate_boundary_role_claim_legality, evaluate_boundary_surface_disposition_legality,
};

use crate::application::{
    ForgeQueryAsyncDeclarationClause, ForgeQueryAsyncDeclarationSupport,
    ForgeQueryAsyncFailurePosture, ForgeQueryAsyncLoadingPosture, ForgeQueryAsyncSourceFamily,
    ForgeQueryDeclarationBridgeTruthContext, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationFamilySupportReport, ForgeQueryDeclarationInput,
    ForgeQueryDomainEntryMarker, ForgeQueryTemporalDeclarationSupport,
};
use crate::basis_lifecycle::BasisFamily;
use crate::identity::hash_parts;
use crate::runtime::ForgeQueryRuntimeFamilySupportStatus;

use super::{
    ForgeQueryAsyncLegalityDenialKind, ForgeQueryDeclarationLegalityChecked,
    ForgeQueryDeclarationLegalityClass, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationLegalityDenial, ForgeQueryDeclarationLegalityEvidence,
    ForgeQueryDeclarationLegalityInput, ForgeQueryTemporalLegalityDenialKind,
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
        let (declaration, support_report, legality_contract, world_basis, _, _) =
            input.into_parts();
        return ForgeQueryDeclarationLegalityChecked::Illegal(
            ForgeQueryDeclarationLegalityDenial::WrongAdmittedWorld {
                declaration,
                expected_handle_identity_digest: expected_handle_identity_digest.to_string(),
                actual_handle_identity_digest,
                operating_context_identity_digest: world_basis
                    .operating_context_identity_digest()
                    .to_string(),
                support_report,
                legality_contract,
            },
        );
    }

    let (
        declaration,
        support_report,
        legality_contract,
        world_basis,
        temporal_runtime_support,
        async_runtime_support,
    ) = input.into_parts();

    match legality_contract.legality_class() {
        ForgeQueryDeclarationLegalityClass::DeferredBoundary => {
            return ForgeQueryDeclarationLegalityChecked::Illegal(
                ForgeQueryDeclarationLegalityDenial::DeferredByLegalityBoundary {
                    declaration,
                    operating_context_identity_digest: world_basis
                        .operating_context_identity_digest()
                        .to_string(),
                    support_report,
                    legality_contract,
                },
            );
        }
        ForgeQueryDeclarationLegalityClass::UnsupportedBoundary => {
            return ForgeQueryDeclarationLegalityChecked::Illegal(
                ForgeQueryDeclarationLegalityDenial::UnsupportedLegalityClass {
                    declaration,
                    operating_context_identity_digest: world_basis
                        .operating_context_identity_digest()
                        .to_string(),
                    support_report,
                    legality_contract,
                },
            );
        }
        _ => {}
    }

    if let Some(kind) =
        temporal_legality_denial_kind::<D, I>(&declaration, temporal_runtime_support)
    {
        return ForgeQueryDeclarationLegalityChecked::Illegal(
            ForgeQueryDeclarationLegalityDenial::TemporalProjectionUnsupported {
                declaration,
                kind,
                operating_context_identity_digest: world_basis
                    .operating_context_identity_digest()
                    .to_string(),
                support_report,
                legality_contract,
            },
        );
    }

    if let Some(kind) = async_legality_denial_kind::<D, I>(&declaration, async_runtime_support) {
        return ForgeQueryDeclarationLegalityChecked::Illegal(
            ForgeQueryDeclarationLegalityDenial::AsyncProjectionUnsupported {
                declaration,
                kind,
                operating_context_identity_digest: world_basis
                    .operating_context_identity_digest()
                    .to_string(),
                support_report,
                legality_contract,
            },
        );
    }

    if let Err(denial) = evaluate_boundary_role_claim_legality(
        legality_contract.category(),
        legality_contract.role(),
    ) {
        return ForgeQueryDeclarationLegalityChecked::Illegal(
            ForgeQueryDeclarationLegalityDenial::IllegalRoleClaim {
                declaration,
                denial,
                operating_context_identity_digest: world_basis
                    .operating_context_identity_digest()
                    .to_string(),
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
                    operating_context_identity_digest: world_basis
                        .operating_context_identity_digest()
                        .to_string(),
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
        world_basis,
        legality_contract.category(),
        legality_contract.role(),
        surface_disposition,
        legality_digest,
    ))
}

fn async_legality_denial_kind<D, I>(
    declaration: &crate::application::ForgeQueryCanonicalDeclarationArtifact<D, I>,
    async_runtime_support: Option<ForgeQueryRuntimeFamilySupportStatus>,
) -> Option<ForgeQueryAsyncLegalityDenialKind>
where
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
{
    let clauses = declaration.async_resource_clauses();
    if clauses.is_empty() {
        return None;
    }

    if I::Family::async_declaration_support() == ForgeQueryAsyncDeclarationSupport::Unsupported {
        return Some(ForgeQueryAsyncLegalityDenialKind::RuntimeFacadeUnsupported);
    }

    if let Some(kind) = async_basis_legality_denial_kind::<D, I>() {
        return Some(kind);
    }

    for clause in clauses {
        if let Some(kind) = async_clause_legality_denial_kind(clause) {
            return Some(kind);
        }
    }

    match async_runtime_support {
        Some(ForgeQueryRuntimeFamilySupportStatus::DeferredDebt) => {
            Some(ForgeQueryAsyncLegalityDenialKind::RuntimeFacadeDeferred)
        }
        Some(ForgeQueryRuntimeFamilySupportStatus::Unsupported) | None => {
            Some(ForgeQueryAsyncLegalityDenialKind::RuntimeFacadeUnsupported)
        }
        Some(ForgeQueryRuntimeFamilySupportStatus::Supported) => None,
    }
}

fn async_basis_legality_denial_kind<D, I>() -> Option<ForgeQueryAsyncLegalityDenialKind>
where
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
{
    if let Some(contract) = I::Family::bridge_continuation_contract() {
        match contract.request().truth_context() {
            ForgeQueryDeclarationBridgeTruthContext::Historical => {
                return Some(ForgeQueryAsyncLegalityDenialKind::HistoricalTruthBasisUnsupported);
            }
            ForgeQueryDeclarationBridgeTruthContext::Preview => {
                return Some(ForgeQueryAsyncLegalityDenialKind::PreviewTruthBasisUnsupported);
            }
            ForgeQueryDeclarationBridgeTruthContext::Current => {}
        }
    }

    if let Some(contract) = I::Family::signal_compatibility_contract() {
        if contract
            .required_basis_families()
            .contains(&BasisFamily::HistoricalSnapshot)
        {
            return Some(ForgeQueryAsyncLegalityDenialKind::HistoricalSignalBasisUnsupported);
        }
        if contract
            .required_basis_families()
            .contains(&BasisFamily::PreviewDerived)
        {
            return Some(ForgeQueryAsyncLegalityDenialKind::PreviewSignalBasisUnsupported);
        }
    }

    None
}

fn async_clause_legality_denial_kind(
    clause: &ForgeQueryAsyncDeclarationClause,
) -> Option<ForgeQueryAsyncLegalityDenialKind> {
    match clause {
        ForgeQueryAsyncDeclarationClause::ResourceRequest {
            source_family,
            loading_posture,
            failure_posture,
            ..
        } => {
            if *source_family != ForgeQueryAsyncSourceFamily::BridgeResource {
                return Some(ForgeQueryAsyncLegalityDenialKind::UnsupportedSourceFamily(
                    *source_family,
                ));
            }
            if *loading_posture != ForgeQueryAsyncLoadingPosture::Blocking {
                return Some(
                    ForgeQueryAsyncLegalityDenialKind::UnsupportedLoadingPosture(*loading_posture),
                );
            }
            if *failure_posture != ForgeQueryAsyncFailurePosture::FailClosed {
                return Some(
                    ForgeQueryAsyncLegalityDenialKind::UnsupportedFailurePosture(*failure_posture),
                );
            }
            None
        }
        ForgeQueryAsyncDeclarationClause::CompletionRequest { .. } => {
            Some(ForgeQueryAsyncLegalityDenialKind::CompletionLifecycleUnsupported)
        }
    }
}

fn temporal_legality_denial_kind<D, I>(
    declaration: &crate::application::ForgeQueryCanonicalDeclarationArtifact<D, I>,
    temporal_runtime_support: Option<ForgeQueryRuntimeFamilySupportStatus>,
) -> Option<ForgeQueryTemporalLegalityDenialKind>
where
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
{
    if declaration.temporal_clauses().is_empty() {
        return None;
    }

    if I::Family::temporal_declaration_support()
        == ForgeQueryTemporalDeclarationSupport::Unsupported
    {
        return Some(ForgeQueryTemporalLegalityDenialKind::RuntimeFacadeUnsupported);
    }

    if let Some(contract) = I::Family::bridge_continuation_contract() {
        match contract.request().truth_context() {
            ForgeQueryDeclarationBridgeTruthContext::Historical => {
                return Some(ForgeQueryTemporalLegalityDenialKind::HistoricalTruthBasisUnsupported);
            }
            ForgeQueryDeclarationBridgeTruthContext::Preview => {
                return Some(ForgeQueryTemporalLegalityDenialKind::PreviewTruthBasisUnsupported);
            }
            ForgeQueryDeclarationBridgeTruthContext::Current => {}
        }
    }

    if let Some(contract) = I::Family::signal_compatibility_contract() {
        if contract
            .required_basis_families()
            .contains(&BasisFamily::HistoricalSnapshot)
        {
            return Some(ForgeQueryTemporalLegalityDenialKind::HistoricalSignalBasisUnsupported);
        }
        if contract
            .required_basis_families()
            .contains(&BasisFamily::PreviewDerived)
        {
            return Some(ForgeQueryTemporalLegalityDenialKind::PreviewSignalBasisUnsupported);
        }
    }

    match temporal_runtime_support {
        Some(ForgeQueryRuntimeFamilySupportStatus::DeferredDebt) => {
            Some(ForgeQueryTemporalLegalityDenialKind::RuntimeFacadeDeferred)
        }
        Some(ForgeQueryRuntimeFamilySupportStatus::Unsupported) | None => {
            Some(ForgeQueryTemporalLegalityDenialKind::RuntimeFacadeUnsupported)
        }
        Some(ForgeQueryRuntimeFamilySupportStatus::Supported) => None,
    }
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
