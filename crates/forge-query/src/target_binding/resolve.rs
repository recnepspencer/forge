use crate::application::{
    ForgeQueryAdmittedDeclarationProgression, ForgeQueryDeclarationInput,
    ForgeQueryDomainEntryMarker,
};

use super::{
    ForgeQueryAdmittedDeclarationProgressionBindingTarget,
    ForgeQueryAdmittedDeclarationProgressionBindingTargetSource,
};

pub(crate) struct ResolvedAdmittedDeclarationProgressionTarget<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    target: ForgeQueryAdmittedDeclarationProgressionBindingTarget,
    source: ForgeQueryAdmittedDeclarationProgressionBindingTargetSource<D, I>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ResolvedAdmittedDeclarationProgressionTarget<D, I>
{
    pub(crate) fn target(&self) -> &ForgeQueryAdmittedDeclarationProgressionBindingTarget {
        &self.target
    }

    pub(crate) fn into_progressed(self) -> ForgeQueryAdmittedDeclarationProgression<D, I> {
        self.source.progressed
    }
}

pub(crate) fn resolve_admitted_progression_target<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
) -> ResolvedAdmittedDeclarationProgressionTarget<D, I> {
    let source = ForgeQueryAdmittedDeclarationProgressionBindingTargetSource::new(progressed);
    let target = ForgeQueryAdmittedDeclarationProgressionBindingTarget::from_source(&source);
    ResolvedAdmittedDeclarationProgressionTarget { target, source }
}
