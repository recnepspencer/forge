use worth_spatial::facade::evidence_lookup_family_catalog::EvidenceLookupQueryImportEvidence;
use worth_spatial::facade::workload_vocabulary::SpatialEvidenceLookupProduct;

fn main() {
    fn lookup_product_is_not_query_import(product: SpatialEvidenceLookupProduct) {
        let _: EvidenceLookupQueryImportEvidence = product;
    }
}
