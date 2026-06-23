use forge_query::facade::ForgeQueryRetainedRefreshContext;

fn assert_no_neutral_touched_path_alias(refresh: &ForgeQueryRetainedRefreshContext) {
    let _ = refresh.touched_aspect_paths();
}

fn main() {}
