use crate::identity::{BasisDigest, SchemaBasisDigest};
use crate::planning::{ExecutionPlanBundle, PlannedExecutionRoute};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BasisAuthorityFamily {
    Runtime,
    Store,
}

impl BasisAuthorityFamily {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::Runtime => "runtime",
            Self::Store => "store",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SnapshotLineageClass {
    CurrentHead,
    ReplayEquivalent,
    FutureExtension,
}

impl SnapshotLineageClass {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::CurrentHead => "current_head",
            Self::ReplayEquivalent => "replay_equivalent",
            Self::FutureExtension => "future_extension",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BasisResolutionMode {
    RuntimeDirect,
    StoreDirect,
}

impl BasisResolutionMode {
    #[allow(dead_code)]
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::RuntimeDirect => "runtime_direct",
            Self::StoreDirect => "store_direct",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionBasisIntent {
    authority_family: BasisAuthorityFamily,
    lineage_class: SnapshotLineageClass,
    fallback_allowed: bool,
}

impl ExecutionBasisIntent {
    pub fn new(
        authority_family: BasisAuthorityFamily,
        lineage_class: SnapshotLineageClass,
        fallback_allowed: bool,
    ) -> Self {
        Self {
            authority_family,
            lineage_class,
            fallback_allowed,
        }
    }

    pub fn authority_family(&self) -> &BasisAuthorityFamily {
        &self.authority_family
    }

    pub fn lineage_class(&self) -> &SnapshotLineageClass {
        &self.lineage_class
    }

    pub fn fallback_allowed(&self) -> bool {
        self.fallback_allowed
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ResolvedSnapshotIdentity {
    authority_family: BasisAuthorityFamily,
    workspace_scope: Option<String>,
    snapshot_token: String,
    schema_basis: SchemaBasisDigest,
    lineage_class: SnapshotLineageClass,
}

impl ResolvedSnapshotIdentity {
    #[allow(dead_code)]
    pub(crate) fn new(
        authority_family: BasisAuthorityFamily,
        workspace_scope: Option<String>,
        snapshot_token: impl Into<String>,
        schema_basis: SchemaBasisDigest,
        lineage_class: SnapshotLineageClass,
    ) -> Self {
        Self {
            authority_family,
            workspace_scope,
            snapshot_token: snapshot_token.into(),
            schema_basis,
            lineage_class,
        }
    }

    pub fn authority_family(&self) -> &BasisAuthorityFamily {
        &self.authority_family
    }

    pub fn workspace_scope(&self) -> Option<&str> {
        self.workspace_scope.as_deref()
    }

    pub fn snapshot_token(&self) -> &str {
        &self.snapshot_token
    }

    pub fn schema_basis(&self) -> &SchemaBasisDigest {
        &self.schema_basis
    }

    pub fn lineage_class(&self) -> &SnapshotLineageClass {
        &self.lineage_class
    }

    fn digest_parts(&self) -> Vec<String> {
        let mut parts = vec![
            format!("authority:{}", self.authority_family.as_str()),
            format!("snapshot:{}", self.snapshot_token),
            format!("schema:{}", self.schema_basis.as_str()),
            format!("lineage:{}", self.lineage_class.as_str()),
        ];
        if let Some(scope) = &self.workspace_scope {
            parts.push(format!("workspace:{scope}"));
        }
        parts
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedBasisProof {
    digest: BasisDigest,
    resolution_mode: BasisResolutionMode,
}

impl ResolvedBasisProof {
    pub fn digest(&self) -> &BasisDigest {
        &self.digest
    }

    pub fn resolution_mode(&self) -> &BasisResolutionMode {
        &self.resolution_mode
    }

    pub(crate) fn new(digest: BasisDigest, resolution_mode: BasisResolutionMode) -> Self {
        Self {
            digest,
            resolution_mode,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedSnapshotBasis {
    intent: ExecutionBasisIntent,
    identity: ResolvedSnapshotIdentity,
    resolution_mode: BasisResolutionMode,
    proof: ResolvedBasisProof,
}

impl ResolvedSnapshotBasis {
    pub fn intent(&self) -> &ExecutionBasisIntent {
        &self.intent
    }

    pub fn identity(&self) -> &ResolvedSnapshotIdentity {
        &self.identity
    }

    pub fn resolution_mode(&self) -> &BasisResolutionMode {
        &self.resolution_mode
    }

    pub fn proof(&self) -> &ResolvedBasisProof {
        &self.proof
    }

    pub(crate) fn new(
        intent: ExecutionBasisIntent,
        identity: ResolvedSnapshotIdentity,
        resolution_mode: BasisResolutionMode,
    ) -> Self {
        let digest = BasisDigest::from_parts(&identity.digest_parts());
        let proof = ResolvedBasisProof::new(digest, resolution_mode.clone());
        Self {
            intent,
            identity,
            resolution_mode,
            proof,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SnapshotResolutionReport {
    basis_digest: BasisDigest,
    resolution_mode: BasisResolutionMode,
    snapshot_basis_resolution_count: usize,
}

impl SnapshotResolutionReport {
    pub fn basis_digest(&self) -> &BasisDigest {
        &self.basis_digest
    }

    pub fn resolution_mode(&self) -> &BasisResolutionMode {
        &self.resolution_mode
    }

    pub fn snapshot_basis_resolution_count(&self) -> usize {
        self.snapshot_basis_resolution_count
    }

    pub(crate) fn from_resolved_basis(basis: &ResolvedSnapshotBasis) -> Self {
        Self {
            basis_digest: basis.proof().digest().clone(),
            resolution_mode: basis.resolution_mode().clone(),
            snapshot_basis_resolution_count: 1,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BasisResolutionError {
    UnsupportedBasisKind,
    ResolutionIdentityMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BasisPreflightError {
    BasisIntentMismatch,
    PlannedRouteBasisMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionPreflightBundle {
    plan: ExecutionPlanBundle,
    basis: ResolvedSnapshotBasis,
    report: SnapshotResolutionReport,
}

impl ExecutionPreflightBundle {
    pub fn plan(&self) -> &ExecutionPlanBundle {
        &self.plan
    }

    pub fn basis(&self) -> &ResolvedSnapshotBasis {
        &self.basis
    }

    pub fn report(&self) -> &SnapshotResolutionReport {
        &self.report
    }
}

pub fn resolve_snapshot_basis(
    intent: ExecutionBasisIntent,
    identity: ResolvedSnapshotIdentity,
    resolution_mode: BasisResolutionMode,
) -> Result<ResolvedSnapshotBasis, BasisResolutionError> {
    if intent.authority_family() != identity.authority_family() {
        return Err(BasisResolutionError::ResolutionIdentityMismatch);
    }

    if intent.lineage_class() != identity.lineage_class() {
        return Err(BasisResolutionError::ResolutionIdentityMismatch);
    }

    let mode_matches_authority = matches!(
        (intent.authority_family(), &resolution_mode),
        (BasisAuthorityFamily::Runtime, BasisResolutionMode::RuntimeDirect)
            | (BasisAuthorityFamily::Store, BasisResolutionMode::StoreDirect)
    );
    if !mode_matches_authority {
        return Err(BasisResolutionError::UnsupportedBasisKind);
    }

    Ok(ResolvedSnapshotBasis::new(intent, identity, resolution_mode))
}

pub fn snapshot_resolution_report(basis: &ResolvedSnapshotBasis) -> SnapshotResolutionReport {
    SnapshotResolutionReport::from_resolved_basis(basis)
}

pub fn preflight_execution_basis(
    plan: ExecutionPlanBundle,
    basis: ResolvedSnapshotBasis,
) -> Result<ExecutionPreflightBundle, BasisPreflightError> {
    if plan.request_context().semantic().basis_intent() != basis.intent() {
        return Err(BasisPreflightError::BasisIntentMismatch);
    }

    let route_matches_basis = match basis.identity().authority_family() {
        BasisAuthorityFamily::Runtime => matches!(
            plan.query().route(),
            PlannedExecutionRoute::RuntimeSnapshotRead
                | PlannedExecutionRoute::RuntimeExpandedSnapshotRead
        ),
        BasisAuthorityFamily::Store => {
            plan.query().route() == &PlannedExecutionRoute::StoreSnapshotRead
        }
    };

    if !route_matches_basis {
        return Err(BasisPreflightError::PlannedRouteBasisMismatch);
    }

    Ok(ExecutionPreflightBundle {
        report: SnapshotResolutionReport::from_resolved_basis(&basis),
        plan,
        basis,
    })
}
