use crate::performance::FoundationalPerformanceWorkClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalPerformanceAttachmentConstructionDenial {
    EmptyName,
    InvalidNameCharacter,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalPerformanceContractName(String);

impl FoundationalPerformanceContractName {
    pub fn new(
        name: impl Into<String>,
    ) -> Result<Self, FoundationalPerformanceAttachmentConstructionDenial> {
        let name = validate_attachment_name(name.into())?;
        Ok(Self(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalPerformanceCounterName(String);

impl FoundationalPerformanceCounterName {
    pub fn new(
        name: impl Into<String>,
    ) -> Result<Self, FoundationalPerformanceAttachmentConstructionDenial> {
        let name = validate_attachment_name(name.into())?;
        Ok(Self(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalPerformanceSupportingEvidenceCode(String);

impl FoundationalPerformanceSupportingEvidenceCode {
    pub fn new(
        code: impl Into<String>,
    ) -> Result<Self, FoundationalPerformanceAttachmentConstructionDenial> {
        let code = validate_attachment_name(code.into())?;
        Ok(Self(code))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalPerformanceCounterSpec {
    name: FoundationalPerformanceCounterName,
    work_class: FoundationalPerformanceWorkClass,
    expected_exact_count: u64,
}

impl FoundationalPerformanceCounterSpec {
    pub const fn new(
        name: FoundationalPerformanceCounterName,
        work_class: FoundationalPerformanceWorkClass,
        expected_exact_count: u64,
    ) -> Self {
        Self {
            name,
            work_class,
            expected_exact_count,
        }
    }

    pub const fn name(&self) -> &FoundationalPerformanceCounterName {
        &self.name
    }

    pub const fn work_class(&self) -> FoundationalPerformanceWorkClass {
        self.work_class
    }

    pub const fn expected_exact_count(&self) -> u64 {
        self.expected_exact_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalPerformanceCounterRow {
    name: FoundationalPerformanceCounterName,
    observed_count: u64,
}

impl FoundationalPerformanceCounterRow {
    pub const fn new(name: FoundationalPerformanceCounterName, observed_count: u64) -> Self {
        Self {
            name,
            observed_count,
        }
    }

    pub const fn name(&self) -> &FoundationalPerformanceCounterName {
        &self.name
    }

    pub const fn observed_count(&self) -> u64 {
        self.observed_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalPerformanceSupportingEvidenceRow {
    code: FoundationalPerformanceSupportingEvidenceCode,
    related_work: FoundationalPerformanceWorkClass,
}

impl FoundationalPerformanceSupportingEvidenceRow {
    pub const fn new(
        code: FoundationalPerformanceSupportingEvidenceCode,
        related_work: FoundationalPerformanceWorkClass,
    ) -> Self {
        Self { code, related_work }
    }

    pub const fn code(&self) -> &FoundationalPerformanceSupportingEvidenceCode {
        &self.code
    }

    pub const fn related_work(&self) -> FoundationalPerformanceWorkClass {
        self.related_work
    }
}

fn validate_attachment_name(
    name: String,
) -> Result<String, FoundationalPerformanceAttachmentConstructionDenial> {
    if name.trim().is_empty() {
        return Err(FoundationalPerformanceAttachmentConstructionDenial::EmptyName);
    }
    if !name.chars().all(|character| {
        character.is_ascii_lowercase()
            || character.is_ascii_digit()
            || matches!(character, '.' | '_' | '-')
    }) {
        return Err(FoundationalPerformanceAttachmentConstructionDenial::InvalidNameCharacter);
    }
    Ok(name)
}
