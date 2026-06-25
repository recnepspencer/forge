use worth_ui::facade::WorthUiFocusScopeParticipationReceipt;

fn main() {
    let _receipt = WorthUiFocusScopeParticipationReceipt {
        focus_scope_id: "scope".to_owned(),
        owner_node_id: "owner".to_owned(),
        focus_nodes: Vec::new(),
        receipt_digest: 0,
    };
}
