use std::path::PathBuf;

use crate::facade::WorthUiApp;
use crate::runtime::{WorthUiRuntimeDiagnosticPolicy, WorthUiRuntimeLaunch};
use crate::source::{
    build_content_slot_catalog, build_layout_topology_catalog, WorthUiArtifact,
    WorthUiArtifactInput, WorthUiArtifactInputResolver, WorthUiBindingSemanticsLowerer,
    WorthUiBoundArtifactInput, WorthUiCanonicalArtifactAssembler, WorthUiContentSlotCatalog,
    WorthUiIdentitySeedLowerer, WorthUiIdentitySeededArtifactInput, WorthUiLayoutTopologyCatalog,
    WorthUiLegallyStructuredArtifactInput, WorthUiParsedSourcePackage,
    WorthUiParsedSourceToArtifactInputLowerer, WorthUiResolvedArtifactInput, WorthUiSourcePackage,
    WorthUiSourcePackageLoader, WorthUiSourceParser, WorthUiStructuralLegalityLowerer,
};

/// Public file-authored source module used to prepare a runtime launch.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiRuntimeSourceModule {
    relative_path: PathBuf,
    source_text: String,
}

/// Builder for preparing runtime launch artifacts through the public facade.
#[derive(Clone, Debug, Default)]
pub struct WorthUiRuntimeLaunchBuilder {
    modules: Vec<WorthUiRuntimeSourceModule>,
    diagnostic_policy: WorthUiRuntimeDiagnosticPolicy,
}

/// Prepared authoring support derived from the same source package as a runtime launch.
#[derive(Debug)]
pub struct WorthUiPreparedRuntimeAuthoring {
    layout_topology: WorthUiLayoutTopologyCatalog,
    content_slots: WorthUiContentSlotCatalog,
    runtime_launch: WorthUiRuntimeLaunch,
}

/// Structured denial for public runtime launch preparation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiRuntimeLaunchPreparationDenial {
    EmptySourcePackage,
    SourcePackageRejected { diagnostic_count: usize },
    ParseRejected { diagnostic_count: usize },
    AuthoringEntryRejected { diagnostic_count: usize },
    SnapshotResolutionRejected { diagnostic_count: usize },
    StructuralLegalityRejected { diagnostic_count: usize },
    BindingSemanticsRejected { diagnostic_count: usize },
    IdentitySeedingRejected { diagnostic_count: usize },
    ArtifactAssemblyRejected { diagnostic_count: usize },
    ContentSlotCatalogRejected { diagnostic_count: usize },
}

impl WorthUiRuntimeSourceModule {
    pub fn new(relative_path: impl Into<PathBuf>, source_text: impl Into<String>) -> Self {
        Self {
            relative_path: relative_path.into(),
            source_text: source_text.into(),
        }
    }

    pub fn relative_path(&self) -> &PathBuf {
        &self.relative_path
    }

    pub fn source_text(&self) -> &str {
        &self.source_text
    }
}

impl WorthUiRuntimeLaunchBuilder {
    pub fn from_source_module(mut self, module: WorthUiRuntimeSourceModule) -> Self {
        self.modules.push(module);
        self
    }

    pub fn from_source_modules(
        mut self,
        modules: impl IntoIterator<Item = WorthUiRuntimeSourceModule>,
    ) -> Self {
        self.modules.extend(modules);
        self
    }

    pub fn with_diagnostics(mut self, diagnostic_policy: WorthUiRuntimeDiagnosticPolicy) -> Self {
        self.diagnostic_policy = diagnostic_policy;
        self
    }

    pub fn prepare_for(
        self,
        app: &WorthUiApp,
    ) -> Result<WorthUiRuntimeLaunch, WorthUiRuntimeLaunchPreparationDenial> {
        self.prepare_authoring_for(app)
            .map(WorthUiPreparedRuntimeAuthoring::into_runtime_launch)
    }

    pub fn prepare_authoring_for(
        self,
        app: &WorthUiApp,
    ) -> Result<WorthUiPreparedRuntimeAuthoring, WorthUiRuntimeLaunchPreparationDenial> {
        reject_empty_source_package(&self.modules)?;
        let source_package = compile_source_package(self.modules)?;
        let parsed_package = parse_source_package(&source_package)?;
        let artifact_input = lower_parsed_source_package(&parsed_package)?;
        let layout_topology = prepare_layout_topology(&parsed_package);
        let resolved = resolve_artifact_input(&artifact_input, app)?;
        let structured = enforce_structural_legality(&resolved, app)?;
        let bound = enforce_binding_semantics(&structured, app)?;
        let identity_seeded = seed_artifact_identity(&bound)?;
        let content_slots = prepare_content_slots(&parsed_package, &layout_topology)?;
        let artifact = assemble_canonical_artifact(&identity_seeded)?;
        let content_slots = verify_content_slots_against_artifact(content_slots, &artifact)?;

        Ok(WorthUiPreparedRuntimeAuthoring::new(
            layout_topology,
            content_slots,
            WorthUiRuntimeLaunch::from_facade_artifact(artifact, self.diagnostic_policy),
        ))
    }
}

impl WorthUiPreparedRuntimeAuthoring {
    fn new(
        layout_topology: WorthUiLayoutTopologyCatalog,
        content_slots: WorthUiContentSlotCatalog,
        runtime_launch: WorthUiRuntimeLaunch,
    ) -> Self {
        Self {
            layout_topology,
            content_slots,
            runtime_launch,
        }
    }

    pub fn layout_topology(&self) -> &WorthUiLayoutTopologyCatalog {
        &self.layout_topology
    }

    pub fn content_slots(&self) -> &WorthUiContentSlotCatalog {
        &self.content_slots
    }

    pub fn into_runtime_launch(self) -> WorthUiRuntimeLaunch {
        self.runtime_launch
    }

    pub fn into_parts(
        self,
    ) -> (
        WorthUiRuntimeLaunch,
        WorthUiLayoutTopologyCatalog,
        WorthUiContentSlotCatalog,
    ) {
        (
            self.runtime_launch,
            self.layout_topology,
            self.content_slots,
        )
    }
}

fn reject_empty_source_package(
    modules: &[WorthUiRuntimeSourceModule],
) -> Result<(), WorthUiRuntimeLaunchPreparationDenial> {
    if modules.is_empty() {
        Err(WorthUiRuntimeLaunchPreparationDenial::EmptySourcePackage)
    } else {
        Ok(())
    }
}

fn compile_source_package(
    modules: Vec<WorthUiRuntimeSourceModule>,
) -> Result<WorthUiSourcePackage, WorthUiRuntimeLaunchPreparationDenial> {
    let mut loader = WorthUiSourcePackageLoader::from_workspace_root("worth-ui-runtime-source");
    for module in modules {
        loader = loader.register_module_with_source(module.relative_path, module.source_text);
    }
    loader.compile().map_err(|report| {
        WorthUiRuntimeLaunchPreparationDenial::SourcePackageRejected {
            diagnostic_count: report.diagnostics().len(),
        }
    })
}

fn parse_source_package(
    source_package: &WorthUiSourcePackage,
) -> Result<WorthUiParsedSourcePackage, WorthUiRuntimeLaunchPreparationDenial> {
    WorthUiSourceParser::parse_package(source_package).map_err(|report| {
        WorthUiRuntimeLaunchPreparationDenial::ParseRejected {
            diagnostic_count: report.diagnostics().len(),
        }
    })
}

fn lower_parsed_source_package(
    parsed_package: &WorthUiParsedSourcePackage,
) -> Result<WorthUiArtifactInput, WorthUiRuntimeLaunchPreparationDenial> {
    WorthUiParsedSourceToArtifactInputLowerer::lower(parsed_package).map_err(|report| {
        WorthUiRuntimeLaunchPreparationDenial::AuthoringEntryRejected {
            diagnostic_count: report.diagnostics().len(),
        }
    })
}

fn prepare_layout_topology(
    parsed_package: &WorthUiParsedSourcePackage,
) -> WorthUiLayoutTopologyCatalog {
    build_layout_topology_catalog(parsed_package).expect(
        "authoring-entry validation should certify page layout topology before facade preparation",
    )
}

fn resolve_artifact_input(
    artifact_input: &WorthUiArtifactInput,
    app: &WorthUiApp,
) -> Result<WorthUiResolvedArtifactInput, WorthUiRuntimeLaunchPreparationDenial> {
    WorthUiArtifactInputResolver::resolve(artifact_input, app.capabilities()).map_err(|report| {
        WorthUiRuntimeLaunchPreparationDenial::SnapshotResolutionRejected {
            diagnostic_count: report.diagnostics().len(),
        }
    })
}

fn enforce_structural_legality(
    resolved: &WorthUiResolvedArtifactInput,
    app: &WorthUiApp,
) -> Result<WorthUiLegallyStructuredArtifactInput, WorthUiRuntimeLaunchPreparationDenial> {
    WorthUiStructuralLegalityLowerer::lower(resolved, app.capabilities()).map_err(|report| {
        WorthUiRuntimeLaunchPreparationDenial::StructuralLegalityRejected {
            diagnostic_count: report.diagnostics().len(),
        }
    })
}

fn enforce_binding_semantics(
    structured: &WorthUiLegallyStructuredArtifactInput,
    app: &WorthUiApp,
) -> Result<WorthUiBoundArtifactInput, WorthUiRuntimeLaunchPreparationDenial> {
    WorthUiBindingSemanticsLowerer::lower(structured, app.capabilities()).map_err(|report| {
        WorthUiRuntimeLaunchPreparationDenial::BindingSemanticsRejected {
            diagnostic_count: report.diagnostics().len(),
        }
    })
}

fn seed_artifact_identity(
    bound: &WorthUiBoundArtifactInput,
) -> Result<WorthUiIdentitySeededArtifactInput, WorthUiRuntimeLaunchPreparationDenial> {
    WorthUiIdentitySeedLowerer::lower(bound)
        .map(|seeded| seeded.0)
        .map_err(
            |report| WorthUiRuntimeLaunchPreparationDenial::IdentitySeedingRejected {
                diagnostic_count: report.diagnostics().len(),
            },
        )
}

fn assemble_canonical_artifact(
    identity_seeded: &WorthUiIdentitySeededArtifactInput,
) -> Result<WorthUiArtifact, WorthUiRuntimeLaunchPreparationDenial> {
    WorthUiCanonicalArtifactAssembler::assemble(identity_seeded).map_err(|report| {
        WorthUiRuntimeLaunchPreparationDenial::ArtifactAssemblyRejected {
            diagnostic_count: report.diagnostics().len(),
        }
    })
}

fn prepare_content_slots(
    parsed_package: &WorthUiParsedSourcePackage,
    layout_topology: &WorthUiLayoutTopologyCatalog,
) -> Result<WorthUiContentSlotCatalog, WorthUiRuntimeLaunchPreparationDenial> {
    build_content_slot_catalog(parsed_package, layout_topology).map_err(|report| {
        WorthUiRuntimeLaunchPreparationDenial::ContentSlotCatalogRejected {
            diagnostic_count: report.diagnostics().len(),
        }
    })
}

fn verify_content_slots_against_artifact(
    content_slots: WorthUiContentSlotCatalog,
    artifact: &WorthUiArtifact,
) -> Result<WorthUiContentSlotCatalog, WorthUiRuntimeLaunchPreparationDenial> {
    content_slots
        .verify_canonical_mount_order(artifact)
        .map_err(
            |report| WorthUiRuntimeLaunchPreparationDenial::ContentSlotCatalogRejected {
                diagnostic_count: report.diagnostics().len(),
            },
        )
}
