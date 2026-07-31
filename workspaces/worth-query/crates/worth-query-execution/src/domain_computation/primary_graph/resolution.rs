use worth_query_admission::facade::authenticated_principal::{
    WorthQueryAuthenticatedExternalPrincipal, WorthQueryRequestInterruption, WorthQueryRequestScope,
};
use worth_query_installation::facade::{
    ApplicationSchema, TypedApplicationIdentityValue, TypedApplicationValue,
    WorthQueryInstalledPrincipalBinding, WorthQueryPrincipalBindingInstallationDenialKind,
};
use worth_relational::facade::indexes::{
    BoundedEntityFieldLookupDenialKind, BoundedEntityFieldLookupRequest, BoundedIndexParityMode,
};

use crate::domain_computation::execution_runtime::WorthQueryExecutionRuntime;

use super::authenticated_principal::WorthQueryResolvedPrincipalEvidence;
use super::freshness::WorthQueryPrincipalFreshnessEvidence;
use super::observations::{
    observe_exact_principal_target, observe_mapping, resolve_principal_target,
};
use super::schema_layout::WorthQueryPrimaryPrincipalBindingLayout;
use super::{
    WorthQueryAuthenticatedPrincipal, WorthQueryPrincipalResolutionDenial,
    WorthQueryPrincipalResolutionDenialKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryPrincipalResolutionMode {
    Ordinary,
    Certification,
}

impl WorthQueryExecutionRuntime {
    pub fn resolve_authenticated_principal<Schema, Binding, Mapping, Principal, PrincipalIdentity>(
        &self,
        installed_binding: &WorthQueryInstalledPrincipalBinding<
            Schema,
            Binding,
            Mapping,
            Principal,
            PrincipalIdentity,
        >,
        external: WorthQueryAuthenticatedExternalPrincipal<Schema>,
        scope: &WorthQueryRequestScope,
        mode: WorthQueryPrincipalResolutionMode,
    ) -> Result<
        WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
        WorthQueryPrincipalResolutionDenial,
    >
    where
        Schema: ApplicationSchema,
        PrincipalIdentity: TypedApplicationIdentityValue,
    {
        admit_request(scope, installed_binding.binding())?;
        if external.is_expired() {
            return Err(resolution_denial(
                WorthQueryPrincipalResolutionDenialKind::ExpiredAuthentication,
                installed_binding.binding(),
            ));
        }
        self.installed_packages()
            .validate_principal_binding(installed_binding)
            .map_err(|denial| {
                resolution_denial(
                    map_binding_denial_kind(denial.kind()),
                    installed_binding.binding(),
                )
            })?;
        let graph = self.primary_graph().ok_or_else(|| {
            resolution_denial(
                WorthQueryPrincipalResolutionDenialKind::PrimaryGraphNotInstalled,
                installed_binding.binding(),
            )
        })?;
        if graph.binding_identity() != installed_binding.binding_identity()
            || external.binding_identity() != installed_binding.binding_identity()
        {
            return Err(resolution_denial(
                WorthQueryPrincipalResolutionDenialKind::ForeignRuntime,
                installed_binding.binding(),
            ));
        }
        let layout = graph
            .layout
            .principal_binding(installed_binding.binding())
            .cloned()
            .ok_or_else(|| {
                resolution_denial(
                    WorthQueryPrincipalResolutionDenialKind::BindingNotInstalled,
                    installed_binding.binding(),
                )
            })?;
        let expected_identity = external.identity().clone().into_foundational_value();
        let evidence = graph.integration_handle().with_runtime_mut(|runtime| {
            let snapshot = runtime.snapshots().snapshot();
            let result = resolve_at_snapshot(
                runtime,
                &snapshot,
                installed_binding.binding(),
                &layout,
                &expected_identity,
                mode,
                self.authority_identity(),
                graph.binding_identity().clone(),
            );
            runtime.snapshots().release_snapshot(&snapshot);
            result
        })?;
        admit_request(scope, installed_binding.binding())?;
        if external.is_expired() {
            return Err(resolution_denial(
                WorthQueryPrincipalResolutionDenialKind::ExpiredAuthentication,
                installed_binding.binding(),
            ));
        }
        Ok(WorthQueryAuthenticatedPrincipal::mint(external, evidence))
    }

    pub fn validate_authenticated_principal<Schema, Principal, PrincipalIdentity>(
        &self,
        principal: &WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
        scope: &WorthQueryRequestScope,
    ) -> Result<(), WorthQueryPrincipalResolutionDenial>
    where
        Schema: ApplicationSchema,
    {
        admit_request(scope, principal.binding())?;
        if principal.is_expired() {
            return Err(resolution_denial(
                WorthQueryPrincipalResolutionDenialKind::ExpiredAuthentication,
                principal.binding(),
            ));
        }
        if principal.runtime_authority() != self.authority_identity() {
            return Err(resolution_denial(
                WorthQueryPrincipalResolutionDenialKind::ForeignRuntime,
                principal.binding(),
            ));
        }
        let graph = self.primary_graph().ok_or_else(|| {
            resolution_denial(
                WorthQueryPrincipalResolutionDenialKind::PrimaryGraphNotInstalled,
                principal.binding(),
            )
        })?;
        validate_current_schema_binding::<Schema>(
            self,
            graph.binding_identity(),
            principal.binding_identity(),
            principal.binding(),
        )?;
        let layout = graph
            .layout
            .principal_binding(principal.binding())
            .cloned()
            .ok_or_else(|| {
                resolution_denial(
                    WorthQueryPrincipalResolutionDenialKind::BindingNotInstalled,
                    principal.binding(),
                )
            })?;
        let expected_identity = principal
            .external_identity()
            .clone()
            .into_foundational_value();
        graph.integration_handle().with_runtime_mut(|runtime| {
            let snapshot = runtime.snapshots().snapshot();
            let result = validate_freshness_at_snapshot(
                runtime,
                &snapshot,
                principal,
                &layout,
                &expected_identity,
            );
            runtime.snapshots().release_snapshot(&snapshot);
            result
        })?;
        admit_request(scope, principal.binding())?;
        if principal.is_expired() {
            return Err(resolution_denial(
                WorthQueryPrincipalResolutionDenialKind::ExpiredAuthentication,
                principal.binding(),
            ));
        }
        Ok(())
    }
}

fn resolve_at_snapshot<PrincipalIdentity>(
    runtime: &worth_relational::facade::runtime::RelationalRuntime,
    snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
    binding: &str,
    layout: &WorthQueryPrimaryPrincipalBindingLayout,
    expected_identity: &worth_foundational::facade::AspectValue,
    mode: WorthQueryPrincipalResolutionMode,
    runtime_authority: crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity,
    binding_identity: worth_query_installation::facade::ApplicationSchemaBindingIdentity,
) -> Result<
    WorthQueryResolvedPrincipalEvidence<PrincipalIdentity>,
    WorthQueryPrincipalResolutionDenial,
>
where
    PrincipalIdentity: TypedApplicationIdentityValue,
{
    let request = BoundedEntityFieldLookupRequest::new(
        snapshot.clone(),
        layout.index_id,
        layout.mapping_kind,
        layout.identity_locator.clone(),
        expected_identity.clone(),
        2,
    )
    .map_err(|denial| map_index_denial(denial.kind(), binding))?;
    let parity_mode = match mode {
        WorthQueryPrincipalResolutionMode::Ordinary => BoundedIndexParityMode::Production,
        WorthQueryPrincipalResolutionMode::Certification => BoundedIndexParityMode::Certification,
    };
    let lookup = runtime
        .index_access()
        .execute_bounded_entity_field_lookup(request, parity_mode)
        .map_err(|denial| map_index_denial(denial.kind(), binding))?;
    if lookup.overflowed() || lookup.candidate_entity_ids().len() > 1 {
        return Err(resolution_denial(
            WorthQueryPrincipalResolutionDenialKind::AmbiguousPrincipal,
            binding,
        ));
    }
    let mapping_id = *lookup.candidate_entity_ids().first().ok_or_else(|| {
        resolution_denial(
            WorthQueryPrincipalResolutionDenialKind::UnknownPrincipal,
            binding,
        )
    })?;
    let mapping = observe_mapping(runtime, snapshot, mapping_id, layout, binding)?;
    if &mapping.identity != expected_identity {
        return Err(resolution_denial(
            WorthQueryPrincipalResolutionDenialKind::CorruptIdentityIndex,
            binding,
        ));
    }
    if !mapping.enabled {
        return Err(resolution_denial(
            WorthQueryPrincipalResolutionDenialKind::DisabledPrincipal,
            binding,
        ));
    }
    let target = resolve_principal_target(runtime, snapshot, mapping_id, layout, binding)?;
    let freshness = WorthQueryPrincipalFreshnessEvidence::new(mapping.clone(), target.clone());
    let principal_identity = PrincipalIdentity::from_foundational_value(&target.principal_identity)
        .ok_or_else(|| {
            resolution_denial(
                WorthQueryPrincipalResolutionDenialKind::StalePrincipalProof,
                binding,
            )
        })?;
    Ok(WorthQueryResolvedPrincipalEvidence {
        principal_entity_id: target.target,
        principal_identity,
        runtime_authority,
        binding_identity,
        binding: binding.to_string(),
        mapping_entity_id: mapping.entity_id,
        target_relation_id: target.relation_id,
        freshness,
        examined_candidate_count: lookup.examined_entry_count(),
    })
}

pub(in crate::domain_computation) fn validate_freshness_at_snapshot<
    Schema,
    Principal,
    PrincipalIdentity,
>(
    runtime: &worth_relational::facade::runtime::RelationalRuntime,
    snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
    principal: &WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
    layout: &WorthQueryPrimaryPrincipalBindingLayout,
    expected_identity: &worth_foundational::facade::AspectValue,
) -> Result<(), WorthQueryPrincipalResolutionDenial> {
    let mapping = observe_mapping(
        runtime,
        snapshot,
        principal.mapping_entity_id(),
        layout,
        principal.binding(),
    )?;
    let target = observe_exact_principal_target(
        runtime,
        snapshot,
        principal.target_relation_id(),
        principal.principal_entity_id(),
        layout,
        principal.binding(),
    )?;
    if !mapping.enabled
        || &mapping.identity != expected_identity
        || target.source != principal.mapping_entity_id()
        || target.target != principal.principal_entity_id()
        || !principal.freshness().matches(&mapping, &target)
    {
        return Err(resolution_denial(
            WorthQueryPrincipalResolutionDenialKind::StalePrincipalProof,
            principal.binding(),
        ));
    }
    Ok(())
}

fn validate_current_schema_binding<Schema>(
    runtime: &WorthQueryExecutionRuntime,
    graph_identity: &worth_query_installation::facade::ApplicationSchemaBindingIdentity,
    proof_identity: &worth_query_installation::facade::ApplicationSchemaBindingIdentity,
    binding: &str,
) -> Result<(), WorthQueryPrincipalResolutionDenial>
where
    Schema: ApplicationSchema,
{
    let declaration = Schema::declaration().map_err(|_| {
        resolution_denial(
            WorthQueryPrincipalResolutionDenialKind::StaleInstalledSchema,
            binding,
        )
    })?;
    let current = runtime
        .installed_packages()
        .bind_application_schema(declaration)
        .map_err(|_| {
            resolution_denial(
                WorthQueryPrincipalResolutionDenialKind::StaleInstalledSchema,
                binding,
            )
        })?;
    if current.binding_identity() == *graph_identity
        && current.binding_identity() == *proof_identity
    {
        Ok(())
    } else {
        Err(resolution_denial(
            WorthQueryPrincipalResolutionDenialKind::StaleInstalledSchema,
            binding,
        ))
    }
}

fn admit_request(
    scope: &WorthQueryRequestScope,
    binding: &str,
) -> Result<(), WorthQueryPrincipalResolutionDenial> {
    match scope.interruption() {
        Some(WorthQueryRequestInterruption::Cancelled) => Err(resolution_denial(
            WorthQueryPrincipalResolutionDenialKind::Cancelled,
            binding,
        )),
        Some(WorthQueryRequestInterruption::DeadlineExceeded) => Err(resolution_denial(
            WorthQueryPrincipalResolutionDenialKind::DeadlineExceeded,
            binding,
        )),
        None => Ok(()),
    }
}

fn map_binding_denial_kind(
    kind: WorthQueryPrincipalBindingInstallationDenialKind,
) -> WorthQueryPrincipalResolutionDenialKind {
    match kind {
        WorthQueryPrincipalBindingInstallationDenialKind::ForeignRuntime => {
            WorthQueryPrincipalResolutionDenialKind::ForeignRuntime
        }
        WorthQueryPrincipalBindingInstallationDenialKind::StaleGeneration => {
            WorthQueryPrincipalResolutionDenialKind::StaleInstalledSchema
        }
        _ => WorthQueryPrincipalResolutionDenialKind::BindingNotInstalled,
    }
}

fn map_index_denial(
    kind: BoundedEntityFieldLookupDenialKind,
    binding: &str,
) -> WorthQueryPrincipalResolutionDenial {
    let kind = match kind {
        BoundedEntityFieldLookupDenialKind::SnapshotUnavailable => {
            WorthQueryPrincipalResolutionDenialKind::StalePrincipalProof
        }
        BoundedEntityFieldLookupDenialKind::IndexNotInstalled
        | BoundedEntityFieldLookupDenialKind::WrongIndexKind
        | BoundedEntityFieldLookupDenialKind::ExactGenerationUnavailable
        | BoundedEntityFieldLookupDenialKind::InvalidCandidateLimit => {
            WorthQueryPrincipalResolutionDenialKind::IdentityIndexUnavailable
        }
        BoundedEntityFieldLookupDenialKind::CorruptIndexEntries
        | BoundedEntityFieldLookupDenialKind::StorageParityMismatch => {
            WorthQueryPrincipalResolutionDenialKind::CorruptIdentityIndex
        }
    };
    resolution_denial(kind, binding)
}

fn resolution_denial(
    kind: WorthQueryPrincipalResolutionDenialKind,
    binding: impl Into<String>,
) -> WorthQueryPrincipalResolutionDenial {
    WorthQueryPrincipalResolutionDenial::new(kind, binding)
}
