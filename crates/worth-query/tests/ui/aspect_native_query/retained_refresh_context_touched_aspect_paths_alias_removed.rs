use worth_query::facade::runtime::WorthQueryRetainedRefreshContext;

fn assert_no_neutral_touched_path_alias(refresh: &WorthQueryRetainedRefreshContext) {
    let _ = refresh.touched_aspect_paths();
}

fn main() {}
