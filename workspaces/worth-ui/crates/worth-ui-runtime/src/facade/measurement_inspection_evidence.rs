use crate::declaration::UiDeclarationArtifact;
use crate::evidence::UiMeasurementResult;
use worth_ui_host_contract::WorthUiHostCapabilityReport;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct UiMeasurementInspectionEvidenceBundle {
    module_path: String,
    declaration_index: usize,
    settled_query_fact: Option<(
        crate::capability::ViewBindingId,
        worth_ui_query_binding::WorthUiSettledSnapshotFact,
    )>,
    host_capability_report: Option<WorthUiHostCapabilityReport>,
    host_measurement_results: Box<[UiMeasurementResult]>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct UiMeasurementInspectionEvidenceSnapshot {
    bundles: Box<[UiMeasurementInspectionEvidenceBundle]>,
}

impl UiMeasurementInspectionEvidenceBundle {
    pub fn declared_surface(module_path: impl Into<String>, declaration_index: usize) -> Self {
        Self {
            module_path: module_path.into(),
            declaration_index,
            settled_query_fact: None,
            host_capability_report: None,
            host_measurement_results: Box::new([]),
        }
    }

    pub fn with_settled_query_fact(
        mut self,
        view_binding_id: crate::capability::ViewBindingId,
        fact: worth_ui_query_binding::WorthUiSettledSnapshotFact,
    ) -> Self {
        self.settled_query_fact = Some((view_binding_id, fact));
        self
    }

    pub fn with_host_capability_report(
        mut self,
        host_capability_report: WorthUiHostCapabilityReport,
    ) -> Self {
        self.host_capability_report = Some(host_capability_report);
        self
    }

    pub fn with_host_measurement_results(
        mut self,
        host_measurement_results: impl Into<Box<[UiMeasurementResult]>>,
    ) -> Self {
        self.host_measurement_results = host_measurement_results.into();
        self
    }

    pub(crate) fn settled_query_fact(
        &self,
    ) -> Option<&(
        crate::capability::ViewBindingId,
        worth_ui_query_binding::WorthUiSettledSnapshotFact,
    )> {
        self.settled_query_fact.as_ref()
    }

    pub(crate) fn host_capability_report(&self) -> Option<&WorthUiHostCapabilityReport> {
        self.host_capability_report.as_ref()
    }

    pub(crate) fn host_measurement_results(&self) -> &[UiMeasurementResult] {
        &self.host_measurement_results
    }

    fn matches_artifact(&self, artifact: &UiDeclarationArtifact) -> bool {
        let provenance = artifact.provenance().source_provenance();
        provenance.module_path() == self.module_path
            && provenance.declaration_index() == self.declaration_index
    }
}

impl UiMeasurementInspectionEvidenceSnapshot {
    pub(crate) fn from_bundles(
        bundles: impl Into<Box<[UiMeasurementInspectionEvidenceBundle]>>,
    ) -> Self {
        Self {
            bundles: bundles.into(),
        }
    }

    pub(crate) fn bundle_for_artifact(
        &self,
        artifact: &UiDeclarationArtifact,
    ) -> Option<&UiMeasurementInspectionEvidenceBundle> {
        self.bundles
            .iter()
            .find(|bundle| bundle.matches_artifact(artifact))
    }
}
