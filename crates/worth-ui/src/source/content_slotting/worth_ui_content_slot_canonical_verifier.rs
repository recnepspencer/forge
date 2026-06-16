use crate::source::{
    WorthUiArtifact, WorthUiArtifactNode, WorthUiContentSlotCatalog, WorthUiContentSlotDiagnostic,
    WorthUiContentSlotDiagnosticCode, WorthUiContentSlotReport, WorthUiMosaicRegionFacts,
};

impl WorthUiContentSlotCatalog {
    pub(crate) fn verify_canonical_mount_order(
        &self,
        artifact: &WorthUiArtifact,
    ) -> Result<Self, WorthUiContentSlotReport> {
        let diagnostics = canonical_mount_order_diagnostics(self, artifact);
        if diagnostics.is_empty() {
            Ok(self.clone())
        } else {
            Err(WorthUiContentSlotReport::new(diagnostics))
        }
    }
}

fn canonical_mount_order_diagnostics(
    catalog: &WorthUiContentSlotCatalog,
    artifact: &WorthUiArtifact,
) -> Vec<WorthUiContentSlotDiagnostic> {
    let mut diagnostics = Vec::new();
    for page in catalog.pages() {
        let Some(artifact_page) = artifact_page_by_name(artifact, page.page_name()) else {
            diagnostics.push(missing_page_structure_diagnostic(page.page_name()));
            continue;
        };

        let canonical_surface_ids = structure_surface_ids(artifact_page.structure().root_regions());
        let catalog_surface_ids = catalog_page_surface_ids(page.assignments());
        push_mount_order_diagnostics(
            page.page_name(),
            &catalog_surface_ids,
            &canonical_surface_ids,
            &mut diagnostics,
        );
    }
    diagnostics
}

fn push_mount_order_diagnostics(
    page_name: &str,
    catalog_surface_ids: &[&str],
    canonical_surface_ids: &[String],
    diagnostics: &mut Vec<WorthUiContentSlotDiagnostic>,
) {
    if catalog_surface_ids.len() != canonical_surface_ids.len() {
        diagnostics.push(mount_count_mismatch_diagnostic(
            page_name,
            catalog_surface_ids.len(),
            canonical_surface_ids.len(),
        ));
        return;
    }
    if catalog_surface_ids
        .iter()
        .zip(canonical_surface_ids.iter())
        .any(|(catalog, canonical)| *catalog != canonical.as_str())
    {
        diagnostics.push(mount_order_mismatch_diagnostic(page_name));
    }
}

fn catalog_page_surface_ids(
    assignments: &[crate::source::WorthUiContentSlotAssignment],
) -> Vec<&str> {
    assignments
        .iter()
        .map(|assignment| assignment.surface_id())
        .collect()
}

fn missing_page_structure_diagnostic(page_name: &str) -> WorthUiContentSlotDiagnostic {
    WorthUiContentSlotDiagnostic::new(
        WorthUiContentSlotDiagnosticCode::MissingPreparedPageStructure,
        page_name,
        "prepared artifact did not contain the page named by layout topology",
    )
}

fn mount_count_mismatch_diagnostic(
    page_name: &str,
    catalog_count: usize,
    canonical_count: usize,
) -> WorthUiContentSlotDiagnostic {
    WorthUiContentSlotDiagnostic::new(
        WorthUiContentSlotDiagnosticCode::SlotMountCountMismatch,
        page_name,
        format!(
            "content catalog declared {catalog_count} mounted surfaces but canonical structure mounted {canonical_count} surfaces"
        ),
    )
}

fn mount_order_mismatch_diagnostic(page_name: &str) -> WorthUiContentSlotDiagnostic {
    WorthUiContentSlotDiagnostic::new(
        WorthUiContentSlotDiagnosticCode::CanonicalMountOrderMismatch,
        page_name,
        "content catalog mount order does not match canonical page structure",
    )
}

fn artifact_page_by_name<'a>(
    artifact: &'a WorthUiArtifact,
    page_name: &str,
) -> Option<&'a crate::source::WorthUiArtifactPageNode> {
    artifact.module_ids().iter().find_map(|module_id| {
        artifact.module(module_id).and_then(|module| {
            module.nodes().iter().find_map(|node| match node {
                WorthUiArtifactNode::Page(page) if page.name_text() == page_name => Some(page),
                _ => None,
            })
        })
    })
}

fn structure_surface_ids(regions: &[WorthUiMosaicRegionFacts]) -> Vec<String> {
    let mut surface_ids = Vec::new();
    collect_structure_surface_ids(regions, &mut surface_ids);
    surface_ids
}

fn collect_structure_surface_ids(
    regions: &[WorthUiMosaicRegionFacts],
    surface_ids: &mut Vec<String>,
) {
    for region in regions {
        for mount in region.mounts() {
            surface_ids.push(mount.surface().id().as_str().to_owned());
        }
        collect_structure_surface_ids(region.child_regions(), surface_ids);
    }
}
