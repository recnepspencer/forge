use crate::{PhysicalIntegrityCloseoutDenial, S3OwnedCloseoutFileEvidence};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum S3CloseoutModuleKind {
    Checksum,
    Scrub,
    Quarantine,
    Evidence,
    Handoff,
    CloseoutSuite,
    CloseoutReport,
    CloseoutTest,
}

impl S3CloseoutModuleKind {
    pub const REQUIRED: [Self; 8] = [
        Self::Checksum,
        Self::Scrub,
        Self::Quarantine,
        Self::Evidence,
        Self::Handoff,
        Self::CloseoutSuite,
        Self::CloseoutReport,
        Self::CloseoutTest,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S3LineCapModuleEvidence {
    module: S3CloseoutModuleKind,
    line_count: u32,
    line_cap: u32,
}

impl S3LineCapModuleEvidence {
    pub fn checked(
        module: S3CloseoutModuleKind,
        line_count: u32,
        line_cap: u32,
    ) -> Result<Self, PhysicalIntegrityCloseoutDenial> {
        if line_count == 0 {
            return Err(PhysicalIntegrityCloseoutDenial::MissingLineCapModule(
                module,
            ));
        }
        if line_count > line_cap {
            return Err(PhysicalIntegrityCloseoutDenial::LineCapModuleOverBudget(
                module,
            ));
        }
        Ok(Self {
            module,
            line_count,
            line_cap,
        })
    }

    pub const fn module(self) -> S3CloseoutModuleKind {
        self.module
    }

    pub const fn line_count(self) -> u32 {
        self.line_count
    }

    pub const fn line_cap(self) -> u32 {
        self.line_cap
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S3LineCapCompositionEvidence {
    modules: Vec<S3LineCapModuleEvidence>,
    owned_files: Vec<S3OwnedCloseoutFileEvidence>,
}

impl S3LineCapCompositionEvidence {
    pub fn from_checked_modules(
        modules: Vec<S3LineCapModuleEvidence>,
    ) -> Result<Self, PhysicalIntegrityCloseoutDenial> {
        Self::from_checked_modules_and_owned_files(modules, Vec::new())
    }

    pub fn from_checked_modules_and_owned_files(
        modules: Vec<S3LineCapModuleEvidence>,
        owned_files: Vec<S3OwnedCloseoutFileEvidence>,
    ) -> Result<Self, PhysicalIntegrityCloseoutDenial> {
        for required in S3CloseoutModuleKind::REQUIRED {
            require_module(&modules, required)?;
        }
        require_distinct_responsibilities(&modules)?;
        require_physical_integrity_owned_closeout_files(&owned_files)?;
        Ok(Self {
            modules,
            owned_files,
        })
    }

    pub fn modules(&self) -> &[S3LineCapModuleEvidence] {
        &self.modules
    }

    pub fn owned_files(&self) -> &[S3OwnedCloseoutFileEvidence] {
        &self.owned_files
    }

    pub fn checked_surface_count(&self) -> usize {
        self.modules.len() + self.owned_files.len()
    }

    pub fn contains_module(&self, module: S3CloseoutModuleKind) -> bool {
        self.modules
            .iter()
            .any(|evidence| evidence.module == module)
    }
}

fn require_module(
    modules: &[S3LineCapModuleEvidence],
    required: S3CloseoutModuleKind,
) -> Result<(), PhysicalIntegrityCloseoutDenial> {
    if modules.iter().any(|evidence| evidence.module == required) {
        Ok(())
    } else {
        Err(PhysicalIntegrityCloseoutDenial::MissingLineCapModule(
            required,
        ))
    }
}

fn require_distinct_responsibilities(
    modules: &[S3LineCapModuleEvidence],
) -> Result<(), PhysicalIntegrityCloseoutDenial> {
    for required in S3CloseoutModuleKind::REQUIRED {
        let count = modules
            .iter()
            .filter(|evidence| evidence.module == required)
            .count();
        if count > 1 {
            return Err(PhysicalIntegrityCloseoutDenial::CollapsedCloseoutResponsibility(required));
        }
    }
    Ok(())
}

fn require_physical_integrity_owned_closeout_files(
    owned_files: &[S3OwnedCloseoutFileEvidence],
) -> Result<(), PhysicalIntegrityCloseoutDenial> {
    if owned_files.is_empty() {
        return Ok(());
    }
    for required in [
        "physical_integrity_closeout_bundle.rs",
        "physical_integrity_closeout_denial.rs",
        "physical_integrity_closeout_handoff.rs",
        "physical_integrity_closeout_harness.rs",
        "physical_integrity_closeout_harness_execution.rs",
        "physical_integrity_closeout_harness_runner.rs",
        "physical_integrity_closeout_line_cap.rs",
        "physical_integrity_closeout_line_cap_tests.rs",
        "physical_integrity_closeout_owned_file.rs",
        "physical_integrity_closeout_proof.rs",
        "physical_integrity_closeout_report.rs",
        "physical_integrity_closeout_suite.rs",
        "physical_integrity_closeout_suite_kind.rs",
        "physical_integrity_closeout_tests.rs",
    ] {
        if !owned_files.iter().any(|file| file.file_name() == required) {
            return Err(PhysicalIntegrityCloseoutDenial::OmittedS3OwnedCloseoutFile(
                required.to_string(),
            ));
        }
    }
    Ok(())
}
