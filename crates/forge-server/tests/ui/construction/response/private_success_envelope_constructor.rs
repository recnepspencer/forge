use forge_server::{
    ForgeServerResponseReceipt, ForgeServerResponseTransform, ForgeServerSuccessEnvelope,
};

fn main() {
    let _ = ForgeServerSuccessEnvelope {
        transform: ForgeServerResponseTransform::forge_native(),
        diagnostics_profile: forge_server::request_context::DiagnosticRichnessProfile::Standard,
        payload: loop {},
        provenance: loop {},
        receipt: ForgeServerResponseReceipt::Completed(loop {}),
        canonical_digest: "forged".to_string(),
    };
}
