use worth_ui_query_binding::WorthUiPresentationAsyncOwner;

fn reporting_material_cannot_open_pending(
    owner: &mut WorthUiPresentationAsyncOwner,
    reporting: String,
) {
    let _ = owner.admit_pending(reporting);
}

fn main() {}
