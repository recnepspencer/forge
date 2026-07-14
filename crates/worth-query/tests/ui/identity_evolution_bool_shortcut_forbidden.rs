use worth_query::facade::foundation::CorrespondenceIdentityComparison;

fn main() {
    let _: fn(bool) -> CorrespondenceIdentityComparison =
        CorrespondenceIdentityComparison::from_lineage_match_bool;
}
