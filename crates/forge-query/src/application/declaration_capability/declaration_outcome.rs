use super::support::{
    ForgeQueryDeclarationCapabilityStatus, ForgeQueryDeclarationFamilySupportReport,
};
use super::support_checked::{
    forge_query_checked_family_support, ForgeQueryDeclarationFamilySupportChecked,
};
use crate::application::{
    forge_query_canonical_declaration, ForgeQueryAdmittedConfiguredDomainHandle,
    ForgeQueryCanonicalDeclarationArtifact, ForgeQueryDeclarationCanonicalizationError,
    ForgeQueryDeclarationCanonicalizationVersion, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationCapabilityDenial<
    D: ForgeQueryDomainEntryMarker,
    F: ForgeQueryDeclarationFamilyMarker<D>,
> {
    support_report: ForgeQueryDeclarationFamilySupportReport<D, F>,
}

impl<D: ForgeQueryDomainEntryMarker, F: ForgeQueryDeclarationFamilyMarker<D>>
    ForgeQueryDeclarationCapabilityDenial<D, F>
{
    fn new(support_report: ForgeQueryDeclarationFamilySupportReport<D, F>) -> Self {
        Self { support_report }
    }

    pub fn capability_status(&self) -> ForgeQueryDeclarationCapabilityStatus {
        self.support_report.declare_status()
    }

    pub fn support_report(&self) -> &ForgeQueryDeclarationFamilySupportReport<D, F> {
        &self.support_report
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationAdmissionError<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Deferred(ForgeQueryDeclarationCapabilityDenial<D, I::Family>),
    Unsupported(ForgeQueryDeclarationCapabilityDenial<D, I::Family>),
    InvalidContext(ForgeQueryDeclarationCapabilityDenial<D, I::Family>),
    Canonicalization(ForgeQueryDeclarationCanonicalizationError),
}

pub enum ForgeQueryDeclaredFamilyChecked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Admitted(ForgeQueryCanonicalDeclarationArtifact<D, I>),
    Deferred(ForgeQueryDeclarationCapabilityDenial<D, I::Family>),
    Unsupported(ForgeQueryDeclarationCapabilityDenial<D, I::Family>),
    InvalidContext(ForgeQueryDeclarationCapabilityDenial<D, I::Family>),
    Canonicalization(ForgeQueryDeclarationCanonicalizationError),
}

pub(crate) fn forge_query_checked_family_declaration<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    input: I,
    version: ForgeQueryDeclarationCanonicalizationVersion,
) -> ForgeQueryDeclaredFamilyChecked<D, I> {
    match forge_query_checked_family_support::<D, C, I::Family>(handle) {
        ForgeQueryDeclarationFamilySupportChecked::Admitted(_) => {}
        ForgeQueryDeclarationFamilySupportChecked::Deferred(report) => {
            return ForgeQueryDeclaredFamilyChecked::Deferred(
                ForgeQueryDeclarationCapabilityDenial::new(report),
            );
        }
        ForgeQueryDeclarationFamilySupportChecked::Unsupported(report) => {
            return ForgeQueryDeclaredFamilyChecked::Unsupported(
                ForgeQueryDeclarationCapabilityDenial::new(report),
            );
        }
        ForgeQueryDeclarationFamilySupportChecked::InvalidContext(report) => {
            return ForgeQueryDeclaredFamilyChecked::InvalidContext(
                ForgeQueryDeclarationCapabilityDenial::new(report),
            );
        }
    }

    match forge_query_canonical_declaration(handle, input, version) {
        Ok(artifact) => ForgeQueryDeclaredFamilyChecked::Admitted(artifact),
        Err(error) => ForgeQueryDeclaredFamilyChecked::Canonicalization(error),
    }
}
