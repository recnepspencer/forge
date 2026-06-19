use crate::{ForgeServerOperationAuthorityMetadata, ForgeServerQuerySupportPosture};

use super::ForgeServerOperationSupportCompositionReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerOperationSupportPosture {
    query_support_posture: Option<ForgeServerQuerySupportPosture>,
    product_support_label: String,
    shared_read_comparable: bool,
    composition_receipt: ForgeServerOperationSupportCompositionReceipt,
    canonical_digest: String,
}

impl ForgeServerOperationSupportPosture {
    pub(crate) fn new(
        query_support_posture: Option<ForgeServerQuerySupportPosture>,
        authority_metadata: &ForgeServerOperationAuthorityMetadata,
        composition_receipt: ForgeServerOperationSupportCompositionReceipt,
    ) -> Self {
        let product_support_label = product_support_label(authority_metadata);
        let shared_read_comparable = shared_read_comparable(authority_metadata);
        let canonical_digest = format!(
            "forge-server-operation-support-posture-v1|query={}|product={product_support_label}|shared_read_comparable={shared_read_comparable}|receipt={}",
            query_support_posture
                .as_ref()
                .map(ForgeServerQuerySupportPosture::canonical_label)
                .unwrap_or_else(|| "none".to_string()),
            composition_receipt.canonical_digest(),
        );
        Self {
            query_support_posture,
            product_support_label,
            shared_read_comparable,
            composition_receipt,
            canonical_digest,
        }
    }

    pub fn query_support_posture(&self) -> Option<&ForgeServerQuerySupportPosture> {
        self.query_support_posture.as_ref()
    }

    pub fn product_support_label(&self) -> &str {
        &self.product_support_label
    }

    pub fn shared_read_comparable(&self) -> bool {
        self.shared_read_comparable
    }

    pub fn composition_receipt(&self) -> &ForgeServerOperationSupportCompositionReceipt {
        &self.composition_receipt
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

fn product_support_label(authority_metadata: &ForgeServerOperationAuthorityMetadata) -> String {
    match authority_metadata {
        ForgeServerOperationAuthorityMetadata::SharedReadOnly {
            basis_kind,
            product_support_posture,
            ..
        } => {
            format!("shared-read:{basis_kind}:{product_support_posture}")
        }
        ForgeServerOperationAuthorityMetadata::DeterministicSubmission {
            submission_lane, ..
        } => format!("deterministic-submission:{submission_lane}"),
        ForgeServerOperationAuthorityMetadata::ProductDraftMutation { draft_scope, .. } => {
            format!("product-draft-mutation:{draft_scope}")
        }
        ForgeServerOperationAuthorityMetadata::ProductSessionCoordination {
            coordination_lane,
            ..
        } => format!("product-session-coordination:{coordination_lane}"),
        ForgeServerOperationAuthorityMetadata::BinaryStreaming { stream_kind, .. } => {
            format!("binary-streaming:{stream_kind}")
        }
        ForgeServerOperationAuthorityMetadata::DiagnosticsOnly { diagnostics_lane } => {
            format!("diagnostics-only:{diagnostics_lane}")
        }
        ForgeServerOperationAuthorityMetadata::LeaseCoordination {
            coordination_lane, ..
        } => format!("lease-coordination:{coordination_lane}"),
    }
}

fn shared_read_comparable(authority_metadata: &ForgeServerOperationAuthorityMetadata) -> bool {
    matches!(
        authority_metadata,
        ForgeServerOperationAuthorityMetadata::SharedReadOnly {
            basis_kind,
            product_support_posture,
            ..
        } if product_support_posture == "production-admitted"
            && (basis_kind == "query-shared-read-basis"
                || basis_kind == "query-derived"
                || basis_kind == "product-session-derived"
                || basis_kind == "durable-product-derived")
    )
}
