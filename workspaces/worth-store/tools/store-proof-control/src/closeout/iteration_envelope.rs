use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::evidence::sha256_serialized;

use super::{DeveloperEditCase, DeveloperIterationCaseEvidence};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReferenceDevelopmentProfile {
    pub operating_system: String,
    pub filesystem: String,
    pub cpu: String,
    pub storage_class: String,
    pub antivirus_posture: String,
    pub rust_toolchain: String,
    pub source_revision: String,
    pub lockfile_sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeveloperIterationEnvelope {
    schema_version: u32,
    evidence_identity: String,
    reference_profile: ReferenceDevelopmentProfile,
    cases: Vec<DeveloperIterationCaseEvidence>,
}

impl DeveloperIterationEnvelope {
    pub fn certify(
        reference_profile: ReferenceDevelopmentProfile,
        mut cases: Vec<DeveloperIterationCaseEvidence>,
    ) -> Result<Self, String> {
        cases.sort_by_key(|case| case.edit.case);
        let mut envelope = Self {
            schema_version: 1,
            evidence_identity: String::new(),
            reference_profile,
            cases,
        };
        envelope.validate_surface()?;
        envelope.evidence_identity = envelope.expected_identity()?;
        Ok(envelope)
    }

    pub fn validate(&self) -> Result<(), String> {
        self.validate_surface()?;
        if self.expected_identity()? != self.evidence_identity {
            return Err(
                "developer iteration envelope identity does not match its contents".to_owned(),
            );
        }
        Ok(())
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    pub fn cases(&self) -> &[DeveloperIterationCaseEvidence] {
        &self.cases
    }

    pub fn reference_profile(&self) -> &ReferenceDevelopmentProfile {
        &self.reference_profile
    }

    fn validate_surface(&self) -> Result<(), String> {
        if self.schema_version != 1 {
            return Err(format!(
                "unsupported developer iteration envelope schema: {}",
                self.schema_version
            ));
        }
        self.reference_profile.validate()?;
        let observed: BTreeSet<_> = self.cases.iter().map(|case| case.edit.case).collect();
        let expected: BTreeSet<_> = DeveloperEditCase::ALL.into_iter().collect();
        if observed != expected || observed.len() != self.cases.len() {
            return Err(format!(
                "developer iteration envelope is incomplete: expected {expected:?}, observed {observed:?}"
            ));
        }
        for case in &self.cases {
            case.validate()?;
        }
        Ok(())
    }

    fn expected_identity(&self) -> Result<String, String> {
        let mut basis = self.clone();
        basis.evidence_identity.clear();
        sha256_serialized(&basis)
    }
}

impl ReferenceDevelopmentProfile {
    fn validate(&self) -> Result<(), String> {
        if self.operating_system != "windows"
            || self.filesystem.trim().is_empty()
            || self.cpu.trim().is_empty()
            || self.storage_class.trim().is_empty()
            || self.antivirus_posture.trim().is_empty()
            || !self.rust_toolchain.contains("rustc")
            || !is_revision(&self.source_revision)
            || !is_sha256(&self.lockfile_sha256)
        {
            return Err(
                "reference development profile is incomplete or is not the declared Windows lane"
                    .to_owned(),
            );
        }
        Ok(())
    }
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn is_revision(value: &str) -> bool {
    matches!(value.len(), 40 | 64) && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}
