use std::path::{Path, PathBuf};

use syn::visit::Visit;

#[test]
fn consumer_raster_authority_mutants_are_rejected() {
    for crate_name in [
        "worth-ui-host-headless",
        "worth-ui-host-native",
        "worth-ui-host-contract",
    ] {
        for path in rust_sources(&crate_root().join(crate_name).join("src")) {
            let source = std::fs::read_to_string(&path).expect("consumer source is readable");
            assert!(
                consumer_authority_violation(&source).is_none(),
                "{} exposes consumer raster authority",
                path.display()
            );
        }
    }
    for mutant in [
        "fn reshape_layout() {}",
        "fn refallback() {}",
        "fn rebreak_line() {}",
        "const FONT: &str = \"C:/Windows/Fonts/arial.ttf\";",
    ] {
        assert!(consumer_authority_violation(mutant).is_some());
    }
}

fn consumer_authority_violation(source: &str) -> Option<&'static str> {
    if source.contains("C:/Windows/Fonts") {
        return Some("ambient-system-font");
    }
    let file = syn::parse_file(source).ok()?;
    let mut finder = ForbiddenFinder::default();
    finder.visit_file(&file);
    finder.found
}

#[derive(Default)]
struct ForbiddenFinder {
    found: Option<&'static str>,
}

impl Visit<'_> for ForbiddenFinder {
    fn visit_ident(&mut self, ident: &syn::Ident) {
        self.found = self.found.or_else(|| {
            ["reshape_layout", "refallback", "rebreak_line"]
                .into_iter()
                .find(|name| ident == name)
        });
    }
}

fn rust_sources(root: &Path) -> Vec<PathBuf> {
    let mut pending = vec![root.to_path_buf()];
    let mut sources = Vec::new();
    while let Some(path) = pending.pop() {
        if path.is_dir() {
            pending.extend(
                std::fs::read_dir(path)
                    .expect("consumer directory is readable")
                    .map(|entry| entry.expect("consumer entry is readable").path()),
            );
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            sources.push(path);
        }
    }
    sources
}

fn crate_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace crates directory")
        .to_path_buf()
}
