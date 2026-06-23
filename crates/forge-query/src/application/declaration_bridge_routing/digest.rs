use forge_foundational::facade::CanonicalDerivedDigest;

use crate::application::{
    ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationAspectCoverage,
    ForgeQueryDeclarationAspectCoverageBasis, ForgeQueryDeclarationAspectFit,
    ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationFutureProjection,
    ForgeQueryDeclarationReceiptDenialCause, ForgeQueryDeclarationRoutePlanDenialCause,
};
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

use super::{
    artifact::ForgeQueryDeclarationBridgeRoutingClass,
    contract::ForgeQueryDeclarationBridgeContinuationFamily,
    request::ForgeQueryDeclarationBridgeContinuationRequest,
};
use crate::application::declaration_aspect::terminal_declaration_aspect_projection;

pub(crate) fn derive_bridge_routing_digest<
    D: crate::application::ForgeQueryDomainEntryMarker,
    I: crate::application::ForgeQueryDeclarationInput<D>,
>(
    envelope: &ForgeQueryDeclarationEnvelope<D, I>,
    class: ForgeQueryDeclarationBridgeRoutingClass,
    continuation_request: ForgeQueryDeclarationBridgeContinuationRequest,
    continuation_family: ForgeQueryDeclarationBridgeContinuationFamily,
    binding_surface: &'static str,
    aspect_contract: &ForgeQueryDeclarationAspectContract,
    aspect_coverage: &ForgeQueryDeclarationAspectCoverage,
    aspect_coverage_basis: ForgeQueryDeclarationAspectCoverageBasis,
    aspect_fit: ForgeQueryDeclarationAspectFit,
    mapped_aspects: &ForgeQueryDeclarationAspectCoverage,
    mapping_fit: ForgeQueryDeclarationAspectFit,
    route_cause: Option<ForgeQueryDeclarationRoutePlanDenialCause>,
    receipt_cause: Option<ForgeQueryDeclarationReceiptDenialCause>,
    future_projection: &ForgeQueryDeclarationFutureProjection,
    basis_lifecycle_support_digest: &str,
) -> String {
    let receipt_digest = canonical_digest_token(envelope.receipt_digest());
    let envelope_digest = canonical_digest_token(envelope.envelope_digest());

    forge_query_evidence_identity(ForgeQueryEvidenceScope::DeclarationBridgeRoutingDigest)
        .field_value(
            ForgeQueryEvidenceTag::new("handle"),
            envelope.handle_identity_digest(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("operating_context"),
            envelope.operating_context_identity_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("family"),
            envelope.declaration_family_key(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("declaration"),
            envelope.declaration_digest(),
        )
        .optional_value(
            ForgeQueryEvidenceTag::new("progression"),
            envelope.progression_digest(),
        )
        .optional_value(
            ForgeQueryEvidenceTag::new("route_plan"),
            envelope.route_plan_digest(),
        )
        .field_value(ForgeQueryEvidenceTag::new("receipt"), &receipt_digest)
        .field_value(ForgeQueryEvidenceTag::new("envelope"), &envelope_digest)
        .field_shape(ForgeQueryEvidenceTag::new("class"), class.as_str())
        .field_shape(
            ForgeQueryEvidenceTag::new("mode"),
            continuation_request.mode().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("truth_context"),
            continuation_request.truth_context().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("continuation_family"),
            continuation_family.as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("binding_surface"),
            binding_surface,
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("aspect_contract_required"),
            aspect_contract
                .required()
                .iter()
                .map(terminal_declaration_aspect_projection),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("aspect_contract_preserved"),
            aspect_contract
                .preserved()
                .iter()
                .map(terminal_declaration_aspect_projection),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("aspect_contract_published"),
            aspect_contract
                .published()
                .iter()
                .map(terminal_declaration_aspect_projection),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("aspect_contract_masked"),
            aspect_contract
                .masked()
                .iter()
                .map(terminal_declaration_aspect_projection),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("aspect_contract_incompatible"),
            aspect_contract
                .incompatible()
                .iter()
                .map(terminal_declaration_aspect_projection),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("aspect_coverage_present"),
            aspect_coverage
                .present()
                .iter()
                .map(terminal_declaration_aspect_projection),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("aspect_coverage_masked"),
            aspect_coverage
                .masked()
                .iter()
                .map(terminal_declaration_aspect_projection),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("aspect_coverage_conflicting"),
            aspect_coverage
                .conflicting()
                .iter()
                .map(terminal_declaration_aspect_projection),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("aspect_coverage_basis"),
            aspect_coverage_basis.as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("aspect_fit"),
            aspect_fit.as_str(),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("mapped_aspect_present"),
            mapped_aspects
                .present()
                .iter()
                .map(terminal_declaration_aspect_projection),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("mapped_aspect_masked"),
            mapped_aspects
                .masked()
                .iter()
                .map(terminal_declaration_aspect_projection),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("mapped_aspect_conflicting"),
            mapped_aspects
                .conflicting()
                .iter()
                .map(terminal_declaration_aspect_projection),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("mapping_fit"),
            mapping_fit.as_str(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("future_projection"),
            future_projection.projection_digest(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("basis_lifecycle_support"),
            basis_lifecycle_support_digest,
        )
        .field_value(
            ForgeQueryEvidenceTag::new("evidence_origin"),
            envelope.evidence_origin().as_str(),
        )
        .optional_shape(
            ForgeQueryEvidenceTag::new("route_cause"),
            route_cause.map(ForgeQueryDeclarationRoutePlanDenialCause::as_str),
        )
        .optional_shape(
            ForgeQueryEvidenceTag::new("receipt_cause"),
            receipt_cause.map(ForgeQueryDeclarationReceiptDenialCause::as_str),
        )
        .seal()
        .as_str()
        .to_string()
}

fn canonical_digest_token(digest: &CanonicalDerivedDigest) -> String {
    let hex = digest
        .value()
        .bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format!("{}:{hex}", digest.metadata().algorithm().id().as_str())
}
