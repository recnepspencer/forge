use std::fmt;

use worth_ui_platform_pulse::observation_contract::{
    PlatformPulseLifecycleObservation, PlatformPulseLifecycleObservationEnvelope,
    PlatformPulseVisualComparison, PlatformPulseVisualIdentityTraceObservation,
    PlatformPulseVisualPointTrace, PlatformPulseVisualSnapshotCaptured,
    PlatformPulseVisualSnapshotRetired,
};
use worth_ui_platform_pulse::visual_identity_pulse::{
    PLATFORM_PULSE_BACKGROUND_LOGICAL_POINT, PLATFORM_PULSE_CANONICAL_LOGICAL_EXTENT,
    PLATFORM_PULSE_IDENTITY_TARGET_AUTHORED_NAME, PLATFORM_PULSE_MAXIMUM_CAPTURE_SCALE,
    PLATFORM_PULSE_MAXIMUM_PIXEL_BYTES, PLATFORM_PULSE_TARGET_LOGICAL_POINT,
};

const TARGET_LOGICAL_INSET: [u32; 2] = [48, 24];

#[derive(Clone, Debug)]
pub(crate) struct ExecutableVisualSnapshotEvidence {
    snapshot: PlatformPulseVisualSnapshotCaptured,
}

#[derive(Clone, Debug)]
pub(crate) struct ExecutableVisualTraceEvidence {
    trace: PlatformPulseVisualPointTrace,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ExecutableVisualRetirementEvidence {
    retirement: PlatformPulseVisualSnapshotRetired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ExecutableVisualComparisonEvidence {
    comparison: PlatformPulseVisualComparison,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ExecutableVisualIdentityFailure {
    WrongEvent(&'static str),
    WrongSequence {
        expected: u64,
        observed: u64,
    },
    SnapshotAffinity,
    ComparisonSnapshotIdentity {
        expected: [u64; 2],
        observed: [u64; 2],
    },
    ComparisonMeaning {
        identity_rebound: bool,
        retained_pixels_differ: Option<bool>,
    },
    ComparisonCost {
        structural_entries_examined: u64,
        retained_pixel_bytes_examined: u64,
    },
    SnapshotExtent,
    SnapshotPixelBudget,
    SnapshotIndexCardinality,
    SnapshotCost,
    PointContract,
    TargetIdentity,
    BackgroundIdentity,
    AuthoredName,
    IncompleteTrace,
    OverlayAffinity,
    ClearAffinity,
    RetirementAffinity,
    NativeCaptureExtent,
    NativeProcessIdentity,
    BorderNotVisible {
        matching: usize,
        sampled: usize,
    },
    BorderStillVisible {
        matching: usize,
        sampled: usize,
    },
    TargetPixelChanged,
    BackgroundPixelChanged,
}

pub(crate) fn adjudicate_visual_snapshot(
    envelope: PlatformPulseLifecycleObservationEnvelope,
    expected_frame: u64,
) -> Result<ExecutableVisualSnapshotEvidence, ExecutableVisualIdentityFailure> {
    adjudicate_snapshot_at_sequence(envelope, expected_frame, 3)
}

pub(crate) fn adjudicate_successor_visual_snapshot(
    envelope: PlatformPulseLifecycleObservationEnvelope,
    expected_frame: u64,
) -> Result<ExecutableVisualSnapshotEvidence, ExecutableVisualIdentityFailure> {
    adjudicate_snapshot_at_sequence(envelope, expected_frame, 8)
}

fn adjudicate_snapshot_at_sequence(
    envelope: PlatformPulseLifecycleObservationEnvelope,
    expected_frame: u64,
    expected_sequence: u64,
) -> Result<ExecutableVisualSnapshotEvidence, ExecutableVisualIdentityFailure> {
    require_sequence(&envelope, expected_sequence)?;
    let PlatformPulseLifecycleObservation::VisualSnapshotCaptured(snapshot) = envelope.outcome()
    else {
        return Err(ExecutableVisualIdentityFailure::WrongEvent(
            "visual snapshot captured",
        ));
    };
    if snapshot.affinity().frame() != expected_frame
        || snapshot.affinity().snapshot() == 0
        || snapshot.affinity().relation()
            != worth_ui_platform_pulse::observation_contract::PlatformPulseVisualSnapshotRelationObservation::Current
    {
        return Err(ExecutableVisualIdentityFailure::SnapshotAffinity);
    }
    let physical_extent = expected_physical_extent(snapshot)?;
    if snapshot.captured_client_extent() != [0, 0, physical_extent[0], physical_extent[1]]
        || snapshot.coordinates().client_physical_dimensions() != physical_extent
        || snapshot.pixels().dimensions() != physical_extent
    {
        return Err(ExecutableVisualIdentityFailure::SnapshotExtent);
    }
    let expected_stride = physical_extent[0]
        .checked_mul(4)
        .ok_or(ExecutableVisualIdentityFailure::SnapshotPixelBudget)?;
    let expected_bytes = u64::from(expected_stride)
        .checked_mul(u64::from(physical_extent[1]))
        .ok_or(ExecutableVisualIdentityFailure::SnapshotPixelBudget)?;
    if snapshot.pixels().stride() != expected_stride
        || snapshot.pixels().byte_count() != expected_bytes
        || expected_bytes > PLATFORM_PULSE_MAXIMUM_PIXEL_BYTES
    {
        return Err(ExecutableVisualIdentityFailure::SnapshotPixelBudget);
    }
    if snapshot.visible_region_count() != 2 || snapshot.hit_test_region_count() != 2 {
        return Err(ExecutableVisualIdentityFailure::SnapshotIndexCardinality);
    }
    let cost = snapshot.cost_counters();
    if cost[4] != expected_bytes
        || cost[5] != expected_bytes
        || cost[6] != expected_bytes
        || cost[9] == 0
    {
        return Err(ExecutableVisualIdentityFailure::SnapshotCost);
    }
    Ok(ExecutableVisualSnapshotEvidence {
        snapshot: snapshot.clone(),
    })
}

pub(crate) fn adjudicate_visual_trace(
    envelope: PlatformPulseLifecycleObservationEnvelope,
    snapshot: &ExecutableVisualSnapshotEvidence,
) -> Result<ExecutableVisualTraceEvidence, ExecutableVisualIdentityFailure> {
    require_sequence(&envelope, 4)?;
    let PlatformPulseLifecycleObservation::VisualPointTrace(trace) = envelope.outcome() else {
        return Err(ExecutableVisualIdentityFailure::WrongEvent(
            "visual point trace",
        ));
    };
    if trace.snapshot() != snapshot.snapshot.affinity().snapshot()
        || trace.target().point()
            != snapshot.project_logical_point(PLATFORM_PULSE_TARGET_LOGICAL_POINT)?
        || trace.background().point()
            != snapshot.project_logical_point(PLATFORM_PULSE_BACKGROUND_LOGICAL_POINT)?
        || trace.target().visible_region() != snapshot.expected_target_region()?
    {
        return Err(ExecutableVisualIdentityFailure::PointContract);
    }
    require_same_resolution_identity(trace.target().visible(), trace.target().hit())
        .map_err(|_| ExecutableVisualIdentityFailure::TargetIdentity)?;
    require_same_resolution_identity(trace.background().visible(), trace.background().hit())
        .map_err(|_| ExecutableVisualIdentityFailure::BackgroundIdentity)?;
    if trace.target().hit().mounted().node_receipt()
        == trace.background().hit().mounted().node_receipt()
    {
        return Err(ExecutableVisualIdentityFailure::BackgroundIdentity);
    }
    if trace.target().hit().authored_semantic_name() != PLATFORM_PULSE_IDENTITY_TARGET_AUTHORED_NAME
    {
        return Err(ExecutableVisualIdentityFailure::AuthoredName);
    }
    require_complete_trace(trace.target().visible())?;
    require_complete_trace(trace.target().hit())?;
    require_complete_trace(trace.background().visible())?;
    require_complete_trace(trace.background().hit())?;
    Ok(ExecutableVisualTraceEvidence {
        trace: trace.clone(),
    })
}

fn expected_physical_extent(
    snapshot: &PlatformPulseVisualSnapshotCaptured,
) -> Result<[u32; 2], ExecutableVisualIdentityFailure> {
    let logical = float_pair(snapshot.coordinates().viewport_logical_dimension_bits());
    if logical != [160.0, 96.0] {
        return Err(ExecutableVisualIdentityFailure::SnapshotExtent);
    }
    let scale = float_pair(snapshot.coordinates().scale_bits());
    if scale.iter().any(|value| {
        !value.is_finite() || *value <= 0.0 || *value > PLATFORM_PULSE_MAXIMUM_CAPTURE_SCALE as f32
    }) {
        return Err(ExecutableVisualIdentityFailure::SnapshotExtent);
    }
    Ok([
        project_axis(PLATFORM_PULSE_CANONICAL_LOGICAL_EXTENT[0], scale[0], 0.0)?,
        project_axis(PLATFORM_PULSE_CANONICAL_LOGICAL_EXTENT[1], scale[1], 0.0)?,
    ])
}

fn float_pair(bits: [u32; 2]) -> [f32; 2] {
    [f32::from_bits(bits[0]), f32::from_bits(bits[1])]
}

fn project_axis(
    logical: u32,
    scale: f32,
    translation: f32,
) -> Result<u32, ExecutableVisualIdentityFailure> {
    let physical = logical as f64 * f64::from(scale) + f64::from(translation);
    if !physical.is_finite() || physical < 0.0 || physical > f64::from(u32::MAX) {
        return Err(ExecutableVisualIdentityFailure::SnapshotExtent);
    }
    Ok(physical.round() as u32)
}

pub(crate) fn adjudicate_visual_retirement(
    envelope: PlatformPulseLifecycleObservationEnvelope,
    snapshot: &ExecutableVisualSnapshotEvidence,
    successor_frame: u64,
) -> Result<ExecutableVisualRetirementEvidence, ExecutableVisualIdentityFailure> {
    require_sequence(&envelope, 10)?;
    let PlatformPulseLifecycleObservation::VisualSnapshotRetired(retirement) = envelope.outcome()
    else {
        return Err(ExecutableVisualIdentityFailure::WrongEvent(
            "visual snapshot retired",
        ));
    };
    if retirement.snapshot() != snapshot.snapshot.affinity().snapshot()
        || retirement.predecessor_frame() != snapshot.snapshot.affinity().frame()
        || retirement.successor_frame() != successor_frame
        || !retirement.explicitly_superseded()
        || !retirement.released_registered_resource()
    {
        return Err(ExecutableVisualIdentityFailure::RetirementAffinity);
    }
    Ok(ExecutableVisualRetirementEvidence {
        retirement: *retirement,
    })
}

pub(crate) fn adjudicate_visual_comparison(
    envelope: PlatformPulseLifecycleObservationEnvelope,
    predecessor: &ExecutableVisualSnapshotEvidence,
    successor: &ExecutableVisualSnapshotEvidence,
) -> Result<ExecutableVisualComparisonEvidence, ExecutableVisualIdentityFailure> {
    require_sequence(&envelope, 9)?;
    let PlatformPulseLifecycleObservation::VisualComparison(comparison) = envelope.outcome() else {
        return Err(ExecutableVisualIdentityFailure::WrongEvent(
            "visual comparison",
        ));
    };
    let expected = [
        predecessor.snapshot.affinity().snapshot(),
        successor.snapshot.affinity().snapshot(),
    ];
    let observed = [
        comparison.predecessor_snapshot(),
        comparison.successor_snapshot(),
    ];
    if observed != expected || observed[0] == observed[1] {
        return Err(
            ExecutableVisualIdentityFailure::ComparisonSnapshotIdentity { expected, observed },
        );
    }
    if comparison.identity_rebound() || comparison.retained_pixels_differ() != Some(true) {
        return Err(ExecutableVisualIdentityFailure::ComparisonMeaning {
            identity_rebound: comparison.identity_rebound(),
            retained_pixels_differ: comparison.retained_pixels_differ(),
        });
    }
    if comparison.structural_entries_examined() == 0
        || comparison.structural_entries_examined() > 128
        || comparison.retained_pixel_bytes_examined() == 0
    {
        return Err(ExecutableVisualIdentityFailure::ComparisonCost {
            structural_entries_examined: comparison.structural_entries_examined(),
            retained_pixel_bytes_examined: comparison.retained_pixel_bytes_examined(),
        });
    }
    Ok(ExecutableVisualComparisonEvidence {
        comparison: *comparison,
    })
}

fn require_sequence(
    envelope: &PlatformPulseLifecycleObservationEnvelope,
    expected: u64,
) -> Result<(), ExecutableVisualIdentityFailure> {
    let observed = envelope.sequence().value();
    if observed == expected {
        Ok(())
    } else {
        Err(ExecutableVisualIdentityFailure::WrongSequence { expected, observed })
    }
}

fn require_same_resolution_identity(
    visible: &PlatformPulseVisualIdentityTraceObservation,
    hit: &PlatformPulseVisualIdentityTraceObservation,
) -> Result<(), ()> {
    (visible.mounted() == hit.mounted() && visible.declaration() == hit.declaration())
        .then_some(())
        .ok_or(())
}

fn require_complete_trace(
    trace: &PlatformPulseVisualIdentityTraceObservation,
) -> Result<(), ExecutableVisualIdentityFailure> {
    let mounted = trace.mounted();
    if mounted.node_receipt() == 0
        || mounted.mounted_instance() == 0
        || mounted.incarnation() == 0
        || trace.graph_node() == 0
        || trace.declaration() == 0
        || trace.authored_semantic_name().is_empty()
        || trace.source_artifact_path().is_empty()
        || trace.evidence().is_empty()
    {
        Err(ExecutableVisualIdentityFailure::IncompleteTrace)
    } else {
        Ok(())
    }
}

impl ExecutableVisualSnapshotEvidence {
    pub(crate) fn snapshot(&self) -> &PlatformPulseVisualSnapshotCaptured {
        &self.snapshot
    }

    pub(crate) fn physical_extent(&self) -> [u32; 2] {
        self.snapshot.coordinates().client_physical_dimensions()
    }

    pub(crate) fn project_logical_point(
        &self,
        logical: [u32; 2],
    ) -> Result<[u32; 2], ExecutableVisualIdentityFailure> {
        let scale = float_pair(self.snapshot.coordinates().scale_bits());
        let translation = float_pair(self.snapshot.coordinates().translation_bits());
        Ok([
            project_axis(logical[0], scale[0], translation[0])?,
            project_axis(logical[1], scale[1], translation[1])?,
        ])
    }

    pub(crate) fn expected_target_region(
        &self,
    ) -> Result<[u32; 4], ExecutableVisualIdentityFailure> {
        let logical_right = PLATFORM_PULSE_CANONICAL_LOGICAL_EXTENT[0] - TARGET_LOGICAL_INSET[0];
        let logical_bottom = PLATFORM_PULSE_CANONICAL_LOGICAL_EXTENT[1] - TARGET_LOGICAL_INSET[1];
        let [left, top] = self.project_logical_point(TARGET_LOGICAL_INSET)?;
        let [right, bottom] = self.project_logical_point([logical_right, logical_bottom])?;
        Ok([left, top, right, bottom])
    }
}

impl ExecutableVisualTraceEvidence {
    pub(crate) fn trace(&self) -> &PlatformPulseVisualPointTrace {
        &self.trace
    }
}

impl ExecutableVisualRetirementEvidence {
    pub(crate) fn retirement(self) -> PlatformPulseVisualSnapshotRetired {
        self.retirement
    }
}

impl ExecutableVisualComparisonEvidence {
    pub(crate) fn comparison(self) -> PlatformPulseVisualComparison {
        self.comparison
    }
}

impl fmt::Display for ExecutableVisualIdentityFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}
