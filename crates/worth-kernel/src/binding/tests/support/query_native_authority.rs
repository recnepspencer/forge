use super::query_proof::admitted_rebinding_handle;
use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationEnvelope,
    ForgeQueryDomainOperatingContext, ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryOutcome,
    ForgeQueryOrdinaryPosture, ForgeQueryOrdinaryPostureKind,
};
use worth_spatial::facade::bindings::{
    author_primitive_rebinding_declaration, primitive_anchor_binding_rebinding_candidate_fact,
    primitive_anchor_binding_rebinding_prior_binding_fact,
    primitive_binding_rebinding_candidate_fact, primitive_binding_rebinding_prior_binding_fact,
    primitive_rebinding_projection_facts, primitive_rebinding_retained_fact_source,
    AuthorPrimitiveRebindingIntent, LocalTopologyReplacementNeighborhood,
    PrimitiveAnchorBindingDeclarationEntry, PrimitiveBindingDeclarationEntry,
    PrimitiveRebindingAuthoringError, PrimitiveRebindingCandidateFactError,
    PrimitiveRebindingDeclarationEntry, PrimitiveRebindingFactReceipt,
    PrimitiveRebindingPriorBindingFact, PrimitiveRebindingProjectionFactError,
    PrimitiveRebindingQueryDomain, RebindingOutcomeClass, ReplacementCandidate,
    SpatialRebindingAuthorityError,
};

pub(crate) fn rebinding_receipt_for_entry(
    entry: &PrimitiveRebindingDeclarationEntry,
    world: &'static str,
) -> Result<PrimitiveRebindingFactReceipt, SpatialRebindingAuthorityError> {
    let handle = admitted_rebinding_handle(world);
    match primitive_rebinding_retained_fact_source(entry, &handle) {
        Ok(source) => Ok(source.receipt().clone()),
        Err(
            worth_spatial::facade::bindings::PrimitiveRebindingProjectionFactError::DeclarationDenied(
                PrimitiveRebindingAuthoringError::Spatial(error),
            ),
        ) => Err(error),
        Err(error) => panic!("expected query-backed rebinding receipt, found {error:?}"),
    }
}

pub(crate) fn rebinding_ordinary_outcome_for_entry<C>(
    entry: &PrimitiveRebindingDeclarationEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
) -> ForgeQueryOrdinaryOutcome<
    ForgeQueryDeclarationEnvelope<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    >,
>
where
    C: ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>,
{
    match handle.orchestrate_declaration_entry_outcome(entry.clone()) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => {
            match primitive_rebinding_projection_facts(entry, handle) {
                Ok(facts) => ordinary_outcome_from_projection(facts.outcome_class(), envelope),
                Err(PrimitiveRebindingProjectionFactError::DeclarationDenied(error)) => {
                    ForgeQueryOrdinaryOutcome::Denied(rebinding_posture(
                        Box::leak(
                            format!(
                                "rebinding declaration denied before admitted family workflow: {error:?}"
                            )
                            .into_boxed_str(),
                        ),
                        ForgeQueryOrdinaryPostureKind::Denied,
                        ForgeQueryOrdinaryNextStep::CheckSupport,
                    ))
                }
                Err(PrimitiveRebindingProjectionFactError::OutcomeNotBound {
                    kind,
                    reason,
                    next_step,
                }) => ForgeQueryOrdinaryOutcome::Denied(rebinding_posture(
                    Box::leak(
                        format!(
                            "rebinding projection drifted after bound envelope: {kind:?} {reason} {next_step:?}"
                        )
                        .into_boxed_str(),
                    ),
                    ForgeQueryOrdinaryPostureKind::Denied,
                    ForgeQueryOrdinaryNextStep::CheckSupport,
                )),
            }
        }
        other => other,
    }
}

pub(crate) fn rebinding_prior_fact_from_binding_declaration(
    declaration: &PrimitiveBindingDeclarationEntry,
    world: &'static str,
) -> PrimitiveRebindingPriorBindingFact {
    primitive_binding_rebinding_prior_binding_fact(
        declaration,
        &super::query_proof::admitted_binding_handle(world),
    )
    .unwrap_or_else(|error| panic!("expected binding rebinding prior fact, found {error:?}"))
}

pub(crate) fn rebinding_prior_fact_from_anchor_declaration(
    declaration: &PrimitiveAnchorBindingDeclarationEntry,
    world: &'static str,
) -> PrimitiveRebindingPriorBindingFact {
    primitive_anchor_binding_rebinding_prior_binding_fact(
        declaration,
        &super::anchor_query_proof::admitted_anchor_binding_handle(world),
    )
    .unwrap_or_else(|error| panic!("expected anchor rebinding prior fact, found {error:?}"))
}

pub(crate) fn rebinding_candidate_from_binding_declaration(
    label: impl Into<String>,
    declaration: &PrimitiveBindingDeclarationEntry,
    world: &'static str,
) -> Result<ReplacementCandidate, SpatialRebindingAuthorityError> {
    ReplacementCandidate::new(
        label,
        primitive_binding_rebinding_candidate_fact(
            declaration,
            &super::query_proof::admitted_binding_handle(world),
        )
        .map_err(candidate_fact_error)?,
    )
}

pub(crate) fn rebinding_candidate_from_anchor_declaration(
    label: impl Into<String>,
    declaration: &PrimitiveAnchorBindingDeclarationEntry,
    world: &'static str,
) -> Result<ReplacementCandidate, SpatialRebindingAuthorityError> {
    ReplacementCandidate::new(
        label,
        primitive_anchor_binding_rebinding_candidate_fact(
            declaration,
            &super::anchor_query_proof::admitted_anchor_binding_handle(world),
        )
        .map_err(candidate_fact_error)?,
    )
}

pub(crate) fn replace_surface_binding(
    prior_binding: PrimitiveRebindingPriorBindingFact,
    neighborhood: LocalTopologyReplacementNeighborhood,
) -> AuthorPrimitiveRebindingIntent {
    AuthorPrimitiveRebindingIntent::replace_surface_binding(prior_binding, neighborhood)
}

pub(crate) fn replace_surface_binding_with_motion(
    prior_binding: PrimitiveRebindingPriorBindingFact,
    neighborhood: LocalTopologyReplacementNeighborhood,
    motion: worth_spatial::facade::bindings::BindingMotionSemanticsInput,
) -> AuthorPrimitiveRebindingIntent {
    AuthorPrimitiveRebindingIntent::replace_surface_binding_with_motion(
        prior_binding,
        neighborhood,
        motion,
    )
}

pub(crate) fn replace_curve_binding(
    prior_binding: PrimitiveRebindingPriorBindingFact,
    neighborhood: LocalTopologyReplacementNeighborhood,
) -> AuthorPrimitiveRebindingIntent {
    AuthorPrimitiveRebindingIntent::replace_curve_binding(prior_binding, neighborhood)
}

pub(crate) fn replace_curve_binding_with_motion(
    prior_binding: PrimitiveRebindingPriorBindingFact,
    neighborhood: LocalTopologyReplacementNeighborhood,
    motion: worth_spatial::facade::bindings::BindingMotionSemanticsInput,
) -> AuthorPrimitiveRebindingIntent {
    AuthorPrimitiveRebindingIntent::replace_curve_binding_with_motion(
        prior_binding,
        neighborhood,
        motion,
    )
}

pub(crate) fn replace_pcurve_binding(
    prior_binding: PrimitiveRebindingPriorBindingFact,
    neighborhood: LocalTopologyReplacementNeighborhood,
) -> AuthorPrimitiveRebindingIntent {
    AuthorPrimitiveRebindingIntent::replace_pcurve_binding(prior_binding, neighborhood)
}

pub(crate) fn replace_pcurve_binding_with_motion(
    prior_binding: PrimitiveRebindingPriorBindingFact,
    neighborhood: LocalTopologyReplacementNeighborhood,
    motion: worth_spatial::facade::bindings::BindingMotionSemanticsInput,
) -> AuthorPrimitiveRebindingIntent {
    AuthorPrimitiveRebindingIntent::replace_pcurve_binding_with_motion(
        prior_binding,
        neighborhood,
        motion,
    )
}

pub(crate) fn replace_geometry_binding_with_motion(
    prior_binding: PrimitiveRebindingPriorBindingFact,
    neighborhood: LocalTopologyReplacementNeighborhood,
    motion: worth_spatial::facade::bindings::BindingMotionSemanticsInput,
) -> AuthorPrimitiveRebindingIntent {
    AuthorPrimitiveRebindingIntent::replace_geometry_binding_with_motion(
        prior_binding,
        neighborhood,
        motion,
    )
}

pub(crate) fn rebind_surface_on_face(
    prior_binding: PrimitiveRebindingPriorBindingFact,
    neighborhood: LocalTopologyReplacementNeighborhood,
) -> Result<PrimitiveRebindingFactReceipt, SpatialRebindingAuthorityError> {
    let entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(prior_binding, neighborhood),
    );
    rebinding_receipt_for_entry(&entry, "support-rebinding-surface")
}

pub(crate) fn rebind_surface_on_face_with_motion(
    prior_binding: PrimitiveRebindingPriorBindingFact,
    neighborhood: LocalTopologyReplacementNeighborhood,
    motion: worth_spatial::facade::bindings::BindingMotionSemanticsInput,
) -> Result<PrimitiveRebindingFactReceipt, SpatialRebindingAuthorityError> {
    let entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding_with_motion(
            prior_binding,
            neighborhood,
            motion,
        ),
    );
    rebinding_receipt_for_entry(&entry, "support-rebinding-surface-motion")
}

fn candidate_fact_error(
    error: PrimitiveRebindingCandidateFactError,
) -> SpatialRebindingAuthorityError {
    match error {
        PrimitiveRebindingCandidateFactError::Binding(error) => {
            panic!("expected query-backed binding rebinding candidate fact, found {error:?}")
        }
        PrimitiveRebindingCandidateFactError::Anchor(error) => {
            panic!("expected query-backed anchor rebinding candidate fact, found {error:?}")
        }
        PrimitiveRebindingCandidateFactError::QueryNotBound => {
            panic!("expected query-backed rebinding candidate fact from a bound declaration")
        }
    }
}

fn ordinary_outcome_from_projection(
    outcome_class: RebindingOutcomeClass,
    envelope: ForgeQueryDeclarationEnvelope<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    >,
) -> ForgeQueryOrdinaryOutcome<
    ForgeQueryDeclarationEnvelope<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    >,
> {
    match outcome_class {
        RebindingOutcomeClass::Preserved
        | RebindingOutcomeClass::ExactReattachment
        | RebindingOutcomeClass::ContinuityJustifiedReattachment
        | RebindingOutcomeClass::CorrespondenceOnly => ForgeQueryOrdinaryOutcome::Bound(envelope),
        RebindingOutcomeClass::Ambiguous => {
            ForgeQueryOrdinaryOutcome::Ambiguous(rebinding_posture(
                "rebinding remained ambiguous within the admitted local replacement neighborhood",
                ForgeQueryOrdinaryPostureKind::Ambiguous,
                ForgeQueryOrdinaryNextStep::NarrowInput,
            ))
        }
        RebindingOutcomeClass::Orphaned => {
            ForgeQueryOrdinaryOutcome::RebindRequired(rebinding_posture(
                "rebinding remained orphaned within the admitted local replacement neighborhood",
                ForgeQueryOrdinaryPostureKind::RebindRequired,
                ForgeQueryOrdinaryNextStep::RebindContext,
            ))
        }
        RebindingOutcomeClass::Unsupported => {
            ForgeQueryOrdinaryOutcome::Unsupported(rebinding_posture(
                "rebinding family is unsupported for the admitted local replacement neighborhood",
                ForgeQueryOrdinaryPostureKind::Unsupported,
                ForgeQueryOrdinaryNextStep::CheckSupport,
            ))
        }
    }
}

fn rebinding_posture(
    reason: &'static str,
    kind: ForgeQueryOrdinaryPostureKind,
    next_step: ForgeQueryOrdinaryNextStep,
) -> ForgeQueryOrdinaryPosture {
    ForgeQueryOrdinaryPosture::new(
        reason,
        kind,
        next_step,
        forge_query::facade::ForgeQueryOrdinaryCheckedTopology::orchestration(
            forge_query::facade::ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
            None,
            None,
        ),
    )
}
