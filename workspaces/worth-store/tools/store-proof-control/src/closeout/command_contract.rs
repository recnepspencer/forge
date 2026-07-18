use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct StableProofCommand {
    pub product: String,
    pub command: Vec<String>,
    pub selection_contract: String,
}

pub(super) fn validate_commands(commands: &[StableProofCommand]) -> Result<(), String> {
    let mut products = BTreeSet::new();
    for command in commands {
        if command.product.trim().is_empty()
            || command.command.first().map(String::as_str) != Some("cargo")
            || command.command.len() < 2
            || command.selection_contract.trim().is_empty()
            || !products.insert(command.product.as_str())
        {
            return Err(format!(
                "stable proof command is missing or duplicated: {:?}",
                command.product
            ));
        }
        let alias = command.product.as_str();
        if command.command.get(1).map(String::as_str) != Some(alias) {
            return Err(format!(
                "stable proof command {} does not enter through cargo {}",
                command.product, alias
            ));
        }
    }
    for required in REQUIRED_PRODUCTS {
        if !products.contains(required) {
            return Err(format!("closeout omits stable command contract {required}"));
        }
    }
    require_option(commands, "store-owner", "-p")?;
    require_option(commands, "store-soak", "--seed")?;
    require_option(commands, "store-soak", "--profile")?;
    require_option(commands, "store-release", "--backend")?;
    require_option(commands, "store-hardware", "--profile")?;
    Ok(())
}

fn require_option(
    commands: &[StableProofCommand],
    product: &str,
    option: &str,
) -> Result<(), String> {
    if commands
        .iter()
        .find(|command| command.product == product)
        .is_some_and(|command| command.command.iter().any(|argument| argument == option))
    {
        Ok(())
    } else {
        Err(format!("stable proof command {product} omits {option}"))
    }
}

const REQUIRED_PRODUCTS: &[&str] = &[
    "store-owner",
    "store-smoke",
    "store-ui",
    "store-ci",
    "store-soak",
    "store-release",
    "store-hardware",
];
