use super::support::{
    derive_family_support_report, ForgeQueryDeclarationCapabilityStatus,
    ForgeQueryDeclarationFamilySupportReport,
};
use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationFamilySupportChecked<
    D: ForgeQueryDomainEntryMarker,
    F: ForgeQueryDeclarationFamilyMarker<D>,
> {
    Admitted(ForgeQueryDeclarationFamilySupportReport<D, F>),
    Deferred(ForgeQueryDeclarationFamilySupportReport<D, F>),
    Unsupported(ForgeQueryDeclarationFamilySupportReport<D, F>),
    InvalidContext(ForgeQueryDeclarationFamilySupportReport<D, F>),
}

pub(crate) fn forge_query_checked_family_support<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    F: ForgeQueryDeclarationFamilyMarker<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
) -> ForgeQueryDeclarationFamilySupportChecked<D, F> {
    let report = derive_family_support_report::<D, C, F>(handle);
    match report.declare_status() {
        ForgeQueryDeclarationCapabilityStatus::Admitted => {
            ForgeQueryDeclarationFamilySupportChecked::Admitted(report)
        }
        ForgeQueryDeclarationCapabilityStatus::DeferredDebt => {
            ForgeQueryDeclarationFamilySupportChecked::Deferred(report)
        }
        ForgeQueryDeclarationCapabilityStatus::Unsupported => {
            ForgeQueryDeclarationFamilySupportChecked::Unsupported(report)
        }
        ForgeQueryDeclarationCapabilityStatus::InvalidContext => {
            ForgeQueryDeclarationFamilySupportChecked::InvalidContext(report)
        }
    }
}
