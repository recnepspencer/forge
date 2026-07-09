use crate::application::{
    WorthQueryDeclarationInput, WorthQueryDeclarationRoutePlan,
    WorthQueryDeclarationRoutePlanChecked, WorthQueryDeclarationRoutePlanDeferred,
    WorthQueryDeclarationRoutePlanDenied, WorthQueryDeclarationRoutePlanFailed,
    WorthQueryDomainEntryMarker,
};

pub enum WorthQueryDeclarationReceiptInput<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    PlannedRoute(WorthQueryDeclarationRoutePlan<D, I>),
    DeferredRoute(WorthQueryDeclarationRoutePlanDeferred<D, I>),
    DeniedRoute(WorthQueryDeclarationRoutePlanDenied<D, I>),
    FailedRoute(WorthQueryDeclarationRoutePlanFailed<D, I>),
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryDeclarationReceiptInput<D, I>
{
    pub fn planned(plan: WorthQueryDeclarationRoutePlan<D, I>) -> Self {
        Self::PlannedRoute(plan)
    }

    pub fn deferred(plan: WorthQueryDeclarationRoutePlanDeferred<D, I>) -> Self {
        Self::DeferredRoute(plan)
    }

    pub fn denied(plan: WorthQueryDeclarationRoutePlanDenied<D, I>) -> Self {
        Self::DeniedRoute(plan)
    }

    pub fn failed(plan: WorthQueryDeclarationRoutePlanFailed<D, I>) -> Self {
        Self::FailedRoute(plan)
    }

    pub fn route_checked(checked: WorthQueryDeclarationRoutePlanChecked<D, I>) -> Self {
        match checked {
            WorthQueryDeclarationRoutePlanChecked::Planned(plan) => Self::PlannedRoute(plan),
            WorthQueryDeclarationRoutePlanChecked::Deferred(plan) => Self::DeferredRoute(plan),
            WorthQueryDeclarationRoutePlanChecked::Denied(plan) => Self::DeniedRoute(plan),
            WorthQueryDeclarationRoutePlanChecked::Failed(plan) => Self::FailedRoute(plan),
        }
    }
}
