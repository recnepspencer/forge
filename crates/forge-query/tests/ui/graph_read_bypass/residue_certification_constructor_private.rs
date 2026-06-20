use forge_query::facade::consumer_kit::ForgeQueryGraphReadBypassResidueCertification;

fn main() {
    let _ = ForgeQueryGraphReadBypassResidueCertification {
        previous_manifest_digest: String::new(),
        candidate_manifest_digest: String::new(),
        certified_row_count: 0,
        certification_digest: String::new(),
    };
}
