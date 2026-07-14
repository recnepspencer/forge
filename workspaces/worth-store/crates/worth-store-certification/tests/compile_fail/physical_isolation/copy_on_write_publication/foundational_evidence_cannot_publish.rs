use worth_store_physical_isolation::{
    PhysicalPublicationFoundationalEvidence, ReadCopyUpdateRootPublication,
};

fn misuse(evidence: PhysicalPublicationFoundationalEvidence) {
    let _ = ReadCopyUpdateRootPublication::publish(evidence);
}

fn main() {}
