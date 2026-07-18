use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::evidence::sha256_serialized;

use super::BuildArtifactClass;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BuildArtifactRetentionPolicy {
    schema_version: u32,
    policy_identity: String,
    policy_name: String,
    removable_classes: BTreeSet<BuildArtifactClass>,
    requires_reuse_basis: bool,
}

impl BuildArtifactRetentionPolicy {
    pub fn bounded_local() -> Result<Self, String> {
        let mut policy = Self {
            schema_version: 1,
            policy_identity: String::new(),
            policy_name: "bounded-local-derived-residue".to_owned(),
            removable_classes: BTreeSet::from([
                BuildArtifactClass::IncrementalState,
                BuildArtifactClass::StaleHashedVariant,
                BuildArtifactClass::Symbol,
            ]),
            requires_reuse_basis: true,
        };
        policy.policy_identity = sha256_serialized(&policy)?;
        Ok(policy)
    }

    pub fn validate_integrity(&self) -> Result<(), String> {
        if self.schema_version != 1 {
            return Err(format!(
                "unsupported artifact retention policy schema {}",
                self.schema_version
            ));
        }
        let mut basis = self.clone();
        basis.policy_identity.clear();
        if sha256_serialized(&basis)? != self.policy_identity {
            return Err(
                "artifact retention policy identity does not match its contents".to_owned(),
            );
        }
        Ok(())
    }

    pub fn removes(&self, class: BuildArtifactClass) -> bool {
        self.removable_classes.contains(&class)
    }

    pub const fn requires_reuse_basis(&self) -> bool {
        self.requires_reuse_basis
    }

    pub fn policy_name(&self) -> &str {
        &self.policy_name
    }
}
