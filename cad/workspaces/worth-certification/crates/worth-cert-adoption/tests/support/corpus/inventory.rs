use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use super::Specimen;

pub fn exact_inventory(
    directory: &Path,
    specimens: &[Specimen],
    omitted: &str,
) -> Result<BTreeSet<String>, String> {
    let registered = catalog_files_without(specimens, omitted);
    let actual = fs::read_dir(directory)
        .map_err(|error| error.to_string())?
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let path = entry.path();
            (path.extension().and_then(|value| value.to_str()) == Some("rs"))
                .then(|| entry.file_name().to_string_lossy().into_owned())
        })
        .collect();
    if actual == registered {
        Ok(actual)
    } else {
        Err(format!("specimen inventory mismatch; omitted or unregistered: {omitted}; registered={registered:?}, actual={actual:?}"))
    }
}

pub fn catalog_files_without(specimens: &[Specimen], omitted: &str) -> BTreeSet<String> {
    specimens
        .iter()
        .filter(|row| row.path != omitted)
        .map(|row| row.path.to_owned())
        .collect()
}

pub fn validate_registered_deletion(specimens: &[Specimen], omitted: &str) -> Result<(), String> {
    let registered: BTreeSet<_> = specimens.iter().map(|row| row.path.to_owned()).collect();
    let physical = catalog_files_without(specimens, omitted);
    if registered == physical {
        Ok(())
    } else {
        Err(format!(
            "specimen inventory mismatch after deleting {omitted}"
        ))
    }
}
