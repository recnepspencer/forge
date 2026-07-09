use worth_query::facade::WorthQueryIntentAdmissionPublicBoundaryAudit;

fn main() {
    let _ = WorthQueryIntentAdmissionPublicBoundaryAudit {
        compile_fail_targets: &[],
        golden_transcripts: &[],
        compile_fail_boundary_digest: String::new(),
        negative_dx_boundary_digest: String::new(),
        golden_transcript_digest: String::new(),
        public_surface_digest: String::new(),
        target_dx_digest: String::new(),
    };
}
