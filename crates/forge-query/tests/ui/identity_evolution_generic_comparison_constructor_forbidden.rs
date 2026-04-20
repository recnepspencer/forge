use forge_query::facade::CorrespondenceIdentityComparison;

fn main() {
    let _: fn(String, String, bool) -> CorrespondenceIdentityComparison =
        CorrespondenceIdentityComparison::between_with_intent_bool;
}
