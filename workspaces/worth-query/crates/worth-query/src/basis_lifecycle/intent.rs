use super::normalized_intent::{
    NormalizedBasisAdmissionOutcome, NormalizedBasisAuthorityBindings, NormalizedBasisCoordinates,
    NormalizedBasisIntent, NormalizedBasisIntentInput,
};
use super::proofs::BasisIntentDenial;
#[cfg(test)]
use super::taxonomy::BasisIntentDenialKind;
use super::taxonomy::{
    BasisAuthorityPosture, BasisEligibilityDenialCause, BasisFamily, BasisLifecyclePosture,
    BasisScopePosture, BasisVisibilityPosture,
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
    #[cfg(test)]
    TemporalFuture {
        temporal_identity: String,
    },
}

pub fn normalize_raw_basis_intent(
    raw: RawBasisIntent,
    operation_lane: impl Into<String>,
) -> Result<NormalizedBasisIntent, BasisIntentDenial> {
    let operation_lane = operation_lane.into();
    match raw {
        RawBasisIntent::CurrentHead => Ok(NormalizedBasisIntent::new(
            NormalizedBasisIntentInput::admitted(
                NormalizedBasisCoordinates {
                    family: BasisFamily::CurrentHead,
                    authority: BasisAuthorityPosture::Runtime,
                    scope: BasisScopePosture::Global,
                    visibility: BasisVisibilityPosture::Full,
                    lifecycle: BasisLifecyclePosture::Current,
                    operation_lane,
                },
                NormalizedBasisAuthorityBindings::lower_runtime(Some(
                    "runtime-current-head".to_string(),
                )),
                "raw.current_head",
            ),
        )),
        RawBasisIntent::BranchHead {
            branch_identity,
            accessible,
        } => {
            let coordinates = NormalizedBasisCoordinates {
                family: BasisFamily::BranchHead,
                authority: BasisAuthorityPosture::RelationalFacade,
                scope: BasisScopePosture::Branch,
                visibility: if accessible {
                    BasisVisibilityPosture::Full
                } else {
                    BasisVisibilityPosture::PolicyMasked
                },
                lifecycle: BasisLifecyclePosture::Current,
                operation_lane,
            };
            let authority_bindings = NormalizedBasisAuthorityBindings::lower_runtime(Some(
                format!("relational-branch:{branch_identity}"),
            ));
            let input = if accessible {
                NormalizedBasisIntentInput::admitted(
                    coordinates,
                    authority_bindings,
                    "raw.branch_head",
                )
            } else {
                NormalizedBasisIntentInput::denied(
                    coordinates,
                    authority_bindings,
                    BasisEligibilityDenialCause::Inaccessible,
                    "raw.branch_head",
                )
            };
            Ok(NormalizedBasisIntent::new(input))
        }
        RawBasisIntent::BranchSnapshot {
            branch_identity,
            snapshot_identity,
        } => Ok(NormalizedBasisIntent::new(
            NormalizedBasisIntentInput::admitted(
                NormalizedBasisCoordinates {
                    family: BasisFamily::BranchSnapshot,
                    authority: BasisAuthorityPosture::RelationalFacade,
                    scope: BasisScopePosture::Snapshot,
                    visibility: BasisVisibilityPosture::Full,
                    lifecycle: BasisLifecyclePosture::SnapshotPinned,
                    operation_lane,
                },
                NormalizedBasisAuthorityBindings::lower_runtime(Some(format!(
                    "relational-branch:{branch_identity}:snapshot:{snapshot_identity}"
                ))),
                "raw.branch_snapshot",
            ),
        )),
        RawBasisIntent::Preview {
            preview_identity,
            stale,
        } => Ok(NormalizedBasisIntent::new(
            NormalizedBasisIntentInput::admitted(
                NormalizedBasisCoordinates {
                    family: BasisFamily::Preview,
                    authority: BasisAuthorityPosture::RuntimeBridgeFacade,
                    scope: BasisScopePosture::Preview,
                    visibility: BasisVisibilityPosture::Full,
                    lifecycle: if stale {
                        BasisLifecyclePosture::PreviewStale
                    } else {
                        BasisLifecyclePosture::PreviewActive
                    },
                    operation_lane,
                },
                NormalizedBasisAuthorityBindings::lower_runtime(Some(format!(
                    "bridge-preview:{preview_identity}"
                ))),
                "raw.preview",
            ),
        )),
        RawBasisIntent::PreviewDerived {
            preview_identity,
            source_basis_identity,
        } => Ok(NormalizedBasisIntent::new(
            NormalizedBasisIntentInput::admitted(
                NormalizedBasisCoordinates {
                    family: BasisFamily::PreviewDerived,
                    authority: BasisAuthorityPosture::RuntimeBridgeFacade,
                    scope: BasisScopePosture::Preview,
                    visibility: BasisVisibilityPosture::Advisory,
                    lifecycle: BasisLifecyclePosture::PreviewActive,
                    operation_lane,
                },
                NormalizedBasisAuthorityBindings::lower_runtime(Some(format!(
                    "bridge-preview:{preview_identity}:source:{source_basis_identity}"
                ))),
                "raw.preview_derived",
            ),
        )),
        RawBasisIntent::RuntimeSnapshot {
            snapshot_identity,
            lower_runtime_binding_digest,
        } => Ok(NormalizedBasisIntent::new(
            NormalizedBasisIntentInput::admitted(
                NormalizedBasisCoordinates {
                    family: BasisFamily::RuntimeSnapshot,
                    authority: BasisAuthorityPosture::RuntimeBridgeFacade,
                    scope: BasisScopePosture::Snapshot,
                    visibility: BasisVisibilityPosture::Full,
                    lifecycle: BasisLifecyclePosture::SnapshotPinned,
                    operation_lane,
                },
                NormalizedBasisAuthorityBindings::lower_runtime(
                    lower_runtime_binding_digest
                        .or_else(|| Some(format!("missing-runtime-snapshot:{snapshot_identity}"))),
                ),
                "raw.runtime_snapshot",
            ),
        )),
        RawBasisIntent::HistoricalSnapshot {
            snapshot_identity,
            replay_supported,
        } => Ok(NormalizedBasisIntent::new(
            NormalizedBasisIntentInput::admitted(
                NormalizedBasisCoordinates {
                    family: BasisFamily::HistoricalSnapshot,
                    authority: BasisAuthorityPosture::RelationalFacade,
                    scope: BasisScopePosture::Snapshot,
                    visibility: replay_visibility(replay_supported),
                    lifecycle: BasisLifecyclePosture::HistoricalRetained,
                    operation_lane,
                },
                NormalizedBasisAuthorityBindings::lower_runtime(Some(format!(
                    "relational-historical:{snapshot_identity}"
                ))),
                "raw.historical_snapshot",
            ),
        )),
        RawBasisIntent::HistoricalCommit {
            commit_identity,
            replay_supported,
        } => Ok(NormalizedBasisIntent::new(
            NormalizedBasisIntentInput::admitted(
                NormalizedBasisCoordinates {
                    family: BasisFamily::HistoricalCommit,
                    authority: BasisAuthorityPosture::RelationalFacade,
                    scope: BasisScopePosture::Snapshot,
                    visibility: replay_visibility(replay_supported),
                    lifecycle: BasisLifecyclePosture::HistoricalRetained,
                    operation_lane,
                },
                NormalizedBasisAuthorityBindings::lower_runtime(Some(format!(
                    "relational-historical-commit:{commit_identity}"
                ))),
                "raw.historical_commit",
            ),
        )),
        RawBasisIntent::TenantScoped {
            tenant_identity,
            branch_identity,
            schema_identity,
            tenant_schema_matches,
        } => Ok(NormalizedBasisIntent::new(
            NormalizedBasisIntentInput::with_admission(
                NormalizedBasisCoordinates {
                    family: BasisFamily::TenantScoped,
                    authority: BasisAuthorityPosture::RelationalFacade,
                    scope: BasisScopePosture::Tenant,
                    visibility: if tenant_schema_matches {
                        BasisVisibilityPosture::Full
                    } else {
                        BasisVisibilityPosture::PolicyMasked
                    },
                    lifecycle: BasisLifecyclePosture::Current,
                    operation_lane,
                },
                NormalizedBasisAuthorityBindings {
                    tenant_schema_digest: Some(format!(
                        "tenant:{tenant_identity}:schema:{schema_identity}"
                    )),
                    lower_runtime_binding_digest: Some(format!(
                        "tenant:{tenant_identity}:branch:{branch_identity}"
                    )),
                    ..Default::default()
                },
                tenant_schema_admission(tenant_schema_matches),
                "raw.tenant_scoped",
            ),
        )),
        RawBasisIntent::PolicyScoped {
            policy_digest,
            tenant_identity,
            branch_identity,
            schema_identity,
            tenant_schema_matches,
            policy_masks_operation,
            advisory_visibility,
        } => Ok(NormalizedBasisIntent::new(
            NormalizedBasisIntentInput::with_admission(
                NormalizedBasisCoordinates {
                    family: BasisFamily::PolicyScoped,
                    authority: BasisAuthorityPosture::RelationalFacade,
                    scope: BasisScopePosture::PolicyTenant,
                    visibility: policy_visibility(
                        policy_masks_operation,
                        advisory_visibility,
                        tenant_schema_matches,
                    ),
                    lifecycle: BasisLifecyclePosture::Current,
                    operation_lane,
                },
                NormalizedBasisAuthorityBindings {
                    policy_digest: Some(policy_digest),
                    tenant_schema_digest: Some(format!(
                        "tenant:{tenant_identity}:schema:{schema_identity}"
                    )),
                    lower_runtime_binding_digest: Some(format!(
                        "tenant:{tenant_identity}:branch:{branch_identity}"
                    )),
                },
                policy_admission(policy_masks_operation, tenant_schema_matches),
                "raw.policy_scoped",
            ),
        )),
        RawBasisIntent::StoreBacked {
            store_basis_identity,
        } => Ok(NormalizedBasisIntent::new(
            NormalizedBasisIntentInput::admitted(
                NormalizedBasisCoordinates {
                    family: BasisFamily::StoreBacked,
                    authority: BasisAuthorityPosture::StoreDeferred,
                    scope: BasisScopePosture::FutureNeighbor,
                    visibility: BasisVisibilityPosture::Deferred,
                    lifecycle: BasisLifecyclePosture::DeferredFuture,
                    operation_lane,
                },
                NormalizedBasisAuthorityBindings::lower_runtime(Some(format!(
                    "store:{store_basis_identity}"
                ))),
                "raw.store_backed",
            ),
        )),
        RawBasisIntent::DurableReload { reload_identity } => Ok(NormalizedBasisIntent::new(
            NormalizedBasisIntentInput::admitted(
                NormalizedBasisCoordinates {
                    family: BasisFamily::DurableReload,
                    authority: BasisAuthorityPosture::StoreDeferred,
                    scope: BasisScopePosture::FutureNeighbor,
                    visibility: BasisVisibilityPosture::Deferred,
                    lifecycle: BasisLifecyclePosture::DeferredFuture,
                    operation_lane,
                },
                NormalizedBasisAuthorityBindings::lower_runtime(Some(format!(
                    "durable-reload:{reload_identity}"
                ))),
                "raw.durable_reload",
            ),
        )),
        #[cfg(test)]
        RawBasisIntent::TemporalFuture { .. } => Err(BasisIntentDenial::new(
            BasisIntentDenialKind::TemporalDeferred,
            "temporal basis remains deferred to the temporal milestone",
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

fn policy_admission(
    policy_masks_operation: bool,
    tenant_schema_matches: bool,
) -> NormalizedBasisAdmissionOutcome {
    if !tenant_schema_matches {
        NormalizedBasisAdmissionOutcome::Denied(BasisEligibilityDenialCause::TenantMismatched)
    } else if policy_masks_operation {
        NormalizedBasisAdmissionOutcome::Denied(BasisEligibilityDenialCause::PolicyMasked)
    } else {
        NormalizedBasisAdmissionOutcome::Admitted
    }
}

fn tenant_schema_admission(matches: bool) -> NormalizedBasisAdmissionOutcome {
    if matches {
        NormalizedBasisAdmissionOutcome::Admitted
    } else {
        NormalizedBasisAdmissionOutcome::Denied(BasisEligibilityDenialCause::SchemaIncompatible)
    }
}

fn replay_visibility(replay_supported: bool) -> BasisVisibilityPosture {
    if replay_supported {
        BasisVisibilityPosture::Full
    } else {
        BasisVisibilityPosture::Advisory
    }
}
