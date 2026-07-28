use super::lifecycle::PlatformPulseLifecycleObservation;
use super::projection::{
    PlatformPulseLifecycleObservationProjectionDenial, PlatformPulseLifecycleObservationStream,
    PlatformPulseVisualObservationState,
};
use super::visual::{
    PlatformPulseVisualCoordinateObservation, PlatformPulseVisualCoordinateOrientationObservation,
    PlatformPulseVisualCoordinateRoundingObservation,
    PlatformPulseVisualPixelColorSpaceObservation, PlatformPulseVisualPixelObservation,
    PlatformPulseVisualSnapshotAffinityObservation, PlatformPulseVisualSnapshotCaptured,
    PlatformPulseVisualSnapshotRelationObservation,
};
use super::{
    PlatformPulseLifecycleObservationEnvelope, PLATFORM_PULSE_LIFECYCLE_OBSERVATION_SCHEMA_VERSION,
};

#[test]
fn schema_v2_snapshot_observation_round_trips_without_pixel_payload_bytes() {
    let (mut stream, _) = PlatformPulseLifecycleObservationStream::start();
    let envelope = stream
        .next_envelope(PlatformPulseLifecycleObservation::VisualSnapshotCaptured(
            snapshot_observation(),
        ))
        .expect("bounded fixture projects");
    let encoded = envelope.encode_prefixed_line().expect("bounded v2 encodes");
    assert!(encoded.len() < 4_096);
    assert!(!encoded.contains("rgba"));
    assert!(!encoded.contains("screenshot"));
    let decoded =
        PlatformPulseLifecycleObservationEnvelope::decode_prefixed_line(&encoded).expect("v2");
    assert_eq!(
        decoded.protocol().schema_version(),
        PLATFORM_PULSE_LIFECYCLE_OBSERVATION_SCHEMA_VERSION
    );
    assert_eq!(decoded, envelope);
}

#[test]
fn replacement_rejects_a_partially_observed_visual_pulse() {
    let partial = PlatformPulseVisualObservationState::SnapshotCaptured {
        snapshot: 7,
        frame: 11,
    };
    assert_eq!(
        partial.after_replacement(12),
        Err(PlatformPulseLifecycleObservationProjectionDenial::VisualPulseIncomplete)
    );
}

#[test]
fn cleared_overlay_advances_to_exact_snapshot_retirement_basis() {
    let cleared = PlatformPulseVisualObservationState::OverlayCleared {
        snapshot: 7,
        snapshot_frame: 11,
        overlay: 13,
        published_frame: 12,
        cleared_frame: 14,
    };
    assert_eq!(
        cleared.after_replacement(15),
        Ok(PlatformPulseVisualObservationState::AwaitingRetirement {
            snapshot: 7,
            snapshot_frame: 11,
            successor_frame: 15,
        })
    );
}

fn snapshot_observation() -> PlatformPulseVisualSnapshotCaptured {
    PlatformPulseVisualSnapshotCaptured {
        affinity: PlatformPulseVisualSnapshotAffinityObservation {
            snapshot: 7,
            presentation_attempt: 8,
            frame: 9,
            semantic_surface: 10,
            host_surface: 11,
            binding_generation: 12,
            presentation_epoch: 13,
            relation: PlatformPulseVisualSnapshotRelationObservation::Current,
        },
        captured_client_extent: [0, 0, 160, 96],
        coordinates: PlatformPulseVisualCoordinateObservation {
            native_client_origin: [20, 30],
            client_physical_dimensions: [160, 96],
            viewport_logical_dimension_bits: [160.0_f32.to_bits(), 96.0_f32.to_bits()],
            scale_bits: [1.0_f32.to_bits(), 1.0_f32.to_bits()],
            translation_bits: [0.0_f32.to_bits(), 0.0_f32.to_bits()],
            orientation: PlatformPulseVisualCoordinateOrientationObservation::TopLeftOrigin,
            rounding: PlatformPulseVisualCoordinateRoundingObservation::PixelCenterNearest,
        },
        pixels: PlatformPulseVisualPixelObservation {
            dimensions: [160, 96],
            stride: 640,
            byte_count: 61_440,
            color_space: PlatformPulseVisualPixelColorSpaceObservation::Srgb,
        },
        visible_region_count: 2,
        hit_test_region_count: 2,
        cost_counters: [2, 1, 2, 8, 61_440, 61_440, 61_440, 1, 0, 1, 4_096],
    }
}
