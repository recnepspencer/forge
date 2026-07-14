use worth_query::facade::runtime::WorthQueryDerivedPatch;

fn assert_no_neutral_path_alias(patch: &WorthQueryDerivedPatch) {
    let _ = patch.aspect_paths();
}

fn main() {}
