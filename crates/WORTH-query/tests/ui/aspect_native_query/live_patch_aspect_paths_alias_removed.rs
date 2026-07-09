use worth_query::facade::WorthQueryLivePatch;

fn assert_no_neutral_path_alias(patch: &WorthQueryLivePatch) {
    let _ = patch.aspect_paths();
}

fn main() {}
