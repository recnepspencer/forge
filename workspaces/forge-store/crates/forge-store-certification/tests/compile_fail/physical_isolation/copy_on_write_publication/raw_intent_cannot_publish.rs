use forge_store_physical_isolation::{
    PhysicalPublicationIntent, ReadCopyUpdateRootPublication,
};

fn misuse(intent: PhysicalPublicationIntent) {
    let _ = ReadCopyUpdateRootPublication::publish(intent);
}

fn main() {}
