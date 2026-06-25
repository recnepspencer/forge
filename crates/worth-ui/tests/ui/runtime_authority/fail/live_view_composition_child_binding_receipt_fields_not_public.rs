use worth_ui::facade::{
    WorthUiLiveViewCompositionChildBindingReceipt, WorthUiLiveViewCompositionChildSubjectKind,
};

fn main() {
    let _forged = WorthUiLiveViewCompositionChildBindingReceipt {
        subject_kind: WorthUiLiveViewCompositionChildSubjectKind::Control,
        subject_id: "first_name_input".to_owned(),
        composition_node_id: "live_view.control.first_name_input".to_owned(),
        authority_identity: "first_name_input".to_owned(),
        parent_id: "input_stack".to_owned(),
        order: 0,
        sizing_token: "fill(1)".to_owned(),
        child_access_row_digest: 1,
        consumed_facts: Vec::new(),
        binding_digest: 1,
    };
}
