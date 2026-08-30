use super::{
    UiHostObservationMountedBasis, UiHostObservationPayload, UiHostObservationPresentationBasis,
    UiHostScrollDeltaPhase, UiHostScrollDeltaPrecision, UiHostScrollDeltaSource,
    UiHostScrollDeltaTargetAffinity, UiHostSurfacePosition,
};
use crate::{
    UiHostPresentationEpoch, UiHostSurfaceIdentity, UiMountedFrameIdentity,
    UiMountedNodeReceiptIdentity, UiSurfaceBindingGeneration,
};

#[test]
fn scroll_identity_covers_source_phase_precision_delta_and_target_form() {
    let presentation = presentation();
    let position = UiHostSurfacePosition::viewport_logical(12_000, 34_000);
    let receipt = UiMountedNodeReceiptIdentity::mint_unbound().unwrap();
    let mounted = UiHostObservationMountedBasis::new(receipt.mounted_instance(), receipt);
    let base = scroll(
        UiHostScrollDeltaSource::PointerWheel,
        UiHostScrollDeltaPhase::Updated,
        UiHostScrollDeltaPrecision::Pixel,
        UiHostScrollDeltaTargetAffinity::exact_coordinate(presentation, position),
        4_000,
        -8_000,
    );
    assert_eq!(base.encoded_len(), 70);
    assert_eq!(base.coalescing_identity(), None);
    let successor_frame_presentation = UiHostObservationPresentationBasis::new(
        presentation.host_surface(),
        UiMountedFrameIdentity::mint_unbound().unwrap(),
        presentation.binding(),
        presentation.epoch(),
    );
    let successor_surface_presentation = UiHostObservationPresentationBasis::new(
        UiHostSurfaceIdentity::mint_unbound().unwrap(),
        presentation.frame(),
        presentation.binding(),
        presentation.epoch(),
    );
    let successor_binding_presentation = UiHostObservationPresentationBasis::new(
        presentation.host_surface(),
        presentation.frame(),
        UiSurfaceBindingGeneration::mint_unbound().unwrap(),
        presentation.epoch(),
    );
    let successor_epoch_presentation = UiHostObservationPresentationBasis::new(
        presentation.host_surface(),
        presentation.frame(),
        presentation.binding(),
        UiHostPresentationEpoch::issued_by_host(2),
    );
    let variants = [
        scroll(
            UiHostScrollDeltaSource::PointerWheel,
            UiHostScrollDeltaPhase::Updated,
            UiHostScrollDeltaPrecision::Pixel,
            UiHostScrollDeltaTargetAffinity::exact_coordinate(
                successor_frame_presentation,
                position,
            ),
            4_000,
            -8_000,
        ),
        scroll(
            UiHostScrollDeltaSource::PointerWheel,
            UiHostScrollDeltaPhase::Updated,
            UiHostScrollDeltaPrecision::Pixel,
            UiHostScrollDeltaTargetAffinity::exact_coordinate(
                successor_surface_presentation,
                position,
            ),
            4_000,
            -8_000,
        ),
        scroll(
            UiHostScrollDeltaSource::PointerWheel,
            UiHostScrollDeltaPhase::Updated,
            UiHostScrollDeltaPrecision::Pixel,
            UiHostScrollDeltaTargetAffinity::exact_coordinate(
                successor_binding_presentation,
                position,
            ),
            4_000,
            -8_000,
        ),
        scroll(
            UiHostScrollDeltaSource::PointerWheel,
            UiHostScrollDeltaPhase::Updated,
            UiHostScrollDeltaPrecision::Pixel,
            UiHostScrollDeltaTargetAffinity::exact_coordinate(
                successor_epoch_presentation,
                position,
            ),
            4_000,
            -8_000,
        ),
        scroll(
            UiHostScrollDeltaSource::Touch,
            UiHostScrollDeltaPhase::Updated,
            UiHostScrollDeltaPrecision::Pixel,
            UiHostScrollDeltaTargetAffinity::exact_coordinate(presentation, position),
            4_000,
            -8_000,
        ),
        scroll(
            UiHostScrollDeltaSource::PointerWheel,
            UiHostScrollDeltaPhase::Ended,
            UiHostScrollDeltaPrecision::Pixel,
            UiHostScrollDeltaTargetAffinity::exact_coordinate(presentation, position),
            4_000,
            -8_000,
        ),
        scroll(
            UiHostScrollDeltaSource::PointerWheel,
            UiHostScrollDeltaPhase::Updated,
            UiHostScrollDeltaPrecision::Line,
            UiHostScrollDeltaTargetAffinity::exact_coordinate(presentation, position),
            4_000,
            -8_000,
        ),
        scroll(
            UiHostScrollDeltaSource::PointerWheel,
            UiHostScrollDeltaPhase::Updated,
            UiHostScrollDeltaPrecision::Pixel,
            UiHostScrollDeltaTargetAffinity::exact_coordinate(
                presentation,
                UiHostSurfacePosition::viewport_logical(12_001, 34_000),
            ),
            4_000,
            -8_000,
        ),
        scroll(
            UiHostScrollDeltaSource::PointerWheel,
            UiHostScrollDeltaPhase::Updated,
            UiHostScrollDeltaPrecision::Pixel,
            UiHostScrollDeltaTargetAffinity::exact_mounted_target(presentation, mounted),
            4_000,
            -8_000,
        ),
        scroll(
            UiHostScrollDeltaSource::PointerWheel,
            UiHostScrollDeltaPhase::Updated,
            UiHostScrollDeltaPrecision::Pixel,
            UiHostScrollDeltaTargetAffinity::presented_surface_fallback(presentation),
            4_000,
            -8_000,
        ),
        scroll(
            UiHostScrollDeltaSource::PointerWheel,
            UiHostScrollDeltaPhase::Updated,
            UiHostScrollDeltaPrecision::Pixel,
            UiHostScrollDeltaTargetAffinity::exact_coordinate(presentation, position),
            4_001,
            -8_000,
        ),
        scroll(
            UiHostScrollDeltaSource::PointerWheel,
            UiHostScrollDeltaPhase::Updated,
            UiHostScrollDeltaPrecision::Pixel,
            UiHostScrollDeltaTargetAffinity::exact_coordinate(presentation, position),
            4_000,
            -8_001,
        ),
    ];
    for variant in variants {
        assert_ne!(base.integrity_digest(), variant.integrity_digest());
    }
}

#[test]
fn scroll_target_accessors_preserve_honest_adapter_knowledge() {
    let presentation = presentation();
    let position = UiHostSurfacePosition::viewport_logical(7, 9);
    let exact = UiHostScrollDeltaTargetAffinity::exact_coordinate(presentation, position);
    let fallback = UiHostScrollDeltaTargetAffinity::presented_surface_fallback(presentation);

    assert_eq!(exact.position(), Some(position));
    assert_eq!(exact.mounted_target(), None);
    assert!(!exact.is_surface_fallback());
    assert_eq!(fallback.position(), None);
    assert_eq!(fallback.mounted_target(), None);
    assert!(fallback.is_surface_fallback());
    assert_eq!(
        scroll(
            UiHostScrollDeltaSource::PointerWheel,
            UiHostScrollDeltaPhase::Updated,
            UiHostScrollDeltaPrecision::Line,
            fallback,
            0,
            1,
        )
        .encoded_len(),
        52,
    );
}

fn presentation() -> UiHostObservationPresentationBasis {
    UiHostObservationPresentationBasis::new(
        UiHostSurfaceIdentity::mint_unbound().unwrap(),
        UiMountedFrameIdentity::mint_unbound().unwrap(),
        UiSurfaceBindingGeneration::mint_unbound().unwrap(),
        UiHostPresentationEpoch::issued_by_host(1),
    )
}

fn scroll(
    source: UiHostScrollDeltaSource,
    phase: UiHostScrollDeltaPhase,
    precision: UiHostScrollDeltaPrecision,
    target: UiHostScrollDeltaTargetAffinity,
    x_subpixels: i64,
    y_subpixels: i64,
) -> UiHostObservationPayload {
    UiHostObservationPayload::ScrollDelta {
        source,
        phase,
        precision,
        target,
        x_subpixels,
        y_subpixels,
    }
}
