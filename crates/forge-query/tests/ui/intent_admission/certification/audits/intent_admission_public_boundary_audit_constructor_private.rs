use forge_query::facade::ForgeQueryIntentAdmissionPublicBoundaryAudit;

fn main() {
    let _ = ForgeQueryIntentAdmissionPublicBoundaryAudit {
        compile_fail_targets: &[],
        golden_transcripts: &[],
        compile_fail_boundary_digest: String::new(),
        negative_dx_boundary_digest: String::new(),
        golden_transcript_digest: String::new(),
        public_surface_digest: String::new(),
        target_dx_digest: String::new(),
    };
}
