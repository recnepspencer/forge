use super::support::{
    ForgeQueryDeclarationCapabilityStatus, ForgeQueryDeclarationFamilySupportReport,
};
use super::support_checked::{
    forge_query_checked_family_support, ForgeQueryDeclarationFamilySupportChecked,
};
use crate::application::{
    forge_query_canonical_declaration, ForgeQueryAdmittedConfiguredDomainHandle,
    ForgeQueryAsyncDeclarationSupport, ForgeQueryCanonicalDeclarationArtifact,
    ForgeQueryDeclarationCanonicalizationError, ForgeQueryDeclarationCanonicalizationVersion,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryTemporalDeclarationSupport,
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
pub struct ForgeQueryAsyncDeclarationDenial<
    D: ForgeQueryDomainEntryMarker,
    F: ForgeQueryDeclarationFamilyMarker<D>,
> {
    support_report: ForgeQueryDeclarationFamilySupportReport<D, F>,
    async_support: ForgeQueryAsyncDeclarationSupport,
}

impl<D: ForgeQueryDomainEntryMarker, F: ForgeQueryDeclarationFamilyMarker<D>>
    ForgeQueryAsyncDeclarationDenial<D, F>
{
    fn new(
        support_report: ForgeQueryDeclarationFamilySupportReport<D, F>,
        async_support: ForgeQueryAsyncDeclarationSupport,
    ) -> Self {
        Self {
            support_report,
            async_support,
        }
    }

    pub fn support_report(&self) -> &ForgeQueryDeclarationFamilySupportReport<D, F> {
        &self.support_report
    }

    pub fn async_support(&self) -> ForgeQueryAsyncDeclarationSupport {
        self.async_support
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryTemporalDeclarationDenial<
    D: ForgeQueryDomainEntryMarker,
    F: ForgeQueryDeclarationFamilyMarker<D>,
> {
    support_report: ForgeQueryDeclarationFamilySupportReport<D, F>,
    temporal_support: ForgeQueryTemporalDeclarationSupport,
}

impl<D: ForgeQueryDomainEntryMarker, F: ForgeQueryDeclarationFamilyMarker<D>>
    ForgeQueryTemporalDeclarationDenial<D, F>
{
    fn new(
        support_report: ForgeQueryDeclarationFamilySupportReport<D, F>,
        temporal_support: ForgeQueryTemporalDeclarationSupport,
    ) -> Self {
        Self {
            support_report,
            temporal_support,
        }
    }

    pub fn support_report(&self) -> &ForgeQueryDeclarationFamilySupportReport<D, F> {
        &self.support_report
    }

    pub fn temporal_support(&self) -> ForgeQueryTemporalDeclarationSupport {
        self.temporal_support
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
    AsyncDeferred(ForgeQueryAsyncDeclarationDenial<D, I::Family>),
    AsyncUnsupported(ForgeQueryAsyncDeclarationDenial<D, I::Family>),
    TemporalDeferred(ForgeQueryTemporalDeclarationDenial<D, I::Family>),
    TemporalUnsupported(ForgeQueryTemporalDeclarationDenial<D, I::Family>),
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
    AsyncDeferred(ForgeQueryAsyncDeclarationDenial<D, I::Family>),
    AsyncUnsupported(ForgeQueryAsyncDeclarationDenial<D, I::Family>),
    TemporalDeferred(ForgeQueryTemporalDeclarationDenial<D, I::Family>),
    TemporalUnsupported(ForgeQueryTemporalDeclarationDenial<D, I::Family>),
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
    let support_report = match forge_query_checked_family_support::<D, C, I::Family>(handle) {
        ForgeQueryDeclarationFamilySupportChecked::Admitted(report) => report,
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
    };

    if !input.async_resource_declaration_clauses().is_empty() {
        match I::Family::async_declaration_support() {
            ForgeQueryAsyncDeclarationSupport::CanonicalIdentityOnly => {}
            ForgeQueryAsyncDeclarationSupport::DeferredDebt => {
                return ForgeQueryDeclaredFamilyChecked::AsyncDeferred(
                    ForgeQueryAsyncDeclarationDenial::new(
                        support_report,
                        ForgeQueryAsyncDeclarationSupport::DeferredDebt,
                    ),
                );
            }
            ForgeQueryAsyncDeclarationSupport::Unsupported => {
                return ForgeQueryDeclaredFamilyChecked::AsyncUnsupported(
                    ForgeQueryAsyncDeclarationDenial::new(
                        support_report,
                        ForgeQueryAsyncDeclarationSupport::Unsupported,
                    ),
                );
            }
        }
    }

    if !input.temporal_declaration_clauses().is_empty() {
        match I::Family::temporal_declaration_support() {
            ForgeQueryTemporalDeclarationSupport::CanonicalIdentityOnly => {}
            ForgeQueryTemporalDeclarationSupport::DeferredDebt => {
                return ForgeQueryDeclaredFamilyChecked::TemporalDeferred(
                    ForgeQueryTemporalDeclarationDenial::new(
                        support_report,
                        ForgeQueryTemporalDeclarationSupport::DeferredDebt,
                    ),
                );
            }
            ForgeQueryTemporalDeclarationSupport::Unsupported => {
                return ForgeQueryDeclaredFamilyChecked::TemporalUnsupported(
                    ForgeQueryTemporalDeclarationDenial::new(
                        support_report,
                        ForgeQueryTemporalDeclarationSupport::Unsupported,
                    ),
                );
            }
        }
    }

    match forge_query_canonical_declaration(handle, input, version) {
        Ok(artifact) => ForgeQueryDeclaredFamilyChecked::Admitted(artifact),
        Err(error) => ForgeQueryDeclaredFamilyChecked::Canonicalization(error),
    }
}
