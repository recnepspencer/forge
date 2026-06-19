use crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanLoopClassifiedProductKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopIdentityRow {
    row_identity: String,
    tracked_loop_identity: String,
    canonical_loop_identity: String,
    loop_kind: PlanarBooleanLoopClassifiedProductKind,
    source_loop_identities: Vec<String>,
    fragment_identities: Vec<String>,
    split_vertex_identities: Vec<String>,
    role_outcome_identity: String,
    degenerate_outcome_identity: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopPersistentNamePropagationRow {
    row_identity: String,
    canonical_loop_identity: String,
    tracked_loop_identity: String,
    loop_kind: PlanarBooleanLoopClassifiedProductKind,
    upstream_persistent_name_identity: String,
    upstream_artifact_identity: String,
    propagated_persistent_name_identity: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopSubshapeSignatureRow {
    row_identity: String,
    canonical_loop_identity: String,
    tracked_loop_identity: String,
    loop_kind: PlanarBooleanLoopClassifiedProductKind,
    upstream_artifact_identity: String,
    propagated_signature_identity: String,
    signature_basis_identity: String,
}

impl PlanarBooleanLoopIdentityRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        row_identity: String,
        tracked_loop_identity: String,
        canonical_loop_identity: String,
        loop_kind: PlanarBooleanLoopClassifiedProductKind,
        source_loop_identities: Vec<String>,
        fragment_identities: Vec<String>,
        split_vertex_identities: Vec<String>,
        role_outcome_identity: String,
        degenerate_outcome_identity: String,
    ) -> Self {
        Self {
            row_identity,
            tracked_loop_identity,
            canonical_loop_identity,
            loop_kind,
            source_loop_identities,
            fragment_identities,
            split_vertex_identities,
            role_outcome_identity,
            degenerate_outcome_identity,
        }
    }

    pub fn row_identity(&self) -> &str {
        &self.row_identity
    }

    pub fn tracked_loop_identity(&self) -> &str {
        &self.tracked_loop_identity
    }

    pub fn canonical_loop_identity(&self) -> &str {
        &self.canonical_loop_identity
    }

    pub fn loop_kind(&self) -> PlanarBooleanLoopClassifiedProductKind {
        self.loop_kind
    }

    pub fn source_loop_identities(&self) -> &[String] {
        &self.source_loop_identities
    }

    pub fn fragment_identities(&self) -> &[String] {
        &self.fragment_identities
    }

    pub fn split_vertex_identities(&self) -> &[String] {
        &self.split_vertex_identities
    }

    pub fn role_outcome_identity(&self) -> &str {
        &self.role_outcome_identity
    }

    pub fn degenerate_outcome_identity(&self) -> &str {
        &self.degenerate_outcome_identity
    }
}

impl PlanarBooleanLoopPersistentNamePropagationRow {
    pub(crate) fn new(
        row_identity: String,
        canonical_loop_identity: String,
        tracked_loop_identity: String,
        loop_kind: PlanarBooleanLoopClassifiedProductKind,
        upstream_persistent_name_identity: String,
        upstream_artifact_identity: String,
        propagated_persistent_name_identity: String,
    ) -> Self {
        Self {
            row_identity,
            canonical_loop_identity,
            tracked_loop_identity,
            loop_kind,
            upstream_persistent_name_identity,
            upstream_artifact_identity,
            propagated_persistent_name_identity,
        }
    }

    pub fn row_identity(&self) -> &str {
        &self.row_identity
    }

    pub fn canonical_loop_identity(&self) -> &str {
        &self.canonical_loop_identity
    }

    pub fn tracked_loop_identity(&self) -> &str {
        &self.tracked_loop_identity
    }

    pub fn loop_kind(&self) -> PlanarBooleanLoopClassifiedProductKind {
        self.loop_kind
    }

    pub fn upstream_persistent_name_identity(&self) -> &str {
        &self.upstream_persistent_name_identity
    }

    pub fn upstream_artifact_identity(&self) -> &str {
        &self.upstream_artifact_identity
    }

    pub fn propagated_persistent_name_identity(&self) -> &str {
        &self.propagated_persistent_name_identity
    }
}

impl PlanarBooleanLoopSubshapeSignatureRow {
    pub(crate) fn new(
        row_identity: String,
        canonical_loop_identity: String,
        tracked_loop_identity: String,
        loop_kind: PlanarBooleanLoopClassifiedProductKind,
        upstream_artifact_identity: String,
        propagated_signature_identity: String,
        signature_basis_identity: String,
    ) -> Self {
        Self {
            row_identity,
            canonical_loop_identity,
            tracked_loop_identity,
            loop_kind,
            upstream_artifact_identity,
            propagated_signature_identity,
            signature_basis_identity,
        }
    }

    pub fn row_identity(&self) -> &str {
        &self.row_identity
    }

    pub fn canonical_loop_identity(&self) -> &str {
        &self.canonical_loop_identity
    }

    pub fn tracked_loop_identity(&self) -> &str {
        &self.tracked_loop_identity
    }

    pub fn loop_kind(&self) -> PlanarBooleanLoopClassifiedProductKind {
        self.loop_kind
    }

    pub fn upstream_artifact_identity(&self) -> &str {
        &self.upstream_artifact_identity
    }

    pub fn propagated_signature_identity(&self) -> &str {
        &self.propagated_signature_identity
    }

    pub fn signature_basis_identity(&self) -> &str {
        &self.signature_basis_identity
    }
}
