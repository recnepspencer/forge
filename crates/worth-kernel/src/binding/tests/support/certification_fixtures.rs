use forge_query::facade::{
    admit_basis_capability, evaluate_basis_inspection_eligibility, normalize_raw_basis_intent,
    scope_basis_for_inspection, ForgeQueryAdmittedConfiguredDomainHandle,
    LowerRuntimeBasisEvidence, RawBasisIntent, ScopedInspectionBasis,
};
use worth_spatial::facade::bindings::{
    primitive_rebinding_retained_fact_source, PrimitiveRebindingDeclarationEntry,
    PrimitiveRebindingQueryDomain, PrimitiveRebindingQueryWorld,
    PrimitiveRebindingRetainedFactSource,
};
use worth_spatial::facade::inspection::{
    branch_local_geometry_inspection_entry, historical_geometry_inspection_entry,
    primitive_rebinding_retained_subject, PrimitiveRebindingBranchLocalInspection,
    PrimitiveRebindingHistoricalInspection,
};

use super::{primitive_rebinding_certification_bundle, BindingLayerCertificationBundle};
use crate::binding::tests::support::progress_rebinding_entry;

pub(crate) fn certification_bundle_for_pair(
    handle: ForgeQueryAdmittedConfiguredDomainHandle<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingQueryWorld,
    >,
    branch_basis: ScopedInspectionBasis,
    left_entry: PrimitiveRebindingDeclarationEntry,
    right_entry: PrimitiveRebindingDeclarationEntry,
    left_evidence: &str,
    right_evidence: &str,
) -> BindingLayerCertificationBundle {
    primitive_rebinding_certification_bundle(
        retained_fact_source(&left_entry, &handle),
        retained_fact_source(&right_entry, &handle),
        historical_inspection(&left_entry, &handle),
        historical_inspection(&right_entry, &handle),
        branch_local_inspection(&left_entry, &handle, &branch_basis, left_evidence),
        branch_local_inspection(&right_entry, &handle, &branch_basis, right_evidence),
        &handle,
    )
    .expect("binding-layer certification bundle")
}

pub(crate) fn historical_rebinding_inspection(
    entry: &PrimitiveRebindingDeclarationEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingQueryWorld,
    >,
) -> PrimitiveRebindingHistoricalInspection {
    historical_inspection(entry, handle)
}

pub(crate) fn branch_local_rebinding_inspection(
    entry: &PrimitiveRebindingDeclarationEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingQueryWorld,
    >,
    branch_basis: &ScopedInspectionBasis,
    evidence_digest: &str,
) -> PrimitiveRebindingBranchLocalInspection {
    branch_local_inspection(entry, handle, branch_basis, evidence_digest)
}

pub(crate) fn scoped_branch_head_inspection_basis(branch_identity: &str) -> ScopedInspectionBasis {
    let normalized = normalize_raw_basis_intent(
        RawBasisIntent::BranchHead {
            branch_identity: branch_identity.to_string(),
            accessible: true,
        },
        "inspection",
    )
    .expect("branch-head basis");
    let eligibility = evaluate_basis_inspection_eligibility(normalized).expect("eligibility");
    scope_basis_for_inspection(admit_basis_capability(eligibility))
}

fn historical_inspection(
    entry: &PrimitiveRebindingDeclarationEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingQueryWorld,
    >,
) -> PrimitiveRebindingHistoricalInspection {
    let subject = handle
        .orchestrate_envelope_from_progressed_checked(progress_rebinding_entry(entry, handle));
    historical_geometry_inspection_entry(
        retained_fact_source(entry, handle),
        primitive_rebinding_retained_subject(entry.binding_kind(), &subject),
    )
    .inspect_checked(handle, subject)
    .expect("historical inspection")
}

fn branch_local_inspection(
    entry: &PrimitiveRebindingDeclarationEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingQueryWorld,
    >,
    branch_basis: &ScopedInspectionBasis,
    evidence_digest: &str,
) -> PrimitiveRebindingBranchLocalInspection {
    let subject = handle
        .orchestrate_envelope_from_progressed_checked(progress_rebinding_entry(entry, handle));
    branch_local_geometry_inspection_entry(
        retained_fact_source(entry, handle),
        branch_basis.clone(),
        branch_basis_evidence(branch_basis, evidence_digest),
        primitive_rebinding_retained_subject(entry.binding_kind(), &subject),
    )
    .inspect_checked(handle, subject)
    .expect("branch-local inspection")
}

fn branch_basis_evidence(
    scoped_basis: &ScopedInspectionBasis,
    evidence_digest: &str,
) -> LowerRuntimeBasisEvidence {
    LowerRuntimeBasisEvidence::from_relational_facade(
        scoped_basis
            .expected_lower_runtime_binding_digest()
            .expect("basis digest"),
        evidence_digest,
        1,
    )
}

fn retained_fact_source(
    entry: &PrimitiveRebindingDeclarationEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingQueryWorld,
    >,
) -> PrimitiveRebindingRetainedFactSource {
    primitive_rebinding_retained_fact_source(entry, handle).expect("retained fact source")
}
