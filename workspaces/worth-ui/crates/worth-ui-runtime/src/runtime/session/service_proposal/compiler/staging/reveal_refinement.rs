//! Admission of the Focus owner's one lawful reveal refinement.
//!
//! Coordination stage 4 lets Focus emit at most one reveal refinement, which
//! requires the Scroll owner to replan for it. The stage machine admits
//! `ResolveFocusAndReveal` exactly once, so "at most one" is structural. What
//! this step adds is proof that a claimed refinement was really replanned: the
//! witness must name a Scroll owner participating at the exact scope it staged.

/// Validates a claimed reveal refinement against the proposal's participating
/// families, returning the scope to retain when the claim is backed.
pub(super) fn admit(
    candidate: &super::super::UiServiceProposalCandidate,
    staged_families: super::super::super::UiServiceFamilyParticipation,
    reveal_refinement: Option<super::super::super::UiServiceProposalOccupancyScopeIdentity>,
) -> Result<
    Option<super::super::super::UiServiceProposalOccupancyScopeIdentity>,
    super::UiServiceProposalStagingDenial,
> {
    let Some(scope) = reveal_refinement else {
        return Ok(None);
    };
    if !replanned_by_scroll_owner(candidate, staged_families, scope) {
        return Err(super::UiServiceProposalStagingDenial::UnbackedRevealRefinement);
    }
    Ok(Some(scope))
}

fn replanned_by_scroll_owner(
    candidate: &super::super::UiServiceProposalCandidate,
    staged_families: super::super::super::UiServiceFamilyParticipation,
    scope: super::super::super::UiServiceProposalOccupancyScopeIdentity,
) -> bool {
    let declared_at_scope = candidate.family_proposals().iter().any(|proposal| {
        proposal.family() == crate::capability::UiRuntimeServiceFamily::Scroll
            && proposal.scope() == scope
    });
    declared_at_scope && staged_families.contains(crate::capability::UiRuntimeServiceFamily::Scroll)
}
