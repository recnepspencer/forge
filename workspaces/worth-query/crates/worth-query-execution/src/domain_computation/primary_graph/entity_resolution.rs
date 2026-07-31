use worth_query_admission::facade::authenticated_principal::{
    WorthQueryRequestInterruption, WorthQueryRequestScope,
};
use worth_query_installation::facade::{
    ApplicationFieldCurrency, ApplicationFieldRef, ApplicationSchema, EqualityPredicate,
    TypedApplicationValue, WritePosture,
};
use worth_relational::facade::indexes::{
    BoundedEntityFieldLookupDenialKind, BoundedEntityFieldLookupRequest, BoundedIndexParityMode,
};

use super::entity_identity::WorthQueryResolvedEntityEvidence;
use super::schema_layout::WorthQueryPrimaryFieldLayout;
use super::{
    WorthQueryApplicationEntityIdentity, WorthQueryEntityResolutionDenial,
    WorthQueryEntityResolutionDenialKind, WorthQueryPrimaryGraphApplicationRuntime,
    WorthQueryPrincipalResolutionMode,
};

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub fn resolve_entity<Aspect, Entity, Field, Value, Write, Currency>(
        &self,
        field: ApplicationFieldRef<
            Schema,
            Entity,
            Aspect,
            Field,
            Value,
            Write,
            EqualityPredicate,
            Currency,
        >,
        value: Value,
        scope: &WorthQueryRequestScope,
        mode: WorthQueryPrincipalResolutionMode,
    ) -> Result<WorthQueryApplicationEntityIdentity<Schema, Entity>, WorthQueryEntityResolutionDenial>
    where
        Value: TypedApplicationValue,
        Write: WritePosture,
        Currency: ApplicationFieldCurrency,
    {
        admit_request(scope, field.field())?;
        let graph = self.runtime.primary_graph().ok_or_else(|| {
            entity_denial(
                WorthQueryEntityResolutionDenialKind::PrimaryGraphNotInstalled,
                field.entity(),
            )
        })?;
        let layout = graph
            .layout
            .equality_field(field.entity(), field.aspect(), field.field())
            .cloned()
            .ok_or_else(|| {
                entity_denial(
                    WorthQueryEntityResolutionDenialKind::FieldNotInstalled,
                    field.field(),
                )
            })?;
        let expected = value.into_foundational_value();
        let evidence = graph.integration_handle().with_runtime_mut(|runtime| {
            let snapshot = runtime.snapshots().snapshot();
            let result = resolve_at_snapshot(
                runtime,
                &snapshot,
                &layout,
                expected,
                mode,
                self.runtime.authority_identity(),
                graph.binding_identity().clone(),
                field.entity(),
                field.field(),
            );
            runtime.snapshots().release_snapshot(&snapshot);
            result
        })?;
        admit_request(scope, field.field())?;
        Ok(WorthQueryApplicationEntityIdentity::mint(evidence))
    }
}

pub(in crate::domain_computation) fn resolve_at_snapshot(
    runtime: &worth_relational::facade::runtime::RelationalRuntime,
    snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
    layout: &WorthQueryPrimaryFieldLayout,
    expected: worth_foundational::facade::AspectValue,
    mode: WorthQueryPrincipalResolutionMode,
    runtime_authority: crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity,
    binding_identity: worth_query_installation::facade::ApplicationSchemaBindingIdentity,
    entity_name: &str,
    subject: &str,
) -> Result<WorthQueryResolvedEntityEvidence, WorthQueryEntityResolutionDenial> {
    resolve_at_snapshot_with_work(
        runtime,
        snapshot,
        layout,
        expected,
        mode,
        runtime_authority,
        binding_identity,
        entity_name,
        subject,
    )
    .0
}

pub(super) fn resolve_at_snapshot_with_work(
    runtime: &worth_relational::facade::runtime::RelationalRuntime,
    snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
    layout: &WorthQueryPrimaryFieldLayout,
    expected: worth_foundational::facade::AspectValue,
    mode: WorthQueryPrincipalResolutionMode,
    runtime_authority: crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity,
    binding_identity: worth_query_installation::facade::ApplicationSchemaBindingIdentity,
    entity_name: &str,
    subject: &str,
) -> (
    Result<WorthQueryResolvedEntityEvidence, WorthQueryEntityResolutionDenial>,
    usize,
) {
    let mut examined_candidate_count = 0;
    let result = (|| {
        let index_id = layout.equality_index_id.ok_or_else(|| {
            entity_denial(
                WorthQueryEntityResolutionDenialKind::EqualityIndexUnavailable,
                subject,
            )
        })?;
        let request = BoundedEntityFieldLookupRequest::new(
            snapshot.clone(),
            index_id,
            layout.entity_kind,
            layout.locator.clone(),
            expected.clone(),
            2,
        )
        .map_err(|denial| map_index_denial(denial.kind(), subject))?;
        let parity = match mode {
            WorthQueryPrincipalResolutionMode::Ordinary => BoundedIndexParityMode::Production,
            WorthQueryPrincipalResolutionMode::Certification => {
                BoundedIndexParityMode::Certification
            }
        };
        let lookup = runtime
            .index_access()
            .execute_bounded_entity_field_lookup(request, parity)
            .map_err(|denial| map_index_denial(denial.kind(), subject))?;
        examined_candidate_count = lookup.examined_entry_count();
        if lookup.overflowed() || lookup.candidate_entity_ids().len() > 1 {
            return Err(entity_denial(
                WorthQueryEntityResolutionDenialKind::AmbiguousEntity,
                subject,
            ));
        }
        let entity_id = lookup
            .candidate_entity_ids()
            .first()
            .copied()
            .ok_or_else(|| {
                entity_denial(WorthQueryEntityResolutionDenialKind::UnknownEntity, subject)
            })?;
        Ok(WorthQueryResolvedEntityEvidence {
            entity_id,
            entity_kind: layout.entity_kind,
            entity_name: entity_name.to_string(),
            binding_identity,
            runtime_authority,
            identity_index_id: index_id,
            identity_index_generation: lookup.generation_id(),
            identity_locator: layout.locator.clone(),
            identity_value: expected,
            examined_candidate_count,
            resolution_mode: mode,
        })
    })();
    (result, examined_candidate_count)
}

pub(in crate::domain_computation) fn validate_entity_freshness_at_snapshot<Schema, Entity>(
    runtime: &worth_relational::facade::runtime::RelationalRuntime,
    snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
    identity: &WorthQueryApplicationEntityIdentity<Schema, Entity>,
) -> Result<(), WorthQueryEntityResolutionDenial> {
    let request = BoundedEntityFieldLookupRequest::new(
        snapshot.clone(),
        identity.identity_index_id(),
        identity.entity_kind(),
        identity.identity_locator().clone(),
        identity.identity_value().clone(),
        2,
    )
    .map_err(|denial| map_index_denial(denial.kind(), identity.entity_name()))?;
    let parity = match identity.resolution_mode() {
        WorthQueryPrincipalResolutionMode::Ordinary => BoundedIndexParityMode::Production,
        WorthQueryPrincipalResolutionMode::Certification => BoundedIndexParityMode::Certification,
    };
    let lookup = runtime
        .index_access()
        .execute_bounded_entity_field_lookup(request, parity)
        .map_err(|denial| map_index_denial(denial.kind(), identity.entity_name()))?;
    if lookup.overflowed()
        || lookup.candidate_entity_ids().len() != 1
        || lookup.candidate_entity_ids()[0] != identity.entity_id()
    {
        return Err(entity_denial(
            WorthQueryEntityResolutionDenialKind::UnknownEntity,
            identity.entity_name(),
        ));
    }
    Ok(())
}

fn admit_request(
    scope: &WorthQueryRequestScope,
    subject: &str,
) -> Result<(), WorthQueryEntityResolutionDenial> {
    match scope.interruption() {
        Some(WorthQueryRequestInterruption::Cancelled) => Err(entity_denial(
            WorthQueryEntityResolutionDenialKind::Cancelled,
            subject,
        )),
        Some(WorthQueryRequestInterruption::DeadlineExceeded) => Err(entity_denial(
            WorthQueryEntityResolutionDenialKind::DeadlineExceeded,
            subject,
        )),
        None => Ok(()),
    }
}

fn map_index_denial(
    kind: BoundedEntityFieldLookupDenialKind,
    subject: &str,
) -> WorthQueryEntityResolutionDenial {
    let kind = match kind {
        BoundedEntityFieldLookupDenialKind::CorruptIndexEntries
        | BoundedEntityFieldLookupDenialKind::StorageParityMismatch => {
            WorthQueryEntityResolutionDenialKind::CorruptIdentityIndex
        }
        _ => WorthQueryEntityResolutionDenialKind::EqualityIndexUnavailable,
    };
    entity_denial(kind, subject)
}

fn entity_denial(
    kind: WorthQueryEntityResolutionDenialKind,
    subject: impl Into<String>,
) -> WorthQueryEntityResolutionDenial {
    WorthQueryEntityResolutionDenial::new(kind, subject)
}
