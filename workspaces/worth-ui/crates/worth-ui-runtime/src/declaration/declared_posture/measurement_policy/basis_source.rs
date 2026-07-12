#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiDeclaredMeasurementBasisSource {
    ViewportExtent,
    ScrollViewport,
    PortalAnchor,
}

pub(crate) fn measurement_basis_source_claim(
    claim: &str,
) -> Option<UiDeclaredMeasurementBasisSource> {
    match claim {
        "measurement:basis:viewport-extent" => {
            Some(UiDeclaredMeasurementBasisSource::ViewportExtent)
        }
        "measurement:basis:scroll-viewport" => {
            Some(UiDeclaredMeasurementBasisSource::ScrollViewport)
        }
        "measurement:basis:portal-anchor" => Some(UiDeclaredMeasurementBasisSource::PortalAnchor),
        _ => None,
    }
}
