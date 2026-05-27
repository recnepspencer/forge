use crate::application::{
    ForgeQueryCanonicalDeclarationArtifact, ForgeQueryDeclarationInput,
    ForgeQueryDomainEntryMarker, ForgeQueryGroupedDeclarationPosture,
};
use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryGroupedSemantics {
    LocalNeighborhood,
}

impl ForgeQueryGroupedSemantics {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LocalNeighborhood => "local_neighborhood",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryGroupedOrdering {
    Declared,
}

impl ForgeQueryGroupedOrdering {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Declared => "declared",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGroupedDeclarationMember<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    member_index: usize,
    member_input: I,
    declaration: ForgeQueryCanonicalDeclarationArtifact<D, I>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryGroupedDeclarationMember<D, I>
{
    pub(crate) fn new(
        member_index: usize,
        member_input: I,
        declaration: ForgeQueryCanonicalDeclarationArtifact<D, I>,
    ) -> Self {
        Self {
            member_index,
            member_input,
            declaration,
        }
    }

    pub fn member_index(&self) -> usize {
        self.member_index
    }

    pub fn declaration(&self) -> &ForgeQueryCanonicalDeclarationArtifact<D, I> {
        &self.declaration
    }

    pub fn input(&self) -> &I {
        &self.member_input
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
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
    shared_rationale: Option<String>,
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
        shared_rationale: Option<String>,
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
            format!(
                "shared_rationale:{}",
                shared_rationale.as_deref().unwrap_or("none")
            ),
            format!(
                "members:{}",
                members
                    .iter()
                    .map(|member| format!(
                        "{}:{:?}",
                        member.member_index(),
                        member.declaration().declaration_digest()
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
            shared_rationale,
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

    pub fn shared_rationale(&self) -> Option<&str> {
        self.shared_rationale.as_deref()
    }

    pub fn members(&self) -> &[ForgeQueryGroupedDeclarationMember<D, I>] {
        &self.members
    }

    pub fn group_digest(&self) -> &str {
        &self.group_digest
    }
}
