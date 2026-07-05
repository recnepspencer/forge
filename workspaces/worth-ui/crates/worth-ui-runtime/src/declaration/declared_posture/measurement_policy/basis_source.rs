#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiDeclaredMeasurementBasisSource {
    ScrollViewport,
    PortalAnchor,
}

pub(crate) fn measurement_basis_source_claim(
    claim: &str,
) -> Option<UiDeclaredMeasurementBasisSource> {
    match claim {
        "measurement:basis:scroll-viewport" => {
            Some(UiDeclaredMeasurementBasisSource::ScrollViewport)
        }
        "measurement:basis:portal-anchor" => Some(UiDeclaredMeasurementBasisSource::PortalAnchor),
        _ => None,
    }
}
