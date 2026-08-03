use std::path::Path;

use worth_store_offline_verifier::{
    observe_hostile_physical_truth, OfflineHostilePhysicalTruthBudget,
    OfflineHostilePhysicalTruthObservation,
};
use worth_store_physical_format::PhysicalRecordFormatDeclaration;

const MAX_ARTIFACTS: usize = 4_096;
const MAX_TOTAL_BYTES: u64 = 1024 * 1024 * 1024;
const MUTATION_OBSERVATION_PREFIX_BYTES: u64 = 512;

pub(super) fn run(root: &Path) {
    let observation = match observe(root) {
        Ok(observation) => observation,
        Err(denial) => {
            eprintln!("C5_1_OFFLINE_DENIED {denial}");
            std::process::exit(1);
        }
    };
    emit(&observation);
}

pub(super) fn observe(root: &Path) -> Result<OfflineHostilePhysicalTruthObservation, String> {
    let format = PhysicalRecordFormatDeclaration::builder()
        .admit()
        .expect("the canonical v1 declaration is valid");
    let budget = OfflineHostilePhysicalTruthBudget::new(
        MAX_ARTIFACTS,
        MAX_TOTAL_BYTES,
        MUTATION_OBSERVATION_PREFIX_BYTES,
    )
    .expect("fixed observer budget is valid");
    observe_hostile_physical_truth(root, format, budget).map_err(|denial| format!("{denial:?}"))
}

pub(super) fn emit(observation: &OfflineHostilePhysicalTruthObservation) {
    println!("C5_1_OFFLINE_PROCESS {}", std::process::id());
    match observation.current() {
        Ok(current) => println!(
            "C5_1_OFFLINE_CURRENT accepted {} {} {} {} {}",
            super::hex(&current.store_identity()),
            current.root_generation(),
            current.records(),
            current.payload_bytes(),
            super::hex(&current.payload_digest()),
        ),
        Err(denial) => println!("C5_1_OFFLINE_CURRENT denied {denial:?}"),
    }
    for artifact in observation.artifacts() {
        println!(
            "C5_1_OFFLINE_ARTIFACT {} {} {} {} {}",
            super::hex(artifact.path().as_bytes()),
            artifact.byte_length(),
            super::hex(&artifact.digest()),
            super::hex(artifact.prefix()),
            artifact.is_recovery_obligation(),
        );
    }
    println!(
        "C5_1_OFFLINE_SUMMARY {} {} {}",
        observation.artifacts().len(),
        observation.total_bytes(),
        observation.recovery_obligations(),
    );
}
