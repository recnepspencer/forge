use worth_ui_host_contract::{UiHostMeasurementRequest, UiNativeControlKind};

pub(crate) fn host_measurement_request_shape_digest(request: &UiHostMeasurementRequest) -> u64 {
    let identity = request.identity().as_u64().rotate_left(7);
    let evidence = stable_text_digest(request.evidence_family().as_str()).rotate_left(13);

    match request.family() {
        worth_ui_host_contract::UiMeasurementRequestFamily::TextIntrinsicSize => {
            let input = request
                .text_intrinsic_size_input()
                .expect("text intrinsic request must carry intrinsic input");
            stable_text_digest("text-intrinsic-size")
                ^ identity
                ^ evidence
                ^ stable_text_digest(input.text()).rotate_left(17)
                ^ stable_text_digest(input.font().token()).rotate_left(23)
        }
        worth_ui_host_contract::UiMeasurementRequestFamily::TextBaselineMetrics => {
            let input = request
                .text_baseline_metrics_input()
                .expect("text baseline request must carry baseline input");
            stable_text_digest("text-baseline-metrics")
                ^ identity
                ^ evidence
                ^ stable_text_digest(input.text()).rotate_left(17)
                ^ stable_text_digest(input.font().token()).rotate_left(23)
        }
        worth_ui_host_contract::UiMeasurementRequestFamily::FontMetrics => {
            let input = request
                .font_metrics_input()
                .expect("font metrics request must carry font input");
            stable_text_digest("font-metrics")
                ^ identity
                ^ evidence
                ^ stable_text_digest(input.font().token()).rotate_left(23)
        }
        worth_ui_host_contract::UiMeasurementRequestFamily::NativeControlIntrinsicSize => {
            let input = request
                .native_control_intrinsic_size_input()
                .expect("native control request must carry control input");
            stable_text_digest("native-control-intrinsic-size")
                ^ identity
                ^ evidence
                ^ stable_text_digest(match input.kind() {
                    UiNativeControlKind::Button => "button",
                    UiNativeControlKind::Checkbox => "checkbox",
                    UiNativeControlKind::TextField => "text-field",
                })
                .rotate_left(19)
                ^ input
                    .label()
                    .map(stable_text_digest)
                    .unwrap_or(0)
                    .rotate_left(29)
        }
        worth_ui_host_contract::UiMeasurementRequestFamily::ViewportExtent => {
            stable_text_digest("viewport-extent") ^ identity ^ evidence
        }
        worth_ui_host_contract::UiMeasurementRequestFamily::DpiScaleFactor => {
            stable_text_digest("dpi-scale-factor") ^ identity ^ evidence
        }
        worth_ui_host_contract::UiMeasurementRequestFamily::PortalAnchorRect => {
            let input = request
                .portal_anchor_rect_input()
                .expect("portal anchor request must carry anchor input");
            stable_text_digest("portal-anchor-rect")
                ^ identity
                ^ evidence
                ^ input.anchor_identity().rotate_left(19)
        }
        worth_ui_host_contract::UiMeasurementRequestFamily::ScrollContainerViewport => {
            let input = request
                .scroll_container_viewport_input()
                .expect("scroll container request must carry container input");
            stable_text_digest("scroll-container-viewport")
                ^ identity
                ^ evidence
                ^ input.container_identity().rotate_left(19)
        }
    }
}

fn stable_text_digest(text: &str) -> u64 {
    text.as_bytes()
        .iter()
        .fold(0xCBF2_9CE4_8422_2325, |digest, byte| {
            digest.wrapping_mul(0x0000_0100_0000_01B3) ^ u64::from(*byte)
        })
}
