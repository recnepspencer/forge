use worth_ui::facade::{WorthUiPlanNodeInput, WorthUiPlanNodeInputFamily};

fn main() {
    let _ = WorthUiPlanNodeInput {
        identity_basis: "workspace.local.binding".to_owned(),
        family: WorthUiPlanNodeInputFamily::QueryViewBinding,
        transition: None,
        query_binding_identity: None,
    };
}
