use forge_server::{
    ForgeServerEvidenceTransform, ForgeServerOperatorEvidenceClass,
    ForgeServerOperatorEvidenceRecord,
};

fn main() {
    let _ = ForgeServerOperatorEvidenceRecord {
        transform: ForgeServerEvidenceTransform::operator_default(),
        diagnostics_profile: forge_server::request_context::DiagnosticRichnessProfile::Standard,
        response_digest: "forged".to_string(),
        classification: ForgeServerOperatorEvidenceClass::QueryReadSucceeded,
        counter_receipt: loop {},
        attachment_bundle: loop {},
        materialized_attachment_bundle: loop {},
    };
}
