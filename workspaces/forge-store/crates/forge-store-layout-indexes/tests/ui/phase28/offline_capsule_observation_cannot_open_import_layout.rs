use forge_store_offline_verifier::OfflineCustodyCapsuleObservation;
use forge_store_operations::ImportLayoutEvidenceReport;

fn main() {
    let observation: OfflineCustodyCapsuleObservation = todo!();
    let _ = ImportLayoutEvidenceReport::from_readmitted_blob_import(&observation);
}
