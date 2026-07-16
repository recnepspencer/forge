use std::collections::BTreeSet;

use super::evidence::WorthUiQueryBindingEvidence;
use crate::runtime::query_binding::{WorthUiQueryBindingIdentity, WorthUiQueryBindingPosture};
use crate::runtime::{WorthUiQuerySupportReceipt, WorthUiQuerySupportStatus};
use crate::source::{WorthUiBoundViewBindingReference, WorthUiRuntimeDependencyHook};
use crate::{capability::ViewBindingId, source::WorthUiRuntimeDependencyHookKind};

#[derive(Default)]
pub(super) struct WorthUiQueryBindingEvidenceAccumulator {
    query_capability_digest: Option<String>,
    query_composition_profile_digest: Option<String>,
    result_shape_digest: Option<String>,
    basis_capability_digest: Option<String>,
    live_compatibility_digest: Option<String>,
    denial_presentation_digest: Option<String>,
    query_support_status: Option<WorthUiQuerySupportStatus>,
    query_support_contract_identity: Option<String>,
    runtime_surfaces: BTreeSet<WorthUiRuntimeDependencyHookKind>,
    inspection_links: BTreeSet<String>,
    projection_links: BTreeSet<String>,
}

impl WorthUiQueryBindingEvidenceAccumulator {
    pub(super) fn record_bound_view_binding(
        &mut self,
        view_binding: &WorthUiBoundViewBindingReference,
    ) {
        let query = view_binding.query_semantics();
        let definition = query.definition();
        self.query_capability_digest = Some("installed-domain".to_owned());
        self.query_composition_profile_digest = Some(definition.identity().as_str().to_owned());
        self.result_shape_digest = Some(format!("{:?}", definition.shape()));
        self.basis_capability_digest = Some("installed-authority".to_owned());
        self.live_compatibility_digest = Some(format!("{:?}", definition.lifecycle()));
        self.denial_presentation_digest =
            Some(query.denial_presentation().digest_basis().to_owned());
        self.inspection_links.insert(query_evidence_identity(
            "inspection",
            [
                definition.identity().as_str().to_owned(),
                format!("{:?}", definition.shape()),
            ],
        ));
        self.projection_links.insert(query_evidence_identity(
            "projection",
            [format!("{:?}", definition.shape())],
        ));
    }

    pub(super) fn record_runtime_hook(&mut self, hook: &WorthUiRuntimeDependencyHook) {
        let definition = hook.definition();
        self.query_capability_digest = Some("installed-domain".to_owned());
        self.query_composition_profile_digest = Some(definition.identity().as_str().to_owned());
        self.result_shape_digest = Some(format!("{:?}", definition.shape()));
        self.basis_capability_digest = Some("installed-authority".to_owned());
        self.live_compatibility_digest = Some(format!("{:?}", definition.lifecycle()));
        self.denial_presentation_digest =
            Some(hook.denial_presentation().digest_basis().to_owned());
        self.runtime_surfaces.insert(hook.kind());
    }

    pub(super) fn record_query_support_receipt(&mut self, receipt: WorthUiQuerySupportReceipt) {
        self.query_support_status = Some(receipt.status());
        self.query_support_contract_identity = Some(query_evidence_identity(
            "support-contract",
            [
                format!("status:{:?}", receipt.status()),
                format!("runtime_hooks:{}", receipt.runtime_hook_count()),
                format!("identity:{}", receipt.contract_identity().as_u64()),
            ],
        ));
    }

    pub(super) fn finish(self, view_binding_id: &str) -> Option<WorthUiQueryBindingEvidence> {
        let query_capability_digest = self.query_capability_digest?;
        let query_composition_profile_digest = self.query_composition_profile_digest?;
        let result_shape_digest = self.result_shape_digest?;
        let identity = WorthUiQueryBindingIdentity::new(
            &ViewBindingId::new(view_binding_id).expect("indexed view binding id remains valid"),
            query_capability_digest.clone(),
            query_composition_profile_digest,
            result_shape_digest.clone(),
        );
        let posture = WorthUiQueryBindingPosture::new(
            self.query_support_status
                .unwrap_or(WorthUiQuerySupportStatus::Unsupported),
            query_evidence_identity(
                "support-admission",
                [
                    query_capability_digest,
                    self.query_support_contract_identity
                        .unwrap_or_else(|| "support_contract:missing".to_owned()),
                ],
            ),
            self.basis_capability_digest
                .unwrap_or_else(|| "basis:missing".to_owned()),
            self.live_compatibility_digest
                .unwrap_or_else(|| "live:missing".to_owned()),
            digest_runtime_surface(
                "async",
                &self.runtime_surfaces,
                WorthUiRuntimeDependencyHookKind::QueryAsyncResultState,
            ),
            digest_runtime_surface(
                "recovery",
                &self.runtime_surfaces,
                WorthUiRuntimeDependencyHookKind::QuerySignalContinuation,
            ),
            digest_all("inspection", &self.inspection_links),
            digest_all("projection", &self.projection_links),
            self.denial_presentation_digest
                .unwrap_or_else(|| "denial:missing".to_owned()),
        );
        Some(WorthUiQueryBindingEvidence::new(identity, posture))
    }
}

fn query_evidence_identity<const N: usize>(surface: &'static str, values: [String; N]) -> String {
    format!("query-binding-evidence:{surface}:{}", values.join("|"))
}

fn digest_runtime_surface(
    label: &str,
    surfaces: &BTreeSet<WorthUiRuntimeDependencyHookKind>,
    required_surface: WorthUiRuntimeDependencyHookKind,
) -> String {
    if surfaces.contains(&required_surface) {
        format!("{}:{:?}", label, required_surface)
    } else {
        format!("{}:missing", label)
    }
}

fn digest_all(label: &str, values: &BTreeSet<String>) -> String {
    if values.is_empty() {
        return format!("{}:missing", label);
    }
    format!(
        "{}:{}",
        label,
        values.iter().cloned().collect::<Vec<_>>().join(",")
    )
}

#[cfg(test)]
mod tests {
    use super::query_evidence_identity;

    #[test]
    fn query_evidence_identity_distinguishes_different_declared_result_shapes() {
        let collection = query_evidence_identity("projection", ["result_shape:collection".into()]);
        let detail = query_evidence_identity("projection", ["result_shape:detail".into()]);

        assert_ne!(collection, detail);
        assert!(collection.starts_with("query-binding-evidence:"));
        assert!(detail.starts_with("query-binding-evidence:"));
    }

    #[test]
    fn query_evidence_identity_does_not_collapse_support_receipts_with_different_hook_posture() {
        let first = query_evidence_identity(
            "support-receipt",
            ["status:Supported".into(), "runtime_hooks:4".into()],
        );
        let second = query_evidence_identity(
            "support-receipt",
            ["status:Supported".into(), "runtime_hooks:5".into()],
        );

        assert_ne!(first, second);
    }
}
