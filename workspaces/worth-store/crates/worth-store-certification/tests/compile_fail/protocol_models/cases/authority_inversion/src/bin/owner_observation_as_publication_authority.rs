use worth_store_operations::complete_import_publication;
use worth_store_physical_isolation::CompactionOwnerCaseObservation;

fn attempt(observation: CompactionOwnerCaseObservation) {
    let _ = complete_import_publication(observation, observation);
}

fn main() {}
