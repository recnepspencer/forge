use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::proof_unavailable::ProofProductUnavailable;
use crate::ValidatedProofInventory;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StoreProofMode {
    Owner,
    Smoke,
    Ui,
    Ci,
    Soak,
    Release,
    Hardware,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreProofRequest {
    mode: StoreProofMode,
    package: Option<String>,
    partition: Option<String>,
    proof_profile: Option<String>,
    scenario_identity: Option<String>,
    seed: Option<u64>,
    backend: Option<String>,
    shard_index: Option<usize>,
    shard_count: Option<usize>,
    plan_only: bool,
}

impl StoreProofRequest {
    pub fn new(
        mode: StoreProofMode,
        package: Option<String>,
        partition: Option<String>,
        proof_profile: Option<String>,
        scenario_identity: Option<String>,
        plan_only: bool,
    ) -> Self {
        Self {
            mode,
            package,
            partition,
            proof_profile,
            scenario_identity,
            seed: None,
            backend: None,
            shard_index: None,
            shard_count: None,
            plan_only,
        }
    }

    pub fn with_seed(mut self, seed: Option<u64>) -> Self {
        self.seed = seed;
        self
    }

    pub fn with_backend(mut self, backend: Option<String>) -> Self {
        self.backend = backend;
        self
    }

    pub fn with_shard(mut self, shard_index: Option<usize>, shard_count: Option<usize>) -> Self {
        self.shard_index = shard_index;
        self.shard_count = shard_count;
        self
    }

    pub const fn mode(&self) -> StoreProofMode {
        self.mode
    }

    pub fn package(&self) -> Option<&str> {
        self.package.as_deref()
    }

    pub fn partition(&self) -> Option<&str> {
        self.partition.as_deref()
    }

    pub fn proof_profile(&self) -> Option<&str> {
        self.proof_profile.as_deref()
    }

    pub const fn plan_only(&self) -> bool {
        self.plan_only
    }

    pub fn scenario_identity(&self) -> Option<&str> {
        self.scenario_identity.as_deref()
    }

    pub const fn seed(&self) -> Option<u64> {
        self.seed
    }

    pub fn backend(&self) -> Option<&str> {
        self.backend.as_deref()
    }

    pub const fn shard(&self) -> Option<(usize, usize)> {
        match (self.shard_index, self.shard_count) {
            (Some(index), Some(count)) => Some((index, count)),
            _ => None,
        }
    }

    pub fn semantic_ci_partition(&self) -> Option<crate::ci::CiProofPartitionKind> {
        (self.mode == StoreProofMode::Ci)
            .then(|| self.partition.as_deref())
            .flatten()
            .and_then(crate::ci::CiProofPartitionKind::parse)
    }

    pub fn display_name(&self) -> String {
        match self.mode {
            StoreProofMode::Owner => {
                format!("store-owner:{}", self.package.as_deref().unwrap_or("?"))
            }
            StoreProofMode::Ci => self.partition.as_ref().map_or_else(
                || "store-ci".to_owned(),
                |value| format!("store-ci:{value}"),
            ),
            StoreProofMode::Smoke => "store-smoke".to_owned(),
            StoreProofMode::Ui => "store-ui".to_owned(),
            StoreProofMode::Soak => "store-soak".to_owned(),
            StoreProofMode::Release => "store-release".to_owned(),
            StoreProofMode::Hardware => "store-hardware".to_owned(),
        }
    }

    pub(crate) fn selected_product_names(
        &self,
        inventory: &ValidatedProofInventory,
    ) -> Result<BTreeSet<String>, ProofProductUnavailable> {
        self.validate_option_ownership()?;
        let mut products = BTreeSet::new();
        match self.mode {
            StoreProofMode::Owner => {
                let package = self
                    .package
                    .as_ref()
                    .ok_or(ProofProductUnavailable::ExplicitOwnerRequired)?;
                if !inventory
                    .inventory()
                    .discovered
                    .packages
                    .iter()
                    .any(|candidate| &candidate.name == package)
                {
                    return Err(ProofProductUnavailable::UnknownOwner(package.clone()));
                }
                products.insert(format!("store-owner:{package}"));
            }
            StoreProofMode::Smoke => {
                products.insert("store-smoke".to_owned());
            }
            StoreProofMode::Ui => {
                products.insert("store-ui".to_owned());
            }
            StoreProofMode::Ci => {
                if let Some(partition) = &self.partition {
                    if let Some(partition_products) =
                        crate::ci::partition_products(partition, inventory)
                    {
                        products.extend(partition_products);
                    } else {
                        products.insert(format!("store-ci:{partition}"));
                    }
                } else {
                    products.extend(ci_products(inventory));
                }
            }
            StoreProofMode::Soak => {
                require_registered_profile(self, SOAK_PROFILES)?;
                if self.seed.is_none() {
                    return Err(ProofProductUnavailable::ExplicitSeedRequired(
                        self.display_name(),
                    ));
                }
                products.insert("store-soak".to_owned());
            }
            StoreProofMode::Release => {
                let backend = self.backend.as_deref().ok_or_else(|| {
                    ProofProductUnavailable::NamedBackendRequired(self.display_name())
                })?;
                if !RELEASE_BACKENDS.contains(&backend) {
                    return Err(ProofProductUnavailable::UnknownBackend {
                        product: self.display_name(),
                        backend: backend.to_owned(),
                    });
                }
                products.extend(ci_products(inventory));
            }
            StoreProofMode::Hardware => {
                require_registered_profile(self, HARDWARE_PROFILES)?;
                products.extend(ci_products(inventory));
            }
        }
        Ok(products)
    }

    pub(crate) fn validate_host(&self) -> Result<(), ProofProductUnavailable> {
        let required = if self.semantic_ci_partition()
            == Some(crate::ci::CiProofPartitionKind::FormalExternal)
        {
            "linux"
        } else {
            match self.mode {
                StoreProofMode::Hardware => {
                    let profile = self.proof_profile.as_deref().unwrap_or_default();
                    hardware_profile_host(profile).ok_or_else(|| {
                        ProofProductUnavailable::UnknownProofProfile {
                            product: self.display_name(),
                            profile: profile.to_owned(),
                        }
                    })?
                }
                StoreProofMode::Release => release_backend_host(
                    self.backend.as_deref().unwrap_or_default(),
                )
                .ok_or_else(|| ProofProductUnavailable::UnknownBackend {
                    product: self.display_name(),
                    backend: self.backend.clone().unwrap_or_default(),
                })?,
                _ => return Ok(()),
            }
        };
        if required != std::env::consts::OS {
            return Err(ProofProductUnavailable::UnsupportedHost {
                product: self.display_name(),
                required: required.to_owned(),
                actual: std::env::consts::OS.to_owned(),
            });
        }
        Ok(())
    }

    fn validate_option_ownership(&self) -> Result<(), ProofProductUnavailable> {
        if self.seed.is_some() && self.mode != StoreProofMode::Soak {
            return Err(ProofProductUnavailable::UnsupportedRequestOption {
                product: self.display_name(),
                option: "--seed".to_owned(),
            });
        }
        if self.backend.is_some() && self.mode != StoreProofMode::Release {
            return Err(ProofProductUnavailable::UnsupportedRequestOption {
                product: self.display_name(),
                option: "--backend".to_owned(),
            });
        }
        if self.proof_profile.is_some()
            && !matches!(self.mode, StoreProofMode::Soak | StoreProofMode::Hardware)
        {
            return Err(ProofProductUnavailable::UnsupportedRequestOption {
                product: self.display_name(),
                option: "--profile".to_owned(),
            });
        }
        match (self.shard_index, self.shard_count) {
            (None, None) => {}
            (Some(index), Some(count))
                if self.mode == StoreProofMode::Ci
                    && self.semantic_ci_partition().is_some_and(|partition| {
                        partition != crate::ci::CiProofPartitionKind::StructuralPreflight
                    })
                    && count > 0
                    && index < count => {}
            (Some(_), Some(_)) => {
                return Err(ProofProductUnavailable::UnsupportedRequestOption {
                    product: self.display_name(),
                    option: "invalid --shard-index/--shard-count selection".to_owned(),
                })
            }
            _ => {
                return Err(ProofProductUnavailable::UnsupportedRequestOption {
                    product: self.display_name(),
                    option: "--shard-index and --shard-count must be provided together".to_owned(),
                })
            }
        }
        Ok(())
    }
}

fn hardware_profile_host(profile: &str) -> Option<&'static str> {
    match profile {
        "windows-ntfs-nvme" => Some("windows"),
        "linux-ext4-nvme" => Some("linux"),
        "macos-apfs-nvme" => Some("macos"),
        _ => None,
    }
}

fn release_backend_host(backend: &str) -> Option<&'static str> {
    match backend {
        "windows-file" => Some("windows"),
        "linux-file" => Some("linux"),
        "macos-file" => Some("macos"),
        _ => None,
    }
}

fn require_registered_profile(
    request: &StoreProofRequest,
    admitted: &[&str],
) -> Result<(), ProofProductUnavailable> {
    let profile = request
        .proof_profile
        .as_deref()
        .ok_or_else(|| ProofProductUnavailable::NamedProfileRequired(request.display_name()))?;
    if admitted.contains(&profile) {
        Ok(())
    } else {
        Err(ProofProductUnavailable::UnknownProofProfile {
            product: request.display_name(),
            profile: profile.to_owned(),
        })
    }
}

const SOAK_PROFILES: &[&str] = &["checkpoint-heavy"];
const HARDWARE_PROFILES: &[&str] = &["windows-ntfs-nvme", "linux-ext4-nvme", "macos-apfs-nvme"];
const RELEASE_BACKENDS: &[&str] = &["windows-file", "linux-file", "macos-file"];

fn ci_products(inventory: &ValidatedProofInventory) -> BTreeSet<String> {
    let mut products: BTreeSet<_> = inventory
        .inventory()
        .proofs
        .iter()
        .flat_map(|proof| proof.products.iter())
        .filter(|product| product.starts_with("store-ci:"))
        .cloned()
        .collect();
    products.insert("store-ci:feature-compatibility".to_owned());
    products.insert("store-ui".to_owned());
    products.extend(REQUIRED_CORE_CI_PRODUCTS.map(str::to_owned));
    products
}

const REQUIRED_CORE_CI_PRODUCTS: [&str; 10] = [
    "store-ci:recovery",
    "store-ci:physical_isolation",
    "store-ci:scheduling",
    "store-ci:layout",
    "store-ci:blobs",
    "store-ci:security",
    "store-ci:certification-owner",
    "store-ci:physical-certification",
    "store-ci:formal-conformance",
    "store-ci:test-control",
];
