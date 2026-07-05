#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiDeclaredMeasurementOwnershipPosture {
    ScrollContainerBasis,
    PortalAnchorBasisRequired,
}

pub(crate) fn measurement_ownership_posture_claim(
    claim: &str,
) -> Option<UiDeclaredMeasurementOwnershipPosture> {
    match claim {
        "measurement:ownership:scroll-container-basis" => {
            Some(UiDeclaredMeasurementOwnershipPosture::ScrollContainerBasis)
        }
        "measurement:ownership:portal-anchor-basis-required" => {
            Some(UiDeclaredMeasurementOwnershipPosture::PortalAnchorBasisRequired)
        }
        _ => None,
    }
}
