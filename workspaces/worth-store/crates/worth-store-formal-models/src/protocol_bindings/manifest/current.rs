use super::{
    durability_bindings, recovery_bindings, replication_bindings, storage_bindings, trust_bindings,
    ProtocolBindingManifest, ProtocolFamily,
};
use crate::protocol_bindings::{OwnerBoundaryGap, OwnerBoundaryGapKind};

pub fn current_protocol_binding_manifest() -> ProtocolBindingManifest {
    let bindings = durability_bindings::current()
        .into_iter()
        .chain(recovery_bindings::current())
        .chain(replication_bindings::current())
        .chain(storage_bindings::current())
        .chain(trust_bindings::current())
        .collect();
    let gaps = vec![OwnerBoundaryGap::new(
        ProtocolFamily::ImportPublication,
        OwnerBoundaryGapKind::CheckedProtocolModelPending,
    )];

    ProtocolBindingManifest {
        bindings,
        gaps,
        composed_protocols: vec![ProtocolFamily::SharedFrontiers],
    }
}
