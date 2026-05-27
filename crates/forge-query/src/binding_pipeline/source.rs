use crate::application::{
    ForgeQueryAdmittedDeclarationProgression, ForgeQueryDeclarationEnvelope,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationReceipt, ForgeQueryDeclarationRoutePlan,
    ForgeQueryDomainEntryMarker,
};

use super::specificity::ForgeQueryBindingSpecificity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryBindingSourceKind {
    ExplicitSelection,
    ActiveToolSelection,
    HoveredSemanticTarget,
    CurrentDeclaration,
    CurrentProgression,
    CurrentRoutePlan,
    CurrentReceipt,
    CurrentEnvelope,
    CurrentContinuationCandidate,
    LastRetainedTarget,
}

impl ForgeQueryBindingSourceKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExplicitSelection => "explicit_selection",
            Self::ActiveToolSelection => "active_tool_selection",
            Self::HoveredSemanticTarget => "hovered_semantic_target",
            Self::CurrentDeclaration => "current_declaration",
            Self::CurrentProgression => "current_progression",
            Self::CurrentRoutePlan => "current_route_plan",
            Self::CurrentReceipt => "current_receipt",
            Self::CurrentEnvelope => "current_envelope",
            Self::CurrentContinuationCandidate => "current_continuation_candidate",
            Self::LastRetainedTarget => "last_retained_target",
        }
    }
}

pub struct ForgeQueryDeclarationContextCandidate<I> {
    label: String,
    source_kind: ForgeQueryBindingSourceKind,
    specificity: ForgeQueryBindingSpecificity,
    input: I,
}

impl<I> ForgeQueryDeclarationContextCandidate<I> {
    pub fn new(
        label: impl Into<String>,
        source_kind: ForgeQueryBindingSourceKind,
        specificity: ForgeQueryBindingSpecificity,
        input: I,
    ) -> Self {
        Self {
            label: label.into(),
            source_kind,
            specificity,
            input,
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn source_kind(&self) -> ForgeQueryBindingSourceKind {
        self.source_kind
    }

    pub fn specificity(&self) -> ForgeQueryBindingSpecificity {
        self.specificity
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        String,
        ForgeQueryBindingSourceKind,
        ForgeQueryBindingSpecificity,
        I,
    ) {
        (self.label, self.source_kind, self.specificity, self.input)
    }
}

macro_rules! context_candidate {
    ($name:ident, $ty:ty) => {
        pub struct $name<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>> {
            label: String,
            source_kind: ForgeQueryBindingSourceKind,
            specificity: ForgeQueryBindingSpecificity,
            subject: $ty,
        }

        impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>> $name<D, I> {
            pub fn new(
                label: impl Into<String>,
                source_kind: ForgeQueryBindingSourceKind,
                specificity: ForgeQueryBindingSpecificity,
                subject: $ty,
            ) -> Self {
                Self {
                    label: label.into(),
                    source_kind,
                    specificity,
                    subject,
                }
            }

            pub fn label(&self) -> &str {
                &self.label
            }

            pub fn source_kind(&self) -> ForgeQueryBindingSourceKind {
                self.source_kind
            }

            pub fn specificity(&self) -> ForgeQueryBindingSpecificity {
                self.specificity
            }

            pub(crate) fn into_parts(
                self,
            ) -> (
                String,
                ForgeQueryBindingSourceKind,
                ForgeQueryBindingSpecificity,
                $ty,
            ) {
                (self.label, self.source_kind, self.specificity, self.subject)
            }
        }
    };
}

context_candidate!(
    ForgeQueryProgressionContextCandidate,
    ForgeQueryAdmittedDeclarationProgression<D, I>
);
context_candidate!(
    ForgeQueryEnvelopeContextCandidate,
    ForgeQueryDeclarationEnvelope<D, I>
);

pub enum ForgeQueryRouteResolverSubject<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Progression(ForgeQueryAdmittedDeclarationProgression<D, I>),
}

pub enum ForgeQueryReceiptResolverSubject<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Progression(ForgeQueryAdmittedDeclarationProgression<D, I>),
    RoutePlan(ForgeQueryDeclarationRoutePlan<D, I>),
}

pub enum ForgeQueryEnvelopeResolverSubject<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Progression(ForgeQueryAdmittedDeclarationProgression<D, I>),
    Receipt(ForgeQueryDeclarationReceipt<D, I>),
}
