use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::source::{
    WorthUiCanonicalModuleOrder, WorthUiSourceImport, WorthUiSourceImportGraph,
    WorthUiSourceModuleId, WorthUiSourceModuleRecord, WorthUiSourcePackage,
    WorthUiSourcePackageDiagnostic, WorthUiSourcePackageDiagnosticCode, WorthUiSourcePackageDigest,
    WorthUiSourcePackageReport,
};

#[derive(Clone, Debug, Default)]
pub(crate) struct WorthUiSourcePackageLoader {
    workspace_root: PathBuf,
    registrations: Vec<PendingModuleRegistration>,
}

#[derive(Clone, Debug)]
pub(crate) struct WorthUiSourcePackagePlan {
    workspace_root: PathBuf,
    registrations: Vec<PendingModuleRegistration>,
}

#[derive(Clone, Debug)]
pub(crate) struct WorthUiValidatedSourcePackagePlan {
    workspace_root: PathBuf,
    modules: BTreeMap<WorthUiSourceModuleId, WorthUiSourceModuleRecord>,
    canonical_module_order: WorthUiCanonicalModuleOrder,
    import_graph: WorthUiSourceImportGraph,
}

#[derive(Clone, Debug)]
struct PendingModuleRegistration {
    relative_path: PathBuf,
    source_text: String,
    import_paths: Vec<PathBuf>,
}

#[derive(Clone, Debug)]
struct WorthUiRegisteredSourceModule {
    module_id: WorthUiSourceModuleId,
    relative_path: PathBuf,
    source_text: String,
    import_paths: Vec<PathBuf>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum VisitState {
    Active,
    Complete,
}

impl WorthUiSourcePackageLoader {
    pub(crate) fn from_workspace_root(workspace_root: impl Into<PathBuf>) -> Self {
        Self {
            workspace_root: workspace_root.into(),
            registrations: Vec::new(),
        }
    }

    pub(crate) fn register_module(mut self, relative_path: impl Into<PathBuf>) -> Self {
        self.registrations
            .push(PendingModuleRegistration::without_source(
                relative_path.into(),
            ));
        self
    }

    pub(crate) fn register_module_with_imports(
        mut self,
        relative_path: impl Into<PathBuf>,
        import_paths: impl IntoIterator<Item = impl Into<PathBuf>>,
    ) -> Self {
        self.registrations
            .push(PendingModuleRegistration::without_source_with_imports(
                relative_path.into(),
                import_paths.into_iter().map(Into::into).collect(),
            ));
        self
    }

    pub(crate) fn register_module_with_source(
        mut self,
        relative_path: impl Into<PathBuf>,
        source_text: impl Into<String>,
    ) -> Self {
        self.registrations
            .push(PendingModuleRegistration::with_source(
                relative_path.into(),
                source_text.into(),
            ));
        self
    }

    pub(crate) fn register_module_with_imports_and_source(
        mut self,
        relative_path: impl Into<PathBuf>,
        import_paths: impl IntoIterator<Item = impl Into<PathBuf>>,
        source_text: impl Into<String>,
    ) -> Self {
        self.registrations
            .push(PendingModuleRegistration::with_source_and_imports(
                relative_path.into(),
                source_text.into(),
                import_paths.into_iter().map(Into::into).collect(),
            ));
        self
    }

    pub(crate) fn build(self) -> WorthUiSourcePackagePlan {
        WorthUiSourcePackagePlan {
            workspace_root: self.workspace_root,
            registrations: self.registrations,
        }
    }

    pub(crate) fn compile(self) -> Result<WorthUiSourcePackage, WorthUiSourcePackageReport> {
        Ok(self.build().validate()?.compile())
    }
}

impl WorthUiSourcePackagePlan {
    pub(crate) fn validate(
        self,
    ) -> Result<WorthUiValidatedSourcePackagePlan, WorthUiSourcePackageReport> {
        let WorthUiSourcePackagePlan {
            workspace_root,
            registrations,
        } = self;
        let mut diagnostics = Vec::new();
        let registered_modules =
            validate_registered_module_identities(&workspace_root, registrations, &mut diagnostics);

        if !diagnostics.is_empty() {
            return Err(WorthUiSourcePackageReport::new(diagnostics));
        }

        let (modules, adjacency) =
            resolve_registered_modules(&workspace_root, &registered_modules, &mut diagnostics);

        if !diagnostics.is_empty() {
            return Err(WorthUiSourcePackageReport::new(diagnostics));
        }

        let graph = WorthUiSourceImportGraph::new(adjacency);
        if let Some(cycle) = first_cycle(&graph) {
            diagnostics.push(WorthUiSourcePackageDiagnostic::new(
                WorthUiSourcePackageDiagnosticCode::CyclicModuleImport,
                "source package import graph contains a cycle",
                None,
                cycle.first().map(ToString::to_string),
                cycle.get(1).map(ToString::to_string),
            ));
            return Err(WorthUiSourcePackageReport::new(diagnostics));
        }

        let canonical_module_order =
            WorthUiCanonicalModuleOrder::from_module_ids(modules.keys().cloned().collect());
        Ok(WorthUiValidatedSourcePackagePlan {
            workspace_root,
            modules,
            canonical_module_order,
            import_graph: graph,
        })
    }
}

impl WorthUiValidatedSourcePackagePlan {
    pub(crate) fn compile(self) -> WorthUiSourcePackage {
        let module_records: Vec<_> = self.modules.values().cloned().collect();
        let digest = WorthUiSourcePackageDigest::from_package_parts(
            &self.canonical_module_order,
            &self.import_graph,
            &module_records,
        );
        WorthUiSourcePackage::new(
            self.workspace_root,
            self.modules,
            self.canonical_module_order,
            self.import_graph,
            digest,
        )
    }
}

fn first_cycle(graph: &WorthUiSourceImportGraph) -> Option<Vec<WorthUiSourceModuleId>> {
    let mut states = BTreeMap::new();
    let mut stack = Vec::new();
    for module_id in graph.module_ids() {
        if let Some(cycle) = visit(module_id, graph, &mut states, &mut stack) {
            return Some(cycle);
        }
    }
    None
}

fn validate_registered_module_identities(
    workspace_root: &Path,
    registrations: Vec<PendingModuleRegistration>,
    diagnostics: &mut Vec<WorthUiSourcePackageDiagnostic>,
) -> Vec<WorthUiRegisteredSourceModule> {
    let mut seen_module_ids = BTreeSet::new();
    let mut registered_modules = Vec::new();

    for registration in registrations {
        let module_id = match WorthUiSourceModuleId::from_workspace_path(
            workspace_root,
            &registration.relative_path,
        ) {
            Ok(module_id) => module_id,
            Err(message) => {
                diagnostics.push(WorthUiSourcePackageDiagnostic::new(
                    WorthUiSourcePackageDiagnosticCode::InvalidModulePath,
                    message,
                    Some(registration.relative_path),
                    None,
                    None,
                ));
                continue;
            }
        };

        if !seen_module_ids.insert(module_id.clone()) {
            diagnostics.push(WorthUiSourcePackageDiagnostic::new(
                WorthUiSourcePackageDiagnosticCode::DuplicateModuleIdentity,
                "duplicate canonical source-module identity",
                Some(registration.relative_path),
                Some(module_id.to_string()),
                None,
            ));
            continue;
        }

        registered_modules.push(WorthUiRegisteredSourceModule {
            module_id,
            relative_path: registration.relative_path,
            source_text: registration.source_text,
            import_paths: registration.import_paths,
        });
    }

    registered_modules
}

fn resolve_registered_modules(
    workspace_root: &Path,
    registered_modules: &[WorthUiRegisteredSourceModule],
    diagnostics: &mut Vec<WorthUiSourcePackageDiagnostic>,
) -> (
    BTreeMap<WorthUiSourceModuleId, WorthUiSourceModuleRecord>,
    BTreeMap<WorthUiSourceModuleId, Vec<WorthUiSourceImport>>,
) {
    let module_ids: BTreeSet<_> = registered_modules
        .iter()
        .map(|registered_module| registered_module.module_id.clone())
        .collect();
    let mut modules = BTreeMap::new();
    let mut adjacency = BTreeMap::new();

    for registered_module in registered_modules {
        let imports =
            resolve_registered_imports(workspace_root, registered_module, &module_ids, diagnostics);
        let canonical_relative_path = canonical_module_relative_path(&registered_module.module_id);

        adjacency.insert(registered_module.module_id.clone(), imports.clone());
        modules.insert(
            registered_module.module_id.clone(),
            WorthUiSourceModuleRecord::new(
                registered_module.module_id.clone(),
                canonical_relative_path,
                registered_module.source_text.clone(),
                imports,
            ),
        );
    }

    (modules, adjacency)
}

fn resolve_registered_imports(
    workspace_root: &Path,
    registered_module: &WorthUiRegisteredSourceModule,
    module_ids: &BTreeSet<WorthUiSourceModuleId>,
    diagnostics: &mut Vec<WorthUiSourcePackageDiagnostic>,
) -> Vec<WorthUiSourceImport> {
    let mut imports = Vec::new();

    for import_path in &registered_module.import_paths {
        let import_id =
            match WorthUiSourceModuleId::from_workspace_path(workspace_root, import_path) {
                Ok(import_id) => import_id,
                Err(message) => {
                    diagnostics.push(WorthUiSourcePackageDiagnostic::new(
                        WorthUiSourcePackageDiagnosticCode::InvalidModulePath,
                        message,
                        Some(import_path.clone()),
                        Some(registered_module.module_id.to_string()),
                        None,
                    ));
                    continue;
                }
            };
        if !module_ids.contains(&import_id) {
            diagnostics.push(WorthUiSourcePackageDiagnostic::new(
                WorthUiSourcePackageDiagnosticCode::UnknownImportTarget,
                "import target is not registered in the source package",
                Some(registered_module.relative_path.clone()),
                Some(registered_module.module_id.to_string()),
                Some(import_id.to_string()),
            ));
            continue;
        }
        imports.push(WorthUiSourceImport::new(import_id));
    }

    imports.sort();
    imports.dedup();
    imports
}

fn visit(
    module_id: &WorthUiSourceModuleId,
    graph: &WorthUiSourceImportGraph,
    states: &mut BTreeMap<WorthUiSourceModuleId, VisitState>,
    stack: &mut Vec<WorthUiSourceModuleId>,
) -> Option<Vec<WorthUiSourceModuleId>> {
    match states.get(module_id) {
        Some(VisitState::Complete) => return None,
        Some(VisitState::Active) => {
            let start = stack
                .iter()
                .position(|current| current == module_id)
                .unwrap_or(0);
            return Some(stack[start..].to_vec());
        }
        None => {}
    }

    states.insert(module_id.clone(), VisitState::Active);
    stack.push(module_id.clone());
    if let Some(imports) = graph.imports_for(module_id) {
        for import in imports {
            if let Some(cycle) = visit(import.target_module_id(), graph, states, stack) {
                return Some(cycle);
            }
        }
    }
    stack.pop();
    states.insert(module_id.clone(), VisitState::Complete);
    None
}

fn canonical_module_relative_path(module_id: &WorthUiSourceModuleId) -> PathBuf {
    Path::new(module_id.as_str()).to_path_buf()
}

impl PendingModuleRegistration {
    fn without_source(relative_path: PathBuf) -> Self {
        Self {
            relative_path,
            source_text: String::new(),
            import_paths: Vec::new(),
        }
    }

    fn without_source_with_imports(relative_path: PathBuf, import_paths: Vec<PathBuf>) -> Self {
        Self {
            relative_path,
            source_text: String::new(),
            import_paths,
        }
    }

    fn with_source(relative_path: PathBuf, source_text: String) -> Self {
        Self {
            relative_path,
            source_text,
            import_paths: Vec::new(),
        }
    }

    fn with_source_and_imports(
        relative_path: PathBuf,
        source_text: String,
        import_paths: Vec<PathBuf>,
    ) -> Self {
        Self {
            relative_path,
            source_text,
            import_paths,
        }
    }
}
