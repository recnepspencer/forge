use forge_query::facade::{
    admit_basis_capability, evaluate_basis_inspection_eligibility, normalize_raw_basis_intent,
    scope_basis_for_inspection, ForgeQueryAdmittedConfiguredDomainHandle,
    ForgeQueryDeclarationEntryInspectionInput, LowerRuntimeBasisEvidence, RawBasisIntent,
    ScopedInspectionBasis,
};

use crate::binding::rebinding::{
    primitive_rebinding_certification_bundle, BindingLayerCertificationBundle,
    PrimitiveRebindingBranchLocalInspection, PrimitiveRebindingDeclarationEntry,
    PrimitiveRebindingHistoricalInspection, PrimitiveRebindingQueryDomain,
    PrimitiveRebindingQueryWorld,
};

use super::progress_rebinding_entry;

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
        &left_entry,
        &right_entry,
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
    entry
        .historical_inspection_with_query(
            handle,
            ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
                handle.orchestrate_envelope_from_progressed_checked(progress_rebinding_entry(
                    entry, handle,
                )),
            ),
        )
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
    entry
        .branch_local_inspection_with_query(
            handle,
            branch_basis,
            branch_basis_evidence(branch_basis, evidence_digest),
            ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
                handle.orchestrate_envelope_from_progressed_checked(progress_rebinding_entry(
                    entry, handle,
                )),
            ),
        )
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
