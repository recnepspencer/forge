use crate::runtime::{
    WorthUiAuthoredCompositionDeclaration, WorthUiPrimitiveContentAdmissionStatus,
    WorthUiPrimitiveContentReceipt, WorthUiPrimitiveContentValueDenialReceipt, WorthUiRuntimeHost,
};

pub(in crate::runtime::live_view) fn lower_composition_content_receipts(
    runtime: &WorthUiRuntimeHost,
    composition: Option<&WorthUiAuthoredCompositionDeclaration>,
) -> Vec<WorthUiPrimitiveContentReceipt> {
    let Some(composition) = composition else {
        return Vec::new();
    };
    composition
        .contents()
        .iter()
        .filter_map(|content| {
            let report = runtime.admit_primitive_content_props_for_subject(
                content.node_id(),
                content.props().to_vec(),
                content_digest_basis(content.node_id(), content.props()),
            );
            report
                .status()
                .accepted_receipt()
                .map(|receipt| receipt.resolved_receipt(runtime))
        })
        .collect()
}

pub(in crate::runtime::live_view) fn composition_content_denials(
    runtime: &WorthUiRuntimeHost,
    composition: Option<&WorthUiAuthoredCompositionDeclaration>,
) -> Vec<WorthUiPrimitiveContentValueDenialReceipt> {
    let Some(composition) = composition else {
        return Vec::new();
    };
    composition
        .contents()
        .iter()
        .flat_map(|content| {
            let report = runtime.admit_primitive_content_props_for_subject(
                content.node_id(),
                content.props().to_vec(),
                content_digest_basis(content.node_id(), content.props()),
            );
            match report.status() {
                WorthUiPrimitiveContentAdmissionStatus::Accepted(_) => Vec::new(),
                WorthUiPrimitiveContentAdmissionStatus::Rejected(denial_set) => {
                    denial_set.denials().to_vec()
                }
            }
        })
        .collect()
}

fn content_digest_basis(
    node_id: &str,
    props: &[crate::runtime::AuthoredPrimitiveContentProp],
) -> u64 {
    crate::runtime::live_view::digest::digest_parts(
        std::iter::once(node_id.to_owned()).chain(
            props
                .iter()
                .map(|prop| format!("{}={}", prop.key, prop.value)),
        ),
    )
}
