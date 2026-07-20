use crate::policy_basis::{tenant_schema_identity, tenant_truth_identity};

use super::{
    SchemaVariantSnapshot, TenantBasisAdmissionError, TenantBasisAdmissionFailureClass,
    TenantBasisCounters, TenantBindingSnapshot, TenantResolutionClass, TenantSchemaBasis,
    TenantTruthBasis,
};

pub(crate) fn admit_tenant_bases(
    tenant: &TenantBindingSnapshot,
    schema: &SchemaVariantSnapshot,
) -> Result<(TenantTruthBasis, TenantSchemaBasis, TenantBasisCounters), TenantBasisAdmissionError> {
    if tenant.ambiguous() {
        return Err(TenantBasisAdmissionError::new(
            TenantBasisAdmissionFailureClass::AmbiguousTenantContext,
            "ambiguous tenant context is denied",
            TenantBasisCounters::denied_ambiguous(),
        ));
    }
    if tenant.hidden_filter() {
        return Err(TenantBasisAdmissionError::new(
            TenantBasisAdmissionFailureClass::HiddenTenantFilter,
            "hidden tenant filters are denied",
            TenantBasisCounters::denied_hidden_filter(),
        ));
    }
    if tenant.resolution_class() == TenantResolutionClass::DerivedBinding {
        return Err(TenantBasisAdmissionError::new(
            TenantBasisAdmissionFailureClass::DerivedBindingDeferred,
            "derived tenant binding remains deferred debt in Phase 1",
            TenantBasisCounters::denied_derived(),
        ));
    }
    if schema.global_fallback() {
        return Err(TenantBasisAdmissionError::new(
            TenantBasisAdmissionFailureClass::GlobalSchemaFallbackForbidden,
            "global schema fallback is denied for tenant-scoped admission",
            TenantBasisCounters::denied_global_fallback(),
        ));
    }
    let Some(branch_identity) = tenant.truth_branch_identity() else {
        return Err(TenantBasisAdmissionError::new(
            TenantBasisAdmissionFailureClass::MissingTenantTruthBasis,
            "tenant truth basis is required",
            TenantBasisCounters::denied_missing_truth(),
        ));
    };
    let Some(schema_identity) = tenant.schema_basis_identity() else {
        return Err(TenantBasisAdmissionError::new(
            TenantBasisAdmissionFailureClass::MissingTenantSchemaBasis,
            "tenant schema basis is required",
            TenantBasisCounters::denied_missing_schema(),
        ));
    };
    if tenant.tenant_identity() != schema.tenant_identity()
        || schema_identity != schema.schema_basis_identity()
    {
        return Err(TenantBasisAdmissionError::new(
            TenantBasisAdmissionFailureClass::TenantSchemaMismatch,
            "tenant schema snapshot must match tenant binding snapshot",
            TenantBasisCounters::denied_missing_schema(),
        ));
    }

    let counters = match tenant.resolution_class() {
        TenantResolutionClass::DirectBinding => TenantBasisCounters::direct_admitted(),
        TenantResolutionClass::CachedBinding => TenantBasisCounters::cached_admitted(),
        TenantResolutionClass::DerivedBinding => unreachable!("derived binding denied above"),
    };
    let truth = TenantTruthBasis::admitted(
        tenant_truth_identity(
            tenant.tenant_identity(),
            branch_identity,
            tenant.resolution_class(),
            tenant.epoch(),
        ),
        tenant.tenant_identity().to_string(),
        branch_identity.to_string(),
        tenant.resolution_class(),
        tenant.epoch(),
    );
    let schema_basis = TenantSchemaBasis::admitted(
        tenant_schema_identity(tenant.tenant_identity(), schema_identity, tenant.epoch()),
        tenant.tenant_identity().to_string(),
        schema_identity.to_string(),
        tenant.epoch(),
    );
    Ok((truth, schema_basis, counters))
}
