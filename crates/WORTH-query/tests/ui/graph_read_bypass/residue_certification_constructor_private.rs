use worth_query::facade::consumer_kit::WorthQueryGraphReadBypassResidueCertification;

fn main() {
    let _ = WorthQueryGraphReadBypassResidueCertification {
        previous_manifest_digest: String::new(),
        candidate_manifest_digest: String::new(),
        certified_row_count: 0,
        certification_digest: String::new(),
    };
}
