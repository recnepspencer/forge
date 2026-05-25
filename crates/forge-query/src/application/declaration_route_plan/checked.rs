use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};

use super::{
    denial::{
        ForgeQueryDeclarationRoutePlanDeferred, ForgeQueryDeclarationRoutePlanDenied,
        ForgeQueryDeclarationRoutePlanFailed,
    },
    plan::ForgeQueryDeclarationRoutePlan,
};

pub enum ForgeQueryDeclarationRoutePlanChecked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Planned(ForgeQueryDeclarationRoutePlan<D, I>),
    Deferred(ForgeQueryDeclarationRoutePlanDeferred<D, I>),
    Denied(ForgeQueryDeclarationRoutePlanDenied<D, I>),
    Failed(ForgeQueryDeclarationRoutePlanFailed<D, I>),
}
