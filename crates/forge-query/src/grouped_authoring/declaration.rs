use std::collections::BTreeSet;

use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationAdmissionError,
    ForgeQueryDeclarationAdmissionOrLegalityError, ForgeQueryDeclarationAspectCoverage,
    ForgeQueryDeclarationAspectCoverageBasis, ForgeQueryDeclarationEntryProgressionError,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityDenial, ForgeQueryDeclarationProgressionTerminalError,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};

use super::artifact::{
    ForgeQueryGroupedAspectParticipationSummary, ForgeQueryGroupedDeclarationArtifact,
    ForgeQueryGroupedDeclarationAspectRecord, ForgeQueryGroupedDeclarationMember,
};
use super::input::ForgeQueryGroupedDeclarationInput;
use super::posture::ForgeQueryGroupedMemberRole;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryGroupedDeclarationStopKind {
    Deferred,
    Unsupported,
    InvalidContext,
    Canonicalization,
    Denied,
    Stale,
    RebindRequired,
    Failed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGroupedDeclarationStop {
    member_index: usize,
    declaration_family_key: &'static str,
    stop_kind: ForgeQueryGroupedDeclarationStopKind,
    reason: String,
}

impl ForgeQueryGroupedDeclarationStop {
    fn new(
        member_index: usize,
        declaration_family_key: &'static str,
        stop_kind: ForgeQueryGroupedDeclarationStopKind,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            member_index,
            declaration_family_key,
            stop_kind,
            reason: reason.into(),
        }
    }

    pub fn member_index(&self) -> usize {
        self.member_index
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.declaration_family_key
    }

    pub fn stop_kind(&self) -> ForgeQueryGroupedDeclarationStopKind {
        self.stop_kind
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

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
    ForgeQueryGroupedDeclarationChecked::Bound(ForgeQueryGroupedDeclarationArtifact::new(
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
    ))
}

fn grouped_declaration_stop<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    member_index: usize,
    error: &ForgeQueryDeclarationEntryProgressionError<D, I>,
) -> ForgeQueryGroupedDeclarationStop {
    match error {
        ForgeQueryDeclarationEntryProgressionError::Entry(entry) => {
            grouped_declaration_entry_stop(member_index, entry)
        }
        ForgeQueryDeclarationEntryProgressionError::Progression(progression) => {
            grouped_progression_stop(member_index, progression)
        }
    }
}

fn grouped_declaration_entry_stop<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    member_index: usize,
    error: &ForgeQueryDeclarationAdmissionOrLegalityError<D, I>,
) -> ForgeQueryGroupedDeclarationStop {
    match error {
        ForgeQueryDeclarationAdmissionOrLegalityError::Admission(admission) => {
            grouped_admission_stop(member_index, admission)
        }
        ForgeQueryDeclarationAdmissionOrLegalityError::Legality(legality) => {
            grouped_legality_stop(member_index, legality)
        }
    }
}

fn grouped_admission_stop<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    member_index: usize,
    error: &ForgeQueryDeclarationAdmissionError<D, I>,
) -> ForgeQueryGroupedDeclarationStop {
    match error {
        ForgeQueryDeclarationAdmissionError::Deferred(denial) => ForgeQueryGroupedDeclarationStop::new(
            member_index,
            I::Family::semantic_family_key(),
            ForgeQueryGroupedDeclarationStopKind::Deferred,
            format!(
                "member {member_index} declaration deferred with capability status {}",
                denial.capability_status().as_str()
            ),
        ),
        ForgeQueryDeclarationAdmissionError::AsyncDeferred(denial) => ForgeQueryGroupedDeclarationStop::new(
            member_index,
            I::Family::semantic_family_key(),
            ForgeQueryGroupedDeclarationStopKind::Deferred,
            format!(
                "member {member_index} declaration deferred because async support is {}",
                denial.async_support().as_str()
            ),
        ),
        ForgeQueryDeclarationAdmissionError::TemporalDeferred(denial) => ForgeQueryGroupedDeclarationStop::new(
            member_index,
            I::Family::semantic_family_key(),
            ForgeQueryGroupedDeclarationStopKind::Deferred,
            format!(
                "member {member_index} declaration deferred because temporal support is {}",
                denial.temporal_support().as_str()
            ),
        ),
        ForgeQueryDeclarationAdmissionError::Unsupported(denial) => ForgeQueryGroupedDeclarationStop::new(
            member_index,
            I::Family::semantic_family_key(),
            ForgeQueryGroupedDeclarationStopKind::Unsupported,
            format!(
                "member {member_index} declaration unsupported with capability status {}",
                denial.capability_status().as_str()
            ),
        ),
        ForgeQueryDeclarationAdmissionError::AsyncUnsupported(denial) => ForgeQueryGroupedDeclarationStop::new(
            member_index,
            I::Family::semantic_family_key(),
            ForgeQueryGroupedDeclarationStopKind::Unsupported,
            format!(
                "member {member_index} declaration unsupported because async support is {}",
                denial.async_support().as_str()
            ),
        ),
        ForgeQueryDeclarationAdmissionError::TemporalUnsupported(denial) => ForgeQueryGroupedDeclarationStop::new(
            member_index,
            I::Family::semantic_family_key(),
            ForgeQueryGroupedDeclarationStopKind::Unsupported,
            format!(
                "member {member_index} declaration unsupported because temporal support is {}",
                denial.temporal_support().as_str()
            ),
        ),
        ForgeQueryDeclarationAdmissionError::InvalidContext(denial) => ForgeQueryGroupedDeclarationStop::new(
            member_index,
            I::Family::semantic_family_key(),
            ForgeQueryGroupedDeclarationStopKind::InvalidContext,
            format!(
                "member {member_index} declaration invalid in the admitted context with capability status {}",
                denial.capability_status().as_str()
            ),
        ),
        ForgeQueryDeclarationAdmissionError::Canonicalization(error) => ForgeQueryGroupedDeclarationStop::new(
            member_index,
            I::Family::semantic_family_key(),
            ForgeQueryGroupedDeclarationStopKind::Canonicalization,
            format!("member {member_index} canonicalization failed: {error:?}"),
        ),
    }
}

fn grouped_legality_stop<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    member_index: usize,
    error: &ForgeQueryDeclarationLegalityDenial<D, I>,
) -> ForgeQueryGroupedDeclarationStop {
    match error {
        ForgeQueryDeclarationLegalityDenial::DeferredByLegalityBoundary { .. } => {
            ForgeQueryGroupedDeclarationStop::new(
                member_index,
                I::Family::semantic_family_key(),
                ForgeQueryGroupedDeclarationStopKind::Deferred,
                format!("member {member_index} declaration deferred by legality boundary"),
            )
        }
        ForgeQueryDeclarationLegalityDenial::UnsupportedLegalityClass { .. } => {
            ForgeQueryGroupedDeclarationStop::new(
                member_index,
                I::Family::semantic_family_key(),
                ForgeQueryGroupedDeclarationStopKind::Unsupported,
                format!("member {member_index} declaration uses an unsupported legality class"),
            )
        }
        ForgeQueryDeclarationLegalityDenial::TemporalProjectionUnsupported { kind, .. }
            if kind.is_deferred() =>
        {
            ForgeQueryGroupedDeclarationStop::new(
                member_index,
                I::Family::semantic_family_key(),
                ForgeQueryGroupedDeclarationStopKind::Deferred,
                format!("member {member_index} declaration deferred by temporal legality boundary"),
            )
        }
        ForgeQueryDeclarationLegalityDenial::AsyncProjectionUnsupported { kind, .. }
            if kind.is_deferred() =>
        {
            ForgeQueryGroupedDeclarationStop::new(
                member_index,
                I::Family::semantic_family_key(),
                ForgeQueryGroupedDeclarationStopKind::Deferred,
                format!("member {member_index} declaration deferred by async legality boundary"),
            )
        }
        ForgeQueryDeclarationLegalityDenial::WrongAdmittedWorld { .. }
        | ForgeQueryDeclarationLegalityDenial::IllegalRoleClaim { .. }
        | ForgeQueryDeclarationLegalityDenial::IllegalSurfaceDisposition { .. }
        | ForgeQueryDeclarationLegalityDenial::TemporalProjectionUnsupported { .. }
        | ForgeQueryDeclarationLegalityDenial::AsyncProjectionUnsupported { .. } => {
            ForgeQueryGroupedDeclarationStop::new(
                member_index,
                I::Family::semantic_family_key(),
                ForgeQueryGroupedDeclarationStopKind::InvalidContext,
                format!("member {member_index} declaration failed legality review"),
            )
        }
    }
}

fn grouped_progression_stop<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    member_index: usize,
    error: &ForgeQueryDeclarationProgressionTerminalError<D, I>,
) -> ForgeQueryGroupedDeclarationStop {
    let (kind, reason) = match error {
        ForgeQueryDeclarationProgressionTerminalError::Deferred(_) => (
            ForgeQueryGroupedDeclarationStopKind::Deferred,
            "declaration progression deferred",
        ),
        ForgeQueryDeclarationProgressionTerminalError::Denied(_) => (
            ForgeQueryGroupedDeclarationStopKind::Denied,
            "declaration progression denied",
        ),
        ForgeQueryDeclarationProgressionTerminalError::Stale(_) => (
            ForgeQueryGroupedDeclarationStopKind::Stale,
            "declaration progression went stale",
        ),
        ForgeQueryDeclarationProgressionTerminalError::RebindRequired(_) => (
            ForgeQueryGroupedDeclarationStopKind::RebindRequired,
            "declaration progression requires rebind",
        ),
        ForgeQueryDeclarationProgressionTerminalError::Failed(_) => (
            ForgeQueryGroupedDeclarationStopKind::Failed,
            "declaration progression failed",
        ),
    };
    ForgeQueryGroupedDeclarationStop::new(
        member_index,
        I::Family::semantic_family_key(),
        kind,
        format!("member {member_index} {reason}"),
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
