use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub(crate) const SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct CrateDagDocument {
    pub(crate) schema_version: u32,
    pub(crate) packages: Vec<DependencyRow>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct DependencyRow {
    pub(crate) package: String,
    pub(crate) dependencies: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct FacadeDocument {
    pub(crate) schema_version: u32,
    pub(crate) facades: Vec<FacadeRow>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct FacadeRow {
    pub(crate) package: String,
    pub(crate) exports: Vec<String>,
}

pub(crate) fn validate_rows<'a, I>(version: u32, rows: I) -> Result<(), String>
where
    I: IntoIterator<Item = (&'a str, &'a [String])>,
{
    if version != SCHEMA_VERSION {
        return Err(format!("unsupported schema_version {version}"));
    }
    let mut packages = BTreeSet::new();
    for (package, values) in rows {
        if !packages.insert(package) {
            return Err(format!("duplicate package row {package}"));
        }
        let mut seen = BTreeSet::new();
        for value in values {
            if !seen.insert(value) {
                return Err(format!("duplicate value {value} in {package}"));
            }
        }
        if values.windows(2).any(|pair| pair[0] >= pair[1]) {
            return Err(format!("values for {package} are not canonically sorted"));
        }
    }
    Ok(())
}
