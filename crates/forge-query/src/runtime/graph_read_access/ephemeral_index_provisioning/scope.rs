use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryEphemeralGraphIndexScopeKind {
    ReadExecution,
    FamilyExecution,
}

impl ForgeQueryEphemeralGraphIndexScopeKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ReadExecution => "read_execution",
            Self::FamilyExecution => "family_execution",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryEphemeralGraphIndexScope {
    digest: String,
    kind: ForgeQueryEphemeralGraphIndexScopeKind,
    admitted_plan_digest: String,
    snapshot_identity_digest: String,
    byte_budget: usize,
}

impl ForgeQueryEphemeralGraphIndexScope {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn kind(&self) -> &ForgeQueryEphemeralGraphIndexScopeKind {
        &self.kind
    }

    pub fn admitted_plan_digest(&self) -> &str {
        &self.admitted_plan_digest
    }

    pub fn snapshot_identity_digest(&self) -> &str {
        &self.snapshot_identity_digest
    }

    pub fn byte_budget(&self) -> usize {
        self.byte_budget
    }

    pub(in crate::runtime::graph_read_access::ephemeral_index_provisioning) fn read_execution(
        admitted_plan_digest: impl Into<String>,
        snapshot_identity_digest: impl Into<String>,
        byte_budget: usize,
    ) -> Self {
        Self::new(
            ForgeQueryEphemeralGraphIndexScopeKind::ReadExecution,
            admitted_plan_digest.into(),
            snapshot_identity_digest.into(),
            byte_budget,
        )
    }

    fn new(
        kind: ForgeQueryEphemeralGraphIndexScopeKind,
        admitted_plan_digest: String,
        snapshot_identity_digest: String,
        byte_budget: usize,
    ) -> Self {
        let digest = hash_parts(&[
            "forge_query_ephemeral_graph_index_scope_v1".to_string(),
            format!("kind:{}", kind.as_str()),
            format!("admitted_plan:{admitted_plan_digest}"),
            format!("snapshot:{snapshot_identity_digest}"),
            format!("byte_budget:{byte_budget}"),
        ]);
        Self {
            digest,
            kind,
            admitted_plan_digest,
            snapshot_identity_digest,
            byte_budget,
        }
    }
}
