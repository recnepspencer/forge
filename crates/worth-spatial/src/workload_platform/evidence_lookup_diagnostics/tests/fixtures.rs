use crate::trusted_boolean_evidence_authority::Seal;
use crate::workload_platform::evidence_ledger::{
    receipt_backed_touch_authority_for_admission_tests,
    receipt_backed_touch_authority_for_admission_tests_with_declared_world,
    BooleanEvidenceRowAuthority, BooleanEvidenceStageKind, SelectedLookupSliceLedger,
    SelectedLookupSliceLedgerAssembly, SpatialGeometryEvidenceTouchAuthority,
};
use crate::workload_platform::evidence_lookup_execution::{
    execute_evidence_lookup, EvidenceLookupExecutionReceipt, EvidenceLookupExecutionRequest,
};
use crate::workload_platform::evidence_lookup_family_catalog::{
    current_evidence_lookup_family_catalog, EvidenceLookupFamilyCatalogCloseout,
    EvidenceLookupProjectionFactFamily, EvidenceLookupStageReceiptFamilyIdentity,
};
use crate::workload_platform::evidence_lookup_index_product::{
    admit_evidence_lookup_index_product, EvidenceLookupIndexProduct,
};
use crate::workload_platform::evidence_lookup_input_admission::{
    admit_evidence_lookup_input, real_projection_consumption_receipt,
    EvidenceLookupInputAdmissionRequest, EvidenceLookupQueryAdmissionEvidenceSet,
    EvidenceLookupStageReceiptAdmission,
};
use crate::workload_platform::evidence_lookup_plan_selection::{
    select_evidence_lookup_plan, EvidenceLookupSelectedPlan,
};

pub(super) struct ProjectionDiagnosticPath {
    selected_plan: EvidenceLookupSelectedPlan,
    execution_receipt: EvidenceLookupExecutionReceipt,
}

impl ProjectionDiagnosticPath {
    pub(super) fn selected_plan(&self) -> &EvidenceLookupSelectedPlan {
        &self.selected_plan
    }

    pub(super) fn execution_receipt(&self) -> &EvidenceLookupExecutionReceipt {
        &self.execution_receipt
    }
}

pub(super) fn current_catalog() -> EvidenceLookupFamilyCatalogCloseout {
    current_evidence_lookup_family_catalog().expect("catalog closes")
}

pub(super) fn supported_projection_path() -> ProjectionDiagnosticPath {
    projection_path(ProjectionFixtureScenario::supported())
}

pub(super) fn required_support_projection_path() -> ProjectionDiagnosticPath {
    projection_path(ProjectionFixtureScenario::required_support())
}

pub(super) fn missing_projection_fact_path() -> ProjectionDiagnosticPath {
    let supported = supported_projection_path();
    ProjectionDiagnosticPath {
        selected_plan: supported.selected_plan,
        execution_receipt: supported
            .execution_receipt
            .with_outcome_for_tests(
                crate::workload_platform::evidence_lookup_execution::EvidenceLookupExecutionOutcome::MissingProjectionConsumptionFact,
            ),
    }
}

pub(super) fn alternate_spatial_projection_path() -> ProjectionDiagnosticPath {
    projection_path(ProjectionFixtureScenario::alternate_spatial_world())
}

pub(super) fn product_swap_projection_receipt() -> EvidenceLookupExecutionReceipt {
    supported_projection_path()
        .execution_receipt
        .with_selected_plan_digest_for_tests("phase-12:synthetic-product-swap")
}

pub(super) fn supported_projection_path_with_extra_unrelated_receipts(
    extra_unrelated_receipt_count: usize,
) -> ProjectionDiagnosticPath {
    projection_path(
        ProjectionFixtureScenario::supported()
            .with_extra_unrelated_receipts(extra_unrelated_receipt_count),
    )
}

pub(super) fn event_path() -> ProjectionDiagnosticPath {
    let catalog = current_catalog();
    let authority = receipt_backed_touch_authority_for_admission_tests(
        BooleanEvidenceStageKind::EventLedger,
        "phase-12-event-stage",
    );
    build_path(
        &catalog,
        &authority,
        EvidenceLookupStageReceiptFamilyIdentity::boolean_event_ledger(),
        None,
        false,
        1,
    )
}

struct ProjectionFixtureScenario {
    authority: SpatialGeometryEvidenceTouchAuthority,
    admitted_query_support: bool,
    include_projection_receipt: bool,
    extra_unrelated_receipt_count: usize,
}

impl ProjectionFixtureScenario {
    fn supported() -> Self {
        Self {
            authority: receipt_backed_touch_authority_for_admission_tests(
                BooleanEvidenceStageKind::OperandAProjectionConsumption,
                "phase-12-projection-supported",
            ),
            admitted_query_support: true,
            include_projection_receipt: true,
            extra_unrelated_receipt_count: 1,
        }
    }

    fn required_support() -> Self {
        Self {
            include_projection_receipt: false,
            ..Self::supported()
        }
    }

    fn alternate_spatial_world() -> Self {
        Self {
            authority: receipt_backed_touch_authority_for_admission_tests_with_declared_world(
                BooleanEvidenceStageKind::OperandAProjectionConsumption,
                "phase-12-projection-supported",
                "phase-12 alternate spatial world",
            ),
            ..Self::supported()
        }
    }

    fn with_extra_unrelated_receipts(mut self, extra_unrelated_receipt_count: usize) -> Self {
        self.extra_unrelated_receipt_count = extra_unrelated_receipt_count;
        self
    }
}

fn projection_path(scenario: ProjectionFixtureScenario) -> ProjectionDiagnosticPath {
    let catalog = current_catalog();
    build_path(
        &catalog,
        &scenario.authority,
        EvidenceLookupStageReceiptFamilyIdentity::boolean_operand_projection_consumption(),
        scenario.admitted_query_support.then_some(
            EvidenceLookupQueryAdmissionEvidenceSet::from_projection_consumption_receipt(
                &real_projection_consumption_receipt(),
                EvidenceLookupProjectionFactFamily::SpatialTouchOperandProjection,
            ),
        ),
        scenario.include_projection_receipt,
        scenario.extra_unrelated_receipt_count,
    )
}

fn build_path(
    catalog: &EvidenceLookupFamilyCatalogCloseout,
    authority: &SpatialGeometryEvidenceTouchAuthority,
    receipt_family: EvidenceLookupStageReceiptFamilyIdentity,
    query_evidence: Option<EvidenceLookupQueryAdmissionEvidenceSet>,
    include_projection_receipt: bool,
    extra_unrelated_receipt_count: usize,
) -> ProjectionDiagnosticPath {
    let mut request = EvidenceLookupInputAdmissionRequest::from_spatial_touch_authority(authority)
        .with_stage_receipt_identity(
            EvidenceLookupStageReceiptAdmission::from_spatial_touch_authority(
                authority,
                receipt_family,
            ),
        );
    if let Some(query_evidence) = query_evidence {
        request = request.with_query_import_evidence(query_evidence);
    }
    let admitted = admit_evidence_lookup_input(catalog, request).expect("admitted input");
    let selected_plan = select_evidence_lookup_plan(catalog, &admitted).expect("selected plan");
    let index_product = admit_index_product(
        &selected_plan,
        &projection_lookup_slice(authority, extra_unrelated_receipt_count),
    );
    let execution_receipt =
        execute_projection_request(&selected_plan, &index_product, include_projection_receipt);
    ProjectionDiagnosticPath {
        selected_plan,
        execution_receipt,
    }
}

fn admit_index_product(
    selected_plan: &EvidenceLookupSelectedPlan,
    ledger: &SelectedLookupSliceLedger,
) -> EvidenceLookupIndexProduct {
    admit_evidence_lookup_index_product(selected_plan, ledger).expect("index product")
}

fn execute_projection_request(
    selected_plan: &EvidenceLookupSelectedPlan,
    index_product: &EvidenceLookupIndexProduct,
    include_projection_receipt: bool,
) -> EvidenceLookupExecutionReceipt {
    let projection_receipt = include_projection_receipt
        .then(|| Box::leak(Box::new(real_projection_consumption_receipt())));
    let request = if let Some(projection_receipt) = projection_receipt {
        EvidenceLookupExecutionRequest::new(selected_plan, index_product)
            .with_projection_consumption_receipt(
                "spatial-touch.boolean.projection-consumption-evidence.v1".to_string(),
                EvidenceLookupProjectionFactFamily::SpatialTouchOperandProjection,
                projection_receipt,
            )
    } else {
        EvidenceLookupExecutionRequest::new(selected_plan, index_product)
    };
    execute_evidence_lookup(&request).expect("execution receipt")
}

fn projection_lookup_slice(
    authority: &SpatialGeometryEvidenceTouchAuthority,
    extra_unrelated_receipt_count: usize,
) -> SelectedLookupSliceLedger {
    let stage_receipt = EvidenceLookupStageReceiptAdmission::from_spatial_touch_authority(
        authority,
        EvidenceLookupStageReceiptFamilyIdentity::boolean_operand_projection_consumption(),
    );
    let unrelated_stage_kinds = [
        BooleanEvidenceStageKind::SharedPlaneIdentity,
        BooleanEvidenceStageKind::LocalFrameSelection,
        BooleanEvidenceStageKind::EventLedger,
        BooleanEvidenceStageKind::Split,
    ];
    for index in 0..extra_unrelated_receipt_count {
        let unrelated_authority = receipt_backed_touch_authority_for_admission_tests(
            unrelated_stage_kinds[index % unrelated_stage_kinds.len()],
            Box::leak(format!("phase-12-unrelated-shared-plane-{index}").into_boxed_str()),
        );
        let _ = UnrelatedBooleanReceipt::from_touch_authority(&unrelated_authority);
    }
    SelectedLookupSliceLedgerAssembly::from_touch_authority(authority, &stage_receipt)
        .assemble_selected_lookup_slice()
        .expect("test lookup slice")
}

struct UnrelatedBooleanReceipt {
    boolean_stage: BooleanEvidenceStageKind,
    evidence_identity: String,
    support: crate::workload_platform::evidence_ledger::WorkloadEvidenceSupport,
    counters: crate::workload_platform::evidence_ledger::WorkloadEvidenceStageCounters,
}

impl UnrelatedBooleanReceipt {
    fn from_touch_authority(authority: &SpatialGeometryEvidenceTouchAuthority) -> Self {
        Self {
            boolean_stage: authority.boolean_stage(),
            evidence_identity: format!("{}:unrelated", authority.evidence_identity()),
            support: authority.support(),
            counters: authority.evidence_counters(),
        }
    }
}

impl crate::workload_platform::evidence_ledger::BooleanEvidenceReceipt for UnrelatedBooleanReceipt {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        self.boolean_stage
    }

    fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    fn evidence_support(
        &self,
    ) -> crate::workload_platform::evidence_ledger::WorkloadEvidenceSupport {
        self.support
    }

    fn evidence_counters(
        &self,
    ) -> crate::workload_platform::evidence_ledger::WorkloadEvidenceStageCounters {
        self.counters
    }
}

impl Seal for UnrelatedBooleanReceipt {}
impl BooleanEvidenceRowAuthority for UnrelatedBooleanReceipt {}
