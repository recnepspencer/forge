use forge_runtime_bridge::facade::BridgeWritebackFamilyKind;

fn family_label(kind: BridgeWritebackFamilyKind) -> &'static str {
    match kind {
        BridgeWritebackFamilyKind::ProjectedStateDiff => "projected",
    }
}

fn main() {
    let _ = family_label(BridgeWritebackFamilyKind::ProjectedStateDiff);
}
