use crate::courtroom::foundational::AspectNativeBoundaryHandoffVerdict;
use worth_foundational::{
    attach_counter_backed_performance_receipt, bridge_certified_performance_bundle_trust_boundary,
    certify_support_expansion_performance_report,
    foundational_performance_certified_attachment_authority,
    foundational_performance_certified_readmission_authority, plan_performance_report, profiles,
    readmit_certified_performance_bundle_after_boundary, AdmissionReadinessProfile,
    CertificationPostureProfile, CompatibilityPostureProfile, DiagnosticRichnessProfile,
    FoundationalCertifiedPerformanceClass, FoundationalCertifiedPerformanceSourceDigest,
    FoundationalPerformanceAttachmentTargetKind,
    FoundationalPerformanceReportMaterializationBoundary, FoundationalPerformanceReportRequest,
    FoundationalPerformanceReportSection, RetentionDeliveryProfile, SupportPostureProfile,
};
use worth_proof::TransitionOutcome;
use worth_store_layout_indexes::LayoutAccessPerformanceReceipt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutFoundationalCloseoutDenial {
    BoundaryEvidenceIncomplete,
    PerformanceAttachmentDenied,
    PerformancePlanIncomplete,
    PerformanceReportIncomplete,
    PerformanceCertificationDenied,
    PerformanceReadmissionMismatch,
}

/// Sealed courtroom receipt for the complete Foundational closeout chain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutFoundationalCloseoutEvidence {
    boundary_handoff: AspectNativeBoundaryHandoffVerdict,
    plan_binding: worth_store_layout_indexes::AccessPlanIdentity,
    report_boundary: FoundationalPerformanceReportMaterializationBoundary,
    counter_row_count: usize,
    support_row_count: usize,
    certified_class: FoundationalCertifiedPerformanceClass,
    readmitted_class: FoundationalCertifiedPerformanceClass,
    source_digest: FoundationalCertifiedPerformanceSourceDigest,
}

pub fn certify_layout_foundational_closeout(
    boundary_handoff: AspectNativeBoundaryHandoffVerdict,
    performance: LayoutAccessPerformanceReceipt,
) -> Result<LayoutFoundationalCloseoutEvidence, LayoutFoundationalCloseoutDenial> {
    require_boundary_evidence(&boundary_handoff)?;
    let plan_binding = performance.plan_binding().clone();
    let attached = attach_counter_backed_performance_receipt(
        FoundationalPerformanceAttachmentTargetKind::BoundaryReport,
        performance.counter_backed().clone(),
    )
    .map_err(|_| LayoutFoundationalCloseoutDenial::PerformanceAttachmentDenied)?;
    let plan = plan_performance_report(FoundationalPerformanceReportRequest {
        source: attached,
        profile: closeout_profile(),
        include_layout_intent: false,
        include_contract_names: true,
        include_counter_specs: true,
        include_counter_rows: true,
        include_supporting_evidence_rows: true,
        include_budget_decisions: false,
        include_denied_work: false,
        include_widened_work: false,
    });
    let required_sections = [
        FoundationalPerformanceReportSection::Claim,
        FoundationalPerformanceReportSection::ContractNames,
        FoundationalPerformanceReportSection::CounterSpecs,
        FoundationalPerformanceReportSection::CounterRows,
        FoundationalPerformanceReportSection::SupportingEvidenceRows,
    ];
    if plan.materialization_boundary()
        != FoundationalPerformanceReportMaterializationBoundary::SupportExpansion
        || !required_sections
            .into_iter()
            .all(|section| plan.included_sections().contains(&section))
    {
        return Err(LayoutFoundationalCloseoutDenial::PerformancePlanIncomplete);
    }
    let report_boundary = plan.materialization_boundary();
    let report = plan.materialize();
    if report.counter_rows().is_empty()
        || report.counter_rows().len() != report.counter_specs().len()
        || report.supporting_evidence_rows().is_empty()
    {
        return Err(LayoutFoundationalCloseoutDenial::PerformanceReportIncomplete);
    }
    let counter_row_count = report.counter_rows().len();
    let support_row_count = report.supporting_evidence_rows().len();
    let certified = match certify_support_expansion_performance_report(
        report,
        foundational_performance_certified_attachment_authority(),
    ) {
        TransitionOutcome::Success(certified) => certified,
        _ => return Err(LayoutFoundationalCloseoutDenial::PerformanceCertificationDenied),
    };
    let certified_class = certified.certified_class();
    let source_digest = certified.source_digest().clone();
    let readmission_basis = certified.readmission_basis().clone();
    let bridged = bridge_certified_performance_bundle_trust_boundary(certified);
    let readmitted = readmit_certified_performance_bundle_after_boundary(
        bridged,
        readmission_basis,
        foundational_performance_certified_readmission_authority(),
    );
    let readmitted_class = readmitted.certified_class();
    if certified_class != readmitted_class || source_digest != *readmitted.source_digest() {
        return Err(LayoutFoundationalCloseoutDenial::PerformanceReadmissionMismatch);
    }
    Ok(LayoutFoundationalCloseoutEvidence {
        boundary_handoff,
        plan_binding,
        report_boundary,
        counter_row_count,
        support_row_count,
        certified_class,
        readmitted_class,
        source_digest,
    })
}

fn require_boundary_evidence(
    evidence: &AspectNativeBoundaryHandoffVerdict,
) -> Result<(), LayoutFoundationalCloseoutDenial> {
    if evidence.canonical_basis_entry_count() == 0
        || evidence.receipt_count() == 0
        || evidence.diagnostic_count() == 0
        || evidence.performance_receipt_count() == 0
        || evidence.denied_input_count() == 0
    {
        Err(LayoutFoundationalCloseoutDenial::BoundaryEvidenceIncomplete)
    } else {
        Ok(())
    }
}

fn closeout_profile() -> worth_foundational::FoundationalProfileSet {
    profiles()
        .set()
        .diagnostic_richness(DiagnosticRichnessProfile::Standard)
        .support_posture(SupportPostureProfile::SupportReady)
        .compatibility_posture(CompatibilityPostureProfile::NativeOnly)
        .admission_readiness(AdmissionReadinessProfile::Admitted)
        .retention_delivery(RetentionDeliveryProfile::Retained)
        .certification_posture(CertificationPostureProfile::Uncertified)
        .compose()
        .expect("layout closeout profile is coherent")
}

impl LayoutFoundationalCloseoutEvidence {
    pub const fn boundary_handoff(&self) -> &AspectNativeBoundaryHandoffVerdict {
        &self.boundary_handoff
    }

    pub const fn plan_binding(&self) -> &worth_store_layout_indexes::AccessPlanIdentity {
        &self.plan_binding
    }

    pub const fn report_boundary(&self) -> FoundationalPerformanceReportMaterializationBoundary {
        self.report_boundary
    }

    pub const fn counter_row_count(&self) -> usize {
        self.counter_row_count
    }

    pub const fn support_row_count(&self) -> usize {
        self.support_row_count
    }

    pub const fn certified_class(&self) -> FoundationalCertifiedPerformanceClass {
        self.certified_class
    }

    pub const fn readmitted_class(&self) -> FoundationalCertifiedPerformanceClass {
        self.readmitted_class
    }

    pub const fn source_entry_count(&self) -> u32 {
        self.source_digest.entry_count()
    }

    pub const fn source_digest(&self) -> &FoundationalCertifiedPerformanceSourceDigest {
        &self.source_digest
    }
}
