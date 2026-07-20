use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

fn main() {
    let mut files = vec![PathBuf::from("Cargo.toml"), PathBuf::from("build.rs")];
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=build.rs");
    let media_sources = Path::new("src/filesystem_media");
    println!("cargo:rerun-if-changed={}", media_sources.display());
    collect_rust_sources(media_sources, &mut files);
    files.sort();
    let mut digest = Sha256::new();
    for path in files {
        let bytes = std::fs::read(&path).expect("filesystem media source must be readable");
        digest.update(path.to_string_lossy().as_bytes());
        digest.update((bytes.len() as u64).to_le_bytes());
        digest.update(bytes);
    }
    for variable in [
        "TARGET",
        "PROFILE",
        "OPT_LEVEL",
        "DEBUG",
        "CARGO_CFG_TARGET_FEATURE",
        "CARGO_FEATURE_CERTIFICATION_TEST_AUTHORITY",
    ] {
        let value = std::env::var(variable).unwrap_or_default();
        digest.update(variable.as_bytes());
        digest.update((value.len() as u64).to_le_bytes());
        digest.update(value.as_bytes());
    }
    println!(
        "cargo:rustc-env=WORTH_STORE_MEDIA_BUILD_ID={:x}",
        digest.finalize()
    );
}

fn collect_rust_sources(directory: &Path, files: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(directory).expect("filesystem media directory must exist") {
        let path = entry
            .expect("filesystem media entry must be readable")
            .path();
        if path.is_dir() {
            if path.file_name().and_then(|value| value.to_str()) != Some("tests") {
                collect_rust_sources(&path, files);
            }
        } else if path.extension().and_then(|value| value.to_str()) == Some("rs") {
            files.push(path);
        }
    }
}
