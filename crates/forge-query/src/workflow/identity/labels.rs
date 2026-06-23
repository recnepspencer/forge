use forge_runtime_bridge::facade::BridgeRequestKind;

pub(crate) fn bridge_request_kind_label(kind: BridgeRequestKind) -> &'static str {
    match kind {
        BridgeRequestKind::Authoritative => "authoritative",
        BridgeRequestKind::Preview => "preview",
    }
}
