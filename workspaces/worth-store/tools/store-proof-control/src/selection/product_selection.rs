use std::collections::{BTreeMap, BTreeSet};

use super::{ProofExecutionUnit, ProofProductUnavailable};
use crate::ValidatedProofInventory;

pub(super) fn validate_selected_product_reachability(
    inventory: &ValidatedProofInventory,
    products: &BTreeSet<String>,
) -> Result<(), ProofProductUnavailable> {
    for product in products {
        if product == "store-ci:feature-compatibility" {
            continue;
        }
        if !inventory
            .inventory()
            .proofs
            .iter()
            .any(|proof| proof.products.contains(product))
        {
            return Err(ProofProductUnavailable::MissingRequiredProofProduct(
                product.clone(),
            ));
        }
    }
    Ok(())
}

pub(super) fn feature_compatibility_units(
    inventory: &ValidatedProofInventory,
) -> Vec<ProofExecutionUnit> {
    inventory
        .inventory()
        .discovered
        .packages
        .iter()
        .filter(|package| package.name.starts_with("worth-store"))
        .flat_map(|package| {
            let production =
                ProofExecutionUnit::feature_compatibility(package.name.clone(), Vec::new());
            let named = package
                .features
                .iter()
                .filter(|feature| *feature != "default")
                .map(|feature| {
                    ProofExecutionUnit::feature_compatibility(
                        package.name.clone(),
                        vec![feature.clone()],
                    )
                });
            std::iter::once(production).chain(named).collect::<Vec<_>>()
        })
        .collect()
}

pub(super) fn excluded_products(included: &BTreeSet<String>) -> BTreeMap<String, String> {
    [
        "store-owner",
        "store-smoke",
        "store-ui",
        "store-ci",
        "store-soak",
        "store-release",
        "store-hardware",
    ]
    .into_iter()
    .filter(|product| {
        !included
            .iter()
            .any(|included| included.starts_with(product))
    })
    .map(|product| {
        (
            product.to_owned(),
            "outside the explicitly requested proof product".to_owned(),
        )
    })
    .collect()
}
