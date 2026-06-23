use worth_kernel::query_graph_authority_gate::{
    WorthGraphAuthorityCloseoutDisposition, WorthGraphAuthorityCloseoutMatrixRow,
    WorthGraphAuthorityDeletionTarget, WorthGraphAuthorityOwner, WorthGraphAuthorityRootFamily,
};

fn main() {
    let _ = WorthGraphAuthorityCloseoutMatrixRow {
        source_id: "forged",
        source_scope: "forged scope",
        owner: WorthGraphAuthorityOwner::Kernel,
        root_family: Some(WorthGraphAuthorityRootFamily::KernelGraphObligationAdoption),
        deletion_target: WorthGraphAuthorityDeletionTarget::None,
        disposition: WorthGraphAuthorityCloseoutDisposition::PublicFacadeStatusOnly,
        ordinary_public_facade: "forged facade",
        proof_boundary: "forged proof boundary",
    };
}
