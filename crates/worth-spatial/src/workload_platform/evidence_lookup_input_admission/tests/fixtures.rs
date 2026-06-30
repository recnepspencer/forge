use crate::workload_platform::evidence_ledger::{
    receipt_backed_event_ledger_touch_authority_for_admission_tests,
    receipt_backed_touch_authority_for_admission_tests, BooleanEvidenceStageKind,
    SpatialGeometryEvidenceTouchAuthority, WorkloadEvidenceStage,
};
use crate::workload_platform::evidence_lookup_family_catalog::{
    current_evidence_lookup_family_catalog, EvidenceLookupFamilyCatalogCloseout,
    EvidenceLookupFamilyDeclaration, EvidenceLookupQueryImportEvidence,
    EvidenceLookupStageReceiptFamilyIdentity,
};

use super::super::{
    admit_evidence_lookup_input, EvidenceLookupAdmittedInput, EvidenceLookupInputAdmissionRequest,
    EvidenceLookupQueryAdmissionEvidenceSet, EvidenceLookupStageReceiptAdmission,
};

pub(super) struct AdmissionSubject {
    catalog: EvidenceLookupFamilyCatalogCloseout,
    authority: SpatialGeometryEvidenceTouchAuthority,
    receipt_family: EvidenceLookupStageReceiptFamilyIdentity,
}

impl AdmissionSubject {
    pub(super) fn event_ledger() -> Self {
        Self {
            catalog: catalog(),
            authority: receipt_backed_event_ledger_touch_authority_for_admission_tests(),
            receipt_family: EvidenceLookupStageReceiptFamilyIdentity::boolean_event_ledger(),
        }
    }

    pub(super) fn projection_consumption() -> Self {
        Self::projection_consumption_with_identity("phase-3-projection-consumption-receipt")
    }

    pub(super) fn projection_consumption_with_identity(evidence_identity: &'static str) -> Self {
        Self {
            catalog: catalog(),
            authority: receipt_backed_touch_authority_for_admission_tests(
                BooleanEvidenceStageKind::OperandAProjectionConsumption,
                evidence_identity,
            ),
            receipt_family:
                EvidenceLookupStageReceiptFamilyIdentity::boolean_operand_projection_consumption(),
        }
    }

    pub(super) fn topology_required_shared_plane() -> Self {
        Self {
            catalog: catalog(),
            authority: receipt_backed_touch_authority_for_admission_tests(
                BooleanEvidenceStageKind::SharedPlaneIdentity,
                "phase-3-shared-plane-receipt",
            ),
            receipt_family: EvidenceLookupStageReceiptFamilyIdentity::boolean_common_plane(),
        }
    }

    pub(super) const fn catalog(&self) -> &EvidenceLookupFamilyCatalogCloseout {
        &self.catalog
    }

    pub(super) const fn authority(&self) -> &SpatialGeometryEvidenceTouchAuthority {
        &self.authority
    }

    pub(super) fn stage_receipt(&self) -> EvidenceLookupStageReceiptAdmission {
        EvidenceLookupStageReceiptAdmission::from_spatial_touch_authority(
            &self.authority,
            self.receipt_family.clone(),
        )
    }

    pub(super) fn request(&self) -> EvidenceLookupInputAdmissionRequest<'_> {
        EvidenceLookupInputAdmissionRequest::from_spatial_touch_authority(&self.authority)
            .with_stage_receipt_identity(self.stage_receipt())
    }

    pub(super) fn admit(&self) -> EvidenceLookupAdmittedInput {
        admit_evidence_lookup_input(&self.catalog, self.request()).expect("input admits")
    }

    pub(super) fn request_with_query_evidence(
        &self,
        evidence: EvidenceLookupQueryAdmissionEvidenceSet,
    ) -> EvidenceLookupInputAdmissionRequest<'_> {
        self.request().with_query_import_evidence(evidence)
    }
}

pub(super) fn query_import_for_stage(
    catalog: &EvidenceLookupFamilyCatalogCloseout,
    stage: WorkloadEvidenceStage,
) -> EvidenceLookupQueryImportEvidence {
    matching_family(catalog, stage)
        .query_posture()
        .imported_evidence()
        .expect("family should require query evidence")
        .clone()
}

pub(super) fn matching_family(
    catalog: &EvidenceLookupFamilyCatalogCloseout,
    stage: WorkloadEvidenceStage,
) -> &EvidenceLookupFamilyDeclaration {
    catalog
        .declarations()
        .iter()
        .find(|family| family.stage_applicability().stages().contains(&stage))
        .expect("fixture stage should have a lookup family")
}

fn catalog() -> EvidenceLookupFamilyCatalogCloseout {
    current_evidence_lookup_family_catalog().expect("catalog closes")
}
