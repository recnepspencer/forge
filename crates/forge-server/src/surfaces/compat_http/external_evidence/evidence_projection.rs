use forge_foundational::facade::{
    counter_backed_performance_receipt, performance, performance_bundle,
    FoundationalAuthoritativePerformanceClaim, FoundationalCounterBackedPerformanceReceipt,
    FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceContractName,
    FoundationalPerformanceCounterName, FoundationalPerformanceCounterRow,
    FoundationalPerformanceCounterSpec, FoundationalPerformanceEvidenceStrength,
    FoundationalPerformanceExecutionTemperature, FoundationalPerformanceFallbackDebtPosture,
    FoundationalPerformanceFreshnessRetentionPosture, FoundationalPerformanceWorkClass,
};

use crate::{
    ForgeServerCompatibilityFileEnvelope, ForgeServerOperatorEvidenceFacade,
    ForgeServerQuerySupportPosture, ForgeServerResponseEnvelope,
};

use super::{
    ForgeServerBinaryCertificationBundle, ForgeServerBinaryCounterSet,
    ForgeServerCompatibilityCertificationBundle, ForgeServerExternalCounterSet,
    ForgeServerExternalEvidenceRecord,
};

const READ_SUCCESSES: &str = "compat_http.external.read.successes";
const INSPECTION_SUCCESSES: &str = "compat_http.external.inspection.successes";
const UPLOAD_SUCCESSES: &str = "compat_http.external.upload.successes";
const DOWNLOAD_SUCCESSES: &str = "compat_http.external.download.successes";
const STREAMING_SUCCESSES: &str = "compat_http.external.streaming.successes";
const BUFFERED_EXPORT_SUCCESSES: &str = "compat_http.external.buffered_export.successes";
const BACKGROUND_EXPORT_SUCCESSES: &str = "compat_http.external.background_export.successes";
const REQUEST_CONTEXT_DENIALS: &str = "compat_http.external.request_context_denials";
const MIDDLEWARE_DENIALS: &str = "compat_http.external.middleware_denials";
const QUERY_HANDOFF_DENIALS: &str = "compat_http.external.query_handoff_denials";
const DENIALS: &str = "compat_http.external.denials";

pub(crate) fn build_read_certification_bundle(
    operator_evidence: &ForgeServerOperatorEvidenceFacade,
    support_posture: &ForgeServerQuerySupportPosture,
    file_envelope: &ForgeServerCompatibilityFileEnvelope,
    response: &ForgeServerResponseEnvelope,
) -> ForgeServerCompatibilityCertificationBundle {
    build_compatibility_bundle(
        "compatibility_read",
        CompatHttpEvidenceSurface::Read,
        operator_evidence,
        support_posture,
        file_envelope,
        response,
    )
}

pub(crate) fn build_inspection_certification_bundle(
    operator_evidence: &ForgeServerOperatorEvidenceFacade,
    support_posture: &ForgeServerQuerySupportPosture,
    file_envelope: &ForgeServerCompatibilityFileEnvelope,
    response: &ForgeServerResponseEnvelope,
) -> ForgeServerCompatibilityCertificationBundle {
    build_compatibility_bundle(
        "compatibility_inspection",
        CompatHttpEvidenceSurface::Inspection,
        operator_evidence,
        support_posture,
        file_envelope,
        response,
    )
}

pub(crate) fn build_upload_certification_bundle(
    operator_evidence: &ForgeServerOperatorEvidenceFacade,
    support_posture: &ForgeServerQuerySupportPosture,
    file_envelope: &ForgeServerCompatibilityFileEnvelope,
    response: &ForgeServerResponseEnvelope,
    receipt: &crate::ForgeServerIngressPerformanceReceipt,
) -> ForgeServerBinaryCertificationBundle {
    build_binary_bundle(
        "compatibility_upload",
        CompatHttpEvidenceSurface::Upload,
        operator_evidence,
        support_posture,
        file_envelope,
        response,
        ForgeServerBinaryCounterSet::new(receipt.receipt().clone()),
    )
}

pub(crate) fn build_download_certification_bundle(
    operator_evidence: &ForgeServerOperatorEvidenceFacade,
    support_posture: &ForgeServerQuerySupportPosture,
    file_envelope: &ForgeServerCompatibilityFileEnvelope,
    response: &ForgeServerResponseEnvelope,
    receipt: &crate::ForgeServerBinaryEgressPerformanceReceipt,
) -> ForgeServerBinaryCertificationBundle {
    build_binary_bundle(
        "compatibility_download",
        CompatHttpEvidenceSurface::Download,
        operator_evidence,
        support_posture,
        file_envelope,
        response,
        ForgeServerBinaryCounterSet::new(receipt.receipt().clone()),
    )
}

pub(crate) fn build_streaming_export_certification_bundle(
    operator_evidence: &ForgeServerOperatorEvidenceFacade,
    support_posture: &ForgeServerQuerySupportPosture,
    file_envelope: &ForgeServerCompatibilityFileEnvelope,
    response: &ForgeServerResponseEnvelope,
    receipt: &crate::ForgeServerStreamingPerformanceReceipt,
) -> ForgeServerBinaryCertificationBundle {
    build_binary_bundle(
        "compatibility_stream",
        CompatHttpEvidenceSurface::Streaming,
        operator_evidence,
        support_posture,
        file_envelope,
        response,
        ForgeServerBinaryCounterSet::new(receipt.receipt().clone()),
    )
}

pub(crate) fn build_buffered_export_certification_bundle(
    operator_evidence: &ForgeServerOperatorEvidenceFacade,
    support_posture: &ForgeServerQuerySupportPosture,
    file_envelope: &ForgeServerCompatibilityFileEnvelope,
    response: &ForgeServerResponseEnvelope,
    receipt: &crate::ForgeServerStreamingPerformanceReceipt,
) -> ForgeServerBinaryCertificationBundle {
    build_binary_bundle(
        "compatibility_buffered_export",
        CompatHttpEvidenceSurface::BufferedExport,
        operator_evidence,
        support_posture,
        file_envelope,
        response,
        ForgeServerBinaryCounterSet::new(receipt.receipt().clone()),
    )
}

pub(crate) fn build_background_export_certification_bundle(
    operator_evidence: &ForgeServerOperatorEvidenceFacade,
    support_posture: &ForgeServerQuerySupportPosture,
    file_envelope: &ForgeServerCompatibilityFileEnvelope,
    response: &ForgeServerResponseEnvelope,
    receipt: &crate::ForgeServerStreamingPerformanceReceipt,
) -> ForgeServerBinaryCertificationBundle {
    build_binary_bundle(
        "compatibility_background_export",
        CompatHttpEvidenceSurface::BackgroundExport,
        operator_evidence,
        support_posture,
        file_envelope,
        response,
        ForgeServerBinaryCounterSet::new(receipt.receipt().clone()),
    )
}

fn build_compatibility_bundle(
    surface_label: &'static str,
    surface: CompatHttpEvidenceSurface,
    operator_evidence: &ForgeServerOperatorEvidenceFacade,
    support_posture: &ForgeServerQuerySupportPosture,
    file_envelope: &ForgeServerCompatibilityFileEnvelope,
    response: &ForgeServerResponseEnvelope,
) -> ForgeServerCompatibilityCertificationBundle {
    let evidence = ForgeServerExternalEvidenceRecord::project(
        surface_label,
        response.clone(),
        operator_evidence,
    )
    .expect("compatibility operator evidence should materialize");
    let external_counters = ForgeServerExternalCounterSet::new(build_external_receipt(
        surface,
        evidence.operator_record(),
    ));
    ForgeServerCompatibilityCertificationBundle::new(
        support_posture.canonical_label(),
        policy_truth_digest(file_envelope),
        provenance_truth_digest(file_envelope),
        response,
        external_counters,
        evidence,
    )
}

fn build_binary_bundle(
    surface_label: &'static str,
    surface: CompatHttpEvidenceSurface,
    operator_evidence: &ForgeServerOperatorEvidenceFacade,
    support_posture: &ForgeServerQuerySupportPosture,
    file_envelope: &ForgeServerCompatibilityFileEnvelope,
    response: &ForgeServerResponseEnvelope,
    binary_counters: ForgeServerBinaryCounterSet,
) -> ForgeServerBinaryCertificationBundle {
    let evidence = ForgeServerExternalEvidenceRecord::project(
        surface_label,
        response.clone(),
        operator_evidence,
    )
    .expect("binary operator evidence should materialize");
    let external_counters = ForgeServerExternalCounterSet::new(build_external_receipt(
        surface,
        evidence.operator_record(),
    ));
    ForgeServerBinaryCertificationBundle::new(
        support_posture.canonical_label(),
        policy_truth_digest(file_envelope),
        provenance_truth_digest(file_envelope),
        response,
        external_counters,
        binary_counters,
        evidence,
    )
}

fn build_external_receipt(
    surface: CompatHttpEvidenceSurface,
    operator_record: &crate::ForgeServerOperatorEvidenceRecord,
) -> FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim> {
    let rows = counter_rows(surface, operator_record.classification());
    let claim = performance()
        .claim()
        .authoritative_execution()
        .boundary(FoundationalPerformanceBoundary::AuthoritativeExecution)
        .evidence_strength(FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt)
        .breadth_locality(FoundationalPerformanceBreadthLocalityPosture::PointLocal)
        .access_pattern(FoundationalPerformanceAccessPatternPosture::PointLookup)
        .execution_temperature(FoundationalPerformanceExecutionTemperature::WarmPath)
        .freshness_retention(FoundationalPerformanceFreshnessRetentionPosture::ExactBasisCurrent)
        .fallback_debt(FoundationalPerformanceFallbackDebtPosture::Verified)
        .include_work(FoundationalPerformanceWorkClass::PublicationDelivery)
        .exclude_work(FoundationalPerformanceWorkClass::ReplayReconstruction)
        .exclude_work(FoundationalPerformanceWorkClass::SupportReportAssembly)
        .finish()
        .expect("external evidence claim should validate");
    let bundle = rows
        .iter()
        .fold(
            performance_bundle(claim).attach_contract_name(
                FoundationalPerformanceContractName::new("compat_http.external.surface")
                    .expect("static external contract name should validate"),
            ),
            |bundle, (name, count)| {
                bundle.attach_counter_spec(FoundationalPerformanceCounterSpec::new(
                    FoundationalPerformanceCounterName::new(*name)
                        .expect("static external counter name should validate"),
                    FoundationalPerformanceWorkClass::PublicationDelivery,
                    *count,
                ))
            },
        )
        .finish()
        .expect("external evidence bundle should validate");
    rows.iter()
        .fold(
            counter_backed_performance_receipt(bundle),
            |receipt, (name, count)| {
                receipt.attach_counter_row(FoundationalPerformanceCounterRow::new(
                    FoundationalPerformanceCounterName::new(*name)
                        .expect("static external counter name should validate"),
                    *count,
                ))
            },
        )
        .finish()
        .expect("external evidence receipt should validate")
}

fn counter_rows(
    surface: CompatHttpEvidenceSurface,
    classification: &crate::ForgeServerOperatorEvidenceClass,
) -> [(&'static str, u64); 11] {
    [
        (
            READ_SUCCESSES,
            u64::from(matches!(surface, CompatHttpEvidenceSurface::Read)),
        ),
        (
            INSPECTION_SUCCESSES,
            u64::from(matches!(surface, CompatHttpEvidenceSurface::Inspection)),
        ),
        (
            UPLOAD_SUCCESSES,
            u64::from(matches!(surface, CompatHttpEvidenceSurface::Upload)),
        ),
        (
            DOWNLOAD_SUCCESSES,
            u64::from(matches!(surface, CompatHttpEvidenceSurface::Download)),
        ),
        (
            STREAMING_SUCCESSES,
            u64::from(matches!(surface, CompatHttpEvidenceSurface::Streaming)),
        ),
        (
            BUFFERED_EXPORT_SUCCESSES,
            u64::from(matches!(surface, CompatHttpEvidenceSurface::BufferedExport)),
        ),
        (
            BACKGROUND_EXPORT_SUCCESSES,
            u64::from(matches!(
                surface,
                CompatHttpEvidenceSurface::BackgroundExport
            )),
        ),
        (
            REQUEST_CONTEXT_DENIALS,
            u64::from(matches!(
                classification,
                crate::ForgeServerOperatorEvidenceClass::RequestContextDenied(_)
            )),
        ),
        (
            MIDDLEWARE_DENIALS,
            u64::from(matches!(
                classification,
                crate::ForgeServerOperatorEvidenceClass::MiddlewareDenied(_)
            )),
        ),
        (
            QUERY_HANDOFF_DENIALS,
            u64::from(matches!(
                classification,
                crate::ForgeServerOperatorEvidenceClass::QueryHandoffDenied(_)
            )),
        ),
        (
            DENIALS,
            u64::from(matches!(
                classification,
                crate::ForgeServerOperatorEvidenceClass::RequestContextDenied(_)
                    | crate::ForgeServerOperatorEvidenceClass::MiddlewareDenied(_)
                    | crate::ForgeServerOperatorEvidenceClass::QueryHandoffDenied(_)
            )),
        ),
    ]
}

fn policy_truth_digest(file_envelope: &ForgeServerCompatibilityFileEnvelope) -> String {
    let policy = file_envelope.policy_decision();
    format!(
        "forge-server-file-policy-truth-v1|identity={}|tenant={}|workspace={}|branch={}|operation={}|lane={}|support={}|authorization={}",
        policy.metadata_identity(),
        policy.tenant_id(),
        policy.workspace_digest(),
        policy.branch_digest(),
        policy.operation_name(),
        policy.policy_lane(),
        policy.support_posture_digest(),
        policy.transfer_authorization_digest().unwrap_or("none"),
    )
}

fn provenance_truth_digest(file_envelope: &ForgeServerCompatibilityFileEnvelope) -> String {
    let provenance = file_envelope.transfer_provenance();
    format!(
        "forge-server-file-transfer-truth-v1|identity={}|tenant={}|workspace={}|branch={}|operation={}|disposition={:?}|content_type={}|bytes={}|range_honored={}",
        provenance.metadata_identity(),
        provenance.tenant_id(),
        provenance.workspace_digest(),
        provenance.branch_digest(),
        provenance.operation_name(),
        provenance.disposition(),
        provenance.content_type().unwrap_or("none"),
        provenance.bytes_selected(),
        provenance.range_honored(),
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CompatHttpEvidenceSurface {
    Read,
    Inspection,
    Upload,
    Download,
    Streaming,
    BufferedExport,
    BackgroundExport,
}
