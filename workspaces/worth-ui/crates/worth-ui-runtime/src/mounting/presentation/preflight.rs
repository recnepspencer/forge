use worth_ui_host_contract::{
    UiHostProtocolNegotiation, UiHostSurfacePresentationDenial, UiMountedEffectFamily,
    WorthUiHostCapability, WorthUiHostCapabilityReport,
};

use super::consumption_view::UiMountedHostPresentationAuthority;
use super::outcome::UiMountedSurfacePresentationRejection;
use super::terminal::frame_rejections;
use crate::host::adapter::WorthUiOperationalHostAdapter;

pub(super) fn validate_before_effects(
    frame: &super::super::UiPreparedMountedFrame,
    host: &dyn WorthUiOperationalHostAdapter,
    authority: UiMountedHostPresentationAuthority<'_>,
) -> Result<(), Vec<UiMountedSurfacePresentationRejection>> {
    let live_protocol = match host.operational_protocol_contract().negotiate() {
        UiHostProtocolNegotiation::Compatible(agreement) => agreement,
        UiHostProtocolNegotiation::Incompatible(denial) => {
            return Err(frame_rejections(
                frame,
                UiHostSurfacePresentationDenial::Protocol(denial),
            ));
        }
    };
    if live_protocol != authority.protocol() {
        return Err(frame_rejections(
            frame,
            UiHostSurfacePresentationDenial::ProtocolChanged,
        ));
    }
    let live_capabilities = host.operational_capability_report();
    if live_capabilities.profile_identity_digest()
        != authority.capability_report().profile_identity_digest()
    {
        return Err(frame_rejections(
            frame,
            UiHostSurfacePresentationDenial::CapabilityProfileChanged,
        ));
    }
    let rejections = frame
        .surfaces()
        .iter()
        .filter_map(|surface| validate_surface(surface, authority, &live_capabilities).err())
        .collect::<Vec<_>>();
    if rejections.is_empty() {
        Ok(())
    } else {
        Err(rejections)
    }
}

fn validate_surface(
    surface: &super::super::UiMountedSurfaceReceipt,
    authority: UiMountedHostPresentationAuthority<'_>,
    capabilities: &WorthUiHostCapabilityReport,
) -> Result<(), UiMountedSurfacePresentationRejection> {
    let requirement = surface.requirement();
    let denial = if requirement.capability_generation()
        != authority.capability_report().observation_generation()
    {
        Some(UiHostSurfacePresentationDenial::CapabilityGenerationChanged)
    } else if requirement.capability_profile_digest()
        != authority.capability_report().profile_identity_digest()
    {
        Some(UiHostSurfacePresentationDenial::CapabilityProfileChanged)
    } else {
        unsupported_effect(surface, capabilities)
    };
    denial.map_or(Ok(()), |denial| {
        Err(UiMountedSurfacePresentationRejection::new(
            requirement.binding(),
            denial,
        ))
    })
}

fn unsupported_effect(
    surface: &super::super::UiMountedSurfaceReceipt,
    capabilities: &WorthUiHostCapabilityReport,
) -> Option<UiHostSurfacePresentationDenial> {
    let effects = surface.presentation_effects();
    effects
        .iter()
        .copied()
        .find(|effect| !supports_effect(capabilities, *effect))
        .map(UiHostSurfacePresentationDenial::UnsupportedEffect)
}

fn supports_effect(report: &WorthUiHostCapabilityReport, effect: UiMountedEffectFamily) -> bool {
    match effect {
        UiMountedEffectFamily::RecordedProjection => {
            report.supports(WorthUiHostCapability::MountedFrameRecording)
        }
        UiMountedEffectFamily::NativePaint => report.supports(WorthUiHostCapability::NativePaint),
        UiMountedEffectFamily::Accessibility => {
            report.supports(WorthUiHostCapability::Accessibility)
        }
        UiMountedEffectFamily::Focus => report.supports(WorthUiHostCapability::NativeFocus),
        UiMountedEffectFamily::Motion | UiMountedEffectFamily::Diagnostic => false,
        UiMountedEffectFamily::IdentityOverlay => {
            report.supports(WorthUiHostCapability::IdentityOverlay)
        }
        UiMountedEffectFamily::CanvasSpatial => supports_canvas(report),
        UiMountedEffectFamily::Realtime => supports_realtime(report),
    }
}

fn supports_canvas(report: &WorthUiHostCapabilityReport) -> bool {
    [
        WorthUiHostCapability::CanvasSpatialDraw,
        WorthUiHostCapability::CanvasSpatialHitTest,
        WorthUiHostCapability::CanvasSpatialOverlay,
        WorthUiHostCapability::CanvasSpatialToolState,
        WorthUiHostCapability::CanvasSpatialRenderResource,
    ]
    .into_iter()
    .all(|capability| report.supports(capability))
}

fn supports_realtime(report: &WorthUiHostCapabilityReport) -> bool {
    [
        WorthUiHostCapability::RealtimeOverlayDraw,
        WorthUiHostCapability::RealtimeOverlaySurface,
        WorthUiHostCapability::RealtimeOverlayHook,
    ]
    .into_iter()
    .all(|capability| report.supports(capability))
}
