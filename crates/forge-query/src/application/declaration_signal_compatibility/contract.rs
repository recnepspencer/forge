use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryCapabilityFamily,
    ForgeQueryConfigSectionFamily, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext,
};
use crate::basis_lifecycle::BasisFamily;

const CURRENT_HEAD: &[BasisFamily] = &[BasisFamily::CurrentHead];
const HISTORICAL_SNAPSHOT: &[BasisFamily] = &[BasisFamily::HistoricalSnapshot];
const PREVIEW_DERIVED: &[BasisFamily] = &[BasisFamily::PreviewDerived];

const SIGNAL_AND_QUERY: &[ForgeQueryCapabilityFamily] =
    &[ForgeQueryCapabilityFamily::QueryComposition];
const SIGNAL_AND_HISTORY: &[ForgeQueryCapabilityFamily] = &[
    ForgeQueryCapabilityFamily::QueryComposition,
    ForgeQueryCapabilityFamily::HistoricalEvaluation,
];
const SIGNAL_AND_PREVIEW: &[ForgeQueryCapabilityFamily] = &[
    ForgeQueryCapabilityFamily::QueryComposition,
    ForgeQueryCapabilityFamily::PreviewSession,
];
const SIGNAL_ONLY_CONFIG: &[ForgeQueryConfigSectionFamily] =
    &[ForgeQueryConfigSectionFamily::Signal];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationSignalExecutionFamily {
    RuntimeDerivedExecution,
    HistoricalDerivedExecution,
    PreviewDerivedExecution,
    MixedDerivedExecution,
}

impl ForgeQueryDeclarationSignalExecutionFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeDerivedExecution => "runtime_derived_execution",
            Self::HistoricalDerivedExecution => "historical_derived_execution",
            Self::PreviewDerivedExecution => "preview_derived_execution",
            Self::MixedDerivedExecution => "mixed_derived_execution",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationSignalCompatibilityContract {
    execution_family: ForgeQueryDeclarationSignalExecutionFamily,
    required_basis_families: &'static [BasisFamily],
    required_capability_families: &'static [ForgeQueryCapabilityFamily],
    required_config_sections: &'static [ForgeQueryConfigSectionFamily],
    reason: &'static str,
}

impl ForgeQueryDeclarationSignalCompatibilityContract {
    pub fn runtime_derived_execution() -> Self {
        Self {
            execution_family: ForgeQueryDeclarationSignalExecutionFamily::RuntimeDerivedExecution,
            required_basis_families: CURRENT_HEAD,
            required_capability_families: SIGNAL_AND_QUERY,
            required_config_sections: SIGNAL_ONLY_CONFIG,
            reason:
                "the declaration can later continue into Signal-backed runtime derived execution",
        }
    }

    pub fn historical_derived_execution() -> Self {
        Self {
            execution_family:
                ForgeQueryDeclarationSignalExecutionFamily::HistoricalDerivedExecution,
            required_basis_families: HISTORICAL_SNAPSHOT,
            required_capability_families: SIGNAL_AND_HISTORY,
            required_config_sections: SIGNAL_ONLY_CONFIG,
            reason:
                "the declaration can later continue into Signal-backed historical derived execution",
        }
    }

    pub fn preview_derived_execution() -> Self {
        Self {
            execution_family: ForgeQueryDeclarationSignalExecutionFamily::PreviewDerivedExecution,
            required_basis_families: PREVIEW_DERIVED,
            required_capability_families: SIGNAL_AND_PREVIEW,
            required_config_sections: SIGNAL_ONLY_CONFIG,
            reason:
                "the declaration can later continue into Signal-backed preview derived execution",
        }
    }

    pub fn execution_family(self) -> ForgeQueryDeclarationSignalExecutionFamily {
        self.execution_family
    }

    pub fn required_basis_families(self) -> &'static [BasisFamily] {
        self.required_basis_families
    }

    pub fn required_capability_families(self) -> &'static [ForgeQueryCapabilityFamily] {
        self.required_capability_families
    }

    pub fn required_config_sections(self) -> &'static [ForgeQueryConfigSectionFamily] {
        self.required_config_sections
    }

    pub fn reason(self) -> &'static str {
        self.reason
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationSignalCompatibilitySupportStatus {
    Admitted,
    Deferred,
    Unsupported,
    InvalidBasis,
}

impl ForgeQueryDeclarationSignalCompatibilitySupportStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Deferred => "deferred",
            Self::Unsupported => "unsupported",
            Self::InvalidBasis => "invalid_basis",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationSignalCompatibilitySupportRow {
    execution_family: ForgeQueryDeclarationSignalExecutionFamily,
    basis_family: BasisFamily,
    status: ForgeQueryDeclarationSignalCompatibilitySupportStatus,
    reason: &'static str,
}

impl ForgeQueryDeclarationSignalCompatibilitySupportRow {
    pub(crate) fn new(
        execution_family: ForgeQueryDeclarationSignalExecutionFamily,
        basis_family: BasisFamily,
        status: ForgeQueryDeclarationSignalCompatibilitySupportStatus,
        reason: &'static str,
    ) -> Self {
        Self {
            execution_family,
            basis_family,
            status,
            reason,
        }
    }

    pub fn execution_family(&self) -> ForgeQueryDeclarationSignalExecutionFamily {
        self.execution_family
    }

    pub fn basis_family(&self) -> BasisFamily {
        self.basis_family
    }

    pub fn status(&self) -> ForgeQueryDeclarationSignalCompatibilitySupportStatus {
        self.status
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationSignalCompatibilitySupportReport<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    declaration_family_key: &'static str,
    rows: Vec<ForgeQueryDeclarationSignalCompatibilitySupportRow>,
    support_digest: String,
    _marker: std::marker::PhantomData<(D, I)>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationSignalCompatibilitySupportReport<D, I>
{
    pub(crate) fn new(
        declaration_family_key: &'static str,
        rows: Vec<ForgeQueryDeclarationSignalCompatibilitySupportRow>,
        support_digest: String,
    ) -> Self {
        Self {
            declaration_family_key,
            rows,
            support_digest,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.declaration_family_key
    }

    pub fn rows(&self) -> &[ForgeQueryDeclarationSignalCompatibilitySupportRow] {
        &self.rows
    }

    pub fn support_digest(&self) -> &str {
        &self.support_digest
    }
}

pub(crate) fn derive_signal_compatibility_support_report<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
) -> ForgeQueryDeclarationSignalCompatibilitySupportReport<D, I> {
    crate::application::forge_query_signal_compatibility_support_from_entry_readiness::<D, C, I>(
        handle,
    )
}
