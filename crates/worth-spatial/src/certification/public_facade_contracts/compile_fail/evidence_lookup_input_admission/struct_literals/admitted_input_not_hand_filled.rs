use worth_spatial::facade::evidence_lookup_input_admission::EvidenceLookupAdmittedInput;

fn main() {
    let _ = EvidenceLookupAdmittedInput {
        admission_digest: String::new(),
        catalog_digest: String::new(),
        spatial_touch_digest: String::new(),
    };
}
