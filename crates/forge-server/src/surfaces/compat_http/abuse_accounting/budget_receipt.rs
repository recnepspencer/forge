use crate::{
    ForgeServerBackgroundExportRequest, ForgeServerBinaryCounterSet, ForgeServerBinaryDownload,
    ForgeServerBinaryIngressSession, ForgeServerCompatHttpRouteFamily,
    ForgeServerCompatibilityDenial, ForgeServerCompatibilityExport,
    ForgeServerCompatibilityInspection, ForgeServerCompatibilityMutation,
    ForgeServerCompatibilityPreparedRequest, ForgeServerCompatibilityRead,
    ForgeServerCompatibilityUpload, ForgeServerExternalCounterSet,
};

use super::counters::{
    binary_counter_set, external_counter_set, BINARY_LANE_ASSERTIONS, BUDGET_ADMITTED,
    BUDGET_CHECKS, BUDGET_DENIED, BYTE_CLASS_ASSERTIONS, METADATA_ONLY_ASSERTIONS,
    ROUTE_FAMILY_ASSERTIONS, SEMANTIC_TRUTH_DRIFT, SLOWLORIS_CUTOFFS, STRUCTURED_LANE_ASSERTIONS,
    TENANT_SCOPE_ASSERTIONS,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerTransferByteClass {
    StructuredPayload,
    BinaryWire,
    BinaryAuthoritative,
    MetadataOnly,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerAbuseBudgetReceipt {
    route_family: ForgeServerCompatHttpRouteFamily,
    byte_class: ForgeServerTransferByteClass,
    tenant_id: String,
    workspace_digest: String,
    branch_digest: String,
    denial: Option<String>,
    external_counters: Option<ForgeServerExternalCounterSet>,
    binary_counters: Option<ForgeServerBinaryCounterSet>,
    canonical_digest: String,
}

impl ForgeServerAbuseBudgetReceipt {
    pub(crate) fn admitted(
        route_family: ForgeServerCompatHttpRouteFamily,
        byte_class: ForgeServerTransferByteClass,
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
        route_family: ForgeServerCompatHttpRouteFamily,
        byte_class: ForgeServerTransferByteClass,
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
            ForgeServerAbuseBudgetDenialClass::OrdinaryDenial,
        )
    }

    fn new(
        route_family: ForgeServerCompatHttpRouteFamily,
        byte_class: ForgeServerTransferByteClass,
        tenant_id: impl Into<String>,
        workspace_digest: impl Into<String>,
        branch_digest: impl Into<String>,
        denial: Option<String>,
    ) -> Self {
        let denial_detail = denial.clone().unwrap_or_default();
        let denial_class = if denial.is_some() {
            ForgeServerAbuseBudgetDenialClass::OrdinaryDenial
        } else {
            ForgeServerAbuseBudgetDenialClass::Admitted
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
        route_family: ForgeServerCompatHttpRouteFamily,
        byte_class: ForgeServerTransferByteClass,
        tenant_id: impl Into<String>,
        workspace_digest: impl Into<String>,
        branch_digest: impl Into<String>,
        denial_detail: impl Into<String>,
        denial_class: ForgeServerAbuseBudgetDenialClass,
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
            ForgeServerTransferByteClass::StructuredPayload
        ));
        let binary_lane_assertions = u64::from(matches!(
            byte_class,
            ForgeServerTransferByteClass::BinaryWire
                | ForgeServerTransferByteClass::BinaryAuthoritative
        ));
        let metadata_only_assertions = u64::from(matches!(
            byte_class,
            ForgeServerTransferByteClass::MetadataOnly
        ));
        let denied = u64::from(denial.is_some());
        let admitted = u64::from(denial.is_none());
        let slowloris_cutoffs = u64::from(matches!(
            denial_class,
            ForgeServerAbuseBudgetDenialClass::SlowlorisCutoff
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
            ForgeServerTransferByteClass::StructuredPayload
            | ForgeServerTransferByteClass::MetadataOnly => Some(external_counter_set(
                "compat_http.abuse.external_budget",
                &rows,
            )),
            ForgeServerTransferByteClass::BinaryWire
            | ForgeServerTransferByteClass::BinaryAuthoritative => None,
        };
        let binary_counters = match byte_class {
            ForgeServerTransferByteClass::StructuredPayload
            | ForgeServerTransferByteClass::MetadataOnly => None,
            ForgeServerTransferByteClass::BinaryWire
            | ForgeServerTransferByteClass::BinaryAuthoritative => {
                Some(binary_counter_set("compat_http.abuse.binary_budget", &rows))
            }
        };
        let canonical_digest = format!(
            "forge-server-abuse-budget-receipt-v1|route={}|byte_class={}|tenant={tenant_id}|workspace={workspace_digest}|branch={branch_digest}|denial={}",
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

    pub fn route_family(&self) -> ForgeServerCompatHttpRouteFamily {
        self.route_family
    }

    pub fn byte_class(&self) -> ForgeServerTransferByteClass {
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

    pub fn external_counters(&self) -> Option<&ForgeServerExternalCounterSet> {
        self.external_counters.as_ref()
    }

    pub fn binary_counters(&self) -> Option<&ForgeServerBinaryCounterSet> {
        self.binary_counters.as_ref()
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ForgeServerAbuseBudgetDenialClass {
    Admitted,
    OrdinaryDenial,
    SlowlorisCutoff,
}

impl ForgeServerAbuseBudgetDenialClass {
    fn into_denial_option(self, detail: String) -> Option<String> {
        match self {
            Self::Admitted => None,
            Self::OrdinaryDenial | Self::SlowlorisCutoff => Some(detail),
        }
    }
}

impl ForgeServerTransferByteClass {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::StructuredPayload => "structured_payload",
            Self::BinaryWire => "binary_wire",
            Self::BinaryAuthoritative => "binary_authoritative",
            Self::MetadataOnly => "metadata_only",
        }
    }
}

impl ForgeServerCompatibilityPreparedRequest {
    pub fn abuse_budget_receipt(&self) -> ForgeServerAbuseBudgetReceipt {
        let request_context = self.admission().request_context();
        ForgeServerAbuseBudgetReceipt::admitted(
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

impl ForgeServerCompatibilityRead {
    pub fn abuse_budget_receipt(&self) -> ForgeServerAbuseBudgetReceipt {
        let context = self.direct_context();
        ForgeServerAbuseBudgetReceipt::admitted(
            ForgeServerCompatHttpRouteFamily::Read,
            ForgeServerTransferByteClass::StructuredPayload,
            context.workspace_target().tenant_id(),
            context.workspace_digest(),
            context.branch_digest(),
        )
    }
}

impl ForgeServerCompatibilityInspection {
    pub fn abuse_budget_receipt(&self) -> ForgeServerAbuseBudgetReceipt {
        let context = self.direct_context();
        ForgeServerAbuseBudgetReceipt::admitted(
            ForgeServerCompatHttpRouteFamily::Read,
            ForgeServerTransferByteClass::StructuredPayload,
            context.workspace_target().tenant_id(),
            context.workspace_digest(),
            context.branch_digest(),
        )
    }
}

impl ForgeServerCompatibilityMutation {
    pub fn abuse_budget_receipt(&self) -> ForgeServerAbuseBudgetReceipt {
        let context = self.envelope().direct_context();
        ForgeServerAbuseBudgetReceipt::admitted(
            ForgeServerCompatHttpRouteFamily::Mutation,
            ForgeServerTransferByteClass::StructuredPayload,
            context.workspace_target().tenant_id(),
            context.workspace_digest(),
            context.branch_digest(),
        )
    }
}

impl ForgeServerBinaryDownload {
    pub fn abuse_budget_receipt(&self) -> ForgeServerAbuseBudgetReceipt {
        let provenance = self.file_envelope().transfer_provenance();
        ForgeServerAbuseBudgetReceipt::admitted(
            ForgeServerCompatHttpRouteFamily::Download,
            if self.session().head_only() {
                ForgeServerTransferByteClass::MetadataOnly
            } else {
                ForgeServerTransferByteClass::BinaryWire
            },
            provenance.tenant_id(),
            provenance.workspace_digest(),
            provenance.branch_digest(),
        )
    }
}

impl ForgeServerCompatibilityUpload {
    pub fn abuse_budget_receipt(&self) -> ForgeServerAbuseBudgetReceipt {
        let provenance = self.file_envelope().transfer_provenance();
        ForgeServerAbuseBudgetReceipt::admitted(
            ForgeServerCompatHttpRouteFamily::Upload,
            ForgeServerTransferByteClass::BinaryAuthoritative,
            provenance.tenant_id(),
            provenance.workspace_digest(),
            provenance.branch_digest(),
        )
    }
}

impl ForgeServerBinaryIngressSession {
    pub fn abuse_budget_receipt(&self) -> ForgeServerAbuseBudgetReceipt {
        ForgeServerAbuseBudgetReceipt::admitted(
            ForgeServerCompatHttpRouteFamily::Upload,
            ForgeServerTransferByteClass::BinaryWire,
            self.tenant_id(),
            self.workspace_digest(),
            self.branch_digest(),
        )
    }
}

impl ForgeServerCompatibilityExport {
    pub fn abuse_budget_receipt(&self) -> ForgeServerAbuseBudgetReceipt {
        let provenance = self.file_envelope().transfer_provenance();
        ForgeServerAbuseBudgetReceipt::admitted(
            ForgeServerCompatHttpRouteFamily::Streaming,
            if self.payload_bytes().is_empty() {
                ForgeServerTransferByteClass::MetadataOnly
            } else {
                ForgeServerTransferByteClass::StructuredPayload
            },
            provenance.tenant_id(),
            provenance.workspace_digest(),
            provenance.branch_digest(),
        )
    }
}

impl ForgeServerBackgroundExportRequest {
    pub fn abuse_budget_receipt(&self) -> ForgeServerAbuseBudgetReceipt {
        let provenance = self.file_envelope().transfer_provenance();
        ForgeServerAbuseBudgetReceipt::admitted(
            ForgeServerCompatHttpRouteFamily::Streaming,
            ForgeServerTransferByteClass::MetadataOnly,
            provenance.tenant_id(),
            provenance.workspace_digest(),
            provenance.branch_digest(),
        )
    }
}

impl ForgeServerCompatibilityDenial {
    pub fn abuse_budget_receipt(&self) -> Option<&ForgeServerAbuseBudgetReceipt> {
        self.abuse_budget_receipt.as_ref()
    }
}

pub(crate) fn denied_budget_receipt_for_prepared_request(
    prepared_request: &ForgeServerCompatibilityPreparedRequest,
    byte_class: ForgeServerTransferByteClass,
    detail: impl Into<String>,
    denial_class: ForgeServerAbuseBudgetDenialClass,
) -> ForgeServerAbuseBudgetReceipt {
    let request_context = prepared_request.admission().request_context();
    ForgeServerAbuseBudgetReceipt::new_denial(
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
    route_family: ForgeServerCompatHttpRouteFamily,
    method: &str,
) -> ForgeServerTransferByteClass {
    if method == "HEAD" || route_family == ForgeServerCompatHttpRouteFamily::Preflight {
        return ForgeServerTransferByteClass::MetadataOnly;
    }
    match route_family {
        ForgeServerCompatHttpRouteFamily::Read
        | ForgeServerCompatHttpRouteFamily::Mutation
        | ForgeServerCompatHttpRouteFamily::Streaming => {
            ForgeServerTransferByteClass::StructuredPayload
        }
        ForgeServerCompatHttpRouteFamily::Upload | ForgeServerCompatHttpRouteFamily::Download => {
            ForgeServerTransferByteClass::BinaryWire
        }
        ForgeServerCompatHttpRouteFamily::Preflight => ForgeServerTransferByteClass::MetadataOnly,
    }
}
