use super::{
    AdmittedWalArtifactStore, WalArtifactScanCounters, WalArtifactStoreDenial,
    WalPersistedArtifact, WalPersistedArtifactSet,
};
use std::path::Path;

pub(super) fn scan(
    store: &AdmittedWalArtifactStore,
) -> Result<WalPersistedArtifactSet, WalArtifactStoreDenial> {
    let mut artifacts = Vec::new();
    let mut counters = WalArtifactScanCounters::default();
    for entry in std::fs::read_dir(&store.identity.root).map_err(|_| WalArtifactStoreDenial::Io)? {
        let entry = entry.map_err(|_| WalArtifactStoreDenial::Io)?;
        let file_type = entry.file_type().map_err(|_| WalArtifactStoreDenial::Io)?;
        if !file_type.is_dir()
            || !entry
                .file_name()
                .to_string_lossy()
                .starts_with("worth-store-durability-")
        {
            continue;
        }
        counters.directories_examined = counters.directories_examined.saturating_add(1);
        for candidate in std::fs::read_dir(entry.path()).map_err(|_| WalArtifactStoreDenial::Io)? {
            let candidate = candidate.map_err(|_| WalArtifactStoreDenial::Io)?;
            if !candidate
                .file_type()
                .map_err(|_| WalArtifactStoreDenial::Io)?
                .is_file()
                || !matches!(candidate.file_name().to_str(), Some("staged" | "published"))
            {
                continue;
            }
            let path =
                std::fs::canonicalize(candidate.path()).map_err(|_| WalArtifactStoreDenial::Io)?;
            if !is_durability_artifact(&store.identity.root, &path) {
                return Err(WalArtifactStoreDenial::StoreBindingMismatch);
            }
            let bytes = std::fs::read(&path).map_err(|_| WalArtifactStoreDenial::Io)?;
            counters.artifacts_read = counters.artifacts_read.saturating_add(1);
            counters.bytes_read = counters.bytes_read.saturating_add(bytes.len() as u64);
            artifacts.push(WalPersistedArtifact { path, bytes });
        }
    }
    artifacts.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(WalPersistedArtifactSet {
        store: store.identity.clone(),
        artifacts,
        counters,
    })
}

pub(super) fn is_durability_artifact(root: &Path, artifact: &Path) -> bool {
    let Some(directory) = artifact.parent() else {
        return false;
    };
    directory.parent() == Some(root)
        && directory.file_name().is_some_and(|name| {
            name.to_string_lossy()
                .starts_with("worth-store-durability-")
        })
        && matches!(
            artifact.file_name().and_then(|name| name.to_str()),
            Some("staged" | "published")
        )
}
