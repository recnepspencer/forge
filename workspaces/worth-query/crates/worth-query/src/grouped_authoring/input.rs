use std::marker::PhantomData;

use crate::application::{
    WorthQueryDeclarationInput, WorthQueryDeclarationSupportsNeighborhoodGrouping,
    WorthQueryDomainEntryMarker,
};

use super::posture::{
    WorthQueryGroupedAtomicity, WorthQueryGroupedContinuityAssumption, WorthQueryGroupedIntent,
    WorthQueryGroupedOrdering, WorthQueryGroupedSemantics, WorthQueryGroupedSharedPostureClaim,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGroupedDeclarationInput<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    semantics: WorthQueryGroupedSemantics,
    ordering: WorthQueryGroupedOrdering,
    atomicity: WorthQueryGroupedAtomicity,
    grouping_intent: WorthQueryGroupedIntent,
    continuity_assumption: WorthQueryGroupedContinuityAssumption,
    shared_posture_claims: Vec<WorthQueryGroupedSharedPostureClaim>,
    shared_rationale: Option<String>,
    member_inputs: Vec<I>,
    _marker: PhantomData<D>,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryGroupedDeclarationInput<D, I>
where
    I::Family: WorthQueryDeclarationSupportsNeighborhoodGrouping<D>,
{
    pub fn local_neighborhood(seed_member: I) -> Self {
        Self {
            semantics: WorthQueryGroupedSemantics::LocalNeighborhood,
            ordering: WorthQueryGroupedOrdering::Declared,
            atomicity: WorthQueryGroupedAtomicity::MemberIndependent,
            grouping_intent: WorthQueryGroupedIntent::Exploratory,
            continuity_assumption: WorthQueryGroupedContinuityAssumption::None,
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

    pub fn with_ordering(mut self, ordering: WorthQueryGroupedOrdering) -> Self {
        self.ordering = ordering;
        self
    }

    pub fn with_atomicity(mut self, atomicity: WorthQueryGroupedAtomicity) -> Self {
        self.atomicity = atomicity;
        self
    }

    pub fn with_grouping_intent(mut self, grouping_intent: WorthQueryGroupedIntent) -> Self {
        self.grouping_intent = grouping_intent;
        self
    }

    pub fn with_continuity_assumption(
        mut self,
        continuity_assumption: WorthQueryGroupedContinuityAssumption,
    ) -> Self {
        self.continuity_assumption = continuity_assumption;
        self
    }

    pub fn with_shared_posture_claim(mut self, claim: WorthQueryGroupedSharedPostureClaim) -> Self {
        if !self.shared_posture_claims.contains(&claim) {
            self.shared_posture_claims.push(claim);
            self.shared_posture_claims.sort();
        }
        self
    }
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryGroupedDeclarationInput<D, I>
{
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

    pub fn member_inputs(&self) -> &[I] {
        &self.member_inputs
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        WorthQueryGroupedSemantics,
        WorthQueryGroupedOrdering,
        WorthQueryGroupedAtomicity,
        WorthQueryGroupedIntent,
        WorthQueryGroupedContinuityAssumption,
        Vec<WorthQueryGroupedSharedPostureClaim>,
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
