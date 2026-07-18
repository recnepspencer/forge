use std::path::Path;

use super::CiPartitionEvidence;

pub fn read_partition_evidence(root: &Path) -> Result<Vec<CiPartitionEvidence>, String> {
    if !root.is_dir() {
        return Err(format!(
            "CI evidence root is not a directory: {}",
            root.display()
        ));
    }
    let mut pending = vec![root.to_path_buf()];
    let mut paths = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(&directory)
            .map_err(|error| format!("could not inspect {}: {error}", directory.display()))?
        {
            let entry = entry.map_err(|error| {
                format!(
                    "could not inspect entry under {}: {error}",
                    directory.display()
                )
            })?;
            let file_type = entry.file_type().map_err(|error| {
                format!("could not classify {}: {error}", entry.path().display())
            })?;
            if file_type.is_symlink() {
                return Err(format!(
                    "CI evidence traversal denies symlink {}",
                    entry.path().display()
                ));
            }
            if file_type.is_dir() {
                pending.push(entry.path());
            } else if file_type.is_file()
                && entry.path().extension().and_then(|value| value.to_str()) == Some("json")
            {
                paths.push(entry.path());
            }
        }
    }
    paths.sort();
    paths
        .into_iter()
        .map(|path| crate::evidence::read_json(&path))
        .collect()
}
