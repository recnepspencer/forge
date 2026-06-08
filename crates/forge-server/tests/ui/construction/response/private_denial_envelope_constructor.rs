use forge_server::{ForgeServerDenialBoundary, ForgeServerDenialCause, ForgeServerDenialEnvelope, ForgeServerResponseReceipt, ForgeServerResponseTransform};

fn main() {
    let cause = ForgeServerDenialCause::RequestContext {
        code: forge_server::ForgeServerRequestContextDenialCode::PreviewTargetingDisabled,
        diagnostics_profile: forge_server::request_context::DiagnosticRichnessProfile::Standard,
        detail: "forged".to_string(),
    };
    let _ = ForgeServerDenialEnvelope {
        transform: ForgeServerResponseTransform::compat_http(),
        diagnostics_profile: forge_server::request_context::DiagnosticRichnessProfile::Standard,
        cause,
        provenance: loop {},
        receipt: ForgeServerResponseReceipt::Completed(loop {}),
        canonical_digest: format!("{:?}", ForgeServerDenialBoundary::RequestContext),
    };
}
