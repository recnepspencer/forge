use worth_ui_inspection::{
    UiInspectionQuery, UiInspectionScope, UiInspectionScopeSupportRow, UiInspectionSupportReason,
    UiInspectionSupportReport, UiInspectionSupportWorld, UiInspectionTarget,
};

use crate::declaration::UiDeclarationArtifact;
use crate::facade::{inspection::WorthUiAuthoredInspectionBoundary, WorthUiApp};
use crate::graph::{
    UiGraphNodeIdentity, WorthUiAspectInspectionBoundary, WorthUiGraphInspectionBoundary,
};

impl WorthUiApp {
    pub fn inspection_support_report_for(
        &self,
        query: &UiInspectionQuery,
    ) -> UiInspectionSupportReport {
        match query.target() {
            UiInspectionTarget::ProductRoot => self.inspection_support_report(query.scope()),
            UiInspectionTarget::DeclaredSurface {
                module_path,
                declaration_index,
            } => self.declared_surface_inspection_support_report(
                module_path,
                *declaration_index,
                query.scope(),
            ),
            UiInspectionTarget::DeclarationIdentity { .. }
            | UiInspectionTarget::AuthoredSourceProvenance { .. } => self
                .authored_inspection_boundary()
                .support_report_for(query)
                .unwrap_or_else(|| unsupported_support_report(query.scope())),
            UiInspectionTarget::GraphNodeIdentity { .. } => self
                .graph_inspection_boundary()
                .support_report_for(query)
                .unwrap_or_else(|| unsupported_support_report(query.scope())),
            UiInspectionTarget::PublishedAspect { .. }
            | UiInspectionTarget::ConsumedAspect { .. } => self
                .aspect_inspection_boundary()
                .support_report_for(query)
                .unwrap_or_else(|| unsupported_support_report(query.scope())),
            _ => unsupported_support_report(query.scope()),
        }
    }

    pub(crate) fn authored_inspection_boundary(&self) -> WorthUiAuthoredInspectionBoundary<'_> {
        WorthUiAuthoredInspectionBoundary::new(
            self.declaration_artifacts(),
            self.authored_evidence_index(),
        )
    }

    pub(crate) fn graph_inspection_boundary(&self) -> WorthUiGraphInspectionBoundary<'_> {
        WorthUiGraphInspectionBoundary::new(
            self.declaration_artifacts(),
            self.graph_snapshot(),
            self.graph_node_evidence_index(),
        )
    }

    pub(crate) fn aspect_inspection_boundary(&self) -> WorthUiAspectInspectionBoundary<'_> {
        WorthUiAspectInspectionBoundary::new(
            self.declaration_artifacts(),
            self.graph_aspect_evidence_indexes(),
        )
    }

    pub(crate) fn measurement_inspection_boundary(
        &self,
    ) -> crate::facade::inspection::WorthUiMeasurementInspectionBoundary<'_> {
        crate::facade::inspection::WorthUiMeasurementInspectionBoundary::new(
            self.declaration_artifacts(),
            self.graph_snapshot(),
            self.authored_evidence_index(),
            self.graph_node_evidence_index(),
            self.measurement_inspection_evidence(),
        )
    }

    pub(crate) fn declaration_artifact_for_graph_node(
        &self,
        graph_node_identity: UiGraphNodeIdentity,
    ) -> Option<&UiDeclarationArtifact> {
        let artifact_index = self
            .graph_node_evidence_index()
            .lookup_graph_node_identity(graph_node_identity)?
            .neighborhood()
            .declaration_artifact_index();
        self.declaration_artifacts().get(artifact_index)
    }

    fn declared_surface_inspection_support_report(
        &self,
        module_path: &str,
        declaration_index: usize,
        scope: UiInspectionScope,
    ) -> UiInspectionSupportReport {
        self.declaration_artifacts()
            .iter()
            .find(|artifact| {
                let provenance = artifact.provenance().source_provenance();
                provenance.module_path() == module_path
                    && provenance.declaration_index() == declaration_index
            })
            .and_then(|artifact| artifact.support_snapshot().ok())
            .map(|snapshot| snapshot.inspection_rows(scope))
            .filter(|rows| !rows.is_empty())
            .map(|rows| UiInspectionSupportReport::from_scope_rows(scope, rows.as_ref()))
            .unwrap_or_else(|| unsupported_support_report(scope))
    }
}

fn unsupported_support_report(scope: UiInspectionScope) -> UiInspectionSupportReport {
    let rows = [UiInspectionScopeSupportRow::unsupported(
        "inspection",
        scope,
        UiInspectionSupportReason::TargetOutsideInspectionBoundary,
        None,
        UiInspectionSupportWorld::Authoritative,
    )];
    UiInspectionSupportReport::from_scope_rows(scope, &rows)
}
