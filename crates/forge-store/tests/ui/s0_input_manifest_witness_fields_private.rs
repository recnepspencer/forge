use forge_store::{S0InputManifestWitness, S0StableDigest};

fn main() {
    let _witness = S0InputManifestWitness {
        schema_version: "storage-foundation-s0/v1",
        source_revision: String::new(),
        manifest_digest: S0StableDigest::new("digest").unwrap(),
    };
}
