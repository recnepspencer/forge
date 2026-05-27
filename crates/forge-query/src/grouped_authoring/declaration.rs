use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationAdmissionError,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext,
};

use super::artifact::{ForgeQueryGroupedDeclarationArtifact, ForgeQueryGroupedDeclarationMember};
use super::input::ForgeQueryGroupedDeclarationInput;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryGroupedDeclarationStopKind {
    Deferred,
    Unsupported,
    InvalidContext,
    Canonicalization,
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
    let (semantics, ordering, shared_rationale, member_inputs) = input.into_parts();
    let mut members = Vec::with_capacity(member_inputs.len());
    for (member_index, member_input) in member_inputs.into_iter().enumerate() {
        match handle.declare(member_input.clone()) {
            Ok(declaration) => members.push(ForgeQueryGroupedDeclarationMember::new(
                member_index,
                member_input,
                declaration,
            )),
            Err(error) => {
                return ForgeQueryGroupedDeclarationChecked::MemberStopped(
                    grouped_declaration_stop::<D, I>(member_index, &error),
                );
            }
        }
    }
    ForgeQueryGroupedDeclarationChecked::Bound(ForgeQueryGroupedDeclarationArtifact::new(
        handle.operating_context_identity_digest().to_string(),
        semantics,
        ordering,
        shared_rationale,
        members,
    ))
}

fn grouped_declaration_stop<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
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
        ForgeQueryDeclarationAdmissionError::Unsupported(denial) => ForgeQueryGroupedDeclarationStop::new(
            member_index,
            I::Family::semantic_family_key(),
            ForgeQueryGroupedDeclarationStopKind::Unsupported,
            format!(
                "member {member_index} declaration unsupported with capability status {}",
                denial.capability_status().as_str()
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
