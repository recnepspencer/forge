use std::path::PathBuf;

use worth_store_offline_verifier::walk_current_durable_record_manifest;
use worth_store_physical_format::PhysicalRecordFormatDeclaration;

fn main() {
    let Some(root) = std::env::args_os().nth(1).map(PathBuf::from) else {
        eprintln!("usage: physical_store_offline_observer <store-root>");
        std::process::exit(2);
    };
    let format = PhysicalRecordFormatDeclaration::builder()
        .admit()
        .expect("the canonical v1 declaration is valid");
    match walk_current_durable_record_manifest(&root, format) {
        Ok(walk) => {
            println!("C5_OFFLINE_PROCESS {}", std::process::id());
            println!(
                "C5_OFFLINE {} {} {} {} {} {} {} {} {} {}",
                hex(&walk.store_identity()),
                walk.root_generation(),
                walk.placements().len(),
                walk.segment_pages().len(),
                walk.free_space().len(),
                walk.manifest_blocks(),
                walk.manifest_bytes(),
                walk.payload_frames(),
                walk.payload_bytes(),
                hex(&walk.payload_digest()),
            );
        }
        Err(denial) => {
            eprintln!("C5_OFFLINE_DENIED {denial:?}");
            std::process::exit(1);
        }
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
