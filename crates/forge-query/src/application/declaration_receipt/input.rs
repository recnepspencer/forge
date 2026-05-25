use crate::application::{
    ForgeQueryDeclarationInput, ForgeQueryDeclarationRoutePlan,
    ForgeQueryDeclarationRoutePlanChecked, ForgeQueryDeclarationRoutePlanDeferred,
    ForgeQueryDeclarationRoutePlanDenied, ForgeQueryDeclarationRoutePlanFailed,
    ForgeQueryDomainEntryMarker,
};

pub enum ForgeQueryDeclarationReceiptInput<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    PlannedRoute(ForgeQueryDeclarationRoutePlan<D, I>),
    DeferredRoute(ForgeQueryDeclarationRoutePlanDeferred<D, I>),
    DeniedRoute(ForgeQueryDeclarationRoutePlanDenied<D, I>),
    FailedRoute(ForgeQueryDeclarationRoutePlanFailed<D, I>),
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationReceiptInput<D, I>
{
    pub fn planned(plan: ForgeQueryDeclarationRoutePlan<D, I>) -> Self {
        Self::PlannedRoute(plan)
    }

    pub fn deferred(plan: ForgeQueryDeclarationRoutePlanDeferred<D, I>) -> Self {
        Self::DeferredRoute(plan)
    }

    pub fn denied(plan: ForgeQueryDeclarationRoutePlanDenied<D, I>) -> Self {
        Self::DeniedRoute(plan)
    }

    pub fn failed(plan: ForgeQueryDeclarationRoutePlanFailed<D, I>) -> Self {
        Self::FailedRoute(plan)
    }

    pub fn route_checked(checked: ForgeQueryDeclarationRoutePlanChecked<D, I>) -> Self {
        match checked {
            ForgeQueryDeclarationRoutePlanChecked::Planned(plan) => Self::PlannedRoute(plan),
            ForgeQueryDeclarationRoutePlanChecked::Deferred(plan) => Self::DeferredRoute(plan),
            ForgeQueryDeclarationRoutePlanChecked::Denied(plan) => Self::DeniedRoute(plan),
            ForgeQueryDeclarationRoutePlanChecked::Failed(plan) => Self::FailedRoute(plan),
        }
    }
}
