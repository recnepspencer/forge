use super::binding::{
    require_text, PhysicalWorkEvidenceBindingDenial, PhysicalWorkEvidenceDigest,
    PhysicalWorkSourceBinding,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkMutantSubject {
    identity: u16,
    predicate: Box<str>,
    source: Box<str>,
}

impl PhysicalWorkMutantSubject {
    pub fn new(
        identity: u16,
        predicate: impl Into<Box<str>>,
        source: impl Into<Box<str>>,
    ) -> Result<Self, PhysicalWorkEvidenceBindingDenial> {
        let predicate = predicate.into();
        let source = source.into();
        require_text(
            &predicate,
            PhysicalWorkEvidenceBindingDenial::EmptyMutantPredicate,
        )?;
        require_text(
            &source,
            PhysicalWorkEvidenceBindingDenial::EmptyMutantSource,
        )?;
        Ok(Self {
            identity,
            predicate,
            source,
        })
    }

    pub const fn identity(&self) -> u16 {
        self.identity
    }

    pub fn predicate(&self) -> &str {
        &self.predicate
    }

    pub fn source(&self) -> &str {
        &self.source
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkMutantExecutionContext {
    profile: Box<str>,
    scenario: Box<str>,
}

impl PhysicalWorkMutantExecutionContext {
    pub fn new(
        profile: impl Into<Box<str>>,
        scenario: impl Into<Box<str>>,
    ) -> Result<Self, PhysicalWorkEvidenceBindingDenial> {
        let profile = profile.into();
        let scenario = scenario.into();
        require_text(
            &profile,
            PhysicalWorkEvidenceBindingDenial::EmptyMutantProfile,
        )?;
        require_text(
            &scenario,
            PhysicalWorkEvidenceBindingDenial::EmptyMutantScenario,
        )?;
        Ok(Self { profile, scenario })
    }

    pub fn profile(&self) -> &str {
        &self.profile
    }

    pub fn scenario(&self) -> &str {
        &self.scenario
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkMutantBinding {
    subject: PhysicalWorkMutantSubject,
    source_digest: PhysicalWorkEvidenceDigest,
    mutant_digest: PhysicalWorkEvidenceDigest,
    binary: PhysicalWorkSourceBinding,
    execution: PhysicalWorkMutantExecutionContext,
}

impl PhysicalWorkMutantBinding {
    pub const fn new(
        subject: PhysicalWorkMutantSubject,
        source_digest: PhysicalWorkEvidenceDigest,
        mutant_digest: PhysicalWorkEvidenceDigest,
        binary: PhysicalWorkSourceBinding,
        execution: PhysicalWorkMutantExecutionContext,
    ) -> Self {
        Self {
            subject,
            source_digest,
            mutant_digest,
            binary,
            execution,
        }
    }

    pub const fn subject(&self) -> &PhysicalWorkMutantSubject {
        &self.subject
    }

    pub const fn source_digest(&self) -> PhysicalWorkEvidenceDigest {
        self.source_digest
    }

    pub const fn mutant_digest(&self) -> PhysicalWorkEvidenceDigest {
        self.mutant_digest
    }

    pub const fn binary(&self) -> &PhysicalWorkSourceBinding {
        &self.binary
    }

    pub const fn execution(&self) -> &PhysicalWorkMutantExecutionContext {
        &self.execution
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkMutantLocalization {
    binding: PhysicalWorkMutantBinding,
    killed: bool,
    localization: Box<str>,
}

impl PhysicalWorkMutantLocalization {
    pub fn new(
        binding: PhysicalWorkMutantBinding,
        outcome: PhysicalWorkMutantOutcome,
    ) -> Result<Self, PhysicalWorkEvidenceBindingDenial> {
        require_text(
            &outcome.localization,
            PhysicalWorkEvidenceBindingDenial::EmptyMutantLocalization,
        )?;
        Ok(Self {
            binding,
            killed: outcome.killed,
            localization: outcome.localization,
        })
    }

    pub const fn binding(&self) -> &PhysicalWorkMutantBinding {
        &self.binding
    }

    pub const fn identity(&self) -> u16 {
        self.binding.subject.identity
    }

    pub fn predicate(&self) -> &str {
        &self.binding.subject.predicate
    }

    pub fn source(&self) -> &str {
        &self.binding.subject.source
    }

    pub const fn killed(&self) -> bool {
        self.killed
    }

    pub fn localization(&self) -> &str {
        &self.localization
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkMutantOutcome {
    killed: bool,
    localization: Box<str>,
}

impl PhysicalWorkMutantOutcome {
    pub fn new(killed: bool, localization: impl Into<Box<str>>) -> Self {
        Self {
            killed,
            localization: localization.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        PhysicalWorkMutantBinding, PhysicalWorkMutantExecutionContext,
        PhysicalWorkMutantLocalization, PhysicalWorkMutantOutcome, PhysicalWorkMutantSubject,
    };
    use crate::physical_runtime::record_serving::{
        PhysicalWorkEvidenceBindingDenial, PhysicalWorkEvidenceDigest, PhysicalWorkSourceBinding,
    };

    #[test]
    fn mutant_localization_cannot_be_an_empty_label() {
        let digest = PhysicalWorkEvidenceDigest::new([7; 32]).unwrap();
        let subject =
            PhysicalWorkMutantSubject::new(1, "raw-backend-dispatch", "executor.rs").unwrap();
        let execution = PhysicalWorkMutantExecutionContext::new("test", "scenario").unwrap();
        let binary = PhysicalWorkSourceBinding::new("test.exe", digest).unwrap();
        let binding = PhysicalWorkMutantBinding::new(subject, digest, digest, binary, execution);
        assert_eq!(
            PhysicalWorkMutantLocalization::new(binding, PhysicalWorkMutantOutcome::new(true, " "),),
            Err(PhysicalWorkEvidenceBindingDenial::EmptyMutantLocalization)
        );
    }
}
