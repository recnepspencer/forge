#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiDeclaredMeasurementEvidenceRequirement {
    HostFontMetrics,
    ScrollContentExtent,
    PortalAnchorMetrics,
}

pub(crate) fn measurement_evidence_requirement_claim(
    claim: &str,
) -> Option<UiDeclaredMeasurementEvidenceRequirement> {
    match claim {
        "measurement:font-metrics-required" | "measurement:evidence:font-metrics-required" => {
            Some(UiDeclaredMeasurementEvidenceRequirement::HostFontMetrics)
        }
        "measurement:evidence:scroll-content-extent-required" => {
            Some(UiDeclaredMeasurementEvidenceRequirement::ScrollContentExtent)
        }
        "measurement:evidence:portal-anchor-metrics-required" => {
            Some(UiDeclaredMeasurementEvidenceRequirement::PortalAnchorMetrics)
        }
        _ => None,
    }
}
