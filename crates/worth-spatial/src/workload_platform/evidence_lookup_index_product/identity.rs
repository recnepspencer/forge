use schema::facade::platform::authority::compiled_product_semantic_graph::{
    admit_compiled_product_rebuild_denial_identity, CompiledProductEquivalencePolicyIdentity,
    CompiledProductIdentity, CompiledProductRebuildDenialIdentity,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::spatial_compiled_product_family::{
    current_spatial_compiled_product_family_catalog, select_spatial_compiled_product_family,
    SpatialCompiledProductConsumer, SpatialCompiledProductLoweredIdentity,
};
use crate::workload_platform::compiled_product_admission::{
    admit_spatial_compiled_product_input, SpatialCompiledProductAdmissionRequest,
};
use crate::workload_platform::evidence_ledger::SelectedLookupSliceLedger;

use super::counters::EvidenceLookupIndexProductCounters;
use super::error::{EvidenceLookupIndexProductError, EvidenceLookupIndexProductErrorKind};
use super::lifecycle_posture::EvidenceLookupIndexLifecyclePosture;

pub(crate) struct AdmittedEvidenceLookupFamilyIdentity {
    lowered_identity: SpatialCompiledProductLoweredIdentity,
    evidence_ledger_basis_digest: String,
    query_support_digest: String,
    topology_support_digest: String,
}

impl AdmittedEvidenceLookupFamilyIdentity {
    pub(crate) fn lowered_identity(&self) -> &SpatialCompiledProductLoweredIdentity {
        &self.lowered_identity
    }

    pub(crate) fn evidence_ledger_basis_digest(&self) -> &str {
        &self.evidence_ledger_basis_digest
    }

    pub(crate) fn query_support_digest(&self) -> &str {
        &self.query_support_digest
    }

    pub(crate) fn topology_support_digest(&self) -> &str {
        &self.topology_support_digest
    }
}

pub(crate) fn lower_index_family_identity(
    selected_plan: &crate::workload_platform::evidence_lookup_plan_selection::EvidenceLookupSelectedPlan,
    ledger: &SelectedLookupSliceLedger,
) -> SpatialCompiledProductLoweredIdentity {
    admit_and_lower_index_family_identity(selected_plan, ledger)
        .expect("evidence lookup index admitted spatial family identity")
        .lowered_identity
}

pub(crate) fn admit_and_lower_index_family_identity(
    selected_plan: &crate::workload_platform::evidence_lookup_plan_selection::EvidenceLookupSelectedPlan,
    ledger: &SelectedLookupSliceLedger,
) -> Result<AdmittedEvidenceLookupFamilyIdentity, EvidenceLookupIndexProductError> {
    let request = SpatialCompiledProductAdmissionRequest::for_evidence_lookup_ledger(
        SpatialCompiledProductConsumer::EvidenceLookupIndexProduct,
        selected_plan,
        ledger,
    );
    lower_index_family_identity_from_request(request, "evidence lookup index")
}

#[cfg(test)]
pub(crate) fn lower_index_family_identity_from_basis(
    selected_plan: &crate::workload_platform::evidence_lookup_plan_selection::EvidenceLookupSelectedPlan,
    basis: &crate::workload_platform::evidence_lookup_index_product::EvidenceLookupLedgerBasis,
) -> SpatialCompiledProductLoweredIdentity {
    let ledger = crate::workload_platform::evidence_ledger::WorkloadEvidenceLedger::from_rows(
        basis.rows().to_vec(),
    )
    .expect("audit basis rows must form a workload ledger")
    .certify_complete()
    .expect("audit basis rows must certify as a complete ledger");
    let selected_slice = SelectedLookupSliceLedger::from_complete_ledger(ledger);
    lower_index_family_identity_from_request(
        SpatialCompiledProductAdmissionRequest::for_evidence_lookup_ledger(
            SpatialCompiledProductConsumer::EvidenceLookupIndexProduct,
            selected_plan,
            &selected_slice,
        ),
        "audit evidence lookup index",
    )
    .expect("audit evidence lookup index admitted spatial family identity")
    .lowered_identity
}

fn lower_index_family_identity_from_request(
    request: SpatialCompiledProductAdmissionRequest<'_>,
    expectation: &str,
) -> Result<AdmittedEvidenceLookupFamilyIdentity, EvidenceLookupIndexProductError> {
    let catalog = current_spatial_compiled_product_family_catalog();
    let admitted_input =
        admit_spatial_compiled_product_input(&catalog, request).map_err(|error| {
            EvidenceLookupIndexProductError::new(
                EvidenceLookupIndexProductErrorKind::SpatialAdmissionDenied,
                format!("{expectation} admission failed: {:?}", error.kind()),
            )
        })?;
    let evidence_lookup = admitted_input
        .evidence_lookup()
        .expect("evidence lookup admission materialization");
    let lowered_identity =
        select_spatial_compiled_product_family(&catalog, admitted_input.family_admitted_input())
            .map_err(|error| {
                EvidenceLookupIndexProductError::new(
                    EvidenceLookupIndexProductErrorKind::SpatialAdmissionDenied,
                    format!("{expectation} family selection failed: {:?}", error.kind()),
                )
            })?
            .compile_product_identity()
            .map_err(|error| {
                EvidenceLookupIndexProductError::new(
                    EvidenceLookupIndexProductErrorKind::SpatialAdmissionDenied,
                    format!("{expectation} lowering failed: {:?}", error.kind()),
                )
            })?;

    Ok(AdmittedEvidenceLookupFamilyIdentity {
        lowered_identity,
        evidence_ledger_basis_digest: evidence_lookup.evidence_ledger_basis_digest().to_string(),
        query_support_digest: evidence_lookup.query_support_digest().to_string(),
        topology_support_digest: evidence_lookup.topology_support_digest().to_string(),
    })
}

pub(crate) fn index_compiled_product_identity(
    selected_plan: &crate::workload_platform::evidence_lookup_plan_selection::EvidenceLookupSelectedPlan,
    ledger: &crate::workload_platform::evidence_ledger::SelectedLookupSliceLedger,
) -> CompiledProductIdentity {
    lower_index_family_identity(selected_plan, ledger)
        .compiled_product_identity()
        .clone()
}

pub(crate) fn index_equivalence_policy_identity() -> CompiledProductEquivalencePolicyIdentity {
    let catalog = current_spatial_compiled_product_family_catalog();
    let declaration = catalog
        .family_for_consumer(SpatialCompiledProductConsumer::EvidenceLookupIndexProduct)
        .expect("evidence lookup index family declaration");
    schema::facade::platform::authority::compiled_product_semantic_graph::admit_compiled_product_equivalence_policy_identity(
        declaration.equivalence_policy_name(),
        declaration.equivalence_dimensions().iter().copied(),
    )
    .expect("evidence lookup index equivalence policy identity")
}

pub(crate) fn rebuild_required_identity(
    compiled_product_identity: &CompiledProductIdentity,
    denial_reason: &str,
) -> CompiledProductRebuildDenialIdentity {
    admit_compiled_product_rebuild_denial_identity(compiled_product_identity, denial_reason)
        .expect("evidence lookup rebuild denial identity")
}

pub(crate) fn index_product_digest(
    compiled_product_identity: &CompiledProductIdentity,
    equivalence_policy_identity: &CompiledProductEquivalencePolicyIdentity,
    lifecycle_posture: EvidenceLookupIndexLifecyclePosture,
    disposal_posture: super::disposal_posture::EvidenceLookupIndexDisposalPosture,
    counters: &EvidenceLookupIndexProductCounters,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-spatial:evidence-lookup-index-product:v2".to_string(),
            format!(
                "compiled-product:{}",
                compiled_product_identity.identity_digest()
            ),
            format!(
                "equivalence-policy:{}",
                equivalence_policy_identity.identity_digest()
            ),
            format!("lifecycle:{:?}", lifecycle_posture.kind()),
            format!("disposal:{:?}", disposal_posture.kind()),
            format!("basis-rows:{}", counters.selected_basis_row_count()),
            format!("resident-bytes:{}", counters.resident_byte_count()),
            format!("reused:{}", counters.reused_index_count()),
            format!("rebuilt:{}", counters.rebuilt_index_count()),
        ],
    )
}
