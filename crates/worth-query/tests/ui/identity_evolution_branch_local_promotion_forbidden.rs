use worth_query::facade::foundation::{PluralIdentitySuccessorSet, SingularIdentityContinuityResult};

fn main() {
    let _: fn(PluralIdentitySuccessorSet) -> SingularIdentityContinuityResult =
        PluralIdentitySuccessorSet::promote_to_global_continuity;
}
