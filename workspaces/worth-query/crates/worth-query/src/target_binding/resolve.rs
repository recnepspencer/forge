use crate::application::{
    WorthQueryAdmittedDeclarationProgression, WorthQueryDeclarationInput,
    WorthQueryDomainEntryMarker,
};

use super::{
    WorthQueryAdmittedDeclarationProgressionBindingTarget,
    WorthQueryAdmittedDeclarationProgressionBindingTargetSource,
};

pub(crate) struct ResolvedAdmittedDeclarationProgressionTarget<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    target: WorthQueryAdmittedDeclarationProgressionBindingTarget,
    source: WorthQueryAdmittedDeclarationProgressionBindingTargetSource<D, I>,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    ResolvedAdmittedDeclarationProgressionTarget<D, I>
{
    pub(crate) fn target(&self) -> &WorthQueryAdmittedDeclarationProgressionBindingTarget {
        &self.target
    }

    pub(crate) fn into_progressed(self) -> WorthQueryAdmittedDeclarationProgression<D, I> {
        self.source.progressed
    }
}

pub(crate) fn resolve_admitted_progression_target<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
) -> ResolvedAdmittedDeclarationProgressionTarget<D, I> {
    let source = WorthQueryAdmittedDeclarationProgressionBindingTargetSource::new(progressed);
    let target = WorthQueryAdmittedDeclarationProgressionBindingTarget::from_source(&source);
    ResolvedAdmittedDeclarationProgressionTarget { target, source }
}
