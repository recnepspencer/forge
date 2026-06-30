use worth_spatial::facade::evidence_lookup_family_catalog::EvidenceLookupFamilyCatalogCloseout;

fn main() {
    let _closeout = EvidenceLookupFamilyCatalogCloseout {
        declarations: Vec::new(),
        counters: unsafe { core::mem::zeroed() },
        catalog_digest: String::new(),
    };
}
