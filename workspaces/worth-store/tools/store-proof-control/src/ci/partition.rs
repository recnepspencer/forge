use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::ValidatedProofInventory;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum CiProofPartitionKind {
    OwnerUnit,
    ScenarioCertification,
    UiDependency,
    FreshProcess,
    StructuralPreflight,
    FormalExternal,
}

impl CiProofPartitionKind {
    pub const fn identity(self) -> &'static str {
        match self {
            Self::OwnerUnit => "owner-unit",
            Self::ScenarioCertification => "scenario-certification",
            Self::UiDependency => "ui-dependency",
            Self::FreshProcess => "fresh-process",
            Self::StructuralPreflight => "structural-preflight",
            Self::FormalExternal => "formal-external",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        ALL_PARTITIONS
            .into_iter()
            .find(|partition| partition.identity() == value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CiProofPartition {
    pub kind: CiProofPartitionKind,
    pub identity: String,
    pub products: BTreeSet<String>,
    pub required_operating_systems: BTreeSet<String>,
    pub structural_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct RequiredCiLane {
    pub partition: String,
    pub operating_system: String,
}

pub fn catalog(inventory: &ValidatedProofInventory) -> Vec<CiProofPartition> {
    ALL_PARTITIONS
        .into_iter()
        .map(|kind| CiProofPartition {
            kind,
            identity: kind.identity().to_owned(),
            products: products_for(kind, inventory),
            required_operating_systems: required_operating_systems(kind),
            structural_only: kind == CiProofPartitionKind::StructuralPreflight,
        })
        .collect()
}

pub fn partition_products(
    partition: &str,
    inventory: &ValidatedProofInventory,
) -> Option<BTreeSet<String>> {
    catalog(inventory)
        .into_iter()
        .find(|candidate| candidate.identity == partition)
        .map(|candidate| candidate.products)
}

pub fn required_lanes(inventory: &ValidatedProofInventory) -> BTreeSet<RequiredCiLane> {
    catalog(inventory)
        .into_iter()
        .flat_map(|partition| {
            partition
                .required_operating_systems
                .into_iter()
                .map(move |operating_system| RequiredCiLane {
                    partition: partition.identity.clone(),
                    operating_system,
                })
        })
        .collect()
}

fn products_for(
    kind: CiProofPartitionKind,
    inventory: &ValidatedProofInventory,
) -> BTreeSet<String> {
    let all: BTreeSet<_> = inventory
        .inventory()
        .proofs
        .iter()
        .flat_map(|proof| proof.products.iter())
        .filter(|product| product.starts_with("store-ci:"))
        .cloned()
        .collect();
    match kind {
        CiProofPartitionKind::ScenarioCertification => SCENARIO_PRODUCTS
            .into_iter()
            .filter(|product| all.contains(*product))
            .map(str::to_owned)
            .collect(),
        CiProofPartitionKind::UiDependency => BTreeSet::from(["store-ui".to_owned()]),
        CiProofPartitionKind::FreshProcess => {
            BTreeSet::from(["store-ci:physical-certification".to_owned()])
        }
        CiProofPartitionKind::FormalExternal => {
            BTreeSet::from(["store-ci:formal-conformance".to_owned()])
        }
        CiProofPartitionKind::StructuralPreflight => BTreeSet::new(),
        CiProofPartitionKind::OwnerUnit => {
            let assigned: BTreeSet<_> = SCENARIO_PRODUCTS
                .into_iter()
                .chain([
                    "store-ci:physical-certification",
                    "store-ci:formal-conformance",
                ])
                .collect();
            let mut products: BTreeSet<_> = all
                .into_iter()
                .filter(|product| !assigned.contains(product.as_str()))
                .collect();
            products.insert("store-ci:feature-compatibility".to_owned());
            products
        }
    }
}

fn required_operating_systems(kind: CiProofPartitionKind) -> BTreeSet<String> {
    let operating_systems = match kind {
        CiProofPartitionKind::ScenarioCertification
        | CiProofPartitionKind::UiDependency
        | CiProofPartitionKind::FreshProcess => &["linux", "windows"][..],
        CiProofPartitionKind::OwnerUnit
        | CiProofPartitionKind::StructuralPreflight
        | CiProofPartitionKind::FormalExternal => &["linux"][..],
    };
    operating_systems
        .iter()
        .map(|value| (*value).to_owned())
        .collect()
}

const ALL_PARTITIONS: [CiProofPartitionKind; 6] = [
    CiProofPartitionKind::OwnerUnit,
    CiProofPartitionKind::ScenarioCertification,
    CiProofPartitionKind::UiDependency,
    CiProofPartitionKind::FreshProcess,
    CiProofPartitionKind::StructuralPreflight,
    CiProofPartitionKind::FormalExternal,
];

const SCENARIO_PRODUCTS: [&str; 6] = [
    "store-ci:recovery",
    "store-ci:physical_isolation",
    "store-ci:scheduling",
    "store-ci:layout",
    "store-ci:blobs",
    "store-ci:security",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_matrix_keeps_windows_for_physical_and_compiler_claims() {
        assert_eq!(
            required_operating_systems(CiProofPartitionKind::FreshProcess),
            BTreeSet::from(["linux".to_owned(), "windows".to_owned()])
        );
        assert_eq!(
            required_operating_systems(CiProofPartitionKind::FormalExternal),
            BTreeSet::from(["linux".to_owned()])
        );
    }
}
