use forge_foundational::facade::FoundationalBoundaryEvidenceMaterializationProfile;

use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationFoundationalEvidenceInput, ForgeQueryDeclarationInput,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};
use crate::binding_pipeline::{
    ForgeQueryBindingAspectFitReport, ForgeQueryBindingLinkedArtifacts,
    ForgeQueryBindingNarrowingDecision, ForgeQueryBindingOutcome,
    ForgeQueryBindingRequestDescriptor, ForgeQueryBindingTranscript,
    ForgeQueryResolveRouteFromTargetRequest, ForgeQueryRouteResolverSubject,
};
use crate::target_binding::ForgeQueryBindingTargetWitness;

use super::common::{alignment_failure, denied_on_fit, digest_for, fit_allowed};

pub(crate) fn bind_route_from_target_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    request: ForgeQueryResolveRouteFromTargetRequest<D, I>,
) -> ForgeQueryBindingTranscript<crate::application::ForgeQueryDeclarationRoutePlanInput<D, I>> {
    let (source, contract, allow_superset, partial_narrowing, route_intent) = request.into_parts();
    let progressed = match source {
        ForgeQueryRouteResolverSubject::Progression(progressed) => progressed,
    };
    let binding_target = progressed.binding_target().into_erased_target();
    let linked = ForgeQueryBindingLinkedArtifacts::new()
        .with_declaration_digest(format!(
            "{:?}",
            progressed.canonical_declaration().declaration_digest()
        ))
        .with_progression_digest(progressed.progression_digest().to_string());
    let same_world = progressed.operating_context_identity_digest()
        == handle.operating_context_identity_digest();
    let same_handle = progressed.canonical_declaration().handle_identity_digest()
        == handle.handle_identity_digest();
    if !same_world || !same_handle {
        return alignment_failure(
            I::Family::semantic_family_key(),
            "resolve_route_from_target",
            contract,
            binding_target,
            linked,
            same_world,
        );
    }
    let coverage = progressed.reviewed_aspect_coverage().clone();
    let fit = coverage.fit_against(&contract);
    if !fit_allowed(fit, allow_superset) {
        return denied_on_fit(
            I::Family::semantic_family_key(),
            "resolve_route_from_target",
            contract,
            coverage,
            fit,
            partial_narrowing,
            binding_target,
            linked,
        );
    }
    let evidence = handle
        .describe_foundational_with_profile(
            ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progressed.clone(),
            ),
            FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness,
        )
        .unwrap_or_else(|_| {
            panic!("same-handle admitted progression should always describe foundational evidence")
        });
    let input = match route_intent {
        Some(intent) => crate::application::ForgeQueryDeclarationRoutePlanInput::with_intent(
            progressed, evidence, intent,
        ),
        None => {
            crate::application::ForgeQueryDeclarationRoutePlanInput::admitted(progressed, evidence)
        }
    };
    ForgeQueryBindingTranscript::new(
        ForgeQueryBindingRequestDescriptor::new(
            I::Family::semantic_family_key(),
            "resolve_route_from_target",
            contract.clone(),
        ),
        ForgeQueryBindingOutcome::Bound(input),
        Vec::new(),
        vec![crate::binding_pipeline::ForgeQueryBindingWitnessCheck::passed("world_alignment")],
        Some(ForgeQueryBindingAspectFitReport::new(
            fit,
            contract.clone(),
            coverage,
        )),
        vec![ForgeQueryBindingNarrowingDecision::new(
            "binding resolved progression into explicit route-plan input",
        )],
        Some(binding_target.clone()),
        digest_for(
            "resolve_route_from_target",
            I::Family::semantic_family_key(),
            &contract,
            "bound",
            Some(&binding_target),
            &linked,
        ),
        linked,
    )
}
