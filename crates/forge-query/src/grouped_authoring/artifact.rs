use crate::application::{
    ForgeQueryAdmittedDeclarationProgression, ForgeQueryCanonicalDeclarationArtifact,
    ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationAspectCoverage,
    ForgeQueryDeclarationAspectCoverageBasis, ForgeQueryDeclarationInput,
    ForgeQueryDomainEntryMarker, ForgeQueryGroupedDeclarationPosture,
};
use crate::identity::hash_parts;

use super::posture::{
    ForgeQueryGroupedAtomicity, ForgeQueryGroupedContinuityAssumption, ForgeQueryGroupedIntent,
    ForgeQueryGroupedMemberRole, ForgeQueryGroupedOrdering, ForgeQueryGroupedSemantics,
    ForgeQueryGroupedSharedPostureClaim,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGroupedDeclarationAspectRecord {
    contract: ForgeQueryDeclarationAspectContract,
    coverage: ForgeQueryDeclarationAspectCoverage,
    coverage_basis: ForgeQueryDeclarationAspectCoverageBasis,
}

impl ForgeQueryGroupedDeclarationAspectRecord {
    pub(crate) fn new(
        contract: ForgeQueryDeclarationAspectContract,
        coverage: ForgeQueryDeclarationAspectCoverage,
        coverage_basis: ForgeQueryDeclarationAspectCoverageBasis,
    ) -> Self {
        Self {
            contract,
            coverage,
            coverage_basis,
        }
    }

    pub fn contract(&self) -> &ForgeQueryDeclarationAspectContract {
        &self.contract
    }

    pub fn coverage(&self) -> &ForgeQueryDeclarationAspectCoverage {
        &self.coverage
    }

    pub fn coverage_basis(&self) -> ForgeQueryDeclarationAspectCoverageBasis {
        self.coverage_basis
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGroupedAspectParticipationSummary {
    present_any: Vec<String>,
    present_all: Vec<String>,
    masked_any: Vec<String>,
    conflicting_any: Vec<String>,
}

impl ForgeQueryGroupedAspectParticipationSummary {
    pub(crate) fn new(
        present_any: Vec<String>,
        present_all: Vec<String>,
        masked_any: Vec<String>,
        conflicting_any: Vec<String>,
    ) -> Self {
        Self {
            present_any,
            present_all,
            masked_any,
            conflicting_any,
        }
    }

    pub fn present_any(&self) -> &[String] {
        &self.present_any
    }

    pub fn present_all(&self) -> &[String] {
        &self.present_all
    }

    pub fn masked_any(&self) -> &[String] {
        &self.masked_any
    }

    pub fn conflicting_any(&self) -> &[String] {
        &self.conflicting_any
    }
}

#[derive(Clone)]
pub struct ForgeQueryGroupedDeclarationMember<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    member_index: usize,
    role: ForgeQueryGroupedMemberRole,
    member_input: I,
    progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
    aspect_record: ForgeQueryGroupedDeclarationAspectRecord,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryGroupedDeclarationMember<D, I>
{
    pub(crate) fn new(
        member_index: usize,
        role: ForgeQueryGroupedMemberRole,
        member_input: I,
        progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
        aspect_record: ForgeQueryGroupedDeclarationAspectRecord,
    ) -> Self {
        Self {
            member_index,
            role,
            member_input,
            progression,
            aspect_record,
        }
    }

    pub fn member_index(&self) -> usize {
        self.member_index
    }

    pub fn role(&self) -> ForgeQueryGroupedMemberRole {
        self.role
    }

    pub fn declaration(&self) -> &ForgeQueryCanonicalDeclarationArtifact<D, I> {
        self.progression.canonical_declaration()
    }

    pub fn input(&self) -> &I {
        &self.member_input
    }

    pub fn aspect_record(&self) -> &ForgeQueryGroupedDeclarationAspectRecord {
        &self.aspect_record
    }

    pub(crate) fn progression(&self) -> &ForgeQueryAdmittedDeclarationProgression<D, I> {
        &self.progression
    }
}

#[derive(Clone)]
pub struct ForgeQueryGroupedDeclarationArtifact<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    handle_identity_digest: String,
    operating_context_identity_digest: String,
    declaration_family_key: &'static str,
    grouped_posture: ForgeQueryGroupedDeclarationPosture,
    semantics: ForgeQueryGroupedSemantics,
    ordering: ForgeQueryGroupedOrdering,
    atomicity: ForgeQueryGroupedAtomicity,
    grouping_intent: ForgeQueryGroupedIntent,
    continuity_assumption: ForgeQueryGroupedContinuityAssumption,
    shared_posture_claims: Vec<ForgeQueryGroupedSharedPostureClaim>,
    shared_rationale: Option<String>,
    aspect_record: ForgeQueryGroupedDeclarationAspectRecord,
    aspect_participation: ForgeQueryGroupedAspectParticipationSummary,
    members: Vec<ForgeQueryGroupedDeclarationMember<D, I>>,
    group_digest: String,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryGroupedDeclarationArtifact<D, I>
{
    pub(crate) fn new(
        operating_context_identity_digest: String,
        semantics: ForgeQueryGroupedSemantics,
        ordering: ForgeQueryGroupedOrdering,
        atomicity: ForgeQueryGroupedAtomicity,
        grouping_intent: ForgeQueryGroupedIntent,
        continuity_assumption: ForgeQueryGroupedContinuityAssumption,
        shared_posture_claims: Vec<ForgeQueryGroupedSharedPostureClaim>,
        shared_rationale: Option<String>,
        aspect_record: ForgeQueryGroupedDeclarationAspectRecord,
        aspect_participation: ForgeQueryGroupedAspectParticipationSummary,
        members: Vec<ForgeQueryGroupedDeclarationMember<D, I>>,
    ) -> Self {
        let first = members
            .first()
            .expect("grouped declaration artifact requires at least one member");
        let handle_identity_digest = first.declaration().handle_identity_digest().to_string();
        let declaration_family_key = first.declaration().declaration_family_key();
        let grouped_posture = first.declaration().declaration_grouped_posture();
        let group_digest = hash_parts(&[
            format!("handle:{handle_identity_digest}"),
            format!("family:{declaration_family_key}"),
            format!("grouped_posture:{}", grouped_posture.as_str()),
            format!("semantics:{}", semantics.as_str()),
            format!("ordering:{}", ordering.as_str()),
            format!("atomicity:{}", atomicity.as_str()),
            format!("grouping_intent:{}", grouping_intent.as_str()),
            format!("continuity_assumption:{}", continuity_assumption.as_str()),
            format!(
                "shared_posture_claims:{}",
                shared_posture_claims
                    .iter()
                    .map(|claim| claim.as_str())
                    .collect::<Vec<_>>()
                    .join("|")
            ),
            format!(
                "shared_rationale:{}",
                shared_rationale.as_deref().unwrap_or("none")
            ),
            format!("aspect_contract:{:?}", aspect_record.contract()),
            format!("aspect_coverage:{:?}", aspect_record.coverage()),
            format!("aspect_coverage_basis:{:?}", aspect_record.coverage_basis()),
            format!(
                "present_all:{}",
                aspect_participation.present_all().join("|")
            ),
            format!(
                "conflicting_any:{}",
                aspect_participation.conflicting_any().join("|")
            ),
            format!(
                "members:{}",
                members
                    .iter()
                    .map(|member| format!(
                        "{}:{}:{:?}:{:?}",
                        member.member_index(),
                        member.role().as_str(),
                        member.declaration().declaration_digest(),
                        member.aspect_record().coverage()
                    ))
                    .collect::<Vec<_>>()
                    .join("|")
            ),
        ]);
        Self {
            handle_identity_digest,
            operating_context_identity_digest,
            declaration_family_key,
            grouped_posture,
            semantics,
            ordering,
            atomicity,
            grouping_intent,
            continuity_assumption,
            shared_posture_claims,
            shared_rationale,
            aspect_record,
            aspect_participation,
            members,
            group_digest,
        }
    }

    pub fn handle_identity_digest(&self) -> &str {
        &self.handle_identity_digest
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.declaration_family_key
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        &self.operating_context_identity_digest
    }

    pub fn grouped_posture(&self) -> ForgeQueryGroupedDeclarationPosture {
        self.grouped_posture
    }

    pub fn semantics(&self) -> ForgeQueryGroupedSemantics {
        self.semantics
    }

    pub fn ordering(&self) -> ForgeQueryGroupedOrdering {
        self.ordering
    }

    pub fn atomicity(&self) -> ForgeQueryGroupedAtomicity {
        self.atomicity
    }

    pub fn grouping_intent(&self) -> ForgeQueryGroupedIntent {
        self.grouping_intent
    }

    pub fn continuity_assumption(&self) -> ForgeQueryGroupedContinuityAssumption {
        self.continuity_assumption
    }

    pub fn shared_posture_claims(&self) -> &[ForgeQueryGroupedSharedPostureClaim] {
        &self.shared_posture_claims
    }

    pub fn shared_rationale(&self) -> Option<&str> {
        self.shared_rationale.as_deref()
    }

    pub fn aspect_record(&self) -> &ForgeQueryGroupedDeclarationAspectRecord {
        &self.aspect_record
    }

    pub fn aspect_participation(&self) -> &ForgeQueryGroupedAspectParticipationSummary {
        &self.aspect_participation
    }

    pub fn members(&self) -> &[ForgeQueryGroupedDeclarationMember<D, I>] {
        &self.members
    }

    pub fn group_digest(&self) -> &str {
        &self.group_digest
    }
}
