use worth_query::facade::runtime::WorthQueryRuntimeFacadeFamily;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerQueryDependencyAuditProvenance {
    QuerySupportPin(WorthServerQueryDependencySupportPinProvenance),
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

impl WorthServerQueryDependencyAuditProvenance {
    pub fn support_pin(&self) -> Option<&WorthServerQueryDependencySupportPinProvenance> {
        match self {
            Self::QuerySupportPin(value) => Some(value),
        }
    }
}

impl WorthServerQueryDependencySupportPinProvenance {
    pub(crate) fn new(parts: WorthServerQueryDependencySupportPinProvenanceParts) -> Self {
        let WorthServerQueryDependencySupportPinProvenanceParts {
            workspace_name,
            required_families,
            support_matrix_digest,
            support_snapshot_digest,
            contract_digest,
            report_digest,
            blocking_finding_count,
            matched_required_count,
        } = parts;
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

pub(crate) struct WorthServerQueryDependencySupportPinProvenanceParts {
    pub(crate) workspace_name: String,
    pub(crate) required_families: Vec<WorthQueryRuntimeFacadeFamily>,
    pub(crate) support_matrix_digest: String,
    pub(crate) support_snapshot_digest: String,
    pub(crate) contract_digest: String,
    pub(crate) report_digest: String,
    pub(crate) blocking_finding_count: usize,
    pub(crate) matched_required_count: usize,
}
