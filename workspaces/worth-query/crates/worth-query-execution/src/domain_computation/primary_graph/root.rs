use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};

use worth_query_installation::facade::ApplicationSchemaBindingIdentity;
use worth_relational::facade::indexes::{DerivedIndexDefinition, DerivedIndexKind};
use worth_relational::facade::runtime::RelationalRuntime;

use super::schema_layout::WorthQueryPrimaryGraphLayout;

/// Execution-owned primary logical graph for one installed application schema.
///
/// This surface exposes graph identity rather than raw Relational access.
/// Query integration code receives a separate hidden handle so product
/// consumers cannot bypass installed Query authority.
pub struct WorthQueryPrimaryGraph {
    runtime_authority:
        crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity,
    relational_runtime_instance_id: u64,
    binding_identity: ApplicationSchemaBindingIdentity,
    pub(super) layout: Arc<WorthQueryPrimaryGraphLayout>,
    runtime: Arc<Mutex<RelationalRuntime>>,
    aggregate_projections: Arc<Mutex<super::aggregate_projection::WorthQueryAggregateProjections>>,
}

impl WorthQueryPrimaryGraph {
    pub(super) fn new(
        runtime_authority: crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity,
        binding_identity: ApplicationSchemaBindingIdentity,
        mut layout: WorthQueryPrimaryGraphLayout,
        mut runtime: RelationalRuntime,
    ) -> Self {
        let runtime_snapshot = runtime.snapshots().snapshot();
        let relational_runtime_instance_id = runtime_snapshot.runtime_instance_id;
        runtime.snapshots().release_snapshot(&runtime_snapshot);
        let mut indexes_by_locator = BTreeMap::new();
        for (binding, binding_layout) in layout.principal_bindings_mut() {
            let installed = runtime.index_authority().register(DerivedIndexDefinition {
                index_id: worth_relational::facade::indexes::DerivedIndexId(0),
                name: format!("application-principal.{binding}"),
                kind: DerivedIndexKind::EntityField {
                    field_locator: binding_layout.identity_locator.clone(),
                },
                branch_scoped: false,
            });
            binding_layout.index_id = installed.index_id;
            indexes_by_locator.insert(binding_layout.identity_locator.clone(), installed.index_id);
        }
        for ((entity, aspect, field), field_layout) in layout.equality_fields_mut() {
            let index_id = indexes_by_locator
                .get(&field_layout.locator)
                .copied()
                .unwrap_or_else(|| {
                    let installed = runtime.index_authority().register(DerivedIndexDefinition {
                        index_id: worth_relational::facade::indexes::DerivedIndexId(0),
                        name: format!("application-entity.{entity}.{aspect}.{field}"),
                        kind: DerivedIndexKind::EntityField {
                            field_locator: field_layout.locator.clone(),
                        },
                        branch_scoped: false,
                    });
                    indexes_by_locator.insert(field_layout.locator.clone(), installed.index_id);
                    installed.index_id
                });
            field_layout.equality_index_id = Some(index_id);
        }
        layout.register_continuation_orderings(|definition| {
            runtime.index_authority().register(definition).index_id
        });
        layout.register_capability_grant_joins(|definition| {
            runtime.index_authority().register(definition).index_id
        });
        let provider_idempotency = layout.provider_idempotency_mut();
        let installed = runtime.index_authority().register(DerivedIndexDefinition {
            index_id: worth_relational::facade::indexes::DerivedIndexId(0),
            name: "worth-query-provider.idempotency-key".to_owned(),
            kind: DerivedIndexKind::EntityField {
                field_locator: provider_idempotency.key_locator.clone(),
            },
            branch_scoped: false,
        });
        provider_idempotency.key_index_id = installed.index_id;
        let aftermath_causality = layout.provider_aftermath_causality_mut();
        let installed = runtime.index_authority().register(DerivedIndexDefinition {
            index_id: worth_relational::facade::indexes::DerivedIndexId(0),
            name: "worth-query-provider.aftermath-causality-key".to_owned(),
            kind: DerivedIndexKind::EntityField {
                field_locator: aftermath_causality.key_locator.clone(),
            },
            branch_scoped: false,
        });
        aftermath_causality.key_index_id = installed.index_id;
        Self {
            runtime_authority,
            relational_runtime_instance_id,
            binding_identity,
            layout: Arc::new(layout),
            runtime: Arc::new(Mutex::new(runtime)),
            aggregate_projections: Arc::new(Mutex::new(
                super::aggregate_projection::WorthQueryAggregateProjections::default(),
            )),
        }
    }

    pub fn binding_identity(&self) -> &ApplicationSchemaBindingIdentity {
        &self.binding_identity
    }

    pub(super) const fn runtime_authority(
        &self,
    ) -> crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity {
        self.runtime_authority
    }

    pub(super) const fn relational_runtime_instance_id(&self) -> u64 {
        self.relational_runtime_instance_id
    }

    pub(in crate::domain_computation) fn layout(&self) -> &WorthQueryPrimaryGraphLayout {
        &self.layout
    }

    pub(in crate::domain_computation) fn retain_layout(&self) -> Arc<WorthQueryPrimaryGraphLayout> {
        Arc::clone(&self.layout)
    }

    pub(crate) fn integration_handle(&self) -> WorthQueryPrimaryGraphIntegrationHandle {
        let principal_identity_index_ids = self
            .layout
            .principal_bindings()
            .map(|(_, binding)| binding.index_id)
            .collect::<BTreeSet<_>>();
        let primary_index_ids = principal_identity_index_ids
            .iter()
            .copied()
            .chain(self.layout.equality_index_ids())
            .chain(self.layout.continuation_ordering_index_ids())
            .chain(self.layout.capability_grant_join_index_ids())
            .chain(std::iter::once(
                self.layout.provider_idempotency().key_index_id,
            ))
            .chain(std::iter::once(
                self.layout.provider_aftermath_causality().key_index_id,
            ))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
            .into();
        WorthQueryPrimaryGraphIntegrationHandle {
            runtime: Arc::clone(&self.runtime),
            layout: Arc::clone(&self.layout),
            primary_index_ids,
            aggregate_projections: Arc::clone(&self.aggregate_projections),
        }
    }

    pub(in crate::domain_computation) fn query_session_port(
        &self,
    ) -> crate::domain_computation::provider_session::WorthQueryGraphReadOwnerPort {
        crate::domain_computation::provider_session::WorthQueryGraphReadOwnerPort::new(
            self.binding_identity.clone(),
            self.integration_handle(),
        )
    }
}

impl std::fmt::Debug for WorthQueryPrimaryGraph {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryPrimaryGraph")
            .field("binding_identity", &self.binding_identity)
            .finish_non_exhaustive()
    }
}

/// Hidden composition handle for sharing the one primary graph with Query's
/// existing runtime backend.
#[doc(hidden)]
#[derive(Clone)]
pub struct WorthQueryPrimaryGraphIntegrationHandle {
    pub(super) runtime: Arc<Mutex<RelationalRuntime>>,
    pub(super) layout: Arc<WorthQueryPrimaryGraphLayout>,
    pub(super) primary_index_ids: Arc<[worth_relational::facade::indexes::DerivedIndexId]>,
    pub(super) aggregate_projections:
        Arc<Mutex<super::aggregate_projection::WorthQueryAggregateProjections>>,
}

impl WorthQueryPrimaryGraphIntegrationHandle {
    #[doc(hidden)]
    pub fn with_runtime<T>(&self, read: impl FnOnce(&RelationalRuntime) -> T) -> T {
        let runtime = self
            .runtime
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        read(&runtime)
    }

    pub(crate) fn with_runtime_mut<T>(
        &self,
        mutate: impl FnOnce(&mut RelationalRuntime) -> T,
    ) -> T {
        let mut runtime = self
            .runtime
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        mutate(&mut runtime)
    }

    pub(in crate::domain_computation) fn with_query_runtime_mut<T>(
        &self,
        read: impl FnOnce(&mut RelationalRuntime, &WorthQueryPrimaryGraphLayout) -> T,
    ) -> T {
        let mut runtime = self
            .runtime
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        read(&mut runtime, &self.layout)
    }

    pub(crate) fn relational_bridge_source(
        &self,
    ) -> worth_relational::facade::bridge::RuntimeBridgeRelationalSource {
        worth_relational::facade::bridge::RuntimeBridgeRelationalSource::for_shared_graph_role(
            Arc::clone(&self.runtime),
            "primary",
        )
        .expect("the installed primary graph role is canonical")
    }

    pub(crate) fn relational_execution_basis_source(
        &self,
    ) -> worth_relational::facade::runtime::RelationalApplicationCommitBasisSource {
        worth_relational::facade::runtime::RelationalApplicationCommitBasisSource::for_shared_runtime(
            Arc::clone(&self.runtime),
        )
    }

    pub(crate) fn ensure_primary_indexes_current(
        &self,
        runtime: &mut RelationalRuntime,
    ) -> Result<(), &'static str> {
        let Some(head) = runtime.history().latest_commit().cloned() else {
            return Ok(());
        };
        self.ensure_primary_indexes_for_commit(runtime, head)
    }

    pub(crate) fn ensure_primary_indexes_current_for_branch(
        &self,
        runtime: &mut RelationalRuntime,
        branch: &worth_relational::facade::history::BranchId,
    ) -> Result<(), &'static str> {
        let head = runtime
            .history()
            .branch_head(branch)
            .cloned()
            .ok_or("primary graph branch has no authoritative head")?;
        self.ensure_primary_indexes_for_commit(runtime, head)
    }

    pub(crate) fn ensure_primary_indexes_for_version(
        &self,
        runtime: &mut RelationalRuntime,
        version: worth_relational::facade::identity::VersionId,
    ) -> Result<(), &'static str> {
        let commit = runtime
            .history()
            .committed_version(version)
            .ok_or("application-query basis version has no retained commit")?
            .commit()
            .clone();
        self.ensure_primary_indexes_for_commit(runtime, commit)
    }

    fn ensure_primary_indexes_for_commit(
        &self,
        runtime: &mut RelationalRuntime,
        head: worth_relational::facade::history::CommitReference,
    ) -> Result<(), &'static str> {
        let branch = head.branch_id.clone();
        let current = self.primary_index_ids.iter().all(|index_id| {
            runtime
                .index_access()
                .latest_generation(*index_id, &branch)
                .is_some_and(|generation| generation.source_commit_id == head.commit_id)
        });
        if current {
            return Ok(());
        }
        let build = runtime.index_authority().build_for_commit(
            worth_relational::facade::indexes::DerivedIndexBuildRequest {
                source_commit_id: head.commit_id,
                branch_id: branch,
                index_ids: self.primary_index_ids.to_vec(),
            },
        );
        if build.failed_indexes.is_empty()
            && build.generations.len() == self.primary_index_ids.len()
        {
            Ok(())
        } else {
            Err("primary graph indexes could not recover to the authoritative head")
        }
    }
}
