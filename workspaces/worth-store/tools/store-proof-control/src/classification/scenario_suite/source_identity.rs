use std::path::{Path, PathBuf};

pub(super) fn repository_relative(workspace_root: &Path, source_path: &Path) -> String {
    let root = canonical(workspace_root);
    let source = canonical(source_path);
    source
        .strip_prefix(&root)
        .unwrap_or(&source)
        .to_string_lossy()
        .trim_start_matches(['/', '\\'])
        .replace('\\', "/")
}

fn canonical(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::repository_relative;

    #[test]
    fn source_identity_does_not_embed_checkout_location() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let source = root.join("Cargo.toml");
        assert_eq!(repository_relative(root, &source), "Cargo.toml");
    }
}
