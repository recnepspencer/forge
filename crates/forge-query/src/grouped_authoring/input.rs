use std::marker::PhantomData;

use crate::application::{
    ForgeQueryDeclarationInput, ForgeQueryDeclarationSupportsNeighborhoodGrouping,
    ForgeQueryDomainEntryMarker,
};

use super::posture::{
    ForgeQueryGroupedAtomicity, ForgeQueryGroupedContinuityAssumption, ForgeQueryGroupedIntent,
    ForgeQueryGroupedOrdering, ForgeQueryGroupedSemantics, ForgeQueryGroupedSharedPostureClaim,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGroupedDeclarationInput<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    semantics: ForgeQueryGroupedSemantics,
    ordering: ForgeQueryGroupedOrdering,
    atomicity: ForgeQueryGroupedAtomicity,
    grouping_intent: ForgeQueryGroupedIntent,
    continuity_assumption: ForgeQueryGroupedContinuityAssumption,
    shared_posture_claims: Vec<ForgeQueryGroupedSharedPostureClaim>,
    shared_rationale: Option<String>,
    member_inputs: Vec<I>,
    _marker: PhantomData<D>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryGroupedDeclarationInput<D, I>
where
    I::Family: ForgeQueryDeclarationSupportsNeighborhoodGrouping<D>,
{
    pub fn local_neighborhood(seed_member: I) -> Self {
        Self {
            semantics: ForgeQueryGroupedSemantics::LocalNeighborhood,
            ordering: ForgeQueryGroupedOrdering::Declared,
            atomicity: ForgeQueryGroupedAtomicity::MemberIndependent,
            grouping_intent: ForgeQueryGroupedIntent::Exploratory,
            continuity_assumption: ForgeQueryGroupedContinuityAssumption::None,
            shared_posture_claims: Vec::new(),
            shared_rationale: None,
            member_inputs: vec![seed_member],
            _marker: PhantomData,
        }
    }

    pub fn with_member(mut self, member_input: I) -> Self {
        self.member_inputs.push(member_input);
        self
    }

    pub fn with_members(mut self, member_inputs: impl IntoIterator<Item = I>) -> Self {
        self.member_inputs.extend(member_inputs);
        self
    }

    pub fn with_shared_rationale(mut self, rationale: impl Into<String>) -> Self {
        self.shared_rationale = Some(rationale.into());
        self
    }

    pub fn with_ordering(mut self, ordering: ForgeQueryGroupedOrdering) -> Self {
        self.ordering = ordering;
        self
    }

    pub fn with_atomicity(mut self, atomicity: ForgeQueryGroupedAtomicity) -> Self {
        self.atomicity = atomicity;
        self
    }

    pub fn with_grouping_intent(mut self, grouping_intent: ForgeQueryGroupedIntent) -> Self {
        self.grouping_intent = grouping_intent;
        self
    }

    pub fn with_continuity_assumption(
        mut self,
        continuity_assumption: ForgeQueryGroupedContinuityAssumption,
    ) -> Self {
        self.continuity_assumption = continuity_assumption;
        self
    }

    pub fn with_shared_posture_claim(mut self, claim: ForgeQueryGroupedSharedPostureClaim) -> Self {
        if !self.shared_posture_claims.contains(&claim) {
            self.shared_posture_claims.push(claim);
            self.shared_posture_claims.sort();
        }
        self
    }
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryGroupedDeclarationInput<D, I>
{
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

    pub fn member_inputs(&self) -> &[I] {
        &self.member_inputs
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        ForgeQueryGroupedSemantics,
        ForgeQueryGroupedOrdering,
        ForgeQueryGroupedAtomicity,
        ForgeQueryGroupedIntent,
        ForgeQueryGroupedContinuityAssumption,
        Vec<ForgeQueryGroupedSharedPostureClaim>,
        Option<String>,
        Vec<I>,
    ) {
        (
            self.semantics,
            self.ordering,
            self.atomicity,
            self.grouping_intent,
            self.continuity_assumption,
            self.shared_posture_claims,
            self.shared_rationale,
            self.member_inputs,
        )
    }
}
