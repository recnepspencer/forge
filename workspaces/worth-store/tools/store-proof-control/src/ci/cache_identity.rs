use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::evidence::{sha256_file, sha256_serialized};
use crate::selection::SelectedProofExecutionPlan;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CiCacheIdentity {
    pub identity: String,
    pub operating_system: String,
    pub architecture: String,
    pub rustc_identity: String,
    pub profile: Vec<String>,
    pub feature_lanes: Vec<String>,
    pub lockfile_digest: String,
    pub workspace_manifest_digest: String,
    pub cargo_config_digest: String,
    pub partition: String,
}

impl CiCacheIdentity {
    pub fn from_plan(
        workspace_root: &Path,
        partition: &str,
        plan: &SelectedProofExecutionPlan,
    ) -> Result<Self, String> {
        let mut profile: Vec<_> = plan
            .units
            .iter()
            .map(|unit| unit.build_profile.cargo_profile().to_owned())
            .collect();
        profile.sort();
        profile.dedup();
        if profile.is_empty() {
            profile.push("ci-test".to_owned());
        }
        let mut feature_lanes: Vec<_> = plan
            .units
            .iter()
            .map(|unit| unit.feature_lane.description())
            .collect();
        feature_lanes.sort();
        feature_lanes.dedup();
        let mut identity = Self {
            identity: String::new(),
            operating_system: plan.repository.operating_system.clone(),
            architecture: plan.repository.architecture.clone(),
            rustc_identity: plan.repository.rustc_identity.clone(),
            profile,
            feature_lanes,
            lockfile_digest: plan.repository.lockfile_digest.clone(),
            workspace_manifest_digest: sha256_file(&workspace_root.join("Cargo.toml"))?,
            cargo_config_digest: sha256_file(&workspace_root.join(".cargo/config.toml"))?,
            partition: partition.to_owned(),
        };
        identity.identity = sha256_serialized(&identity)?;
        Ok(identity)
    }

    pub fn validate(&self) -> Result<(), String> {
        let mut basis = self.clone();
        basis.identity.clear();
        if sha256_serialized(&basis)? == self.identity {
            Ok(())
        } else {
            Err("CI cache identity does not match its toolchain/profile/feature inputs".to_owned())
        }
    }
}
