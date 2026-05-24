use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};

use super::admitted::ForgeQueryAdmittedDeclarationProgression;
use super::denial::{
    ForgeQueryDeclarationProgressionDeferred, ForgeQueryDeclarationProgressionDenied,
};
use super::denial::ForgeQueryDeclarationProgressionFailed;
use super::rebind::ForgeQueryDeclarationProgressionRebindRequired;
use super::stale::ForgeQueryDeclarationProgressionStale;

pub enum ForgeQueryDeclarationProgressionChecked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Admitted(ForgeQueryAdmittedDeclarationProgression<D, I>),
    Deferred(ForgeQueryDeclarationProgressionDeferred<D, I>),
    Denied(ForgeQueryDeclarationProgressionDenied<D, I>),
    Stale(ForgeQueryDeclarationProgressionStale<D, I>),
    RebindRequired(ForgeQueryDeclarationProgressionRebindRequired<D, I>),
    Failed(ForgeQueryDeclarationProgressionFailed<D, I>),
}

pub enum ForgeQueryDeclarationProgressionTerminalError<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Deferred(ForgeQueryDeclarationProgressionDeferred<D, I>),
    Denied(ForgeQueryDeclarationProgressionDenied<D, I>),
    Stale(ForgeQueryDeclarationProgressionStale<D, I>),
    RebindRequired(ForgeQueryDeclarationProgressionRebindRequired<D, I>),
    Failed(ForgeQueryDeclarationProgressionFailed<D, I>),
}
