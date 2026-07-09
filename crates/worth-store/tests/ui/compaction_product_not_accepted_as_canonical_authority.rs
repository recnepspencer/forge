fn require_canonical_authority(_: worth_store::FetchedAuthoritativeCommit) {}

fn main() {
    let product = unsafe {
        std::mem::MaybeUninit::<worth_store::PublishedCompactionProduct>::uninit().assume_init()
    };
    require_canonical_authority(product);
}
