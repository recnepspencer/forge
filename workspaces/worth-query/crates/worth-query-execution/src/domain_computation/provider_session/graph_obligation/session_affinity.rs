//! Exact runtime, subject, access-context, branch, and basis affinity for one graph-work session.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use worth_foundational::facade::CanonicalDigestId;
use worth_query_admission::integration::WorthQueryAdmittedGraphWorkPlan;
use worth_query_declaration::facade::application_schema::ApplicationSchemaBindingIdentity;
use worth_query_installation::facade::{
    WorthQueryInstalledGraphObligationSetIdentity, WorthQueryInstalledGraphParticipationAuthority,
};

use crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity;

use super::branch_affinity::WorthQueryGraphWorkBranchAffinity;
use super::session_start::WorthQueryGraphWorkSessionStartDenial;

static NEXT_GRAPH_WORK_RUN: AtomicU64 = AtomicU64::new(1);

pub(in crate::domain_computation) enum WorthQueryGraphWorkBasisAffinity {
    Query {
        branch: worth_relational::facade::history::BranchId,
        relational_runtime: u64,
        snapshot: u64,
        lease: u64,
    },
    Mutation {
        branch: worth_relational::facade::history::BranchId,
        relational_runtime: u64,
        snapshot: u64,
        version: u64,
    },
}

pub(in crate::domain_computation) enum WorthQueryGraphWorkAccessContextAffinity {
    Entity(worth_relational::facade::identity::EntityId),
    InstalledCapability(CanonicalDigestId),
}

impl WorthQueryGraphWorkAccessContextAffinity {
    pub(in crate::domain_computation) const fn entity(
        entity: worth_relational::facade::identity::EntityId,
    ) -> Self {
        Self::Entity(entity)
    }

    pub(in crate::domain_computation) const fn installed_capability(
        identity: CanonicalDigestId,
    ) -> Self {
        Self::InstalledCapability(identity)
    }
}

impl WorthQueryGraphWorkBasisAffinity {
    pub(in crate::domain_computation) fn query(
        identity: &worth_relational::facade::runtime::RelationalExecutionBasisIdentity,
        branch: &WorthQueryGraphWorkBranchAffinity,
    ) -> Result<Self, WorthQueryGraphWorkSessionStartDenial> {
        branch
            .admits_execution_basis(identity)
            .then(|| Self::Query {
                branch: branch.relational_branch().clone(),
                relational_runtime: identity.runtime_instance_id(),
                snapshot: identity.snapshot_id().0,
                lease: identity.lease_ordinal(),
            })
            .ok_or(WorthQueryGraphWorkSessionStartDenial::BranchMismatch)
    }

    pub(in crate::domain_computation) fn mutation(
        snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
        branch: &WorthQueryGraphWorkBranchAffinity,
    ) -> Result<Self, WorthQueryGraphWorkSessionStartDenial> {
        branch
            .admits_snapshot(snapshot)
            .then(|| Self::Mutation {
                branch: branch.relational_branch().clone(),
                relational_runtime: snapshot.runtime_instance_id,
                snapshot: snapshot.snapshot_id.0,
                version: snapshot.version_id.0,
            })
            .ok_or(WorthQueryGraphWorkSessionStartDenial::BranchMismatch)
    }

    fn branch(&self) -> &worth_relational::facade::history::BranchId {
        match self {
            Self::Query { branch, .. } | Self::Mutation { branch, .. } => branch,
        }
    }

    pub(super) fn encode(
        &self,
        entries: &mut Vec<worth_foundational::facade::CanonicalBasisEntry>,
        domain: worth_foundational::facade::CanonicalBasisDomain,
    ) {
        use super::session_identity::{push_text, push_unsigned};
        match self {
            Self::Query {
                branch: _,
                relational_runtime,
                snapshot,
                lease,
            } => {
                push_text(entries, domain, "basis-kind", "query");
                push_unsigned(entries, domain, "basis-runtime", *relational_runtime);
                push_unsigned(entries, domain, "basis-snapshot", *snapshot);
                push_unsigned(entries, domain, "basis-lease", *lease);
            }
            Self::Mutation {
                branch: _,
                relational_runtime,
                snapshot,
                version,
            } => {
                push_text(entries, domain, "basis-kind", "mutation");
                push_unsigned(entries, domain, "basis-runtime", *relational_runtime);
                push_unsigned(entries, domain, "basis-snapshot", *snapshot);
                push_unsigned(entries, domain, "basis-version", *version);
            }
        }
    }
}

pub(in crate::domain_computation) struct WorthQueryGraphWorkSessionAffinity {
    pub(super) plan_identity: CanonicalDigestId,
    pub(super) obligation_identity: WorthQueryInstalledGraphObligationSetIdentity,
    runtime_authority: WorthQueryRuntimeAuthorityIdentity,
    binding_identity: ApplicationSchemaBindingIdentity,
    subject_authority_identity: Arc<str>,
    principal_entity_id: worth_relational::facade::identity::EntityId,
    access_context: WorthQueryGraphWorkAccessContextAffinity,
    branch: WorthQueryGraphWorkBranchAffinity,
    basis: WorthQueryGraphWorkBasisAffinity,
    provider_identity: Arc<str>,
    managed_run_ordinal: u64,
}

impl WorthQueryGraphWorkSessionAffinity {
    #[allow(clippy::too_many_arguments)]
    pub(in crate::domain_computation) fn new(
        plan: &WorthQueryAdmittedGraphWorkPlan,
        runtime_authority: WorthQueryRuntimeAuthorityIdentity,
        obligation_identity: &WorthQueryInstalledGraphObligationSetIdentity,
        subject_authority_identity: impl Into<Arc<str>>,
        principal_entity_id: worth_relational::facade::identity::EntityId,
        access_context: WorthQueryGraphWorkAccessContextAffinity,
        branch: WorthQueryGraphWorkBranchAffinity,
        basis: WorthQueryGraphWorkBasisAffinity,
        provider: &WorthQueryInstalledGraphParticipationAuthority,
    ) -> Result<Self, WorthQueryGraphWorkSessionStartDenial> {
        let subject_authority_identity = subject_authority_identity.into();
        if plan.obligation_identity() != obligation_identity {
            return Err(WorthQueryGraphWorkSessionStartDenial::ObligationMismatch);
        }
        if provider.runtime_ordinal() != plan.binding_identity().runtime_ordinal()
            || provider.role() != "primary"
        {
            return Err(WorthQueryGraphWorkSessionStartDenial::ProviderMismatch);
        }
        if subject_authority_identity.is_empty() || basis.branch() != branch.relational_branch() {
            return Err(WorthQueryGraphWorkSessionStartDenial::InvalidAffinity);
        }
        let managed_run_ordinal = NEXT_GRAPH_WORK_RUN
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |value| {
                value.checked_add(1)
            })
            .map_err(|_| WorthQueryGraphWorkSessionStartDenial::RunIdentityExhausted)?;
        Ok(Self {
            plan_identity: *plan.identity(),
            obligation_identity: obligation_identity.clone(),
            runtime_authority,
            binding_identity: plan.binding_identity().clone(),
            subject_authority_identity,
            principal_entity_id,
            access_context,
            branch,
            basis,
            provider_identity: Arc::from(provider.authority_identity()),
            managed_run_ordinal,
        })
    }

    pub(super) const fn runtime_authority(&self) -> WorthQueryRuntimeAuthorityIdentity {
        self.runtime_authority
    }

    pub(super) const fn binding_identity(&self) -> &ApplicationSchemaBindingIdentity {
        &self.binding_identity
    }

    pub(super) fn subject_authority_identity(&self) -> &str {
        &self.subject_authority_identity
    }

    pub(super) const fn principal_entity_id(&self) -> worth_relational::facade::identity::EntityId {
        self.principal_entity_id
    }

    pub(super) const fn access_context(&self) -> &WorthQueryGraphWorkAccessContextAffinity {
        &self.access_context
    }

    pub(super) const fn basis(&self) -> &WorthQueryGraphWorkBasisAffinity {
        &self.basis
    }

    pub(super) const fn branch(&self) -> &WorthQueryGraphWorkBranchAffinity {
        &self.branch
    }

    pub(super) fn provider_identity(&self) -> &str {
        &self.provider_identity
    }

    pub(super) const fn managed_run_ordinal(&self) -> u64 {
        self.managed_run_ordinal
    }
}
