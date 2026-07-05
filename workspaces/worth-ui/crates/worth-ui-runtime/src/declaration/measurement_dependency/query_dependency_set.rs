use worth_ui_query_binding::WorthUiQueryMeasurementFactFamily;

use crate::declaration::{
    UiDeclaredMeasurementEvidenceRequirement, UiDeclaredMeasurementPolicyPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiDeclaredMeasurementQueryDependencySet {
    required_measurement_dependencies: Box<[UiDeclaredMeasurementEvidenceRequirement]>,
    fact_families: Box<[WorthUiQueryMeasurementFactFamily]>,
}

impl UiDeclaredMeasurementQueryDependencySet {
    pub fn required_measurement_dependencies(&self) -> &[UiDeclaredMeasurementEvidenceRequirement] {
        &self.required_measurement_dependencies
    }

    pub fn fact_families(&self) -> &[WorthUiQueryMeasurementFactFamily] {
        &self.fact_families
    }
}

pub(crate) fn declared_query_measurement_dependencies(
    posture: &UiDeclaredMeasurementPolicyPosture,
) -> Option<UiDeclaredMeasurementQueryDependencySet> {
    let required_measurement_dependencies = posture.evidence_requirements().to_vec();
    let mut fact_families = required_measurement_dependencies
        .iter()
        .filter_map(query_measurement_fact_family_for_requirement)
        .collect::<Vec<_>>();
    fact_families.sort_unstable();
    fact_families.dedup();

    (!fact_families.is_empty()).then_some(UiDeclaredMeasurementQueryDependencySet {
        required_measurement_dependencies: required_measurement_dependencies.into_boxed_slice(),
        fact_families: fact_families.into_boxed_slice(),
    })
}

const fn query_measurement_fact_family_for_requirement(
    requirement: &UiDeclaredMeasurementEvidenceRequirement,
) -> Option<WorthUiQueryMeasurementFactFamily> {
    match requirement {
        UiDeclaredMeasurementEvidenceRequirement::ScrollContentExtent => {
            Some(WorthUiQueryMeasurementFactFamily::ScrollContentExtent)
        }
        UiDeclaredMeasurementEvidenceRequirement::HostFontMetrics
        | UiDeclaredMeasurementEvidenceRequirement::PortalAnchorMetrics => None,
    }
}

#[cfg(test)]
mod tests {
    use super::declared_query_measurement_dependencies;
    use crate::declaration::{
        UiDeclaredMeasurementConstraintModifier, UiDeclaredMeasurementEvidenceRequirement,
        UiDeclaredMeasurementMode, UiDeclaredMeasurementPolicyPosture,
    };
    use worth_ui_query_binding::WorthUiQueryMeasurementFactFamily;

    #[test]
    fn declared_query_measurement_dependencies_keep_only_query_backed_requirements() {
        let posture = UiDeclaredMeasurementPolicyPosture::new(
            Some(UiDeclaredMeasurementMode::HugHeight),
            Some(UiDeclaredMeasurementConstraintModifier::Bounded),
            None,
            None,
            vec![
                UiDeclaredMeasurementEvidenceRequirement::HostFontMetrics,
                UiDeclaredMeasurementEvidenceRequirement::ScrollContentExtent,
                UiDeclaredMeasurementEvidenceRequirement::ScrollContentExtent,
                UiDeclaredMeasurementEvidenceRequirement::PortalAnchorMetrics,
            ],
        )
        .expect("measurement posture should admit");

        let dependencies = declared_query_measurement_dependencies(&posture)
            .expect("scroll measurement posture should declare one query-backed dependency set");

        assert_eq!(
            dependencies.required_measurement_dependencies(),
            &[
                UiDeclaredMeasurementEvidenceRequirement::HostFontMetrics,
                UiDeclaredMeasurementEvidenceRequirement::ScrollContentExtent,
                UiDeclaredMeasurementEvidenceRequirement::PortalAnchorMetrics,
            ]
        );
        assert_eq!(
            dependencies.fact_families(),
            &[WorthUiQueryMeasurementFactFamily::ScrollContentExtent]
        );
    }
}
