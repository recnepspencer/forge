use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryApplicationFacade,
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryConfiguredDomainHandleAdmissionError, ForgeQueryConfiguredDomainHandleInvalidContext,
    ForgeQueryContributionComposedOrchestrationChecked,
    ForgeQueryContributionComposedOrchestrationInput,
    ForgeQueryContributionComposedOrchestrationTranscript,
    ForgeQueryDeclarationEntryCrossingInventory, ForgeQueryDeclarationEntryOrchestrationChecked,
    ForgeQueryDeclarationEntryReadinessReport, ForgeQueryDeclarationEnvelope,
    ForgeQueryDeclaredFamilyChecked, ForgeQueryGroupedContributionComposition,
    ForgeQueryGroupedContributionInput, ForgeQueryGroupedContributionStop,
    ForgeQueryGroupedOrchestrationChecked, ForgeQueryGroupedOrchestrationTranscript,
    ForgeQueryOrdinaryOutcome, ForgeQueryRecoveryBrief,
};

use crate::domain_declarations::HadwigerResearchDeclarationInput;

use super::{HadwigerResearchDomainEntry, HadwigerResearchOperatingContext};

#[derive(Clone, Eq, PartialEq)]
pub struct HadwigerResearchHandle {
    query_handle: ForgeQueryAdmittedConfiguredDomainHandle<
        HadwigerResearchDomainEntry,
        HadwigerResearchOperatingContext,
    >,
}

impl std::fmt::Debug for HadwigerResearchHandle {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("HadwigerResearchHandle")
            .field("domain_key", &self.domain_key())
            .field("display_name", &self.display_name())
            .field("handle_identity_digest", &self.handle_identity_digest())
            .field(
                "operating_context_identity_digest",
                &self.operating_context_identity_digest(),
            )
            .finish()
    }
}

impl HadwigerResearchHandle {
    pub(crate) fn new(
        query_handle: ForgeQueryAdmittedConfiguredDomainHandle<
            HadwigerResearchDomainEntry,
            HadwigerResearchOperatingContext,
        >,
    ) -> Self {
        Self { query_handle }
    }

    pub fn domain_key(&self) -> &'static str {
        self.query_handle.domain_key()
    }

    pub fn display_name(&self) -> &'static str {
        self.query_handle.display_name()
    }

    pub fn handle_identity_digest(&self) -> &str {
        self.query_handle.handle_identity_digest()
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        self.query_handle.operating_context_identity_digest()
    }

    pub fn required_capability_families(&self) -> &[ForgeQueryCapabilityFamily] {
        self.query_handle.required_capability_families()
    }

    pub fn required_config_sections(&self) -> &[ForgeQueryConfigSectionFamily] {
        self.query_handle.required_config_sections()
    }

    pub fn declare_checked<I>(
        &self,
        input: I,
    ) -> ForgeQueryDeclaredFamilyChecked<HadwigerResearchDomainEntry, I>
    where
        I: HadwigerResearchDeclarationInput,
    {
        self.query_handle.declare_checked(input)
    }

    pub fn orchestrate_declaration_entry_outcome<I>(
        &self,
        input: I,
    ) -> ForgeQueryOrdinaryOutcome<ForgeQueryDeclarationEnvelope<HadwigerResearchDomainEntry, I>>
    where
        I: HadwigerResearchDeclarationInput,
    {
        self.query_handle
            .orchestrate_declaration_entry_outcome(input)
    }

    pub fn declaration_entry_crossing_inventory<I>(
        &self,
    ) -> ForgeQueryDeclarationEntryCrossingInventory<HadwigerResearchDomainEntry, I>
    where
        I: HadwigerResearchDeclarationInput,
    {
        self.query_handle
            .declaration_entry_crossing_inventory::<I>()
    }

    pub fn declaration_entry_readiness<I>(
        &self,
    ) -> ForgeQueryDeclarationEntryReadinessReport<HadwigerResearchDomainEntry, I>
    where
        I: HadwigerResearchDeclarationInput,
    {
        self.query_handle.declaration_entry_readiness::<I>()
    }

    pub fn recover_from_outcome<T>(
        &self,
        outcome: &ForgeQueryOrdinaryOutcome<T>,
    ) -> Option<ForgeQueryRecoveryBrief> {
        self.query_handle.recover_from_outcome(outcome)
    }

    pub fn recover_from_declaration_entry_checked<I>(
        &self,
        checked: ForgeQueryDeclarationEntryOrchestrationChecked<HadwigerResearchDomainEntry, I>,
    ) -> Option<ForgeQueryRecoveryBrief>
    where
        I: HadwigerResearchDeclarationInput,
    {
        self.query_handle
            .recover_from_declaration_entry_checked(checked)
    }

    pub fn recover_from_contribution_composed_checked<I>(
        &self,
        checked: ForgeQueryContributionComposedOrchestrationChecked<HadwigerResearchDomainEntry, I>,
    ) -> Option<ForgeQueryRecoveryBrief>
    where
        I: HadwigerResearchDeclarationInput,
    {
        self.query_handle
            .recover_from_contribution_composed_checked(checked)
    }

    pub fn orchestrate_declaration_with_contributions_checked<I>(
        &self,
        input: ForgeQueryContributionComposedOrchestrationInput<HadwigerResearchDomainEntry, I>,
    ) -> ForgeQueryContributionComposedOrchestrationChecked<HadwigerResearchDomainEntry, I>
    where
        I: HadwigerResearchDeclarationInput,
    {
        self.query_handle
            .orchestrate_declaration_with_contributions_checked(input)
    }

    pub fn orchestrate_declaration_with_contributions_proof<I>(
        &self,
        input: ForgeQueryContributionComposedOrchestrationInput<HadwigerResearchDomainEntry, I>,
    ) -> ForgeQueryContributionComposedOrchestrationTranscript<HadwigerResearchDomainEntry, I>
    where
        I: HadwigerResearchDeclarationInput,
    {
        self.query_handle
            .orchestrate_declaration_with_contributions_proof(input)
    }

    pub fn recover_from_grouped_orchestration_checked<I>(
        &self,
        checked: ForgeQueryGroupedOrchestrationChecked<HadwigerResearchDomainEntry, I>,
    ) -> Option<ForgeQueryRecoveryBrief>
    where
        I: HadwigerResearchDeclarationInput,
    {
        self.query_handle
            .recover_from_grouped_orchestration_checked(checked)
    }

    pub fn grouped_contributions_checked<I>(
        &self,
        input: ForgeQueryGroupedContributionInput<HadwigerResearchDomainEntry, I>,
    ) -> Result<
        ForgeQueryGroupedContributionComposition<HadwigerResearchDomainEntry, I>,
        ForgeQueryGroupedContributionStop<HadwigerResearchDomainEntry, I>,
    >
    where
        I: HadwigerResearchDeclarationInput + Clone,
    {
        self.query_handle.grouped_contributions_checked(input)
    }

    pub fn recover_from_grouped_orchestration_proof<I>(
        &self,
        proof: ForgeQueryGroupedOrchestrationTranscript<HadwigerResearchDomainEntry, I>,
    ) -> Option<ForgeQueryRecoveryBrief>
    where
        I: HadwigerResearchDeclarationInput,
    {
        self.query_handle
            .recover_from_grouped_orchestration_proof(proof)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HadwigerResearchAdmissionError {
    InvalidContext(
        ForgeQueryConfiguredDomainHandleInvalidContext<
            HadwigerResearchDomainEntry,
            HadwigerResearchOperatingContext,
        >,
    ),
    Admission(
        ForgeQueryConfiguredDomainHandleAdmissionError<
            HadwigerResearchDomainEntry,
            HadwigerResearchOperatingContext,
        >,
    ),
}

pub fn admit_hadwiger_research_handle(
    context: HadwigerResearchOperatingContext,
) -> Result<HadwigerResearchHandle, HadwigerResearchAdmissionError> {
    let validated = ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(HadwigerResearchDomainEntry)
        .with_operating_context(context)
        .validate()
        .map_err(HadwigerResearchAdmissionError::InvalidContext)?;

    validated
        .admit()
        .map(HadwigerResearchHandle::new)
        .map_err(HadwigerResearchAdmissionError::Admission)
}
