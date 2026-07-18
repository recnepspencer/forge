use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

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
                    products.insert(format!("store-ci:{partition}"));
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
        let required = match self.mode {
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

#[derive(Debug)]
pub enum ProofProductUnavailable {
    ExplicitOwnerRequired,
    UnknownOwner(String),
    OwnerBoundaryViolation {
        owner: String,
        reached_target: String,
    },
    NamedProfileRequired(String),
    ExplicitSeedRequired(String),
    NamedBackendRequired(String),
    UnknownBackend {
        product: String,
        backend: String,
    },
    UnsupportedRequestOption {
        product: String,
        option: String,
    },
    UnknownProofProfile {
        product: String,
        profile: String,
    },
    MissingRequiredProofProduct(String),
    ScenarioTopology(String),
    UnsupportedHost {
        product: String,
        required: String,
        actual: String,
    },
    NoReachableProof {
        product: String,
    },
    RepositoryObservation(String),
}

impl fmt::Display for ProofProductUnavailable {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExplicitOwnerRequired => write!(formatter, "store-owner requires -p <package>"),
            Self::UnknownOwner(owner) => write!(formatter, "unknown owner package: {owner}"),
            Self::OwnerBoundaryViolation {
                owner,
                reached_target,
            } => write!(
                formatter,
                "store-owner:{owner} reached non-owner target {reached_target}"
            ),
            Self::NamedProfileRequired(product) => {
                write!(formatter, "{product} requires --profile <proof-profile>")
            }
            Self::ExplicitSeedRequired(product) => {
                write!(formatter, "{product} requires --seed <u64>")
            }
            Self::NamedBackendRequired(product) => {
                write!(formatter, "{product} requires --backend <backend-profile>")
            }
            Self::UnknownBackend { product, backend } => {
                write!(
                    formatter,
                    "{product} does not recognize backend {backend:?}"
                )
            }
            Self::UnsupportedRequestOption { product, option } => {
                write!(formatter, "{product} does not admit option {option}")
            }
            Self::UnknownProofProfile { product, profile } => {
                write!(
                    formatter,
                    "{product} does not recognize proof profile {profile:?}"
                )
            }
            Self::MissingRequiredProofProduct(product) => {
                write!(
                    formatter,
                    "required proof product has no reachable proof: {product}"
                )
            }
            Self::ScenarioTopology(reason) => {
                write!(formatter, "scenario topology is invalid: {reason}")
            }
            Self::UnsupportedHost {
                product,
                required,
                actual,
            } => write!(
                formatter,
                "{product} requires {required}; current host is {actual}"
            ),
            Self::NoReachableProof { product } => {
                write!(formatter, "{product} selects no reachable proof")
            }
            Self::RepositoryObservation(reason) => formatter.write_str(reason),
        }
    }
}

impl std::error::Error for ProofProductUnavailable {}
