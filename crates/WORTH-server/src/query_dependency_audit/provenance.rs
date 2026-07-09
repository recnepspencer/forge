use worth_query::facade::WorthQueryRuntimeFacadeFamily;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerQueryDependencyAuditProvenance {
    QuerySupportPin(WorthServerQueryDependencySupportPinProvenance),
    BoundaryAudit(WorthServerQueryDependencyBoundaryAuditProvenance),
    TestBackendResidue(WorthServerQueryDependencyTestBackendResidueProvenance),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerQueryDependencySupportPinProvenance {
    workspace_name: String,
    required_families: Vec<WorthQueryRuntimeFacadeFamily>,
    support_matrix_digest: String,
    support_snapshot_digest: String,
    contract_digest: String,
    report_digest: String,
    blocking_finding_count: usize,
    matched_required_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerQueryDependencyBoundaryAuditProvenance {
    required_roots: Vec<String>,
    source_paths: Vec<String>,
    source_inventory_identity: String,
    report_identity: String,
    finding_count: usize,
    parsed_item_count: usize,
    visited_call_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerQueryDependencyTestBackendResidueProvenance {
    audited_roots: Vec<String>,
    report_identity: String,
    finding_count: usize,
    scanned_file_count: usize,
}

impl WorthServerQueryDependencyAuditProvenance {
    pub fn support_pin(&self) -> Option<&WorthServerQueryDependencySupportPinProvenance> {
        match self {
            Self::QuerySupportPin(value) => Some(value),
            _ => None,
        }
    }

    pub fn boundary_audit(&self) -> Option<&WorthServerQueryDependencyBoundaryAuditProvenance> {
        match self {
            Self::BoundaryAudit(value) => Some(value),
            _ => None,
        }
    }

    pub fn test_backend_residue(
        &self,
    ) -> Option<&WorthServerQueryDependencyTestBackendResidueProvenance> {
        match self {
            Self::TestBackendResidue(value) => Some(value),
            _ => None,
        }
    }
}

impl WorthServerQueryDependencySupportPinProvenance {
    pub(crate) fn new(
        workspace_name: String,
        required_families: Vec<WorthQueryRuntimeFacadeFamily>,
        support_matrix_digest: String,
        support_snapshot_digest: String,
        contract_digest: String,
        report_digest: String,
        blocking_finding_count: usize,
        matched_required_count: usize,
    ) -> Self {
        Self {
            workspace_name,
            required_families,
            support_matrix_digest,
            support_snapshot_digest,
            contract_digest,
            report_digest,
            blocking_finding_count,
            matched_required_count,
        }
    }

    pub fn workspace_name(&self) -> &str {
        &self.workspace_name
    }

    pub fn required_families(&self) -> &[WorthQueryRuntimeFacadeFamily] {
        &self.required_families
    }

    pub fn support_matrix_digest(&self) -> &str {
        &self.support_matrix_digest
    }

    pub fn support_snapshot_digest(&self) -> &str {
        &self.support_snapshot_digest
    }

    pub fn contract_digest(&self) -> &str {
        &self.contract_digest
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }

    pub fn blocking_finding_count(&self) -> usize {
        self.blocking_finding_count
    }

    pub fn matched_required_count(&self) -> usize {
        self.matched_required_count
    }
}

impl WorthServerQueryDependencyBoundaryAuditProvenance {
    pub(crate) fn new(
        required_roots: Vec<String>,
        source_paths: Vec<String>,
        source_inventory_identity: String,
        report_identity: String,
        finding_count: usize,
        parsed_item_count: usize,
        visited_call_count: usize,
    ) -> Self {
        Self {
            required_roots,
            source_paths,
            source_inventory_identity,
            report_identity,
            finding_count,
            parsed_item_count,
            visited_call_count,
        }
    }

    pub fn required_roots(&self) -> &[String] {
        &self.required_roots
    }

    pub fn source_paths(&self) -> &[String] {
        &self.source_paths
    }

    pub fn source_inventory_identity(&self) -> &str {
        &self.source_inventory_identity
    }

    pub fn report_identity(&self) -> &str {
        &self.report_identity
    }

    pub fn finding_count(&self) -> usize {
        self.finding_count
    }

    pub fn parsed_item_count(&self) -> usize {
        self.parsed_item_count
    }

    pub fn visited_call_count(&self) -> usize {
        self.visited_call_count
    }
}

impl WorthServerQueryDependencyTestBackendResidueProvenance {
    pub(crate) fn new(
        audited_roots: Vec<String>,
        report_identity: String,
        finding_count: usize,
        scanned_file_count: usize,
    ) -> Self {
        Self {
            audited_roots,
            report_identity,
            finding_count,
            scanned_file_count,
        }
    }

    pub fn audited_roots(&self) -> &[String] {
        &self.audited_roots
    }

    pub fn report_identity(&self) -> &str {
        &self.report_identity
    }

    pub fn finding_count(&self) -> usize {
        self.finding_count
    }

    pub fn scanned_file_count(&self) -> usize {
        self.scanned_file_count
    }
}
