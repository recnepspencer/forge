use crate::{WorthServerOperationFamily, WorthServerOperationInventory, WorthServerRouteInventory};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerNoProductSemanticsCertification {
    ready: bool,
    detail: String,
    product_families_are_generic: bool,
    product_routes_are_generic: bool,
    support_rows_remain_passthrough: bool,
    semantic_route_metadata_remains_generic: bool,
    canonical_digest: String,
}

impl WorthServerNoProductSemanticsCertification {
    pub(crate) fn derive(
        operation_inventory: &WorthServerOperationInventory,
        route_inventory: &WorthServerRouteInventory,
    ) -> Self {
        let semantic_routes = route_inventory
            .rows()
            .iter()
            .filter(|row| row.operation_name().is_some())
            .collect::<Vec<_>>();
        let product_families_are_generic = operation_inventory.rows().iter().all(|row| {
            matches!(
                row.family(),
                WorthServerOperationFamily::QueryDirectRead
                    | WorthServerOperationFamily::QueryDirectSubmission
                    | WorthServerOperationFamily::QueryDirectProjection
                    | WorthServerOperationFamily::ProductApplicationRead
                    | WorthServerOperationFamily::ProductApplicationMutation
                    | WorthServerOperationFamily::ProductSessionCoordination
                    | WorthServerOperationFamily::BinaryTransfer
                    | WorthServerOperationFamily::SyncLease
            )
        });
        let product_routes_are_generic = route_inventory.rows().iter().all(|row| {
            row.operation_name().is_none()
                || matches!(
                    row.operation_family(),
                    Some(
                        WorthServerOperationFamily::ProductApplicationRead
                            | WorthServerOperationFamily::ProductApplicationMutation
                            | WorthServerOperationFamily::ProductSessionCoordination
                    )
                )
        });
        let support_rows_remain_passthrough = semantic_routes
            .iter()
            .all(|row| row.support_row().map(|value| !value.trim().is_empty()) == Some(true));
        let semantic_route_metadata_remains_generic = semantic_routes.iter().all(|row| {
            row.diagnostics_policy() == "request-context-resolved"
                && row.evidence_policy() == "runtime-derived"
        });
        let ready = product_families_are_generic
            && product_routes_are_generic
            && support_rows_remain_passthrough
            && semantic_route_metadata_remains_generic;
        let detail = format!(
            "families_generic={product_families_are_generic};routes_generic={product_routes_are_generic};support_passthrough={support_rows_remain_passthrough};route_metadata_generic={semantic_route_metadata_remains_generic}"
        );
        let canonical_digest =
            format!("worth-server-no-product-semantics-v1|ready={ready}|detail={detail}");
        Self {
            ready,
            detail,
            product_families_are_generic,
            product_routes_are_generic,
            support_rows_remain_passthrough,
            semantic_route_metadata_remains_generic,
            canonical_digest,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.ready
    }

    pub fn summary(&self) -> &str {
        &self.detail
    }

    pub fn product_families_are_generic(&self) -> bool {
        self.product_families_are_generic
    }

    pub fn product_routes_are_generic(&self) -> bool {
        self.product_routes_are_generic
    }

    pub fn support_rows_remain_passthrough(&self) -> bool {
        self.support_rows_remain_passthrough
    }

    pub fn semantic_route_metadata_remains_generic(&self) -> bool {
        self.semantic_route_metadata_remains_generic
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
