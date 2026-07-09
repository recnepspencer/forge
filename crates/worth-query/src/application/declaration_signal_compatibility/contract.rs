use crate::application::{
    WorthQueryAdmittedConfiguredDomainHandle, WorthQueryCapabilityFamily,
    WorthQueryConfigSectionFamily, WorthQueryDeclarationAspectContract,
    WorthQueryDeclarationAspectCoverage, WorthQueryDeclarationAspectFit,
    WorthQueryDeclarationAuthorityAspectMismatch, WorthQueryDeclarationInput,
    WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext,
};
use crate::basis_lifecycle::BasisFamily;

const CURRENT_HEAD: &[BasisFamily] = &[BasisFamily::CurrentHead];
const HISTORICAL_SNAPSHOT: &[BasisFamily] = &[BasisFamily::HistoricalSnapshot];
const PREVIEW_DERIVED: &[BasisFamily] = &[BasisFamily::PreviewDerived];

const SIGNAL_AND_QUERY: &[WorthQueryCapabilityFamily] =
    &[WorthQueryCapabilityFamily::QueryComposition];
const SIGNAL_AND_HISTORY: &[WorthQueryCapabilityFamily] = &[
    WorthQueryCapabilityFamily::QueryComposition,
    WorthQueryCapabilityFamily::HistoricalEvaluation,
];
const SIGNAL_AND_PREVIEW: &[WorthQueryCapabilityFamily] = &[
    WorthQueryCapabilityFamily::QueryComposition,
    WorthQueryCapabilityFamily::PreviewSession,
];
const SIGNAL_ONLY_CONFIG: &[WorthQueryConfigSectionFamily] =
    &[WorthQueryConfigSectionFamily::Signal];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationSignalExecutionFamily {
    RuntimeDerivedExecution,
    HistoricalDerivedExecution,
    PreviewDerivedExecution,
    MixedDerivedExecution,
}

impl WorthQueryDeclarationSignalExecutionFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeDerivedExecution => "runtime_derived_execution",
            Self::HistoricalDerivedExecution => "historical_derived_execution",
            Self::PreviewDerivedExecution => "preview_derived_execution",
            Self::MixedDerivedExecution => "mixed_derived_execution",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarationSignalCompatibilityContract {
    execution_family: WorthQueryDeclarationSignalExecutionFamily,
    required_basis_families: &'static [BasisFamily],
    required_capability_families: &'static [WorthQueryCapabilityFamily],
    required_config_sections: &'static [WorthQueryConfigSectionFamily],
    dependency_aspects: WorthQueryDeclarationAspectContract,
    produced_aspects: WorthQueryDeclarationAspectContract,
    reason: &'static str,
}

impl WorthQueryDeclarationSignalCompatibilityContract {
    pub fn runtime_derived_execution() -> Self {
        Self {
            execution_family: WorthQueryDeclarationSignalExecutionFamily::RuntimeDerivedExecution,
            required_basis_families: CURRENT_HEAD,
            required_capability_families: SIGNAL_AND_QUERY,
            required_config_sections: SIGNAL_ONLY_CONFIG,
            dependency_aspects: WorthQueryDeclarationAspectContract::empty(),
            produced_aspects: WorthQueryDeclarationAspectContract::empty(),
            reason:
                "the declaration can later continue into Signal-backed runtime derived execution",
        }
    }

    pub fn historical_derived_execution() -> Self {
        Self {
            execution_family:
                WorthQueryDeclarationSignalExecutionFamily::HistoricalDerivedExecution,
            required_basis_families: HISTORICAL_SNAPSHOT,
            required_capability_families: SIGNAL_AND_HISTORY,
            required_config_sections: SIGNAL_ONLY_CONFIG,
            dependency_aspects: WorthQueryDeclarationAspectContract::empty(),
            produced_aspects: WorthQueryDeclarationAspectContract::empty(),
            reason:
                "the declaration can later continue into Signal-backed historical derived execution",
        }
    }

    pub fn preview_derived_execution() -> Self {
        Self {
            execution_family: WorthQueryDeclarationSignalExecutionFamily::PreviewDerivedExecution,
            required_basis_families: PREVIEW_DERIVED,
            required_capability_families: SIGNAL_AND_PREVIEW,
            required_config_sections: SIGNAL_ONLY_CONFIG,
            dependency_aspects: WorthQueryDeclarationAspectContract::empty(),
            produced_aspects: WorthQueryDeclarationAspectContract::empty(),
            reason:
                "the declaration can later continue into Signal-backed preview derived execution",
        }
    }

    pub fn execution_family(&self) -> WorthQueryDeclarationSignalExecutionFamily {
        self.execution_family
    }

    pub fn required_basis_families(&self) -> &'static [BasisFamily] {
        self.required_basis_families
    }

    pub fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        self.required_capability_families
    }

    pub fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        self.required_config_sections
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }

    pub fn dependency_aspects(&self) -> WorthQueryDeclarationAspectContract {
        self.dependency_aspects.clone()
    }

    pub fn produced_aspects(&self) -> WorthQueryDeclarationAspectContract {
        self.produced_aspects.clone()
    }

    pub fn with_aspects(
        mut self,
        dependency_aspects: WorthQueryDeclarationAspectContract,
        produced_aspects: WorthQueryDeclarationAspectContract,
    ) -> Self {
        self.dependency_aspects = dependency_aspects;
        self.produced_aspects = produced_aspects;
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationSignalCompatibilitySupportStatus {
    Admitted,
    Deferred,
    Unsupported,
    InvalidBasis,
}

impl WorthQueryDeclarationSignalCompatibilitySupportStatus {
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
pub struct WorthQueryDeclarationSignalCompatibilitySupportRow {
    execution_family: WorthQueryDeclarationSignalExecutionFamily,
    basis_family: BasisFamily,
    required_dependency_aspects: WorthQueryDeclarationAspectContract,
    produced_aspects: WorthQueryDeclarationAspectContract,
    available_aspect_slice: WorthQueryDeclarationAspectCoverage,
    aspect_fit: WorthQueryDeclarationAspectFit,
    aspect_mismatch: Option<WorthQueryDeclarationAuthorityAspectMismatch>,
    status: WorthQueryDeclarationSignalCompatibilitySupportStatus,
    reason: &'static str,
}

impl WorthQueryDeclarationSignalCompatibilitySupportRow {
    pub(crate) fn new(
        execution_family: WorthQueryDeclarationSignalExecutionFamily,
        basis_family: BasisFamily,
        required_dependency_aspects: WorthQueryDeclarationAspectContract,
        produced_aspects: WorthQueryDeclarationAspectContract,
        available_aspect_slice: WorthQueryDeclarationAspectCoverage,
        aspect_fit: WorthQueryDeclarationAspectFit,
        aspect_mismatch: Option<WorthQueryDeclarationAuthorityAspectMismatch>,
        status: WorthQueryDeclarationSignalCompatibilitySupportStatus,
        reason: &'static str,
    ) -> Self {
        Self {
            execution_family,
            basis_family,
            required_dependency_aspects,
            produced_aspects,
            available_aspect_slice,
            aspect_fit,
            aspect_mismatch,
            status,
            reason,
        }
    }

    pub fn execution_family(&self) -> WorthQueryDeclarationSignalExecutionFamily {
        self.execution_family
    }

    pub fn basis_family(&self) -> BasisFamily {
        self.basis_family
    }

    pub fn required_dependency_aspects(&self) -> &WorthQueryDeclarationAspectContract {
        &self.required_dependency_aspects
    }

    pub fn produced_aspects(&self) -> &WorthQueryDeclarationAspectContract {
        &self.produced_aspects
    }

    pub fn available_aspect_slice(&self) -> &WorthQueryDeclarationAspectCoverage {
        &self.available_aspect_slice
    }

    pub fn aspect_fit(&self) -> WorthQueryDeclarationAspectFit {
        self.aspect_fit
    }

    pub fn aspect_mismatch(&self) -> Option<WorthQueryDeclarationAuthorityAspectMismatch> {
        self.aspect_mismatch
    }

    pub fn status(&self) -> WorthQueryDeclarationSignalCompatibilitySupportStatus {
        self.status
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarationSignalCompatibilitySupportReport<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    declaration_family_key: &'static str,
    rows: Vec<WorthQueryDeclarationSignalCompatibilitySupportRow>,
    support_digest: String,
    _marker: std::marker::PhantomData<(D, I)>,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryDeclarationSignalCompatibilitySupportReport<D, I>
{
    pub(crate) fn new(
        declaration_family_key: &'static str,
        rows: Vec<WorthQueryDeclarationSignalCompatibilitySupportRow>,
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

    pub fn rows(&self) -> &[WorthQueryDeclarationSignalCompatibilitySupportRow] {
        &self.rows
    }

    pub fn support_digest(&self) -> &str {
        &self.support_digest
    }
}

pub(crate) fn derive_signal_compatibility_support_report<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>,
) -> WorthQueryDeclarationSignalCompatibilitySupportReport<D, I> {
    crate::application::worth_query_signal_compatibility_support_from_entry_readiness::<D, C, I>(
        handle,
    )
}
