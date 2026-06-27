use serde::Serialize;

use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyQueryReceiptPosture;

use forge_query::facade::runtime::{ForgeQueryReadReceipt, ForgeQueryWriteReceipt};
use forge_query::facade::ProjectionConsumptionReceipt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationQuerySupportEvidence {
    projection_consumption_receipt_digest: Option<String>,
    native_read_receipt_digest: Option<String>,
    native_write_receipt_digest: Option<String>,
    support_digest: String,
}

impl DerivedInvalidationQuerySupportEvidence {
    pub fn missing() -> Self {
        Self::from_parts(None, None, None)
    }

    pub fn from_projection_consumption_receipt(receipt: &ProjectionConsumptionReceipt) -> Self {
        Self::from_parts(Some(receipt.receipt_digest().to_string()), None, None)
    }

    pub fn from_native_read_receipt(receipt: &ForgeQueryReadReceipt) -> Self {
        Self::from_parts(None, Some(receipt.read_graph_digest().to_string()), None)
    }

    pub fn from_native_write_receipt(receipt: &ForgeQueryWriteReceipt) -> Self {
        Self::from_parts(
            None,
            None,
            Some(
                receipt
                    .commit_evidence_identity()
                    .terminal_projection_for_reporting()
                    .to_string(),
            ),
        )
    }

    pub fn from_query_receipts(
        projection_consumption_receipt: Option<&ProjectionConsumptionReceipt>,
        native_read_receipt: Option<&ForgeQueryReadReceipt>,
        native_write_receipt: Option<&ForgeQueryWriteReceipt>,
    ) -> Self {
        Self::from_parts(
            projection_consumption_receipt.map(|receipt| receipt.receipt_digest().to_string()),
            native_read_receipt.map(|receipt| receipt.read_graph_digest().to_string()),
            native_write_receipt.map(|receipt| {
                receipt
                    .commit_evidence_identity()
                    .terminal_projection_for_reporting()
                    .to_string()
            }),
        )
    }

    #[cfg(any(test, feature = "test-support-lowering"))]
    pub(crate) fn from_receipt_digests_for_tests(
        projection_consumption_receipt_digest: Option<String>,
        native_read_receipt_digest: Option<String>,
        native_write_receipt_digest: Option<String>,
    ) -> Self {
        Self::from_parts(
            projection_consumption_receipt_digest,
            native_read_receipt_digest,
            native_write_receipt_digest,
        )
    }

    fn from_parts(
        projection_consumption_receipt_digest: Option<String>,
        native_read_receipt_digest: Option<String>,
        native_write_receipt_digest: Option<String>,
    ) -> Self {
        let support_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:derived-invalidation-query-support:v1".to_string(),
            digest_part(
                "projection-consumption",
                projection_consumption_receipt_digest.as_deref(),
            ),
            digest_part("native-read", native_read_receipt_digest.as_deref()),
            digest_part("native-write", native_write_receipt_digest.as_deref()),
        ]);
        Self {
            projection_consumption_receipt_digest,
            native_read_receipt_digest,
            native_write_receipt_digest,
            support_digest,
        }
    }

    pub fn supports(&self, posture: DerivedTopologyQueryReceiptPosture) -> bool {
        self.required_receipt_digest(posture).is_some()
            || posture == DerivedTopologyQueryReceiptPosture::NotRequiredForFamilyDeclaration
    }

    pub fn required_receipt_digest(
        &self,
        posture: DerivedTopologyQueryReceiptPosture,
    ) -> Option<&str> {
        match posture {
            DerivedTopologyQueryReceiptPosture::ProjectionConsumptionRequired => {
                self.projection_consumption_receipt_digest.as_deref()
            }
            DerivedTopologyQueryReceiptPosture::NativeReadReceiptRequired => {
                self.native_read_receipt_digest.as_deref()
            }
            DerivedTopologyQueryReceiptPosture::NativeWriteReceiptRequired => {
                self.native_write_receipt_digest.as_deref()
            }
            DerivedTopologyQueryReceiptPosture::NotRequiredForFamilyDeclaration => None,
        }
    }

    pub fn support_digest(&self) -> &str {
        &self.support_digest
    }
}

fn digest_part(label: &str, digest: Option<&str>) -> String {
    format!("{label}:{}", digest.unwrap_or("missing"))
}
