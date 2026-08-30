use worth_query_installation::facade::{
    WorthQueryExecutionProviderRequirements, WorthQueryInstallationCapabilityFamily,
    WorthQueryInstallationConfigSectionFamily, WorthQueryInstallationOperatingRequirement,
    WorthQueryPortablePackageRecord, WorthQueryPortablePackageRecordSet,
};

use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPackageReleaseRequirements {
    capabilities: Vec<WorthQueryInstallationCapabilityFamily>,
    configuration: Vec<WorthQueryInstallationConfigSectionFamily>,
    operating: Vec<WorthQueryInstallationOperatingRequirement>,
    execution_providers: Vec<WorthQueryExecutionProviderRequirements>,
}

impl WorthQueryPackageReleaseRequirements {
    pub fn capabilities(&self) -> &[WorthQueryInstallationCapabilityFamily] {
        &self.capabilities
    }

    pub fn configuration(&self) -> &[WorthQueryInstallationConfigSectionFamily] {
        &self.configuration
    }

    pub fn operating(&self) -> &[WorthQueryInstallationOperatingRequirement] {
        &self.operating
    }

    pub fn execution_providers(&self) -> &[WorthQueryExecutionProviderRequirements] {
        &self.execution_providers
    }

    pub(crate) fn derive(records: &WorthQueryPortablePackageRecordSet) -> Self {
        let mut capabilities = Vec::new();
        let mut configuration = Vec::new();
        let mut operating = Vec::new();
        let mut execution_providers = Vec::new();

        for record in records.records() {
            match record {
                WorthQueryPortablePackageRecord::CapabilityRequirement(value) => {
                    capabilities.push(value.clone());
                }
                WorthQueryPortablePackageRecord::ConfigurationRequirement(value) => {
                    configuration.push(value.clone());
                }
                WorthQueryPortablePackageRecord::OperatingRequirement(value) => {
                    operating.push(value.clone());
                }
                WorthQueryPortablePackageRecord::DomainOperation(operation) => {
                    execution_providers.extend(
                        operation
                            .semantics()
                            .resources()
                            .strategies()
                            .iter()
                            .map(|strategy| strategy.provider_requirements().clone()),
                    );
                }
                _ => {}
            }
        }

        capabilities.sort();
        capabilities.dedup();
        configuration.sort();
        configuration.dedup();
        operating.sort();
        operating.dedup();
        execution_providers.sort_by(compare_provider_requirements);
        execution_providers
            .dedup_by(|left, right| compare_provider_requirements(left, right).is_eq());

        Self {
            capabilities,
            configuration,
            operating,
            execution_providers,
        }
    }

    pub(crate) fn from_untrusted_parts(
        capabilities: Vec<WorthQueryInstallationCapabilityFamily>,
        configuration: Vec<WorthQueryInstallationConfigSectionFamily>,
        operating: Vec<WorthQueryInstallationOperatingRequirement>,
        execution_providers: Vec<WorthQueryExecutionProviderRequirements>,
    ) -> Result<Self, Denial> {
        require_strict_text_sequence(&capabilities, |value| value.as_str())?;
        require_strict_text_sequence(&configuration, |value| value.as_str())?;
        require_strict_text_sequence(&operating, |value| value.as_str())?;
        if execution_providers
            .windows(2)
            .any(|pair| !compare_provider_requirements(&pair[0], &pair[1]).is_lt())
        {
            return Err(Denial::new(Kind::NonCanonicalEnvelopeRequirementSequence));
        }
        Ok(Self {
            capabilities,
            configuration,
            operating,
            execution_providers,
        })
    }

    pub(crate) fn count(&self) -> Result<u32, Denial> {
        [
            self.capabilities.len(),
            self.configuration.len(),
            self.operating.len(),
            self.execution_providers.len(),
        ]
        .into_iter()
        .try_fold(0_u32, |total, length| {
            total
                .checked_add(
                    u32::try_from(length)
                        .map_err(|_| Denial::new(Kind::EnvelopeRequirementBudgetExceeded))?,
                )
                .ok_or_else(|| Denial::new(Kind::EnvelopeRequirementBudgetExceeded))
        })
    }
}

fn require_strict_text_sequence<T>(values: &[T], text: impl Fn(&T) -> &str) -> Result<(), Denial> {
    if values
        .windows(2)
        .any(|pair| text(&pair[0]) >= text(&pair[1]))
    {
        return Err(Denial::new(Kind::NonCanonicalEnvelopeRequirementSequence));
    }
    Ok(())
}

fn compare_provider_requirements(
    left: &WorthQueryExecutionProviderRequirements,
    right: &WorthQueryExecutionProviderRequirements,
) -> std::cmp::Ordering {
    (
        left.provider().as_str(),
        left.access_product().as_str(),
        left.allocator().as_str(),
    )
        .cmp(&(
            right.provider().as_str(),
            right.access_product().as_str(),
            right.allocator().as_str(),
        ))
}
