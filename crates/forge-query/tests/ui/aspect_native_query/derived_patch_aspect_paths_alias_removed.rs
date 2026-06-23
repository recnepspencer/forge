use forge_query::facade::ForgeQueryDerivedPatch;

fn assert_no_neutral_path_alias(patch: &ForgeQueryDerivedPatch) {
    let _ = patch.aspect_paths();
}

fn main() {}
