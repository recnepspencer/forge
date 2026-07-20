use std::collections::BTreeSet;

use crate::application::{
    dispatch_graph_obligations_for_orchestration, WorthQueryDeclarationAspectCoverage,
    WorthQueryDeclarationAspectCoverageBasis, WorthQueryDeclarationFamilyMarker,
    WorthQueryDeclarationInput, WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext,
    WorthQueryGraphObligationOrchestrationBoundary, WorthQueryInstalledDomainDeclarationContext,
};

use super::artifact::{
    WorthQueryGroupedAspectParticipationSummary, WorthQueryGroupedDeclarationArtifact,
    WorthQueryGroupedDeclarationAspectRecord, WorthQueryGroupedDeclarationMember,
};
use super::declaration_stop::{grouped_declaration_stop, WorthQueryGroupedDeclarationStop};
use super::input::WorthQueryGroupedDeclarationInput;
use super::posture::WorthQueryGroupedMemberRole;

pub enum WorthQueryGroupedDeclarationChecked<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    Bound(WorthQueryGroupedDeclarationArtifact<D, I>),
    MemberStopped(WorthQueryGroupedDeclarationStop),
}

pub(crate) fn worth_query_grouped_declaration_checked_on_handle<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D> + Clone,
>(
    handle: &WorthQueryInstalledDomainDeclarationContext<D, C>,
    input: WorthQueryGroupedDeclarationInput<D, I>,
) -> WorthQueryGroupedDeclarationChecked<D, I> {
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
        WorthQueryGraphObligationOrchestrationBoundary::DeclarationEntry,
        handle.operating_context_identity_digest(),
        I::Family::orchestration_graph_touch_descriptor(),
        I::Family::orchestration_graph_touch_collection(),
        I::Family::orchestration_graph_obligation_registrations(),
    ) {
        Ok(dispatch) => dispatch,
        Err(error) => {
            return WorthQueryGroupedDeclarationChecked::MemberStopped(
                WorthQueryGroupedDeclarationStop::graph_obligation_dispatch_failed(
                    0,
                    I::Family::semantic_family_key(),
                    error,
                ),
            )
        }
    };
    if let Some(dispatch) = graph_dispatch.as_ref() {
        if dispatch.blocking_denial_projection().is_some() {
            return WorthQueryGroupedDeclarationChecked::MemberStopped(
                WorthQueryGroupedDeclarationStop::graph_obligation_denied(
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
                let aspect_record = WorthQueryGroupedDeclarationAspectRecord::new(
                    progression.aspect_contract().clone(),
                    progression.reviewed_aspect_coverage().clone(),
                    WorthQueryDeclarationAspectCoverageBasis::ReviewedRetainedCoverage,
                );
                members.push(WorthQueryGroupedDeclarationMember::new(
                    member_index,
                    role,
                    member_input,
                    progression,
                    aspect_record,
                ));
            }
            Err(error) => {
                return WorthQueryGroupedDeclarationChecked::MemberStopped(
                    grouped_declaration_stop::<D, I>(member_index, &error),
                );
            }
        }
    }
    let aspect_record = grouped_aspect_record::<D, I>(&members);
    let participation = grouped_aspect_participation(&members);
    let artifact = WorthQueryGroupedDeclarationArtifact::new(
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
    WorthQueryGroupedDeclarationChecked::Bound(
        artifact.with_graph_obligation_dispatch(graph_dispatch),
    )
}

fn member_role(member_index: usize) -> WorthQueryGroupedMemberRole {
    if member_index == 0 {
        WorthQueryGroupedMemberRole::Seed
    } else {
        WorthQueryGroupedMemberRole::Member
    }
}

fn grouped_aspect_record<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>(
    members: &[WorthQueryGroupedDeclarationMember<D, I>],
) -> WorthQueryGroupedDeclarationAspectRecord {
    let first = members
        .first()
        .expect("grouped declarations require at least one member");
    WorthQueryGroupedDeclarationAspectRecord::new(
        first.aspect_record().contract().clone(),
        coverage_union(
            members
                .iter()
                .map(|member| member.aspect_record().coverage()),
        ),
        WorthQueryDeclarationAspectCoverageBasis::ReviewedRetainedCoverage,
    )
}

fn grouped_aspect_participation<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    members: &[WorthQueryGroupedDeclarationMember<D, I>],
) -> WorthQueryGroupedAspectParticipationSummary {
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
    WorthQueryGroupedAspectParticipationSummary::new(
        present_any,
        present_all,
        union.masked().to_vec(),
        union.conflicting().to_vec(),
    )
}

fn coverage_union<'a>(
    coverages: impl IntoIterator<Item = &'a WorthQueryDeclarationAspectCoverage>,
) -> WorthQueryDeclarationAspectCoverage {
    let mut present = BTreeSet::new();
    let mut masked = BTreeSet::new();
    let mut conflicting = BTreeSet::new();
    for coverage in coverages {
        present.extend(coverage.present().iter().cloned());
        masked.extend(coverage.masked().iter().cloned());
        conflicting.extend(coverage.conflicting().iter().cloned());
    }
    WorthQueryDeclarationAspectCoverage::new(present, masked, conflicting)
}
