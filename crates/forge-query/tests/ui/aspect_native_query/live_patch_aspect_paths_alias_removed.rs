use forge_query::facade::ForgeQueryLivePatch;

fn assert_no_neutral_path_alias(patch: &ForgeQueryLivePatch) {
    let _ = patch.aspect_paths();
}

fn main() {}
