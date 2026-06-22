use std::collections::BTreeSet;

use crate::application::{
    dispatch_graph_obligations_for_orchestration, ForgeQueryAdmittedConfiguredDomainHandle,
    ForgeQueryDeclarationAspectCoverage, ForgeQueryDeclarationAspectCoverageBasis,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryGraphObligationOrchestrationBoundary,
};

use super::artifact::{
    ForgeQueryGroupedAspectParticipationSummary, ForgeQueryGroupedDeclarationArtifact,
    ForgeQueryGroupedDeclarationAspectRecord, ForgeQueryGroupedDeclarationMember,
};
use super::declaration_stop::{grouped_declaration_stop, ForgeQueryGroupedDeclarationStop};
use super::input::ForgeQueryGroupedDeclarationInput;
use super::posture::ForgeQueryGroupedMemberRole;

pub enum ForgeQueryGroupedDeclarationChecked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Bound(ForgeQueryGroupedDeclarationArtifact<D, I>),
    MemberStopped(ForgeQueryGroupedDeclarationStop),
}

pub(crate) fn forge_query_grouped_declaration_checked_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D> + Clone,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    input: ForgeQueryGroupedDeclarationInput<D, I>,
) -> ForgeQueryGroupedDeclarationChecked<D, I> {
    let (
        semantics,
        ordering,
        atomicity,
        grouping_intent,
        continuity_assumption,
        shared_posture_claims,
        shared_rationale,
        member_inputs,
    ) = input.into_parts();
    let graph_dispatch = match dispatch_graph_obligations_for_orchestration(
        ForgeQueryGraphObligationOrchestrationBoundary::DeclarationEntry,
        handle.operating_context_identity_digest(),
        I::Family::orchestration_graph_touch_descriptor(),
        I::Family::orchestration_graph_touch_collection(),
        I::Family::orchestration_graph_obligation_registrations(),
    ) {
        Ok(dispatch) => dispatch,
        Err(error) => {
            return ForgeQueryGroupedDeclarationChecked::MemberStopped(
                ForgeQueryGroupedDeclarationStop::graph_obligation_dispatch_failed(
                    0,
                    I::Family::semantic_family_key(),
                    error,
                ),
            )
        }
    };
    if let Some(dispatch) = graph_dispatch.as_ref() {
        if dispatch.blocking_denial_projection().is_some() {
            return ForgeQueryGroupedDeclarationChecked::MemberStopped(
                ForgeQueryGroupedDeclarationStop::graph_obligation_denied(
                    0,
                    I::Family::semantic_family_key(),
                    dispatch.clone(),
                ),
            );
        }
    }
    let mut members = Vec::with_capacity(member_inputs.len());
    for (member_index, member_input) in member_inputs.into_iter().enumerate() {
        let role = member_role(member_index);
        match handle.declare_review_and_progress(member_input.clone()) {
            Ok(progression) => {
                let aspect_record = ForgeQueryGroupedDeclarationAspectRecord::new(
                    progression.aspect_contract().clone(),
                    progression.reviewed_aspect_coverage().clone(),
                    ForgeQueryDeclarationAspectCoverageBasis::ReviewedRetainedCoverage,
                );
                members.push(ForgeQueryGroupedDeclarationMember::new(
                    member_index,
                    role,
                    member_input,
                    progression,
                    aspect_record,
                ));
            }
            Err(error) => {
                return ForgeQueryGroupedDeclarationChecked::MemberStopped(
                    grouped_declaration_stop::<D, I>(member_index, &error),
                );
            }
        }
    }
    let aspect_record = grouped_aspect_record::<D, I>(&members);
    let participation = grouped_aspect_participation(&members);
    let artifact = ForgeQueryGroupedDeclarationArtifact::new(
        handle.operating_context_identity_digest().to_string(),
        semantics,
        ordering,
        atomicity,
        grouping_intent,
        continuity_assumption,
        shared_posture_claims,
        shared_rationale,
        aspect_record,
        participation,
        members,
    );
    ForgeQueryGroupedDeclarationChecked::Bound(
        artifact.with_graph_obligation_dispatch(graph_dispatch),
    )
}

fn member_role(member_index: usize) -> ForgeQueryGroupedMemberRole {
    if member_index == 0 {
        ForgeQueryGroupedMemberRole::Seed
    } else {
        ForgeQueryGroupedMemberRole::Member
    }
}

fn grouped_aspect_record<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    members: &[ForgeQueryGroupedDeclarationMember<D, I>],
) -> ForgeQueryGroupedDeclarationAspectRecord {
    let first = members
        .first()
        .expect("grouped declarations require at least one member");
    ForgeQueryGroupedDeclarationAspectRecord::new(
        first.aspect_record().contract().clone(),
        coverage_union(
            members
                .iter()
                .map(|member| member.aspect_record().coverage()),
        ),
        ForgeQueryDeclarationAspectCoverageBasis::ReviewedRetainedCoverage,
    )
}

fn grouped_aspect_participation<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    members: &[ForgeQueryGroupedDeclarationMember<D, I>],
) -> ForgeQueryGroupedAspectParticipationSummary {
    let present_sets = members
        .iter()
        .map(|member| {
            member
                .aspect_record()
                .coverage()
                .present()
                .iter()
                .cloned()
                .collect::<BTreeSet<_>>()
        })
        .collect::<Vec<_>>();
    let present_any = coverage_union(
        members
            .iter()
            .map(|member| member.aspect_record().coverage()),
    )
    .present()
    .to_vec();
    let present_all = present_sets
        .iter()
        .cloned()
        .reduce(|left, right| left.intersection(&right).cloned().collect())
        .unwrap_or_default()
        .into_iter()
        .collect::<Vec<_>>();
    let union = coverage_union(
        members
            .iter()
            .map(|member| member.aspect_record().coverage()),
    );
    ForgeQueryGroupedAspectParticipationSummary::new(
        present_any,
        present_all,
        union.masked().to_vec(),
        union.conflicting().to_vec(),
    )
}

fn coverage_union<'a>(
    coverages: impl IntoIterator<Item = &'a ForgeQueryDeclarationAspectCoverage>,
) -> ForgeQueryDeclarationAspectCoverage {
    let mut present = BTreeSet::new();
    let mut masked = BTreeSet::new();
    let mut conflicting = BTreeSet::new();
    for coverage in coverages {
        present.extend(coverage.present().iter().cloned());
        masked.extend(coverage.masked().iter().cloned());
        conflicting.extend(coverage.conflicting().iter().cloned());
    }
    ForgeQueryDeclarationAspectCoverage::new(present, masked, conflicting)
}
