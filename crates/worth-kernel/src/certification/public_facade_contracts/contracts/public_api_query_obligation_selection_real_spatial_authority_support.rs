#[path = "public_api_planar_boolean_event_extraction_metaboss_support/mod.rs"]
#[allow(dead_code, unused_imports)]
mod metaboss_support;

use metaboss_support::MetabossEventExtractionSubject;
use worth_kernel::query_obligation_selection::selection_substrate::{
    QueryObligationSelectionInput, QueryObligationSelectionSubstrate, QuerySelectedGraphObligations,
};
use worth_kernel::workload_composition::{
    BuiltBooleanOperandPairRecipe, WorthWorkload, WorthWorkloadParts,
};
use worth_spatial::facade::workload_vocabulary::{
    lower_spatial_touch_authority_to_query_descriptor, BooleanEvidenceRowAuthority,
    BooleanEvidenceStageKind, SpatialEvidenceLookupProduct, SpatialEvidenceQueryTouchDescriptor,
    SpatialGeometryEvidenceTouchAuthority,
};

pub struct RealSpatialSelectionCase {
    authority: SpatialGeometryEvidenceTouchAuthority,
    lookup: SpatialEvidenceLookupProduct,
    descriptor: SpatialEvidenceQueryTouchDescriptor,
    selected: QuerySelectedGraphObligations,
}

pub fn real_spatial_selection_case(label: &'static str) -> RealSpatialSelectionCase {
    let authority_case = real_spatial_authority_case(label);
    let descriptor = lower_spatial_touch_authority_to_query_descriptor(
        &authority_case.authority,
        &authority_case.lookup,
    )
    .expect("spatial authority plus lookup must lower to Query descriptor");
    let input = QueryObligationSelectionInput::from_spatial_query_descriptor(&descriptor)
        .expect("spatial descriptor must become selection input");
    let selected = QueryObligationSelectionSubstrate::select_execution_backed_obligations(input)
        .expect("kernel substrate must select spatial Query obligations");

    RealSpatialSelectionCase {
        authority: authority_case.authority,
        lookup: authority_case.lookup,
        descriptor,
        selected,
    }
}

pub fn real_spatial_authority_case(label: &'static str) -> RealSpatialAuthorityCase {
    let subject = MetabossEventExtractionSubject::certify(label);
    let event_ledger_receipt = subject.ledger();
    let completed_workload = completed_workload_with_boolean_receipt(
        subject.pair(),
        event_ledger_receipt,
        BooleanEvidenceStageKind::EventLedger,
    );
    let authority = completed_workload
        .admit_spatial_geometry_evidence_touch(event_ledger_receipt)
        .expect("event ledger receipt must admit spatial authority through WorthWorkload");
    let lookup = authority
        .spatial_evidence_lookup(completed_workload.evidence_ledger())
        .expect("spatial authority must produce a real lookup product");

    RealSpatialAuthorityCase {
        workload: completed_workload,
        authority,
        lookup,
    }
}

pub struct RealSpatialAuthorityCase {
    workload: WorthWorkload,
    authority: SpatialGeometryEvidenceTouchAuthority,
    lookup: SpatialEvidenceLookupProduct,
}

impl RealSpatialSelectionCase {
    pub fn authority(&self) -> &SpatialGeometryEvidenceTouchAuthority {
        &self.authority
    }

    pub fn lookup(&self) -> &SpatialEvidenceLookupProduct {
        &self.lookup
    }

    pub fn descriptor(&self) -> &SpatialEvidenceQueryTouchDescriptor {
        &self.descriptor
    }

    pub fn selected(&self) -> &QuerySelectedGraphObligations {
        &self.selected
    }
}

impl RealSpatialAuthorityCase {
    pub fn workload(&self) -> &WorthWorkload {
        &self.workload
    }

    pub fn authority(&self) -> &SpatialGeometryEvidenceTouchAuthority {
        &self.authority
    }

    pub fn lookup(&self) -> &SpatialEvidenceLookupProduct {
        &self.lookup
    }
}

fn completed_workload_with_boolean_receipt<T>(
    pair: &BuiltBooleanOperandPairRecipe,
    receipt: &T,
    expected_stage: BooleanEvidenceStageKind,
) -> WorthWorkload
where
    T: BooleanEvidenceRowAuthority + 'static,
{
    assert_eq!(receipt.boolean_stage(), expected_stage);
    let left = pair.left().workload();
    let evidence_ledger = left
        .evidence_ledger()
        .with_boolean_evidence_receipt(receipt)
        .expect("real boolean receipt should extend the complete workload ledger");

    WorthWorkload::compose(WorthWorkloadParts {
        topology: left.topology().clone(),
        geometry_binding: left.geometry_binding().clone(),
        surface_support: left.surface_support().clone(),
        projection: left.projection().clone(),
        transform: left.transform().clone(),
        retained_replay: left.retained_replay().clone(),
        batch_admission_execution: left.batch_admission_execution().cloned(),
        diagnostics: left.diagnostics().clone(),
        response: left.response().clone(),
        evidence_ledger,
    })
    .expect("completed workload should recompose with real boolean evidence")
}
