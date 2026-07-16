use super::{
    durability_bindings, recovery_bindings, replication_bindings, storage_bindings, trust_bindings,
    ProtocolBindingManifest, ProtocolFamily,
};

pub fn current_protocol_binding_manifest() -> ProtocolBindingManifest {
    let bindings = durability_bindings::current()
        .into_iter()
        .chain(recovery_bindings::current())
        .chain(replication_bindings::current())
        .chain(storage_bindings::current())
        .chain(trust_bindings::current())
        .collect();
    let gaps = Vec::new();

    ProtocolBindingManifest {
        bindings,
        gaps,
        composed_protocols: vec![ProtocolFamily::SharedFrontiers],
    }
}
