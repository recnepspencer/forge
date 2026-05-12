use forge_foundational::{AspectContractRevision, AspectIdentity, AspectKey, FieldKey};

pub fn key(name: &str) -> AspectKey {
    AspectKey::new(name).expect("valid aspect key")
}

pub fn field(name: &str) -> FieldKey {
    FieldKey::new(name).expect("valid field key")
}

pub fn revision(value: u64) -> AspectContractRevision {
    AspectContractRevision(value)
}

pub fn identity(value: u64) -> AspectIdentity {
    AspectIdentity(value)
}
