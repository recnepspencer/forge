use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use worth_store::physical_runtime::PhysicalRecordInitialization;

use super::super::durability;
use super::{configuration, media, success};

const SOURCE_IDENTITY_DOMAIN: &[u8] = b"worth.store.physical.durability-source.v1";

#[test]
fn compiled_closeout_identity_covers_the_complete_store_source_tree() {
    let parent = tempfile::tempdir().unwrap();
    let media = media(&parent.path().join("store"));
    let policy = durability(&media);
    let (format, placement, access) = configuration();
    let serving = success(
        media.initialize_record_store(PhysicalRecordInitialization::new(
            format, placement, access, policy,
        )),
    );
    let actual = serving
        .close()
        .durability_closeout()
        .recovery_handoff()
        .expect("clean close produces one recovery handoff")
        .source_profile_identity()
        .source()
        .bytes();
    let expected = independent_source_identity(Path::new(env!("CARGO_MANIFEST_DIR")));
    if actual != expected {
        panic!("MUTANT_PREDICATE:compiled-source-identity-omits-source-tree");
    }
}

fn independent_source_identity(manifest: &Path) -> [u8; 32] {
    let mut inputs = vec![manifest.join("Cargo.toml"), manifest.join("build.rs")];
    collect_files(&manifest.join("src"), &mut inputs);
    inputs.sort();

    let mut digest = Sha256::new();
    digest.update(SOURCE_IDENTITY_DOMAIN);
    for path in inputs {
        let relative = path
            .strip_prefix(manifest)
            .unwrap()
            .to_string_lossy()
            .replace('\\', "/");
        let bytes = std::fs::read(&path).unwrap();
        digest.update((relative.len() as u64).to_le_bytes());
        digest.update(relative.as_bytes());
        digest.update((bytes.len() as u64).to_le_bytes());
        digest.update(bytes);
    }
    digest.finalize().into()
}

fn collect_files(directory: &Path, inputs: &mut Vec<PathBuf>) {
    let mut entries = std::fs::read_dir(directory)
        .unwrap()
        .map(|entry| entry.unwrap())
        .collect::<Vec<_>>();
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        if entry.file_type().unwrap().is_dir() {
            collect_files(&entry.path(), inputs);
        } else {
            inputs.push(entry.path());
        }
    }
}
