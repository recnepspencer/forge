use crate::declaration::{
    UiDeclaredMeasurementBasisSource, UiDeclaredMeasurementEvidenceRequirement,
    UiDeclaredMeasurementOwnershipPosture, UiDeclaredMeasurementPolicyPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiDeclaredMeasurementBasisRequirementSet {
    basis_source: Option<UiDeclaredMeasurementBasisSource>,
    ownership_posture: Option<UiDeclaredMeasurementOwnershipPosture>,
    required_measurement_dependencies: Box<[UiDeclaredMeasurementEvidenceRequirement]>,
}

impl UiDeclaredMeasurementBasisRequirementSet {
    pub fn basis_source(&self) -> Option<UiDeclaredMeasurementBasisSource> {
        self.basis_source
    }

    pub fn ownership_posture(&self) -> Option<UiDeclaredMeasurementOwnershipPosture> {
        self.ownership_posture
    }

    pub fn required_measurement_dependencies(&self) -> &[UiDeclaredMeasurementEvidenceRequirement] {
        &self.required_measurement_dependencies
    }

    pub fn requires_query_projection_receipt(&self) -> bool {
        self.required_measurement_dependencies
            .contains(&UiDeclaredMeasurementEvidenceRequirement::ScrollContentExtent)
    }

    pub fn requires_host_font_metrics(&self) -> bool {
        self.required_measurement_dependencies
            .contains(&UiDeclaredMeasurementEvidenceRequirement::HostFontMetrics)
    }

    pub fn requires_viewport_extent(&self) -> bool {
        matches!(
            self.basis_source,
            Some(UiDeclaredMeasurementBasisSource::ScrollViewport)
        )
    }

    pub fn requires_portal_anchor_metrics(&self) -> bool {
        matches!(
            self.basis_source,
            Some(UiDeclaredMeasurementBasisSource::PortalAnchor)
        ) || matches!(
            self.ownership_posture,
            Some(UiDeclaredMeasurementOwnershipPosture::PortalAnchorBasisRequired)
        ) || self
            .required_measurement_dependencies
            .contains(&UiDeclaredMeasurementEvidenceRequirement::PortalAnchorMetrics)
    }

    pub fn requires_scroll_container_viewport(&self) -> bool {
        matches!(
            self.ownership_posture,
            Some(UiDeclaredMeasurementOwnershipPosture::ScrollContainerBasis)
        )
    }

    pub fn requires_host_measurement_evidence(&self) -> bool {
        self.requires_host_font_metrics()
            || self.requires_viewport_extent()
            || self.requires_portal_anchor_metrics()
            || self.requires_scroll_container_viewport()
    }
}

pub(crate) fn declared_measurement_basis_requirements(
    posture: &UiDeclaredMeasurementPolicyPosture,
) -> UiDeclaredMeasurementBasisRequirementSet {
    UiDeclaredMeasurementBasisRequirementSet {
        basis_source: posture.basis_source(),
        ownership_posture: posture.ownership_posture(),
        required_measurement_dependencies: posture
            .evidence_requirements()
            .to_vec()
            .into_boxed_slice(),
    }
}

#[cfg(test)]
mod tests {
    use super::declared_measurement_basis_requirements;
    use crate::declaration::{
        UiDeclaredMeasurementBasisSource, UiDeclaredMeasurementConstraintModifier,
        UiDeclaredMeasurementEvidenceRequirement, UiDeclaredMeasurementMode,
        UiDeclaredMeasurementOwnershipPosture, UiDeclaredMeasurementPolicyPosture,
    };

    #[test]
    fn declared_measurement_basis_requirements_preserve_full_declared_shape() {
        let posture = UiDeclaredMeasurementPolicyPosture::new(
            Some(UiDeclaredMeasurementMode::HugHeight),
            Some(UiDeclaredMeasurementConstraintModifier::Bounded),
            Some(UiDeclaredMeasurementBasisSource::ScrollViewport),
            Some(UiDeclaredMeasurementOwnershipPosture::ScrollContainerBasis),
            vec![
                UiDeclaredMeasurementEvidenceRequirement::HostFontMetrics,
                UiDeclaredMeasurementEvidenceRequirement::ScrollContentExtent,
            ],
        )
        .expect("measurement posture should admit");

        let requirements = declared_measurement_basis_requirements(&posture);

        assert!(requirements.requires_query_projection_receipt());
        assert!(requirements.requires_host_font_metrics());
        assert!(requirements.requires_viewport_extent());
        assert!(requirements.requires_scroll_container_viewport());
        assert!(requirements.requires_host_measurement_evidence());
        assert_eq!(
            requirements.required_measurement_dependencies(),
            &[
                UiDeclaredMeasurementEvidenceRequirement::HostFontMetrics,
                UiDeclaredMeasurementEvidenceRequirement::ScrollContentExtent,
            ]
        );
    }
}
