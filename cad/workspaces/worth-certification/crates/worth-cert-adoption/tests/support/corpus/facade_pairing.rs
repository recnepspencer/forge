use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use super::Specimen;

pub fn validate(
    repository_root: &Path,
    specimen_directory: &Path,
    specimens: &[Specimen],
    omitted: &str,
    physical: &BTreeSet<String>,
) -> Result<(), String> {
    validate_unique_obligations(specimens)?;
    let remaining_obligations: BTreeSet<_> = specimens
        .iter()
        .filter(|row| row.path != omitted && physical.contains(row.path))
        .map(|row| row.obligation)
        .collect();
    for row in specimens {
        if !remaining_obligations.contains(row.obligation) {
            return Err(format!(
                "specimen {omitted} deletion leaves constitutional obligation unpaired: {}",
                row.obligation
            ));
        }
    }
    let expected = query_facade_exports(repository_root)?;
    let exercised = exercised_pairs(specimen_directory, specimens, omitted, physical)?;
    for pair in expected {
        if exercised.get(&pair) != Some(&1) {
            return Err(format!("specimen {omitted} deletion leaves facade export unpaired or multiply paired: {pair:?}"));
        }
    }
    Ok(())
}

fn validate_unique_obligations(specimens: &[Specimen]) -> Result<(), String> {
    let mut obligations = BTreeSet::new();
    for row in specimens {
        if row.obligation.trim().is_empty() {
            return Err(format!(
                "specimen {} has no constitutional obligation",
                row.path
            ));
        }
        if !obligations.insert(row.obligation) {
            return Err(format!(
                "constitutional obligation is multiply paired: {}",
                row.obligation
            ));
        }
    }
    Ok(())
}

fn query_facade_exports(repository_root: &Path) -> Result<BTreeSet<(String, String)>, String> {
    let document: toml::Value =
        fs::read_to_string(repository_root.join("tools/boundary-check/snapshots/facades.toml"))
            .map_err(|error| error.to_string())?
            .parse()
            .map_err(|error: toml::de::Error| error.to_string())?;
    let mut exports = BTreeSet::new();
    for facade in document["facades"].as_array().ok_or("missing facades")? {
        let package = facade["package"].as_str().unwrap_or_default();
        if matches!(
            package,
            "worth-query-decl" | "worth-query-host" | "worth-query-replay"
        ) {
            for export in facade["exports"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(toml::Value::as_str)
            {
                exports.insert((package.to_owned(), export.to_owned()));
            }
        }
    }
    Ok(exports)
}

fn exercised_pairs(
    specimen_directory: &Path,
    specimens: &[Specimen],
    omitted: &str,
    physical: &BTreeSet<String>,
) -> Result<BTreeMap<(String, String), usize>, String> {
    let mut counts = BTreeMap::new();
    for row in specimens
        .iter()
        .filter(|row| row.path != omitted && physical.contains(row.path))
    {
        let source = fs::read_to_string(specimen_directory.join(row.path))
            .map_err(|error| error.to_string())?;
        for pair in row.facade_pairs {
            if !source.contains(&pair.0.replace('-', "_")) || !source.contains(pair.1) {
                return Err(format!(
                    "specimen {} declares facade pair {pair:?} without exercising it",
                    row.path
                ));
            }
            *counts
                .entry((pair.0.to_owned(), pair.1.to_owned()))
                .or_insert(0) += 1;
        }
    }
    Ok(counts)
}
