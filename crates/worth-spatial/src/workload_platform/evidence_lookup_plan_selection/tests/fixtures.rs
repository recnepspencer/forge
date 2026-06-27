use crate::workload_platform::evidence_ledger::{
    receipt_backed_event_ledger_touch_authority_for_admission_tests,
    receipt_backed_touch_authority_for_admission_tests, BooleanEvidenceStageKind,
    SpatialGeometryEvidenceTouchAuthority, WorkloadEvidenceStage,
};
use crate::workload_platform::evidence_lookup_family_catalog::{
    current_evidence_lookup_family_catalog, EvidenceLookupFamilyCatalogCloseout,
    EvidenceLookupFamilyDeclaration, EvidenceLookupFamilyIdentity,
    EvidenceLookupQueryImportEvidence, EvidenceLookupStageReceiptFamilyIdentity,
    TestCatalogCloseout,
};
use crate::workload_platform::evidence_lookup_input_admission::{
    admit_evidence_lookup_input, EvidenceLookupAdmittedInput, EvidenceLookupInputAdmissionRequest,
    EvidenceLookupQueryAdmissionEvidenceSet, EvidenceLookupStageReceiptAdmission,
};

use super::super::{select_evidence_lookup_plan, EvidenceLookupSelectedPlan};

pub(super) struct PlanSelectionSubject {
    catalog: EvidenceLookupFamilyCatalogCloseout,
    authority: SpatialGeometryEvidenceTouchAuthority,
    receipt_family: EvidenceLookupStageReceiptFamilyIdentity,
}

impl PlanSelectionSubject {
    pub(super) fn event_ledger() -> Self {
        Self {
            catalog: catalog(),
            authority: receipt_backed_event_ledger_touch_authority_for_admission_tests(),
            receipt_family: EvidenceLookupStageReceiptFamilyIdentity::boolean_event_ledger(),
        }
    }

    pub(super) fn projection_consumption() -> Self {
        Self {
            catalog: catalog(),
            authority: receipt_backed_touch_authority_for_admission_tests(
                BooleanEvidenceStageKind::OperandAProjectionConsumption,
                "phase-4-projection-consumption-receipt",
            ),
            receipt_family:
                EvidenceLookupStageReceiptFamilyIdentity::boolean_operand_projection_consumption(),
        }
    }

    pub(super) fn event_ledger_with_query_required_sibling() -> Self {
        Self {
            catalog: event_ledger_catalog_with_query_required_sibling(),
            authority: receipt_backed_event_ledger_touch_authority_for_admission_tests(),
            receipt_family: EvidenceLookupStageReceiptFamilyIdentity::boolean_event_ledger(),
        }
    }

    pub(super) const fn catalog(&self) -> &EvidenceLookupFamilyCatalogCloseout {
        &self.catalog
    }

    pub(super) fn request(&self) -> EvidenceLookupInputAdmissionRequest<'_> {
        EvidenceLookupInputAdmissionRequest::from_spatial_touch_authority(&self.authority)
            .with_stage_receipt_identity(
                EvidenceLookupStageReceiptAdmission::from_spatial_touch_authority(
                    &self.authority,
                    self.receipt_family.clone(),
                ),
            )
    }

    pub(super) fn request_with_query_evidence(&self) -> EvidenceLookupInputAdmissionRequest<'_> {
        self.request().with_query_import_evidence(
            EvidenceLookupQueryAdmissionEvidenceSet::from_query_import_evidence(
                &query_import_for_stage(self.catalog(), self.authority.evidence_stage()),
            ),
        )
    }

    pub(super) fn admit(&self) -> EvidenceLookupAdmittedInput {
        admit_evidence_lookup_input(&self.catalog, self.request()).expect("input admits")
    }

    pub(super) fn admit_with_query_evidence(&self) -> EvidenceLookupAdmittedInput {
        admit_evidence_lookup_input(&self.catalog, self.request_with_query_evidence())
            .expect("input admits with Query posture")
    }

    pub(super) fn admit_with_query_import(
        &self,
        query_import: &EvidenceLookupQueryImportEvidence,
    ) -> EvidenceLookupAdmittedInput {
        admit_evidence_lookup_input(
            &self.catalog,
            self.request().with_query_import_evidence(
                EvidenceLookupQueryAdmissionEvidenceSet::from_query_import_evidence(query_import),
            ),
        )
        .expect("input admits with supplied Query posture")
    }

    pub(super) fn select_event_plan(&self) -> EvidenceLookupSelectedPlan {
        select_evidence_lookup_plan(&self.catalog, &self.admit()).expect("plan selects")
    }

    pub(super) fn select_projection_plan(&self) -> EvidenceLookupSelectedPlan {
        select_evidence_lookup_plan(&self.catalog, &self.admit_with_query_evidence())
            .expect("plan selects")
    }
}

pub(super) fn query_import_for_family(
    catalog: &EvidenceLookupFamilyCatalogCloseout,
    family_identity: &str,
) -> EvidenceLookupQueryImportEvidence {
    catalog
        .family_by_identity(family_identity)
        .and_then(|family| family.query_posture().imported_evidence())
        .expect("fixture family should require query evidence")
        .clone()
}

pub(super) fn query_import_for_stage(
    catalog: &EvidenceLookupFamilyCatalogCloseout,
    stage: WorkloadEvidenceStage,
) -> EvidenceLookupQueryImportEvidence {
    catalog
        .declarations()
        .iter()
        .find(|family| family.stage_applicability().stages().contains(&stage))
        .and_then(|family| family.query_posture().imported_evidence())
        .expect("fixture stage should require query evidence")
        .clone()
}

fn catalog() -> EvidenceLookupFamilyCatalogCloseout {
    current_evidence_lookup_family_catalog().expect("catalog closes")
}

fn event_ledger_catalog_with_query_required_sibling() -> EvidenceLookupFamilyCatalogCloseout {
    let catalog = catalog();
    let event_family = catalog
        .family_by_identity("spatial-touch.boolean.event-ledger-evidence.v1")
        .expect("event family exists")
        .clone();
    let projection_family = catalog
        .family_by_identity("spatial-touch.boolean.projection-consumption-evidence.v1")
        .expect("projection family exists");
    let query_required_event_sibling =
        copy_family_with_identity_and_query(&event_family, projection_family);
    TestCatalogCloseout::from_declarations(vec![event_family, query_required_event_sibling])
        .expect("custom two-family catalog closes")
}

fn copy_family_with_identity_and_query(
    event_family: &EvidenceLookupFamilyDeclaration,
    query_family: &EvidenceLookupFamilyDeclaration,
) -> EvidenceLookupFamilyDeclaration {
    EvidenceLookupFamilyDeclaration::builder()
        .identity(EvidenceLookupFamilyIdentity::declared(
            "spatial-touch.boolean.event-ledger-query-required-sibling.v1",
        ))
        .spatial_touch_authority(event_family.spatial_touch_authority())
        .topology_input_posture(event_family.topology_input_posture().clone())
        .stage_applicability(event_family.stage_applicability().clone())
        .evidence_classes(event_family.evidence_classes().clone())
        .lookup_product_posture(event_family.lookup_product_posture())
        .index_posture(query_family.index_posture().clone())
        .query_posture(query_family.query_posture().clone())
        .diagnostic_witness(event_family.diagnostic_witness().clone())
        .source_inventory_pressure(event_family.source_inventory_pressure().clone())
        .build()
        .expect("custom event sibling declaration builds")
}
