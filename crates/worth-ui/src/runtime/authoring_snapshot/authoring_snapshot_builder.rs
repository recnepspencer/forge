use crate::capability::CapabilitySnapshot;
use crate::runtime::authoring_snapshot::{
    build_authored_surface_catalogs, WorthUiAppearanceRecipeCatalog, WorthUiAuthoredSurfaceCatalog,
    WorthUiAuthoredSurfacePropsCatalog, WorthUiAuthoringCatalogEntry,
    WorthUiAuthoringSnapshotDigest, WorthUiCandidateRuntimeAuthoringSnapshot,
    WorthUiPageInstanceCatalog, WorthUiPageTemplateCatalog, WorthUiRuntimeBindingCatalog,
    WorthUiWorkspaceShellCatalog,
};
use crate::source::{
    WorthUiContentSlotCatalog, WorthUiLayoutTopologyCatalog, WorthUiParsedSourceDeclaration,
    WorthUiParsedSourcePackage, WorthUiSourceToken,
};

use super::authoring_snapshot_digest::fold_bytes;

pub(crate) struct WorthUiRuntimeAuthoringSnapshotBuilder;

#[derive(Default)]
struct ParsedCatalogEntries {
    workspace_shells: Vec<WorthUiAuthoringCatalogEntry>,
    page_templates: Vec<WorthUiAuthoringCatalogEntry>,
    page_instances: Vec<WorthUiAuthoringCatalogEntry>,
    appearance_recipes: Vec<WorthUiAuthoringCatalogEntry>,
    runtime_bindings: Vec<WorthUiAuthoringCatalogEntry>,
}

impl WorthUiRuntimeAuthoringSnapshotBuilder {
    pub(crate) fn from_source_package(
        parsed: &WorthUiParsedSourcePackage,
        snapshot: &CapabilitySnapshot,
        layout_topology: WorthUiLayoutTopologyCatalog,
        content_slots: WorthUiContentSlotCatalog,
    ) -> Result<WorthUiCandidateRuntimeAuthoringSnapshot, ()> {
        let parsed_entries = collect_parsed_catalog_entries(parsed);
        let workspace_shell =
            WorthUiWorkspaceShellCatalog::from_entries(parsed_entries.workspace_shells);
        let page_templates =
            WorthUiPageTemplateCatalog::from_entries(parsed_entries.page_templates);
        let page_instances =
            WorthUiPageInstanceCatalog::from_entries(parsed_entries.page_instances);
        let (authored_surfaces, authored_surface_props) =
            build_authored_surface_catalogs(parsed, snapshot).map_err(|_| ())?;
        let appearance_recipes =
            WorthUiAppearanceRecipeCatalog::from_entries(parsed_entries.appearance_recipes);
        let runtime_bindings =
            WorthUiRuntimeBindingCatalog::from_entries(parsed_entries.runtime_bindings);
        let digest = snapshot_digest(
            &workspace_shell,
            &page_templates,
            &page_instances,
            &layout_topology,
            &content_slots,
            &authored_surfaces,
            &authored_surface_props,
            &appearance_recipes,
            &runtime_bindings,
        );

        Ok(WorthUiCandidateRuntimeAuthoringSnapshot::new(
            workspace_shell,
            page_templates,
            page_instances,
            layout_topology,
            content_slots,
            authored_surfaces,
            authored_surface_props,
            appearance_recipes,
            runtime_bindings,
            digest,
        ))
    }
}

fn collect_parsed_catalog_entries(parsed: &WorthUiParsedSourcePackage) -> ParsedCatalogEntries {
    let mut entries = ParsedCatalogEntries::default();
    for module_id in parsed.module_ids() {
        let Some(module) = parsed.module(module_id) else {
            continue;
        };
        for declaration in module.declarations() {
            collect_declaration_entry(declaration, &mut entries);
        }
    }
    entries
}

fn collect_declaration_entry(
    declaration: &WorthUiParsedSourceDeclaration,
    entries: &mut ParsedCatalogEntries,
) {
    match declaration {
        WorthUiParsedSourceDeclaration::Workspace(declaration) => {
            entries
                .workspace_shells
                .push(WorthUiAuthoringCatalogEntry::new(
                    declaration.name_text(),
                    token_digest(declaration.body().tokens()),
                ));
        }
        WorthUiParsedSourceDeclaration::Page(declaration) => {
            let digest = page_digest(
                declaration.template_parameters(),
                declaration.body().tokens(),
            );
            entries
                .page_templates
                .push(WorthUiAuthoringCatalogEntry::new(
                    declaration.name_text(),
                    digest,
                ));
            entries
                .page_instances
                .push(WorthUiAuthoringCatalogEntry::new(
                    declaration.name_text(),
                    digest,
                ));
        }
        WorthUiParsedSourceDeclaration::Appearance(declaration) => {
            entries
                .appearance_recipes
                .push(WorthUiAuthoringCatalogEntry::new(
                    declaration.name_text(),
                    token_digest(declaration.body().tokens()),
                ));
        }
        WorthUiParsedSourceDeclaration::Runtime(declaration) => {
            entries
                .runtime_bindings
                .push(WorthUiAuthoringCatalogEntry::new(
                    declaration.name_text(),
                    token_digest(declaration.body().tokens()),
                ));
        }
        WorthUiParsedSourceDeclaration::Binding(declaration) => {
            entries
                .runtime_bindings
                .push(WorthUiAuthoringCatalogEntry::new(
                    declaration.name_text(),
                    token_digest(declaration.body().tokens()),
                ));
        }
        WorthUiParsedSourceDeclaration::Token(declaration) => {
            entries
                .appearance_recipes
                .push(WorthUiAuthoringCatalogEntry::new(
                    declaration.name_text(),
                    fold_basis(&[declaration.name_text(), declaration.value_text()]),
                ));
        }
        _ => {}
    }
}

fn snapshot_digest(
    workspace_shell: &WorthUiWorkspaceShellCatalog,
    page_templates: &WorthUiPageTemplateCatalog,
    page_instances: &WorthUiPageInstanceCatalog,
    layout_topology: &WorthUiLayoutTopologyCatalog,
    content_slots: &WorthUiContentSlotCatalog,
    authored_surfaces: &WorthUiAuthoredSurfaceCatalog,
    authored_surface_props: &WorthUiAuthoredSurfacePropsCatalog,
    appearance_recipes: &WorthUiAppearanceRecipeCatalog,
    runtime_bindings: &WorthUiRuntimeBindingCatalog,
) -> WorthUiAuthoringSnapshotDigest {
    let mut parts = Vec::new();
    parts.extend(workspace_shell.digest_basis("workspace_shell"));
    parts.extend(page_templates.digest_basis("page_template"));
    parts.extend(page_instances.digest_basis("page_instance"));
    parts.extend(layout_topology_basis(layout_topology));
    parts.extend(content_slot_basis(content_slots));
    parts.extend(authored_surfaces.digest_basis());
    parts.extend(authored_surface_props.digest_basis());
    parts.extend(appearance_recipes.digest_basis("appearance_recipe"));
    parts.extend(runtime_bindings.digest_basis("runtime_binding"));
    parts.sort();
    WorthUiAuthoringSnapshotDigest::from_basis(&parts)
}

fn layout_topology_basis(catalog: &WorthUiLayoutTopologyCatalog) -> Vec<String> {
    catalog
        .pages()
        .iter()
        .map(|page| {
            format!(
                "layout_topology|page:{}|dynamic:{}|root:{:?}",
                page.page_name(),
                page.dynamic_template(),
                page.root()
            )
        })
        .collect()
}

fn content_slot_basis(catalog: &WorthUiContentSlotCatalog) -> Vec<String> {
    catalog
        .pages()
        .iter()
        .flat_map(|page| {
            page.assignments().iter().map(|assignment| {
                format!(
                    "content_slot|page:{}|slot:{}|surface:{}",
                    page.page_name(),
                    assignment.slot_name(),
                    assignment.surface_id()
                )
            })
        })
        .collect()
}

fn page_digest(
    parameters: &[crate::source::WorthUiParsedTemplateParameter],
    tokens: &[WorthUiSourceToken],
) -> u64 {
    let mut digest = token_digest(tokens);
    for parameter in parameters {
        digest = fold_bytes(digest, parameter.name_text().as_bytes());
        digest = fold_bytes(digest, parameter.type_text().as_bytes());
    }
    digest
}

fn token_digest(tokens: &[WorthUiSourceToken]) -> u64 {
    let mut digest = 0xcbf2_9ce4_8422_2325;
    for token in tokens {
        digest = fold_bytes(digest, format!("{:?}", token.kind()).as_bytes());
    }
    digest
}

fn fold_basis(parts: &[&str]) -> u64 {
    let mut digest = 0xcbf2_9ce4_8422_2325;
    for part in parts {
        digest = fold_bytes(digest, part.as_bytes());
    }
    digest
}
