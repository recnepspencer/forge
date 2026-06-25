use worth_ui::facade::{
    WorthUiAccessibilityAssociationKind, WorthUiAccessibilityRelationshipReceipt,
};

fn main() {
    let _receipt = WorthUiAccessibilityRelationshipReceipt {
        kind: WorthUiAccessibilityAssociationKind::Label,
        source_node_id: "label".to_owned(),
        target_node_id: "control".to_owned(),
        source_role: "label".to_owned(),
        target_role: "text_input".to_owned(),
        source_resolved_text: Some("Title".to_owned()),
        association_digest: 0,
        receipt_digest: 0,
    };
}
