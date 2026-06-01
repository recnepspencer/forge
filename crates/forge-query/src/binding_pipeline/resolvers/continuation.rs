use crate::application::{
    aspect_coverage_from_publication, bridge_authority_summary_from_coverage,
    merged_authority_aspect_contract, ForgeQueryAdmittedConfiguredDomainHandle,
    ForgeQueryDeclarationAspectCoverageBasis, ForgeQueryDeclarationBridgeRoutingInput,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext,
};
use crate::binding_pipeline::{
    ForgeQueryBindingAspectFitReport, ForgeQueryBindingLinkedArtifacts,
    ForgeQueryBindingNarrowingDecision, ForgeQueryBindingOutcome,
    ForgeQueryBindingRequestDescriptor, ForgeQueryBindingTranscript, ForgeQueryBindingUnsupported,
    ForgeQueryContinuationBindingInput, ForgeQueryResolveContinuationFromTargetRequest,
};
use crate::target_binding::ForgeQueryBindingTargetWitness;

use super::common::{
    alignment_failure, denied_on_authority_mismatch, denied_on_fit, digest_for, fit_allowed,
};

pub(crate) fn bind_continuation_from_target_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    request: ForgeQueryResolveContinuationFromTargetRequest<D, I>,
) -> ForgeQueryBindingTranscript<ForgeQueryContinuationBindingInput<D, I>> {
    let (envelope, contract, allow_superset, partial_narrowing, bridge_request) =
        request.into_parts();
    let binding_target = envelope.binding_target().into_erased_target();
    let linked = {
        let mut linked = ForgeQueryBindingLinkedArtifacts::new()
            .with_declaration_digest(envelope.declaration_digest().to_string())
            .with_receipt_digest(format!("{:?}", envelope.receipt_digest()))
            .with_envelope_digest(format!("{:?}", envelope.envelope_digest()));
        if let Some(progression_digest) = envelope.progression_digest() {
            linked = linked.with_progression_digest(progression_digest.to_string());
        }
        if let Some(route_plan_digest) = envelope.route_plan_digest() {
            linked = linked.with_route_plan_digest(route_plan_digest.to_string());
        }
        linked
    };
    let same_world =
        envelope.operating_context_identity_digest() == handle.operating_context_identity_digest();
    let same_handle = envelope.handle_identity_digest() == handle.handle_identity_digest();
    if !same_world || !same_handle {
        return alignment_failure(
            I::Family::semantic_family_key(),
            "resolve_continuation_from_target",
            contract,
            binding_target,
            linked,
            same_world,
        );
    }
    let Some(family_contract) = I::Family::bridge_continuation_contract() else {
        return ForgeQueryBindingTranscript::new(
            ForgeQueryBindingRequestDescriptor::new(
                I::Family::semantic_family_key(),
                "resolve_continuation_from_target",
                contract.clone(),
            ),
            ForgeQueryBindingOutcome::Unsupported(ForgeQueryBindingUnsupported::new(
                "the declaration family does not expose a bridge continuation contract",
            )),
            Vec::new(),
            Vec::new(),
            None,
            vec![ForgeQueryBindingNarrowingDecision::new(
                "binding stopped because no continuation contract exists for this family",
            )],
            Some(binding_target.clone()),
            digest_for(
                "resolve_continuation_from_target",
                I::Family::semantic_family_key(),
                &contract,
                "unsupported",
                Some(&binding_target),
                &linked,
            ),
            linked,
        );
    };
    let default_request = family_contract.request();
    let merged_required_aspects =
        merged_authority_aspect_contract(family_contract.required_aspects(), &contract);
    let bridge_contract = family_contract.with_required_aspects(merged_required_aspects);
    let coverage = aspect_coverage_from_publication(envelope.aspect_publication());
    let summary = bridge_authority_summary_from_coverage(
        envelope.aspect_contract(),
        coverage.clone(),
        ForgeQueryDeclarationAspectCoverageBasis::EnvelopePublishedCoverage,
        Some(&bridge_contract),
    );
    let fit = summary.aspect_fit();
    if let Some(mismatch) = summary.aspect_mismatch() {
        return denied_on_authority_mismatch(
            I::Family::semantic_family_key(),
            "resolve_continuation_from_target",
            contract,
            coverage,
            fit,
            mismatch,
            binding_target,
            linked,
        );
    }
    if !fit_allowed(fit, allow_superset) {
        return denied_on_fit(
            I::Family::semantic_family_key(),
            "resolve_continuation_from_target",
            contract,
            coverage,
            fit,
            partial_narrowing,
            binding_target,
            linked,
        );
    }
    ForgeQueryBindingTranscript::new(
        ForgeQueryBindingRequestDescriptor::new(
            I::Family::semantic_family_key(),
            "resolve_continuation_from_target",
            contract.clone(),
        ),
        ForgeQueryBindingOutcome::Bound(ForgeQueryContinuationBindingInput::bridge(
            bridge_request.unwrap_or(default_request),
            ForgeQueryDeclarationBridgeRoutingInput::enveloped(envelope),
        )),
        Vec::new(),
        vec![
            crate::binding_pipeline::ForgeQueryBindingWitnessCheck::passed(
                "bridge_authority_summary",
            ),
        ],
        Some(ForgeQueryBindingAspectFitReport::new(
            fit,
            contract.clone(),
            coverage,
        )),
        vec![ForgeQueryBindingNarrowingDecision::new(
            "binding resolved envelope into explicit bridge continuation input",
        )],
        Some(binding_target.clone()),
        digest_for(
            "resolve_continuation_from_target",
            I::Family::semantic_family_key(),
            &contract,
            "bound",
            Some(&binding_target),
            &linked,
        ),
        linked,
    )
}
