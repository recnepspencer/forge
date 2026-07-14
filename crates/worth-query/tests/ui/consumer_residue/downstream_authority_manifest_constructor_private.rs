use worth_query::facade::consumer_kit::WorthQueryDownstreamAuthorityAdoptionManifest;

fn main() {
    let _ = WorthQueryDownstreamAuthorityAdoptionManifest {
        consumer_name: String::new(),
        audited_source_count: 0,
        prohibited_class_count: 0,
        finding_count: 0,
        source_inventory_digest: String::new(),
        report_identity: panic!(),
    };
}
