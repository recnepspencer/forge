fn require_canonical_authority(_: forge_store::FetchedAuthoritativeCommit) {}

fn main() {
    let product = unsafe {
        std::mem::MaybeUninit::<forge_store::PublishedCompactionProduct>::uninit().assume_init()
    };
    require_canonical_authority(product);
}
