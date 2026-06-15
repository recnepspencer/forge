use std::fs;
use std::path::{Path, PathBuf};

use worth_ui_validation_app::guards::{detect_forbidden_web_artifacts, ForbiddenWebArtifactKind};

#[test]
fn validation_app_workbench_cannot_be_implemented_through_web_tooling() {
    let app_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let artifacts = detect_forbidden_web_artifacts(&app_root);

    assert!(
        artifacts.is_empty(),
        "native validation app contains forbidden web artifacts: {artifacts:?}"
    );
}

#[test]
fn forbidden_web_artifact_guard_classifies_hostile_web_fixture() {
    let fixture = WebArtifactFixture::create("web-files");
    fixture.write("package.json", "{}");
    fixture.write("vite.config.ts", "");
    fixture.write("src/App.tsx", "");
    fixture.write("index.html", "");

    let artifacts = detect_forbidden_web_artifacts(fixture.root());
    let kinds = artifact_kinds(&artifacts);

    assert!(kinds.contains(&ForbiddenWebArtifactKind::PackageManifest));
    assert!(kinds.contains(&ForbiddenWebArtifactKind::ViteConfig));
    assert!(kinds.contains(&ForbiddenWebArtifactKind::ReactSource));
    assert!(kinds.contains(&ForbiddenWebArtifactKind::HtmlRoot));
}

#[test]
fn forbidden_web_artifact_guard_rejects_native_webview_dependencies() {
    let fixture = WebArtifactFixture::create("webview-dependencies");
    fixture.write(
        "Cargo.toml",
        r#"
            [package]
            name = "hostile-webview"
            version = "0.0.0"

            [dependencies]
            tauri = "2"
            wry = "0.40"
        "#,
    );

    let artifacts = detect_forbidden_web_artifacts(fixture.root());
    let kinds = artifact_kinds(&artifacts);

    assert!(kinds.contains(&ForbiddenWebArtifactKind::WebViewDependency));
}

struct WebArtifactFixture {
    root: PathBuf,
}

impl WebArtifactFixture {
    fn create(name: &'static str) -> Self {
        let root = std::env::temp_dir().join(format!(
            "worth-ui-validation-app-{name}-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).expect("fixture root should be writable");
        Self { root }
    }

    fn root(&self) -> &Path {
        &self.root
    }

    fn write(&self, relative_path: &str, text: &str) {
        let path = self.root.join(relative_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("fixture parent should be writable");
        }
        fs::write(path, text).expect("fixture file should write");
    }
}

impl Drop for WebArtifactFixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn artifact_kinds(
    artifacts: &[worth_ui_validation_app::guards::ForbiddenWebArtifact],
) -> Vec<ForbiddenWebArtifactKind> {
    artifacts.iter().map(|artifact| artifact.kind()).collect()
}
