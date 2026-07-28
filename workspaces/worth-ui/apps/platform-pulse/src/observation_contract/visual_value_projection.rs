use worth_ui::facade::inspection::{
    UiClientPhysicalPixel, UiPixelsRequired, UiVisualCoordinateOrientation,
    UiVisualCoordinateRounding, UiVisualEvidenceRef, UiVisualHitTestOutcome, UiVisualIdentityTrace,
    UiVisualPixelCaptureSource, UiVisualPixelColorSpace, UiVisualPixelRetentionDisposition,
    UiVisualPointAdjudication, UiVisualSnapshotReceipt, UiVisualSnapshotRelation,
    UiVisualVisibleOutcome,
};

use super::projection::PlatformPulseLifecycleObservationProjectionDenial;
use super::visual::{
    PlatformPulseVisualCoordinateObservation, PlatformPulseVisualCoordinateOrientationObservation,
    PlatformPulseVisualCoordinateRoundingObservation, PlatformPulseVisualEvidenceFamilyObservation,
    PlatformPulseVisualEvidenceObservation, PlatformPulseVisualIdentityTraceObservation,
    PlatformPulseVisualMountedNodeObservation, PlatformPulseVisualPixelColorSpaceObservation,
    PlatformPulseVisualPixelObservation, PlatformPulseVisualPointResolutionObservation,
    PlatformPulseVisualSnapshotAffinityObservation, PlatformPulseVisualSnapshotCaptured,
    PlatformPulseVisualSnapshotRelationObservation,
};

pub(super) fn project_snapshot(
    receipt: &UiVisualSnapshotReceipt<UiPixelsRequired>,
) -> Result<PlatformPulseVisualSnapshotCaptured, PlatformPulseLifecycleObservationProjectionDenial>
{
    let affinity = receipt.affinity();
    let coordinates = receipt.coordinates();
    let pixels = receipt.pixel_artifact();
    if pixels.capture_source() != UiVisualPixelCaptureSource::NativePresentation
        || pixels.retention() != UiVisualPixelRetentionDisposition::Retained
    {
        return Err(PlatformPulseLifecycleObservationProjectionDenial::VisualAffinityMismatch);
    }
    Ok(PlatformPulseVisualSnapshotCaptured {
        affinity: PlatformPulseVisualSnapshotAffinityObservation {
            snapshot: affinity.snapshot(),
            presentation_attempt: affinity.presentation_attempt(),
            frame: affinity.frame(),
            semantic_surface: affinity.semantic_surface(),
            host_surface: affinity.host_surface(),
            binding_generation: affinity.binding_generation(),
            presentation_epoch: affinity.presentation_epoch(),
            relation: relation(affinity.relation()),
        },
        captured_client_extent: rect(receipt.captured_client_extent()),
        coordinates: PlatformPulseVisualCoordinateObservation {
            native_client_origin: coordinates.native_client_origin(),
            client_physical_dimensions: coordinates.client_physical_dimensions(),
            viewport_logical_dimension_bits: bits(coordinates.viewport_logical_dimensions()),
            scale_bits: bits(coordinates.scale()),
            translation_bits: bits(coordinates.translation()),
            orientation: orientation(coordinates.orientation()),
            rounding: rounding(coordinates.rounding()),
        },
        pixels: PlatformPulseVisualPixelObservation {
            dimensions: pixels.dimensions(),
            stride: pixels.stride(),
            byte_count: u64::try_from(pixels.bytes().len()).map_err(|_| {
                PlatformPulseLifecycleObservationProjectionDenial::ObservationValueOverflow
            })?,
            color_space: color_space(pixels.color_space()),
        },
        visible_region_count: u64::try_from(receipt.visible_region_count()).map_err(|_| {
            PlatformPulseLifecycleObservationProjectionDenial::ObservationValueOverflow
        })?,
        hit_test_region_count: u64::try_from(receipt.hit_test_region_count()).map_err(|_| {
            PlatformPulseLifecycleObservationProjectionDenial::ObservationValueOverflow
        })?,
        cost_counters: receipt.cost().counters(),
    })
}

pub(super) fn project_point_resolution(
    point: UiClientPhysicalPixel,
    adjudication: &UiVisualPointAdjudication,
) -> Result<
    PlatformPulseVisualPointResolutionObservation,
    PlatformPulseLifecycleObservationProjectionDenial,
> {
    let UiVisualVisibleOutcome::Contributors(visible) = adjudication.visible() else {
        return Err(PlatformPulseLifecycleObservationProjectionDenial::VisualPointUnsupported);
    };
    let visible = visible
        .frontmost()
        .ok_or(PlatformPulseLifecycleObservationProjectionDenial::VisualPointUnsupported)?;
    let UiVisualHitTestOutcome::Target(hit) = adjudication.hit_test() else {
        return Err(PlatformPulseLifecycleObservationProjectionDenial::VisualPointUnsupported);
    };
    Ok(PlatformPulseVisualPointResolutionObservation {
        point: [point.x(), point.y()],
        visible_region: rect(visible.region()),
        visible: project_trace(visible.identity_trace())?,
        hit: project_trace(hit.identity_trace())?,
    })
}

fn project_trace(
    trace: &UiVisualIdentityTrace,
) -> Result<
    PlatformPulseVisualIdentityTraceObservation,
    PlatformPulseLifecycleObservationProjectionDenial,
> {
    let mounted = trace.mounted_node();
    let provenance = trace.authored_provenance();
    let evidence = trace
        .evidence()
        .iter()
        .map(project_evidence)
        .collect::<Result<Vec<_>, _>>()?
        .into_boxed_slice();
    Ok(PlatformPulseVisualIdentityTraceObservation {
        mounted: PlatformPulseVisualMountedNodeObservation {
            node_receipt: mounted.node_receipt(),
            mounted_instance: mounted.mounted_instance(),
            incarnation: mounted.incarnation(),
        },
        graph_node: trace.graph_node().diagnostic_value(),
        declaration: trace.declaration().diagnostic_value(),
        authored_semantic_name: trace.declaration().authored_semantic_name().to_owned(),
        source_artifact_path: provenance.source_artifact().path().to_owned(),
        source_generation: provenance.source_generation().raw(),
        declaration_index: u64::try_from(provenance.declaration_index()).map_err(|_| {
            PlatformPulseLifecycleObservationProjectionDenial::ObservationValueOverflow
        })?,
        evidence,
    })
}

fn project_evidence(
    evidence: &UiVisualEvidenceRef,
) -> Result<PlatformPulseVisualEvidenceObservation, PlatformPulseLifecycleObservationProjectionDenial>
{
    let family = match evidence.family() {
        worth_ui::facade::inspection::UiEvidenceFamily::Declaration => {
            PlatformPulseVisualEvidenceFamilyObservation::Declaration
        }
        worth_ui::facade::inspection::UiEvidenceFamily::Admission => {
            PlatformPulseVisualEvidenceFamilyObservation::Admission
        }
        worth_ui::facade::inspection::UiEvidenceFamily::Graph => {
            PlatformPulseVisualEvidenceFamilyObservation::Graph
        }
        worth_ui::facade::inspection::UiEvidenceFamily::Planning => {
            PlatformPulseVisualEvidenceFamilyObservation::Planning
        }
        worth_ui::facade::inspection::UiEvidenceFamily::Aspect => {
            PlatformPulseVisualEvidenceFamilyObservation::Aspect
        }
        worth_ui::facade::inspection::UiEvidenceFamily::Obligation => {
            PlatformPulseVisualEvidenceFamilyObservation::Obligation
        }
        _ => return Err(PlatformPulseLifecycleObservationProjectionDenial::VisualPointUnsupported),
    };
    Ok(PlatformPulseVisualEvidenceObservation {
        family,
        authority_generation: evidence.authority_generation(),
        identity_digest: evidence.identity_digest(),
        handle_digest: evidence.handle_digest(),
    })
}

pub(super) fn rect(rect: worth_ui::facade::inspection::UiClientPhysicalRect) -> [u32; 4] {
    [rect.left(), rect.top(), rect.right(), rect.bottom()]
}

fn bits(values: [f32; 2]) -> [u32; 2] {
    [values[0].to_bits(), values[1].to_bits()]
}

fn relation(relation: UiVisualSnapshotRelation) -> PlatformPulseVisualSnapshotRelationObservation {
    match relation {
        UiVisualSnapshotRelation::Current => {
            PlatformPulseVisualSnapshotRelationObservation::Current
        }
        UiVisualSnapshotRelation::RetainedPredecessor => {
            PlatformPulseVisualSnapshotRelationObservation::RetainedPredecessor
        }
        UiVisualSnapshotRelation::Historical => {
            PlatformPulseVisualSnapshotRelationObservation::Historical
        }
    }
}

fn orientation(
    orientation: UiVisualCoordinateOrientation,
) -> PlatformPulseVisualCoordinateOrientationObservation {
    match orientation {
        UiVisualCoordinateOrientation::TopLeftOrigin => {
            PlatformPulseVisualCoordinateOrientationObservation::TopLeftOrigin
        }
        UiVisualCoordinateOrientation::BottomLeftOrigin => {
            PlatformPulseVisualCoordinateOrientationObservation::BottomLeftOrigin
        }
    }
}

fn rounding(
    rounding: UiVisualCoordinateRounding,
) -> PlatformPulseVisualCoordinateRoundingObservation {
    match rounding {
        UiVisualCoordinateRounding::PixelCenterNearest => {
            PlatformPulseVisualCoordinateRoundingObservation::PixelCenterNearest
        }
        UiVisualCoordinateRounding::FloorEdges => {
            PlatformPulseVisualCoordinateRoundingObservation::FloorEdges
        }
    }
}

fn color_space(
    color_space: UiVisualPixelColorSpace,
) -> PlatformPulseVisualPixelColorSpaceObservation {
    match color_space {
        UiVisualPixelColorSpace::Srgb => PlatformPulseVisualPixelColorSpaceObservation::Srgb,
        UiVisualPixelColorSpace::AdapterDeclared => {
            PlatformPulseVisualPixelColorSpaceObservation::AdapterDeclared
        }
    }
}
