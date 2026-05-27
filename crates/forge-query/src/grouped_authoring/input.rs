use std::marker::PhantomData;

use crate::application::{
    ForgeQueryDeclarationInput, ForgeQueryDeclarationSupportsNeighborhoodGrouping,
    ForgeQueryDomainEntryMarker,
};

use super::artifact::{ForgeQueryGroupedOrdering, ForgeQueryGroupedSemantics};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGroupedDeclarationInput<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    semantics: ForgeQueryGroupedSemantics,
    ordering: ForgeQueryGroupedOrdering,
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
        Option<String>,
        Vec<I>,
    ) {
        (
            self.semantics,
            self.ordering,
            self.shared_rationale,
            self.member_inputs,
        )
    }
}
