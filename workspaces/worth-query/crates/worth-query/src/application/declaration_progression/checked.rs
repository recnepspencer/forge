use crate::application::{WorthQueryDeclarationInput, WorthQueryDomainEntryMarker};

use super::admitted::WorthQueryAdmittedDeclarationProgression;
use super::denial::WorthQueryDeclarationProgressionFailed;
use super::denial::{
    WorthQueryDeclarationProgressionDeferred, WorthQueryDeclarationProgressionDenied,
};
use super::rebind::WorthQueryDeclarationProgressionRebindRequired;
use super::stale::WorthQueryDeclarationProgressionStale;

pub enum WorthQueryDeclarationProgressionChecked<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    Admitted(WorthQueryAdmittedDeclarationProgression<D, I>),
    Deferred(WorthQueryDeclarationProgressionDeferred<D, I>),
    Denied(WorthQueryDeclarationProgressionDenied<D, I>),
    Stale(WorthQueryDeclarationProgressionStale<D, I>),
    RebindRequired(WorthQueryDeclarationProgressionRebindRequired<D, I>),
    Failed(WorthQueryDeclarationProgressionFailed<D, I>),
}

pub enum WorthQueryDeclarationProgressionTerminalError<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    Deferred(WorthQueryDeclarationProgressionDeferred<D, I>),
    Denied(WorthQueryDeclarationProgressionDenied<D, I>),
    Stale(WorthQueryDeclarationProgressionStale<D, I>),
    RebindRequired(WorthQueryDeclarationProgressionRebindRequired<D, I>),
    Failed(WorthQueryDeclarationProgressionFailed<D, I>),
}
