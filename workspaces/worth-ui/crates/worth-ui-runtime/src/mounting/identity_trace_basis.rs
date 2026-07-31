use worth_ui_host_contract::{UiMountedInstanceIdentity, UiMountedNodeReceiptIdentity};

/// Retained mounted truth from which the explicit visual-inspection lane may
/// assemble an authored identity trace.
///
/// The mounted lane carries existing proof only: the persistent semantic
/// projection and a clone-only handle to the exact prepared generation's
/// declaration/evidence indexes. It performs no authored lookup or trace
/// projection while preparing or publishing an ordinary frame.
#[derive(Clone)]
pub(crate) struct UiMountedIdentityTraceBasis {
    receipts: super::UiMountedNodeReceiptBasis,
    semantic: super::projection::UiMountedSemanticProjection,
    authored_source:
        crate::facade::prepared_application_authority::WorthUiPreparedVisualTraceSource,
}

impl UiMountedIdentityTraceBasis {
    pub(in crate::mounting) fn new(
        receipts: super::UiMountedNodeReceiptBasis,
        semantic: super::projection::UiMountedSemanticProjection,
        authored_source: crate::facade::prepared_application_authority::WorthUiPreparedVisualTraceSource,
    ) -> Self {
        Self {
            receipts,
            semantic,
            authored_source,
        }
    }

    pub(crate) fn node_with_probes(
        &self,
        mounted_instance: UiMountedInstanceIdentity,
    ) -> (Option<&super::UiMountedNodeReceipt>, usize) {
        self.semantic.node_receipt_with_probes(mounted_instance)
    }

    pub(crate) fn receipt_for_with_probes(
        &self,
        mounted_instance: UiMountedInstanceIdentity,
    ) -> (
        Option<worth_ui_host_contract::UiMountedNodeReceiptIdentity>,
        usize,
    ) {
        self.receipts.receipt_for_with_probes(mounted_instance)
    }

    pub(crate) fn node_for_receipt_with_probes(
        &self,
        node_receipt: UiMountedNodeReceiptIdentity,
    ) -> (Option<&super::UiMountedNodeReceipt>, usize) {
        let mounted_instance = node_receipt.mounted_instance();
        let (expected, receipt_probes) = self.receipt_for_with_probes(mounted_instance);
        if expected != Some(node_receipt) {
            return (None, receipt_probes);
        }
        let (node, node_probes) = self.node_with_probes(mounted_instance);
        (node, receipt_probes.saturating_add(node_probes))
    }

    pub(crate) fn authored_source(
        &self,
    ) -> &crate::facade::prepared_application_authority::WorthUiPreparedVisualTraceSource {
        &self.authored_source
    }

    pub(crate) fn projection_input(
        &self,
        identity: &worth_ui_query_binding::WorthUiQueryViewIdentity,
    ) -> Option<&worth_ui_query_binding::UiProjectionInputFactReference> {
        self.semantic.projection_input(identity)
    }

    pub(crate) fn retained_structural_bytes(&self) -> Option<usize> {
        std::mem::size_of::<Self>()
            .checked_add(self.receipts.retained_structural_bytes()?)?
            .checked_add(self.semantic.retained_structural_bytes()?)
            .and_then(|bytes| {
                bytes.checked_add(self.authored_source.minimum_retained_structural_bytes()?)
            })
    }
}
