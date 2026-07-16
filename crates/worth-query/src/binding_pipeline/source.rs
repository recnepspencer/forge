use crate::application::{
    WorthQueryAdmittedDeclarationProgression, WorthQueryDeclarationEnvelope,
    WorthQueryDeclarationInput, WorthQueryDeclarationReceipt, WorthQueryDeclarationRoutePlan,
    WorthQueryDomainEntryMarker,
};

use super::specificity::WorthQueryBindingSpecificity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryBindingSourceKind {
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

impl WorthQueryBindingSourceKind {
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

#[cfg(test)]
pub struct WorthQueryDeclarationContextCandidate<I> {
    label: String,
    source_kind: WorthQueryBindingSourceKind,
    specificity: WorthQueryBindingSpecificity,
    input: I,
}

#[cfg(test)]
impl<I> WorthQueryDeclarationContextCandidate<I> {
    pub fn new(
        label: impl Into<String>,
        source_kind: WorthQueryBindingSourceKind,
        specificity: WorthQueryBindingSpecificity,
        input: I,
    ) -> Self {
        Self {
            label: label.into(),
            source_kind,
            specificity,
            input,
        }
    }

    #[cfg(test)]
    pub(crate) fn into_parts(
        self,
    ) -> (
        String,
        WorthQueryBindingSourceKind,
        WorthQueryBindingSpecificity,
        I,
    ) {
        (self.label, self.source_kind, self.specificity, self.input)
    }
}

macro_rules! context_candidate {
    ($name:ident, $ty:ty, $accessor_cfg:meta) => {
        pub struct $name<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>> {
            label: String,
            source_kind: WorthQueryBindingSourceKind,
            specificity: WorthQueryBindingSpecificity,
            subject: $ty,
        }

        impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>> $name<D, I> {
            pub fn new(
                label: impl Into<String>,
                source_kind: WorthQueryBindingSourceKind,
                specificity: WorthQueryBindingSpecificity,
                subject: $ty,
            ) -> Self {
                Self {
                    label: label.into(),
                    source_kind,
                    specificity,
                    subject,
                }
            }

            #[$accessor_cfg]
            pub fn label(&self) -> &str {
                &self.label
            }

            #[$accessor_cfg]
            pub fn source_kind(&self) -> WorthQueryBindingSourceKind {
                self.source_kind
            }

            #[$accessor_cfg]
            pub fn specificity(&self) -> WorthQueryBindingSpecificity {
                self.specificity
            }

            pub(crate) fn into_parts(
                self,
            ) -> (
                String,
                WorthQueryBindingSourceKind,
                WorthQueryBindingSpecificity,
                $ty,
            ) {
                (self.label, self.source_kind, self.specificity, self.subject)
            }
        }
    };
}

#[cfg(test)]
context_candidate!(
    WorthQueryProgressionContextCandidate,
    WorthQueryAdmittedDeclarationProgression<D, I>,
    cfg(any())
);
context_candidate!(
    WorthQueryEnvelopeContextCandidate,
    WorthQueryDeclarationEnvelope<D, I>,
    cfg(all())
);

#[cfg(test)]
pub enum WorthQueryRouteResolverSubject<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    Progression(WorthQueryAdmittedDeclarationProgression<D, I>),
}

pub enum WorthQueryReceiptResolverSubject<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    Progression(WorthQueryAdmittedDeclarationProgression<D, I>),
    RoutePlan(WorthQueryDeclarationRoutePlan<D, I>),
}

pub enum WorthQueryEnvelopeResolverSubject<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    Progression(WorthQueryAdmittedDeclarationProgression<D, I>),
    Receipt(WorthQueryDeclarationReceipt<D, I>),
}
