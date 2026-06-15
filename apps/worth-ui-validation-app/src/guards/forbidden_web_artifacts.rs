use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForbiddenWebArtifact {
    path: PathBuf,
    kind: ForbiddenWebArtifactKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForbiddenWebArtifactKind {
    PackageManifest,
    ViteConfig,
    ReactSource,
    HtmlRoot,
    WebViewDependency,
}

impl ForbiddenWebArtifact {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn kind(&self) -> ForbiddenWebArtifactKind {
        self.kind
    }
}

pub fn detect_forbidden_web_artifacts(root: &Path) -> Vec<ForbiddenWebArtifact> {
    let mut artifacts = Vec::new();
    collect_forbidden_web_artifacts(root, &mut artifacts);
    artifacts
}

fn collect_forbidden_web_artifacts(path: &Path, artifacts: &mut Vec<ForbiddenWebArtifact>) {
    let Ok(metadata) = fs::metadata(path) else {
        return;
    };
    if metadata.is_dir() {
        collect_directory(path, artifacts);
    } else if let Some(kind) = classify_path(path) {
        artifacts.push(ForbiddenWebArtifact {
            path: path.to_path_buf(),
            kind,
        });
    }
}

fn collect_directory(path: &Path, artifacts: &mut Vec<ForbiddenWebArtifact>) {
    let Ok(entries) = fs::read_dir(path) else {
        return;
    };
    for entry in entries.flatten() {
        collect_forbidden_web_artifacts(&entry.path(), artifacts);
    }
}

fn classify_path(path: &Path) -> Option<ForbiddenWebArtifactKind> {
    let file_name = path.file_name()?.to_string_lossy();
    let extension = path
        .extension()
        .map(|extension| extension.to_string_lossy());
    if file_name == "package.json" {
        return Some(ForbiddenWebArtifactKind::PackageManifest);
    }
    if file_name.starts_with("vite.config.") {
        return Some(ForbiddenWebArtifactKind::ViteConfig);
    }
    if file_name == "index.html" {
        return Some(ForbiddenWebArtifactKind::HtmlRoot);
    }
    if matches!(extension.as_deref(), Some("tsx" | "jsx")) {
        return Some(ForbiddenWebArtifactKind::ReactSource);
    }
    if file_name == "Cargo.toml" && cargo_mentions_webview(path) {
        return Some(ForbiddenWebArtifactKind::WebViewDependency);
    }
    None
}

fn cargo_mentions_webview(path: &Path) -> bool {
    fs::read_to_string(path)
        .map(|text| text.contains("web-view") || text.contains("wry") || text.contains("tauri"))
        .unwrap_or(false)
}
