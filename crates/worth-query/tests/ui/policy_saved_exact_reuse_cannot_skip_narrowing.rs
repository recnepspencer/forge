use worth_query::facade::policy::SavedPolicyNarrowingReuseDescriptor;

fn main() {
    let _descriptor = SavedPolicyNarrowingReuseDescriptor::new(
        "saved-query",
        "prior-narrowed",
        "policy",
        "tenant-truth",
        "tenant-schema",
        "projection",
        "proof",
        "policy",
        "tenant-truth",
        "tenant-schema",
        "projection",
        "proof",
    );
}
