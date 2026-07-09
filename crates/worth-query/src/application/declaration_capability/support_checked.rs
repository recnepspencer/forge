use super::support::{
    derive_family_support_report, WorthQueryDeclarationCapabilityStatus,
    WorthQueryDeclarationFamilySupportReport,
};
use crate::application::{
    WorthQueryAdmittedConfiguredDomainHandle, WorthQueryDeclarationFamilyMarker,
    WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationFamilySupportChecked<
    D: WorthQueryDomainEntryMarker,
    F: WorthQueryDeclarationFamilyMarker<D>,
> {
    Admitted(WorthQueryDeclarationFamilySupportReport<D, F>),
    Deferred(WorthQueryDeclarationFamilySupportReport<D, F>),
    Unsupported(WorthQueryDeclarationFamilySupportReport<D, F>),
    InvalidContext(WorthQueryDeclarationFamilySupportReport<D, F>),
}

pub(crate) fn worth_query_checked_family_support<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    F: WorthQueryDeclarationFamilyMarker<D>,
>(
    handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>,
) -> WorthQueryDeclarationFamilySupportChecked<D, F> {
    let report = derive_family_support_report::<D, C, F>(handle);
    match report.declare_status() {
        WorthQueryDeclarationCapabilityStatus::Admitted => {
            WorthQueryDeclarationFamilySupportChecked::Admitted(report)
        }
        WorthQueryDeclarationCapabilityStatus::DeferredDebt => {
            WorthQueryDeclarationFamilySupportChecked::Deferred(report)
        }
        WorthQueryDeclarationCapabilityStatus::Unsupported => {
            WorthQueryDeclarationFamilySupportChecked::Unsupported(report)
        }
        WorthQueryDeclarationCapabilityStatus::InvalidContext => {
            WorthQueryDeclarationFamilySupportChecked::InvalidContext(report)
        }
    }
}
