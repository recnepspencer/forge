use crate::{
    WorthServerBackgroundExportRequest, WorthServerBinaryCounterSet, WorthServerBinaryDownload,
    WorthServerBinaryIngressSession, WorthServerCompatHttpRouteFamily,
    WorthServerCompatibilityDenial, WorthServerCompatibilityExport,
    WorthServerCompatibilityInspection, WorthServerCompatibilityMutation,
    WorthServerCompatibilityPreparedRequest, WorthServerCompatibilityRead,
    WorthServerCompatibilityUpload, WorthServerExternalCounterSet,
};

use super::counters::{
    binary_counter_set, external_counter_set, BINARY_LANE_ASSERTIONS, BUDGET_ADMITTED,
    BUDGET_CHECKS, BUDGET_DENIED, BYTE_CLASS_ASSERTIONS, METADATA_ONLY_ASSERTIONS,
    ROUTE_FAMILY_ASSERTIONS, SEMANTIC_TRUTH_DRIFT, SLOWLORIS_CUTOFFS, STRUCTURED_LANE_ASSERTIONS,
    TENANT_SCOPE_ASSERTIONS,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerTransferByteClass {
    StructuredPayload,
    BinaryWire,
    BinaryAuthoritative,
    MetadataOnly,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerAbuseBudgetReceipt {
    route_family: WorthServerCompatHttpRouteFamily,
    byte_class: WorthServerTransferByteClass,
    tenant_id: String,
    workspace_digest: String,
    branch_digest: String,
    denial: Option<String>,
    external_counters: Option<WorthServerExternalCounterSet>,
    binary_counters: Option<WorthServerBinaryCounterSet>,
    canonical_digest: String,
}

impl WorthServerAbuseBudgetReceipt {
    pub(crate) fn admitted(
        route_family: WorthServerCompatHttpRouteFamily,
        byte_class: WorthServerTransferByteClass,
        tenant_id: impl Into<String>,
        workspace_digest: impl Into<String>,
        branch_digest: impl Into<String>,
    ) -> Self {
        Self::new(
            route_family,
            byte_class,
            tenant_id,
            workspace_digest,
            branch_digest,
            None,
        )
    }

    pub(crate) fn denied(
        route_family: WorthServerCompatHttpRouteFamily,
        byte_class: WorthServerTransferByteClass,
        tenant_id: impl Into<String>,
        workspace_digest: impl Into<String>,
        branch_digest: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self::new_denial(
            route_family,
            byte_class,
            tenant_id,
            workspace_digest,
            branch_digest,
            detail,
            WorthServerAbuseBudgetDenialClass::OrdinaryDenial,
        )
    }

    fn new(
        route_family: WorthServerCompatHttpRouteFamily,
        byte_class: WorthServerTransferByteClass,
        tenant_id: impl Into<String>,
        workspace_digest: impl Into<String>,
        branch_digest: impl Into<String>,
        denial: Option<String>,
    ) -> Self {
        let denial_detail = denial.clone().unwrap_or_default();
        let denial_class = if denial.is_some() {
            WorthServerAbuseBudgetDenialClass::OrdinaryDenial
        } else {
            WorthServerAbuseBudgetDenialClass::Admitted
        };
        Self::new_denial(
            route_family,
            byte_class,
            tenant_id,
            workspace_digest,
            branch_digest,
            denial_detail,
            denial_class,
        )
    }

    fn new_denial(
        route_family: WorthServerCompatHttpRouteFamily,
        byte_class: WorthServerTransferByteClass,
        tenant_id: impl Into<String>,
        workspace_digest: impl Into<String>,
        branch_digest: impl Into<String>,
        denial_detail: impl Into<String>,
        denial_class: WorthServerAbuseBudgetDenialClass,
    ) -> Self {
        let tenant_id = tenant_id.into();
        let workspace_digest = workspace_digest.into();
        let branch_digest = branch_digest.into();
        let denial = denial_class
            .into_denial_option(denial_detail.into())
            .filter(|detail| !detail.is_empty());
        let route_family_assertions = 1;
        let byte_class_assertions = 1;
        let tenant_scope_assertions = 1;
        let structured_lane_assertions = u64::from(matches!(
            byte_class,
            WorthServerTransferByteClass::StructuredPayload
        ));
        let binary_lane_assertions = u64::from(matches!(
            byte_class,
            WorthServerTransferByteClass::BinaryWire
                | WorthServerTransferByteClass::BinaryAuthoritative
        ));
        let metadata_only_assertions = u64::from(matches!(
            byte_class,
            WorthServerTransferByteClass::MetadataOnly
        ));
        let denied = u64::from(denial.is_some());
        let admitted = u64::from(denial.is_none());
        let slowloris_cutoffs = u64::from(matches!(
            denial_class,
            WorthServerAbuseBudgetDenialClass::SlowlorisCutoff
        ));
        let rows = [
            (BUDGET_CHECKS, 1),
            (BUDGET_ADMITTED, admitted),
            (BUDGET_DENIED, denied),
            (SLOWLORIS_CUTOFFS, slowloris_cutoffs),
            (TENANT_SCOPE_ASSERTIONS, tenant_scope_assertions),
            (ROUTE_FAMILY_ASSERTIONS, route_family_assertions),
            (BYTE_CLASS_ASSERTIONS, byte_class_assertions),
            (STRUCTURED_LANE_ASSERTIONS, structured_lane_assertions),
            (BINARY_LANE_ASSERTIONS, binary_lane_assertions),
            (METADATA_ONLY_ASSERTIONS, metadata_only_assertions),
            (SEMANTIC_TRUTH_DRIFT, 0),
        ];
        let external_counters = match byte_class {
            WorthServerTransferByteClass::StructuredPayload
            | WorthServerTransferByteClass::MetadataOnly => Some(external_counter_set(
                "compat_http.abuse.external_budget",
                &rows,
            )),
            WorthServerTransferByteClass::BinaryWire
            | WorthServerTransferByteClass::BinaryAuthoritative => None,
        };
        let binary_counters = match byte_class {
            WorthServerTransferByteClass::StructuredPayload
            | WorthServerTransferByteClass::MetadataOnly => None,
            WorthServerTransferByteClass::BinaryWire
            | WorthServerTransferByteClass::BinaryAuthoritative => {
                Some(binary_counter_set("compat_http.abuse.binary_budget", &rows))
            }
        };
        let canonical_digest = format!(
            "worth-server-abuse-budget-receipt-v1|route={}|byte_class={}|tenant={tenant_id}|workspace={workspace_digest}|branch={branch_digest}|denial={}",
            route_family.as_str(),
            byte_class.as_str(),
            denial.as_deref().unwrap_or("none"),
        );
        Self {
            route_family,
            byte_class,
            tenant_id,
            workspace_digest,
            branch_digest,
            denial,
            external_counters,
            binary_counters,
            canonical_digest,
        }
    }

    pub fn route_family(&self) -> WorthServerCompatHttpRouteFamily {
        self.route_family
    }

    pub fn byte_class(&self) -> WorthServerTransferByteClass {
        self.byte_class
    }

    pub fn tenant_id(&self) -> &str {
        &self.tenant_id
    }

    pub fn workspace_digest(&self) -> &str {
        &self.workspace_digest
    }

    pub fn branch_digest(&self) -> &str {
        &self.branch_digest
    }

    pub fn denial(&self) -> Option<&str> {
        self.denial.as_deref()
    }

    pub fn external_counters(&self) -> Option<&WorthServerExternalCounterSet> {
        self.external_counters.as_ref()
    }

    pub fn binary_counters(&self) -> Option<&WorthServerBinaryCounterSet> {
        self.binary_counters.as_ref()
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorthServerAbuseBudgetDenialClass {
    Admitted,
    OrdinaryDenial,
    SlowlorisCutoff,
}

impl WorthServerCompatibilityPreparedRequest {
    pub fn abuse_budget_receipt(&self) -> WorthServerAbuseBudgetReceipt {
        let request_context = self.admission().request_context();
        WorthServerAbuseBudgetReceipt::admitted(
            self.request_contract().route_family(),
            byte_class_for_request(
                self.request_contract().route_family(),
                self.request_contract().method(),
            ),
            request_context.workspace_target().tenant_id(),
            request_context.workspace_target().workspace_digest(),
            request_context.branch_target().branch_digest(),
        )
    }
}

impl WorthServerCompatibilityRead {
    pub fn abuse_budget_receipt(&self) -> WorthServerAbuseBudgetReceipt {
        let context = self.direct_context();
        WorthServerAbuseBudgetReceipt::admitted(
            WorthServerCompatHttpRouteFamily::Read,
            WorthServerTransferByteClass::StructuredPayload,
            context.workspace_target().tenant_id(),
            context.workspace_digest(),
            context.branch_digest(),
        )
    }
}

impl WorthServerCompatibilityInspection {
    pub fn abuse_budget_receipt(&self) -> WorthServerAbuseBudgetReceipt {
        let context = self.direct_context();
        WorthServerAbuseBudgetReceipt::admitted(
            WorthServerCompatHttpRouteFamily::Read,
            WorthServerTransferByteClass::StructuredPayload,
            context.workspace_target().tenant_id(),
            context.workspace_digest(),
            context.branch_digest(),
        )
    }
}

impl WorthServerCompatibilityMutation {
    pub fn abuse_budget_receipt(&self) -> WorthServerAbuseBudgetReceipt {
        let context = self.envelope().direct_context();
        WorthServerAbuseBudgetReceipt::admitted(
            WorthServerCompatHttpRouteFamily::Mutation,
            WorthServerTransferByteClass::StructuredPayload,
            context.workspace_target().tenant_id(),
            context.workspace_digest(),
            context.branch_digest(),
        )
    }
}

impl WorthServerBinaryDownload {
    pub fn abuse_budget_receipt(&self) -> WorthServerAbuseBudgetReceipt {
        let provenance = self.file_envelope().transfer_provenance();
        WorthServerAbuseBudgetReceipt::admitted(
            WorthServerCompatHttpRouteFamily::Download,
            if self.session().head_only() {
                WorthServerTransferByteClass::MetadataOnly
            } else {
                WorthServerTransferByteClass::BinaryWire
            },
            provenance.tenant_id(),
            provenance.workspace_digest(),
            provenance.branch_digest(),
        )
    }
}

impl WorthServerCompatibilityUpload {
    pub fn abuse_budget_receipt(&self) -> WorthServerAbuseBudgetReceipt {
        let provenance = self.file_envelope().transfer_provenance();
        WorthServerAbuseBudgetReceipt::admitted(
            WorthServerCompatHttpRouteFamily::Upload,
            WorthServerTransferByteClass::BinaryAuthoritative,
            provenance.tenant_id(),
            provenance.workspace_digest(),
            provenance.branch_digest(),
        )
    }
}

impl WorthServerBinaryIngressSession {
    pub fn abuse_budget_receipt(&self) -> WorthServerAbuseBudgetReceipt {
        WorthServerAbuseBudgetReceipt::admitted(
            WorthServerCompatHttpRouteFamily::Upload,
            WorthServerTransferByteClass::BinaryWire,
            self.tenant_id(),
            self.workspace_digest(),
            self.branch_digest(),
        )
    }
}

impl WorthServerCompatibilityExport {
    pub fn abuse_budget_receipt(&self) -> WorthServerAbuseBudgetReceipt {
        let provenance = self.file_envelope().transfer_provenance();
        WorthServerAbuseBudgetReceipt::admitted(
            WorthServerCompatHttpRouteFamily::Streaming,
            if self.payload_bytes().is_empty() {
                WorthServerTransferByteClass::MetadataOnly
            } else {
                WorthServerTransferByteClass::StructuredPayload
            },
            provenance.tenant_id(),
            provenance.workspace_digest(),
            provenance.branch_digest(),
        )
    }
}

impl WorthServerBackgroundExportRequest {
    pub fn abuse_budget_receipt(&self) -> WorthServerAbuseBudgetReceipt {
        let provenance = self.file_envelope().transfer_provenance();
        WorthServerAbuseBudgetReceipt::admitted(
            WorthServerCompatHttpRouteFamily::Streaming,
            WorthServerTransferByteClass::MetadataOnly,
            provenance.tenant_id(),
            provenance.workspace_digest(),
            provenance.branch_digest(),
        )
    }
}

impl WorthServerCompatibilityDenial {
    pub fn abuse_budget_receipt(&self) -> Option<&WorthServerAbuseBudgetReceipt> {
        self.abuse_budget_receipt.as_ref()
    }
}

pub(crate) fn denied_budget_receipt_for_prepared_request(
    prepared_request: &WorthServerCompatibilityPreparedRequest,
    byte_class: WorthServerTransferByteClass,
    detail: impl Into<String>,
    denial_class: WorthServerAbuseBudgetDenialClass,
) -> WorthServerAbuseBudgetReceipt {
    let request_context = prepared_request.admission().request_context();
    WorthServerAbuseBudgetReceipt::new_denial(
        prepared_request.request_contract().route_family(),
        byte_class,
        request_context.workspace_target().tenant_id(),
        request_context.workspace_target().workspace_digest(),
        request_context.branch_target().branch_digest(),
        detail,
        denial_class,
    )
}

pub(crate) fn byte_class_for_request(
    route_family: WorthServerCompatHttpRouteFamily,
    method: &str,
) -> WorthServerTransferByteClass {
    if method == "HEAD" || route_family == WorthServerCompatHttpRouteFamily::Preflight {
        return WorthServerTransferByteClass::MetadataOnly;
    }
    match route_family {
        WorthServerCompatHttpRouteFamily::Read
        | WorthServerCompatHttpRouteFamily::Query
        | WorthServerCompatHttpRouteFamily::Mutation
        | WorthServerCompatHttpRouteFamily::Streaming => {
            WorthServerTransferByteClass::StructuredPayload
        }
        WorthServerCompatHttpRouteFamily::Upload | WorthServerCompatHttpRouteFamily::Download => {
            WorthServerTransferByteClass::BinaryWire
        }
        WorthServerCompatHttpRouteFamily::Preflight => WorthServerTransferByteClass::MetadataOnly,
    }
}
