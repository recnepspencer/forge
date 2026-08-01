use worth_ui_query_binding::{
    WorthUiAdmittedCollectionChangePublication, WorthUiRuntimeQueryBinding,
};

fn clone_is_forbidden(admission: &WorthUiAdmittedCollectionChangePublication) {
    let _copy: WorthUiAdmittedCollectionChangePublication = admission.clone();
}

fn reuse_is_forbidden(
    binding: &mut WorthUiRuntimeQueryBinding,
    admission: WorthUiAdmittedCollectionChangePublication,
) {
    let _ = binding.publish_admitted_operation_live_change(admission);
    let _ = binding.withdraw_admitted_operation_live_change(admission);
}

fn main() {}
