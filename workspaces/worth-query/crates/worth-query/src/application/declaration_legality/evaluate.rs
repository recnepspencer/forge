use worth_foundational::facade::{
    evaluate_boundary_role_claim_legality, evaluate_boundary_surface_disposition_legality,
};

use crate::application::{
    WorthQueryAsyncDeclarationClause, WorthQueryAsyncDeclarationSupport,
    WorthQueryAsyncFailurePosture, WorthQueryAsyncLoadingPosture, WorthQueryAsyncSourceFamily,
    WorthQueryDeclarationBridgeTruthContext, WorthQueryDeclarationFamilyMarker,
    WorthQueryDeclarationFamilySupportReport, WorthQueryDeclarationInput,
    WorthQueryDomainEntryMarker, WorthQueryTemporalDeclarationSupport,
};
use crate::basis_lifecycle::BasisFamily;
use crate::identity::hash_parts;
use crate::runtime::WorthQueryRuntimeFamilySupportStatus;

use super::{
    WorthQueryAsyncLegalityDenialKind, WorthQueryDeclarationLegalityChecked,
    WorthQueryDeclarationLegalityClass, WorthQueryDeclarationLegalityContract,
    WorthQueryDeclarationLegalityDenial, WorthQueryDeclarationLegalityEvidence,
    WorthQueryDeclarationLegalityInput, WorthQueryTemporalLegalityDenialKind,
};

pub(crate) fn review_declaration_legality<D, I>(
    expected_handle_identity_digest: &str,
    input: WorthQueryDeclarationLegalityInput<D, I>,
) -> WorthQueryDeclarationLegalityChecked<D, I>
where
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
{
    let actual_handle_identity_digest = input.handle_identity_digest().to_string();
    if actual_handle_identity_digest != expected_handle_identity_digest {
        let (declaration, support_report, legality_contract, world_basis, _, _) =
            input.into_parts();
        return WorthQueryDeclarationLegalityChecked::Illegal(
            WorthQueryDeclarationLegalityDenial::WrongAdmittedWorld {
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
        WorthQueryDeclarationLegalityClass::DeferredBoundary => {
            return WorthQueryDeclarationLegalityChecked::Illegal(
                WorthQueryDeclarationLegalityDenial::DeferredByLegalityBoundary {
                    declaration,
                    operating_context_identity_digest: world_basis
                        .operating_context_identity_digest()
                        .to_string(),
                    support_report,
                    legality_contract,
                },
            );
        }
        WorthQueryDeclarationLegalityClass::UnsupportedBoundary => {
            return WorthQueryDeclarationLegalityChecked::Illegal(
                WorthQueryDeclarationLegalityDenial::UnsupportedLegalityClass {
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
        return WorthQueryDeclarationLegalityChecked::Illegal(
            WorthQueryDeclarationLegalityDenial::TemporalProjectionUnsupported {
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
        return WorthQueryDeclarationLegalityChecked::Illegal(
            WorthQueryDeclarationLegalityDenial::AsyncProjectionUnsupported {
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
        return WorthQueryDeclarationLegalityChecked::Illegal(
            WorthQueryDeclarationLegalityDenial::IllegalRoleClaim {
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
            return WorthQueryDeclarationLegalityChecked::Illegal(
                WorthQueryDeclarationLegalityDenial::IllegalSurfaceDisposition {
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
    WorthQueryDeclarationLegalityChecked::Legal(WorthQueryDeclarationLegalityEvidence::new(
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
    declaration: &crate::application::WorthQueryCanonicalDeclarationArtifact<D, I>,
    async_runtime_support: Option<WorthQueryRuntimeFamilySupportStatus>,
) -> Option<WorthQueryAsyncLegalityDenialKind>
where
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
{
    let clauses = declaration.async_resource_clauses();
    if clauses.is_empty() {
        return None;
    }

    if I::Family::async_declaration_support() == WorthQueryAsyncDeclarationSupport::Unsupported {
        return Some(WorthQueryAsyncLegalityDenialKind::RuntimeFacadeUnsupported);
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
        Some(WorthQueryRuntimeFamilySupportStatus::DeferredDebt) => {
            Some(WorthQueryAsyncLegalityDenialKind::RuntimeFacadeDeferred)
        }
        Some(WorthQueryRuntimeFamilySupportStatus::Unsupported) | None => {
            Some(WorthQueryAsyncLegalityDenialKind::RuntimeFacadeUnsupported)
        }
        Some(WorthQueryRuntimeFamilySupportStatus::Supported) => None,
    }
}

fn async_basis_legality_denial_kind<D, I>() -> Option<WorthQueryAsyncLegalityDenialKind>
where
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
{
    if let Some(contract) = I::Family::bridge_continuation_contract() {
        match contract.request().truth_context() {
            WorthQueryDeclarationBridgeTruthContext::Historical => {
                return Some(WorthQueryAsyncLegalityDenialKind::HistoricalTruthBasisUnsupported);
            }
            WorthQueryDeclarationBridgeTruthContext::Preview => {
                return Some(WorthQueryAsyncLegalityDenialKind::PreviewTruthBasisUnsupported);
            }
            WorthQueryDeclarationBridgeTruthContext::Current => {}
        }
    }

    if let Some(contract) = I::Family::signal_compatibility_contract() {
        if contract
            .required_basis_families()
            .contains(&BasisFamily::HistoricalSnapshot)
        {
            return Some(WorthQueryAsyncLegalityDenialKind::HistoricalSignalBasisUnsupported);
        }
        if contract
            .required_basis_families()
            .contains(&BasisFamily::PreviewDerived)
        {
            return Some(WorthQueryAsyncLegalityDenialKind::PreviewSignalBasisUnsupported);
        }
    }

    None
}

fn async_clause_legality_denial_kind(
    clause: &WorthQueryAsyncDeclarationClause,
) -> Option<WorthQueryAsyncLegalityDenialKind> {
    match clause {
        WorthQueryAsyncDeclarationClause::ResourceRequest {
            source_family,
            loading_posture,
            failure_posture,
            ..
        } => {
            if *source_family != WorthQueryAsyncSourceFamily::BridgeResource {
                return Some(WorthQueryAsyncLegalityDenialKind::UnsupportedSourceFamily(
                    *source_family,
                ));
            }
            if *loading_posture != WorthQueryAsyncLoadingPosture::Blocking {
                return Some(
                    WorthQueryAsyncLegalityDenialKind::UnsupportedLoadingPosture(*loading_posture),
                );
            }
            if *failure_posture != WorthQueryAsyncFailurePosture::FailClosed {
                return Some(
                    WorthQueryAsyncLegalityDenialKind::UnsupportedFailurePosture(*failure_posture),
                );
            }
            None
        }
        WorthQueryAsyncDeclarationClause::CompletionRequest { .. } => {
            Some(WorthQueryAsyncLegalityDenialKind::CompletionLifecycleUnsupported)
        }
    }
}

fn temporal_legality_denial_kind<D, I>(
    declaration: &crate::application::WorthQueryCanonicalDeclarationArtifact<D, I>,
    temporal_runtime_support: Option<WorthQueryRuntimeFamilySupportStatus>,
) -> Option<WorthQueryTemporalLegalityDenialKind>
where
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
{
    if declaration.temporal_clauses().is_empty() {
        return None;
    }

    if I::Family::temporal_declaration_support()
        == WorthQueryTemporalDeclarationSupport::Unsupported
    {
        return Some(WorthQueryTemporalLegalityDenialKind::RuntimeFacadeUnsupported);
    }

    if let Some(contract) = I::Family::bridge_continuation_contract() {
        match contract.request().truth_context() {
            WorthQueryDeclarationBridgeTruthContext::Historical => {
                return Some(WorthQueryTemporalLegalityDenialKind::HistoricalTruthBasisUnsupported);
            }
            WorthQueryDeclarationBridgeTruthContext::Preview => {
                return Some(WorthQueryTemporalLegalityDenialKind::PreviewTruthBasisUnsupported);
            }
            WorthQueryDeclarationBridgeTruthContext::Current => {}
        }
    }

    if let Some(contract) = I::Family::signal_compatibility_contract() {
        if contract
            .required_basis_families()
            .contains(&BasisFamily::HistoricalSnapshot)
        {
            return Some(WorthQueryTemporalLegalityDenialKind::HistoricalSignalBasisUnsupported);
        }
        if contract
            .required_basis_families()
            .contains(&BasisFamily::PreviewDerived)
        {
            return Some(WorthQueryTemporalLegalityDenialKind::PreviewSignalBasisUnsupported);
        }
    }

    match temporal_runtime_support {
        Some(WorthQueryRuntimeFamilySupportStatus::DeferredDebt) => {
            Some(WorthQueryTemporalLegalityDenialKind::RuntimeFacadeDeferred)
        }
        Some(WorthQueryRuntimeFamilySupportStatus::Unsupported) | None => {
            Some(WorthQueryTemporalLegalityDenialKind::RuntimeFacadeUnsupported)
        }
        Some(WorthQueryRuntimeFamilySupportStatus::Supported) => None,
    }
}

fn derive_legality_digest<D, I>(
    declaration: &crate::application::WorthQueryCanonicalDeclarationArtifact<D, I>,
    support_report: &WorthQueryDeclarationFamilySupportReport<D, I::Family>,
    legality_contract: WorthQueryDeclarationLegalityContract,
    surface_disposition: worth_foundational::facade::FoundationalBoundarySurfaceDispositionLegality,
) -> String
where
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
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
