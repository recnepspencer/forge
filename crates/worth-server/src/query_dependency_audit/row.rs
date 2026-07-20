use super::{
    WorthServerQueryDependencyAuditPathKind, WorthServerQueryDependencyAuditProvenance,
    WorthServerQueryDependencyClosurePosture, WorthServerQueryDependencyConsumerKitPosture,
    WorthServerQueryDependencyRuntimeReadiness, WorthServerQueryDependencyScopePosture,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthServerQueryDependencyAuditRowId(String);

impl WorthServerQueryDependencyAuditRowId {
    pub(crate) fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerQueryDependencyAuditRow {
    row_id: WorthServerQueryDependencyAuditRowId,
    path_kind: WorthServerQueryDependencyAuditPathKind,
    runtime_readiness: WorthServerQueryDependencyRuntimeReadiness,
    consumer_kit_posture: WorthServerQueryDependencyConsumerKitPosture,
    scope_posture: WorthServerQueryDependencyScopePosture,
    closure_posture: WorthServerQueryDependencyClosurePosture,
    ordinary_path: bool,
    canonical_digest: String,
    provenance: WorthServerQueryDependencyAuditProvenance,
    reason: String,
}

pub(crate) struct WorthServerQueryDependencyAuditRowParts {
    pub(crate) row_id: WorthServerQueryDependencyAuditRowId,
    pub(crate) path_kind: WorthServerQueryDependencyAuditPathKind,
    pub(crate) runtime_readiness: WorthServerQueryDependencyRuntimeReadiness,
    pub(crate) consumer_kit_posture: WorthServerQueryDependencyConsumerKitPosture,
    pub(crate) scope_posture: WorthServerQueryDependencyScopePosture,
    pub(crate) closure_posture: WorthServerQueryDependencyClosurePosture,
    pub(crate) ordinary_path: bool,
    pub(crate) provenance: WorthServerQueryDependencyAuditProvenance,
    pub(crate) reason: String,
}

impl WorthServerQueryDependencyAuditRow {
    pub(crate) fn new(parts: WorthServerQueryDependencyAuditRowParts) -> Self {
        let WorthServerQueryDependencyAuditRowParts {
            row_id,
            path_kind,
            runtime_readiness,
            consumer_kit_posture,
            scope_posture,
            closure_posture,
            ordinary_path,
            provenance,
            reason,
        } = parts;
        let canonical_digest = format!(
            "{}|{}|{}|{}|{}|{}|{:?}|{}",
            row_id.as_str(),
            path_kind.as_str(),
            runtime_readiness.as_str(),
            consumer_kit_posture.as_str(),
            scope_posture.as_str(),
            closure_posture.as_str(),
            provenance,
            reason
        );
        Self {
            row_id,
            path_kind,
            runtime_readiness,
            consumer_kit_posture,
            scope_posture,
            closure_posture,
            ordinary_path,
            canonical_digest,
            provenance,
            reason,
        }
    }

    pub fn row_id(&self) -> &WorthServerQueryDependencyAuditRowId {
        &self.row_id
    }

    pub fn path_kind(&self) -> WorthServerQueryDependencyAuditPathKind {
        self.path_kind
    }

    pub fn runtime_readiness(&self) -> WorthServerQueryDependencyRuntimeReadiness {
        self.runtime_readiness
    }

    pub fn consumer_kit_posture(&self) -> WorthServerQueryDependencyConsumerKitPosture {
        self.consumer_kit_posture
    }

    pub fn scope_posture(&self) -> WorthServerQueryDependencyScopePosture {
        self.scope_posture
    }

    pub fn closure_posture(&self) -> WorthServerQueryDependencyClosurePosture {
        self.closure_posture
    }

    pub fn ordinary_path(&self) -> bool {
        self.ordinary_path
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }

    pub fn provenance(&self) -> &WorthServerQueryDependencyAuditProvenance {
        &self.provenance
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}
