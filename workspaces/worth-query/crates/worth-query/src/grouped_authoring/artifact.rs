use crate::application::{
    WorthQueryAdmittedDeclarationProgression, WorthQueryCanonicalDeclarationArtifact,
    WorthQueryDeclarationAspectContract, WorthQueryDeclarationAspectCoverage,
    WorthQueryDeclarationAspectCoverageBasis, WorthQueryDeclarationInput,
    WorthQueryDomainEntryMarker, WorthQueryGroupedDeclarationPosture,
};
use crate::authoring::AspectFieldKey;
use crate::identity::hash_parts;

use super::posture::{
    WorthQueryGroupedAtomicity, WorthQueryGroupedContinuityAssumption, WorthQueryGroupedIntent,
    WorthQueryGroupedMemberRole, WorthQueryGroupedOrdering, WorthQueryGroupedSemantics,
    WorthQueryGroupedSharedPostureClaim,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGroupedDeclarationAspectRecord {
    contract: WorthQueryDeclarationAspectContract,
    coverage: WorthQueryDeclarationAspectCoverage,
    coverage_basis: WorthQueryDeclarationAspectCoverageBasis,
}

impl WorthQueryGroupedDeclarationAspectRecord {
    pub(crate) fn new(
        contract: WorthQueryDeclarationAspectContract,
        coverage: WorthQueryDeclarationAspectCoverage,
        coverage_basis: WorthQueryDeclarationAspectCoverageBasis,
    ) -> Self {
        Self {
            contract,
            coverage,
            coverage_basis,
        }
    }

    pub fn contract(&self) -> &WorthQueryDeclarationAspectContract {
        &self.contract
    }

    pub fn coverage(&self) -> &WorthQueryDeclarationAspectCoverage {
        &self.coverage
    }

    pub fn coverage_basis(&self) -> WorthQueryDeclarationAspectCoverageBasis {
        self.coverage_basis
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGroupedAspectParticipationSummary {
    present_any: Vec<AspectFieldKey>,
    present_all: Vec<AspectFieldKey>,
    masked_any: Vec<AspectFieldKey>,
    conflicting_any: Vec<AspectFieldKey>,
}

impl WorthQueryGroupedAspectParticipationSummary {
    pub(crate) fn new(
        present_any: Vec<AspectFieldKey>,
        present_all: Vec<AspectFieldKey>,
        masked_any: Vec<AspectFieldKey>,
        conflicting_any: Vec<AspectFieldKey>,
    ) -> Self {
        Self {
            present_any,
            present_all,
            masked_any,
            conflicting_any,
        }
    }

    pub fn present_any(&self) -> &[AspectFieldKey] {
        &self.present_any
    }

    pub fn present_all(&self) -> &[AspectFieldKey] {
        &self.present_all
    }

    pub fn masked_any(&self) -> &[AspectFieldKey] {
        &self.masked_any
    }

    pub fn conflicting_any(&self) -> &[AspectFieldKey] {
        &self.conflicting_any
    }

    pub(crate) fn terminal_present_all_projections_for_boundary(&self) -> Vec<String> {
        self.present_all
            .iter()
            .map(grouped_terminal_declaration_aspect_projection_for_digest)
            .collect()
    }

    pub(crate) fn terminal_conflicting_any_projections_for_boundary(&self) -> Vec<String> {
        self.conflicting_any
            .iter()
            .map(grouped_terminal_declaration_aspect_projection_for_digest)
            .collect()
    }
}

#[derive(Clone)]
pub struct WorthQueryGroupedDeclarationMember<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    member_index: usize,
    role: WorthQueryGroupedMemberRole,
    member_input: I,
    progression: WorthQueryAdmittedDeclarationProgression<D, I>,
    aspect_record: WorthQueryGroupedDeclarationAspectRecord,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryGroupedDeclarationMember<D, I>
{
    pub(crate) fn new(
        member_index: usize,
        role: WorthQueryGroupedMemberRole,
        member_input: I,
        progression: WorthQueryAdmittedDeclarationProgression<D, I>,
        aspect_record: WorthQueryGroupedDeclarationAspectRecord,
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

    pub fn role(&self) -> WorthQueryGroupedMemberRole {
        self.role
    }

    pub fn declaration(&self) -> &WorthQueryCanonicalDeclarationArtifact<D, I> {
        self.progression.canonical_declaration()
    }

    pub fn input(&self) -> &I {
        &self.member_input
    }

    pub fn aspect_record(&self) -> &WorthQueryGroupedDeclarationAspectRecord {
        &self.aspect_record
    }

    pub(crate) fn progression(&self) -> &WorthQueryAdmittedDeclarationProgression<D, I> {
        &self.progression
    }
}

#[derive(Clone)]
pub struct WorthQueryGroupedDeclarationArtifact<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    handle_identity_digest: String,
    operating_context_identity_digest: String,
    declaration_family_key: &'static str,
    grouped_posture: WorthQueryGroupedDeclarationPosture,
    semantics: WorthQueryGroupedSemantics,
    ordering: WorthQueryGroupedOrdering,
    atomicity: WorthQueryGroupedAtomicity,
    grouping_intent: WorthQueryGroupedIntent,
    continuity_assumption: WorthQueryGroupedContinuityAssumption,
    shared_posture_claims: Vec<WorthQueryGroupedSharedPostureClaim>,
    shared_rationale: Option<String>,
    aspect_record: WorthQueryGroupedDeclarationAspectRecord,
    aspect_participation: WorthQueryGroupedAspectParticipationSummary,
    members: Vec<WorthQueryGroupedDeclarationMember<D, I>>,
    group_digest: String,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryGroupedDeclarationArtifact<D, I>
{
    pub(crate) fn new(
        operating_context_identity_digest: String,
        semantics: WorthQueryGroupedSemantics,
        ordering: WorthQueryGroupedOrdering,
        atomicity: WorthQueryGroupedAtomicity,
        grouping_intent: WorthQueryGroupedIntent,
        continuity_assumption: WorthQueryGroupedContinuityAssumption,
        shared_posture_claims: Vec<WorthQueryGroupedSharedPostureClaim>,
        shared_rationale: Option<String>,
        aspect_record: WorthQueryGroupedDeclarationAspectRecord,
        aspect_participation: WorthQueryGroupedAspectParticipationSummary,
        members: Vec<WorthQueryGroupedDeclarationMember<D, I>>,
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
                aspect_participation
                    .terminal_present_all_projections_for_boundary()
                    .join("|")
            ),
            format!(
                "conflicting_any:{}",
                aspect_participation
                    .terminal_conflicting_any_projections_for_boundary()
                    .join("|")
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

    pub fn grouped_posture(&self) -> WorthQueryGroupedDeclarationPosture {
        self.grouped_posture
    }

    pub fn semantics(&self) -> WorthQueryGroupedSemantics {
        self.semantics
    }

    pub fn ordering(&self) -> WorthQueryGroupedOrdering {
        self.ordering
    }

    pub fn atomicity(&self) -> WorthQueryGroupedAtomicity {
        self.atomicity
    }

    pub fn grouping_intent(&self) -> WorthQueryGroupedIntent {
        self.grouping_intent
    }

    pub fn continuity_assumption(&self) -> WorthQueryGroupedContinuityAssumption {
        self.continuity_assumption
    }

    pub fn shared_posture_claims(&self) -> &[WorthQueryGroupedSharedPostureClaim] {
        &self.shared_posture_claims
    }

    pub fn shared_rationale(&self) -> Option<&str> {
        self.shared_rationale.as_deref()
    }

    pub fn aspect_record(&self) -> &WorthQueryGroupedDeclarationAspectRecord {
        &self.aspect_record
    }

    pub fn aspect_participation(&self) -> &WorthQueryGroupedAspectParticipationSummary {
        &self.aspect_participation
    }

    pub fn members(&self) -> &[WorthQueryGroupedDeclarationMember<D, I>] {
        &self.members
    }

    pub fn group_digest(&self) -> &str {
        &self.group_digest
    }
}

fn grouped_terminal_declaration_aspect_projection_for_digest(key: &AspectFieldKey) -> String {
    format!("{}.{}", key.aspect().as_str(), key.field().as_str())
}
