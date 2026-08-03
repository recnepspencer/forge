use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

const SOURCE_IDENTITY_DOMAIN: &[u8] = b"worth.store.physical.durability-source.v1";

fn main() {
    let manifest = PathBuf::from(
        std::env::var_os("CARGO_MANIFEST_DIR").expect("Cargo supplies CARGO_MANIFEST_DIR"),
    );
    let mut inputs = vec![manifest.join("Cargo.toml"), manifest.join("build.rs")];
    collect_files(&manifest.join("src"), &mut inputs);
    inputs.sort();

    let mut digest = Sha256::new();
    digest.update(SOURCE_IDENTITY_DOMAIN);
    for path in inputs {
        println!("cargo:rerun-if-changed={}", path.display());
        let relative = path
            .strip_prefix(&manifest)
            .expect("source input remains beneath its package root")
            .to_string_lossy()
            .replace('\\', "/");
        let bytes = fs::read(&path).expect("declared Store source input remains readable");
        digest.update((relative.len() as u64).to_le_bytes());
        digest.update(relative.as_bytes());
        digest.update((bytes.len() as u64).to_le_bytes());
        digest.update(bytes);
    }
    let bytes: [u8; 32] = digest.finalize().into();
    let generated = format!(
        "pub(super) const COMPILED_DURABILITY_SOURCE_IDENTITY: [u8; 32] = {:?};\n",
        bytes
    );
    let output = PathBuf::from(std::env::var_os("OUT_DIR").expect("Cargo supplies OUT_DIR"))
        .join("durability_source_identity.rs");
    fs::write(output, generated).expect("write generated Store source identity");
}

fn collect_files(directory: &Path, inputs: &mut Vec<PathBuf>) {
    let mut entries = fs::read_dir(directory)
        .expect("Store source directory remains readable")
        .map(|entry| entry.expect("Store source entry remains readable"))
        .collect::<Vec<_>>();
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let kind = entry
            .file_type()
            .expect("Store source entry type remains readable");
        if kind.is_dir() {
            collect_files(&entry.path(), inputs);
        } else if kind.is_file() {
            inputs.push(entry.path());
        } else {
            panic!(
                "Store source identity refuses non-file input {}",
                entry.path().display()
            );
        }
    }
}
