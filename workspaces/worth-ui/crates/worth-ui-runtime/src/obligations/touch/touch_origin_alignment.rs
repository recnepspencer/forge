use std::collections::BTreeMap;

use crate::obligations::touch::{
    UiGraphTouchAspectFact, UiGraphTouchAspects, UiGraphTouchDenial, UiGraphTouchOriginClass,
};
use crate::runtime::{
    WorthUiActiveRuntimeObservation, WorthUiArtifactToPlanProvenance,
    WorthUiExecutionPlanInspection, WorthUiOrdinaryLaneFrameReceipt,
    WorthUiRuntimeDiagnosticReport,
};

pub(crate) fn require_host_observation_alignment(
    observation: &WorthUiActiveRuntimeObservation,
    inspection: &WorthUiExecutionPlanInspection,
) -> Result<(), UiGraphTouchDenial> {
    if observation.artifact_digest() == inspection.active_artifact_digest()
        && observation.active_plan_digest() == inspection.plan_digest().raw()
    {
        return Ok(());
    }

    Err(UiGraphTouchDenial::OriginOwnerMismatch {
        origin_class: UiGraphTouchOriginClass::HostObservation,
    })
}

pub(crate) fn require_service_event_alignment(
    frame_receipt: &WorthUiOrdinaryLaneFrameReceipt,
    inspection: &WorthUiExecutionPlanInspection,
) -> Result<(), UiGraphTouchDenial> {
    if frame_receipt
        .certification()
        .handle_receipt()
        .basis_digest()
        == inspection.handle_basis_digest()
    {
        return Ok(());
    }

    Err(UiGraphTouchDenial::OriginOwnerMismatch {
        origin_class: UiGraphTouchOriginClass::ServiceEvent,
    })
}

pub(crate) fn require_runtime_diagnostic_alignment(
    report: &WorthUiRuntimeDiagnosticReport,
    inspection: &WorthUiExecutionPlanInspection,
) -> Result<(), UiGraphTouchDenial> {
    if report.active_artifact_digest() == inspection.active_artifact_digest()
        && report.active_plan_digest() == inspection.plan_digest().raw()
    {
        return Ok(());
    }

    Err(UiGraphTouchDenial::OriginOwnerMismatch {
        origin_class: UiGraphTouchOriginClass::DiagnosticOnly,
    })
}

pub(crate) fn inspection_authored_provenance_digests<'a>(
    rows: impl Iterator<Item = &'a WorthUiArtifactToPlanProvenance>,
) -> Vec<u64> {
    let mut digests = rows
        .filter_map(WorthUiArtifactToPlanProvenance::authored_provenance_digest)
        .collect::<Vec<_>>();
    digests.sort_unstable();
    digests.dedup();
    digests
}

pub(crate) fn normalize_aspects(
    aspects: &UiGraphTouchAspects,
) -> Result<Vec<UiGraphTouchAspectFact>, UiGraphTouchDenial> {
    let mut normalized = BTreeMap::new();
    for fact in aspects.facts() {
        match normalized.insert(fact.lane(), fact.posture()) {
            Some(existing) if existing != fact.posture() => {
                return Err(UiGraphTouchDenial::ContradictoryAspectPosture {
                    lane: fact.lane(),
                    first: existing,
                    second: fact.posture(),
                });
            }
            _ => {}
        }
    }

    if normalized.is_empty() {
        return Err(UiGraphTouchDenial::MissingAspectPosture);
    }

    Ok(normalized
        .into_iter()
        .map(|(lane, posture)| UiGraphTouchAspectFact::new(lane, posture))
        .collect())
}
