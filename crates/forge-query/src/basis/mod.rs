use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::identity::{BasisDigest, SchemaBasisDigest};
use crate::planning::{ExecutionPlanBundle, PlannedExecutionRoute};
use forge_runtime_bridge::facade::TruthSnapshotIdentity;

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedSnapshotIdentity {
    authority_family: BasisAuthorityFamily,
    workspace_scope: Option<String>,
    snapshot_identity: ForgeQueryEvidenceIdentity,
    schema_basis: SchemaBasisDigest,
    lineage_class: SnapshotLineageClass,
}

impl ResolvedSnapshotIdentity {
    pub(crate) fn new(
        authority_family: BasisAuthorityFamily,
        workspace_scope: Option<String>,
        snapshot_identity: ForgeQueryEvidenceIdentity,
        schema_basis: SchemaBasisDigest,
        lineage_class: SnapshotLineageClass,
    ) -> Self {
        Self {
            authority_family,
            workspace_scope,
            snapshot_identity,
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

    pub fn snapshot_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.snapshot_identity
    }

    pub fn schema_basis(&self) -> &SchemaBasisDigest {
        &self.schema_basis
    }

    pub fn lineage_class(&self) -> &SnapshotLineageClass {
        &self.lineage_class
    }

    fn evidence_identity(&self) -> ForgeQueryEvidenceIdentity {
        forge_query_evidence_identity(ForgeQueryEvidenceScope::ResolvedSnapshotBasis)
            .field_shape(
                ForgeQueryEvidenceTag::new("authority_family"),
                self.authority_family.as_str(),
            )
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("snapshot_identity"),
                &self.snapshot_identity,
            )
            .field_identity(
                ForgeQueryEvidenceTag::new("schema_basis"),
                self.schema_basis().as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("lineage_class"),
                self.lineage_class.as_str(),
            )
            .optional_value(
                ForgeQueryEvidenceTag::new("workspace_scope"),
                self.workspace_scope.as_deref(),
            )
            .seal()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedBasisProof {
    identity: ForgeQueryEvidenceIdentity,
    digest: BasisDigest,
    resolution_mode: BasisResolutionMode,
}

impl ResolvedBasisProof {
    pub fn identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.identity
    }

    pub fn digest(&self) -> &BasisDigest {
        &self.digest
    }

    pub fn resolution_mode(&self) -> &BasisResolutionMode {
        &self.resolution_mode
    }

    pub(crate) fn new(
        identity: ForgeQueryEvidenceIdentity,
        resolution_mode: BasisResolutionMode,
    ) -> Self {
        let digest = BasisDigest::from_evidence_identity(&identity);
        Self {
            identity,
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
        let proof = ResolvedBasisProof::new(identity.evidence_identity(), resolution_mode.clone());
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
        (
            BasisAuthorityFamily::Runtime,
            BasisResolutionMode::RuntimeDirect
        ) | (
            BasisAuthorityFamily::Store,
            BasisResolutionMode::StoreDirect
        )
    );
    if !mode_matches_authority {
        return Err(BasisResolutionError::UnsupportedBasisKind);
    }

    Ok(ResolvedSnapshotBasis::new(
        intent,
        identity,
        resolution_mode,
    ))
}

pub fn resolve_runtime_current_snapshot_basis(
    snapshot_identity: ForgeQueryEvidenceIdentity,
    schema_basis: SchemaBasisDigest,
) -> Result<ResolvedSnapshotBasis, BasisResolutionError> {
    resolve_snapshot_basis(
        ExecutionBasisIntent::new(
            BasisAuthorityFamily::Runtime,
            SnapshotLineageClass::CurrentHead,
            false,
        ),
        ResolvedSnapshotIdentity::new(
            BasisAuthorityFamily::Runtime,
            None,
            snapshot_identity,
            schema_basis,
            SnapshotLineageClass::CurrentHead,
        ),
        BasisResolutionMode::RuntimeDirect,
    )
}

pub fn snapshot_resolution_report(basis: &ResolvedSnapshotBasis) -> SnapshotResolutionReport {
    SnapshotResolutionReport::from_resolved_basis(basis)
}

pub(crate) fn bridge_snapshot_evidence_identity(
    snapshot_identity: &TruthSnapshotIdentity,
) -> Result<ForgeQueryEvidenceIdentity, BasisResolutionError> {
    let Some(parts) = snapshot_identity.relational_snapshot_parts() else {
        return Err(BasisResolutionError::UnsupportedBasisKind);
    };
    Ok(
        crate::memory_workspace::ForgeQuerySnapshotIdentity::from_relational_snapshot(parts)
            .evidence_identity(),
    )
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
