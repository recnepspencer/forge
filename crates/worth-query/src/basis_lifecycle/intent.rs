use super::proofs::{BasisIntentDenial, NormalizedBasisIntent};
use super::taxonomy::{
    BasisAuthorityPosture, BasisEligibilityDenialCause, BasisFamily, BasisIntentDenialKind,
    BasisLifecyclePosture, BasisScopePosture, BasisVisibilityPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RawBasisIntent {
    CurrentHead,
    BranchHead {
        branch_identity: String,
        accessible: bool,
    },
    BranchSnapshot {
        branch_identity: String,
        snapshot_identity: String,
    },
    Preview {
        preview_identity: String,
        stale: bool,
    },
    PreviewDerived {
        preview_identity: String,
        source_basis_identity: String,
    },
    RuntimeSnapshot {
        snapshot_identity: String,
        lower_runtime_binding_digest: Option<String>,
    },
    HistoricalSnapshot {
        snapshot_identity: String,
        replay_supported: bool,
    },
    HistoricalCommit {
        commit_identity: String,
        replay_supported: bool,
    },
    TenantScoped {
        tenant_identity: String,
        branch_identity: String,
        schema_identity: String,
        tenant_schema_matches: bool,
    },
    PolicyScoped {
        policy_digest: String,
        tenant_identity: String,
        branch_identity: String,
        schema_identity: String,
        tenant_schema_matches: bool,
        policy_masks_operation: bool,
        advisory_visibility: bool,
    },
    StoreBacked {
        store_basis_identity: String,
    },
    DurableReload {
        reload_identity: String,
    },
    TemporalFuture {
        temporal_identity: String,
    },
    AsyncResourceFuture {
        resource_identity: String,
    },
    Malformed {
        reason: &'static str,
    },
    Ambiguous {
        reason: &'static str,
    },
}

pub fn normalize_raw_basis_intent(
    raw: RawBasisIntent,
    operation_lane: impl Into<String>,
) -> Result<NormalizedBasisIntent, BasisIntentDenial> {
    let operation_lane = operation_lane.into();
    match raw {
        RawBasisIntent::CurrentHead => Ok(NormalizedBasisIntent::new(
            BasisFamily::CurrentHead,
            BasisAuthorityPosture::Runtime,
            BasisScopePosture::Global,
            BasisVisibilityPosture::Full,
            BasisLifecyclePosture::Current,
            operation_lane,
            None,
            None,
            Some("runtime-current-head".to_string()),
            "raw.current_head",
        )),
        RawBasisIntent::BranchHead {
            branch_identity,
            accessible,
        } => Ok(NormalizedBasisIntent::new_with_denial_cause(
            BasisFamily::BranchHead,
            BasisAuthorityPosture::RelationalFacade,
            BasisScopePosture::Branch,
            if accessible {
                BasisVisibilityPosture::Full
            } else {
                BasisVisibilityPosture::PolicyMasked
            },
            BasisLifecyclePosture::Current,
            operation_lane,
            None,
            None,
            (!accessible).then_some(BasisEligibilityDenialCause::Inaccessible),
            Some(format!("relational-branch:{branch_identity}")),
            "raw.branch_head",
        )),
        RawBasisIntent::BranchSnapshot {
            branch_identity,
            snapshot_identity,
        } => Ok(NormalizedBasisIntent::new(
            BasisFamily::BranchSnapshot,
            BasisAuthorityPosture::RelationalFacade,
            BasisScopePosture::Snapshot,
            BasisVisibilityPosture::Full,
            BasisLifecyclePosture::SnapshotPinned,
            operation_lane,
            None,
            None,
            Some(format!(
                "relational-branch:{branch_identity}:snapshot:{snapshot_identity}"
            )),
            "raw.branch_snapshot",
        )),
        RawBasisIntent::Preview {
            preview_identity,
            stale,
        } => Ok(NormalizedBasisIntent::new(
            BasisFamily::Preview,
            BasisAuthorityPosture::RuntimeBridgeFacade,
            BasisScopePosture::Preview,
            BasisVisibilityPosture::Full,
            if stale {
                BasisLifecyclePosture::PreviewStale
            } else {
                BasisLifecyclePosture::PreviewActive
            },
            operation_lane,
            None,
            None,
            Some(format!("bridge-preview:{preview_identity}")),
            "raw.preview",
        )),
        RawBasisIntent::PreviewDerived {
            preview_identity,
            source_basis_identity,
        } => Ok(NormalizedBasisIntent::new(
            BasisFamily::PreviewDerived,
            BasisAuthorityPosture::RuntimeBridgeFacade,
            BasisScopePosture::Preview,
            BasisVisibilityPosture::Advisory,
            BasisLifecyclePosture::PreviewActive,
            operation_lane,
            None,
            None,
            Some(format!(
                "bridge-preview:{preview_identity}:source:{source_basis_identity}"
            )),
            "raw.preview_derived",
        )),
        RawBasisIntent::RuntimeSnapshot {
            snapshot_identity,
            lower_runtime_binding_digest,
        } => Ok(NormalizedBasisIntent::new(
            BasisFamily::RuntimeSnapshot,
            BasisAuthorityPosture::RuntimeBridgeFacade,
            BasisScopePosture::Snapshot,
            BasisVisibilityPosture::Full,
            BasisLifecyclePosture::SnapshotPinned,
            operation_lane,
            None,
            None,
            lower_runtime_binding_digest
                .or_else(|| Some(format!("missing-runtime-snapshot:{snapshot_identity}"))),
            "raw.runtime_snapshot",
        )),
        RawBasisIntent::HistoricalSnapshot {
            snapshot_identity,
            replay_supported,
        } => Ok(NormalizedBasisIntent::new(
            BasisFamily::HistoricalSnapshot,
            BasisAuthorityPosture::RelationalFacade,
            BasisScopePosture::Snapshot,
            if replay_supported {
                BasisVisibilityPosture::Full
            } else {
                BasisVisibilityPosture::Advisory
            },
            BasisLifecyclePosture::HistoricalRetained,
            operation_lane,
            None,
            None,
            Some(format!("relational-historical:{snapshot_identity}")),
            "raw.historical_snapshot",
        )),
        RawBasisIntent::HistoricalCommit {
            commit_identity,
            replay_supported,
        } => Ok(NormalizedBasisIntent::new(
            BasisFamily::HistoricalCommit,
            BasisAuthorityPosture::RelationalFacade,
            BasisScopePosture::Snapshot,
            if replay_supported {
                BasisVisibilityPosture::Full
            } else {
                BasisVisibilityPosture::Advisory
            },
            BasisLifecyclePosture::HistoricalRetained,
            operation_lane,
            None,
            None,
            Some(format!("relational-historical-commit:{commit_identity}")),
            "raw.historical_commit",
        )),
        RawBasisIntent::TenantScoped {
            tenant_identity,
            branch_identity,
            schema_identity,
            tenant_schema_matches,
        } => Ok(NormalizedBasisIntent::new_with_denial_cause(
            BasisFamily::TenantScoped,
            BasisAuthorityPosture::RelationalFacade,
            BasisScopePosture::Tenant,
            if tenant_schema_matches {
                BasisVisibilityPosture::Full
            } else {
                BasisVisibilityPosture::PolicyMasked
            },
            BasisLifecyclePosture::Current,
            operation_lane,
            None,
            Some(format!("tenant:{tenant_identity}:schema:{schema_identity}")),
            (!tenant_schema_matches).then_some(BasisEligibilityDenialCause::SchemaIncompatible),
            Some(format!("tenant:{tenant_identity}:branch:{branch_identity}")),
            "raw.tenant_scoped",
        )),
        RawBasisIntent::PolicyScoped {
            policy_digest,
            tenant_identity,
            branch_identity,
            schema_identity,
            tenant_schema_matches,
            policy_masks_operation,
            advisory_visibility,
        } => Ok(NormalizedBasisIntent::new_with_denial_cause(
            BasisFamily::PolicyScoped,
            BasisAuthorityPosture::RelationalFacade,
            BasisScopePosture::PolicyTenant,
            policy_visibility(
                policy_masks_operation,
                advisory_visibility,
                tenant_schema_matches,
            ),
            BasisLifecyclePosture::Current,
            operation_lane,
            Some(policy_digest),
            Some(format!("tenant:{tenant_identity}:schema:{schema_identity}")),
            policy_denial_cause(policy_masks_operation, tenant_schema_matches),
            Some(format!("tenant:{tenant_identity}:branch:{branch_identity}")),
            "raw.policy_scoped",
        )),
        RawBasisIntent::StoreBacked {
            store_basis_identity,
        } => Ok(NormalizedBasisIntent::new(
            BasisFamily::StoreBacked,
            BasisAuthorityPosture::StoreDeferred,
            BasisScopePosture::FutureNeighbor,
            BasisVisibilityPosture::Deferred,
            BasisLifecyclePosture::DeferredFuture,
            operation_lane,
            None,
            None,
            Some(format!("store:{store_basis_identity}")),
            "raw.store_backed",
        )),
        RawBasisIntent::DurableReload { reload_identity } => Ok(NormalizedBasisIntent::new(
            BasisFamily::DurableReload,
            BasisAuthorityPosture::StoreDeferred,
            BasisScopePosture::FutureNeighbor,
            BasisVisibilityPosture::Deferred,
            BasisLifecyclePosture::DeferredFuture,
            operation_lane,
            None,
            None,
            Some(format!("durable-reload:{reload_identity}")),
            "raw.durable_reload",
        )),
        RawBasisIntent::TemporalFuture { .. } => Err(BasisIntentDenial::new(
            BasisIntentDenialKind::TemporalDeferred,
            "temporal basis remains deferred to the temporal milestone",
        )),
        RawBasisIntent::AsyncResourceFuture { .. } => Err(BasisIntentDenial::new(
            BasisIntentDenialKind::AsyncResourceDeferred,
            "async/resource basis remains deferred to the async resource milestone",
        )),
        RawBasisIntent::Malformed { reason } => Err(BasisIntentDenial::new(
            BasisIntentDenialKind::Malformed,
            reason,
        )),
        RawBasisIntent::Ambiguous { reason } => Err(BasisIntentDenial::new(
            BasisIntentDenialKind::Ambiguous,
            reason,
        )),
    }
}

fn policy_visibility(
    policy_masks_operation: bool,
    advisory_visibility: bool,
    tenant_schema_matches: bool,
) -> BasisVisibilityPosture {
    if policy_masks_operation || !tenant_schema_matches {
        BasisVisibilityPosture::PolicyMasked
    } else if advisory_visibility {
        BasisVisibilityPosture::Advisory
    } else {
        BasisVisibilityPosture::Full
    }
}

fn policy_denial_cause(
    policy_masks_operation: bool,
    tenant_schema_matches: bool,
) -> Option<BasisEligibilityDenialCause> {
    if !tenant_schema_matches {
        Some(BasisEligibilityDenialCause::TenantMismatched)
    } else if policy_masks_operation {
        Some(BasisEligibilityDenialCause::PolicyMasked)
    } else {
        None
    }
}
