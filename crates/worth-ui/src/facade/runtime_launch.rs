use std::path::PathBuf;

use crate::facade::WorthUiApp;
use crate::runtime::{WorthUiRuntimeDiagnosticPolicy, WorthUiRuntimeLaunch};
use crate::source::{
    WorthUiArtifact, WorthUiArtifactInput, WorthUiArtifactInputResolver,
    WorthUiBindingSemanticsLowerer, WorthUiBoundArtifactInput, WorthUiCanonicalArtifactAssembler,
    WorthUiIdentitySeedLowerer, WorthUiIdentitySeededArtifactInput,
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

/// Structured denial for public runtime launch preparation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiRuntimeLaunchPreparationDenial {
    EmptySourcePackage,
    SourcePackageRejected { diagnostic_count: usize },
    ParseRejected { diagnostic_count: usize },
    SnapshotResolutionRejected { diagnostic_count: usize },
    StructuralLegalityRejected { diagnostic_count: usize },
    BindingSemanticsRejected { diagnostic_count: usize },
    IdentitySeedingRejected { diagnostic_count: usize },
    ArtifactAssemblyRejected { diagnostic_count: usize },
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
        reject_empty_source_package(&self.modules)?;
        let source_package = compile_source_package(self.modules)?;
        let parsed_package = parse_source_package(&source_package)?;
        let artifact_input = lower_parsed_source_package(&parsed_package);
        let resolved = resolve_artifact_input(&artifact_input, app)?;
        let structured = enforce_structural_legality(&resolved, app)?;
        let bound = enforce_binding_semantics(&structured, app)?;
        let identity_seeded = seed_artifact_identity(&bound)?;
        let artifact = assemble_canonical_artifact(&identity_seeded)?;

        Ok(WorthUiRuntimeLaunch::from_facade_artifact(
            artifact,
            self.diagnostic_policy,
        ))
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
) -> WorthUiArtifactInput {
    WorthUiParsedSourceToArtifactInputLowerer::lower(parsed_package)
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
