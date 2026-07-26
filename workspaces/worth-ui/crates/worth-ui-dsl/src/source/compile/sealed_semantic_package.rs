use std::collections::BTreeMap;

use super::{
    WorthUiAuthoredMode, WorthUiDslProtocolIdentity, WorthUiSealedSemanticArtifact,
    WorthUiSemanticPackageIdentity,
};
use crate::source::{
    WorthUiArtifactInput, WorthUiArtifactInputNode, WorthUiArtifactInputProvenance,
    WorthUiArtifactInputReference, WorthUiAuthoredStructuralBody, WorthUiDslCompileDiagnostic,
    WorthUiDslCompileDiagnosticCode, WorthUiDslCompileReport, WorthUiDslCompileStopClass,
    WorthUiDslSourceSpan, WorthUiSourceModuleId, WorthUiStructuralBodyParser,
    WorthUiStructuralLanguageDiagnosticCode, WorthUiStructuralParseFailure,
};
use crate::UiDslLoweringReceipt;

#[derive(Debug)]
pub struct WorthUiSealedSemanticPackage {
    modules: BTreeMap<WorthUiSourceModuleId, WorthUiSemanticModule>,
    canonical_module_order: Vec<WorthUiSourceModuleId>,
    provenance_table: Box<[WorthUiArtifactInputProvenance]>,
    identity: WorthUiSemanticPackageIdentity,
    protocol: WorthUiDslProtocolIdentity,
    authored_mode: WorthUiAuthoredMode,
    _seal: WorthUiSemanticPackageSeal,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiSemanticModule {
    module_id: WorthUiSourceModuleId,
    declarations: Vec<WorthUiSemanticDeclaration>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiSemanticDeclaration {
    Import(WorthUiSemanticImport),
    Component(WorthUiSemanticBlock),
    Surface(WorthUiSemanticBlock),
    Binding(WorthUiSemanticBlock),
    Token(WorthUiSemanticToken),
    SemanticArtifact(WorthUiSealedSemanticArtifact),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiSemanticImport {
    target: WorthUiArtifactInputReference,
    provenance_ref: WorthUiSemanticProvenanceRef,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiSemanticBlock {
    name_text: String,
    authored_identity: Option<String>,
    structure: WorthUiAuthoredStructuralBody,
    provenance_ref: WorthUiSemanticProvenanceRef,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiSemanticToken {
    name_text: String,
    authored_identity: Option<String>,
    value_text: String,
    provenance_ref: WorthUiSemanticProvenanceRef,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct WorthUiSemanticPackageSeal;

/// Compact, package-local reference to diagnostic provenance.
///
/// Callers cannot construct or resolve this reference independently from the
/// package that minted it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiSemanticProvenanceRef(usize);

pub struct WorthUiSemanticDeclarationView<'package> {
    declaration: &'package WorthUiSemanticDeclaration,
    provenance_ref: WorthUiSemanticProvenanceRef,
    provenance: &'package WorthUiArtifactInputProvenance,
}

impl WorthUiSealedSemanticPackage {
    pub(super) fn seal(
        semantic_input: WorthUiArtifactInput,
        authored_mode: WorthUiAuthoredMode,
    ) -> Result<Self, WorthUiDslCompileReport> {
        let canonical_module_order = semantic_input.module_ids().to_vec();
        let mut modules = BTreeMap::new();
        let mut provenance_table = Vec::new();
        let mut diagnostics = Vec::new();

        for module_id in &canonical_module_order {
            let input_module = semantic_input
                .module(module_id)
                .expect("normalized semantic input should contain every canonical module");
            let mut declarations = Vec::new();
            for declaration in input_module.nodes() {
                let provenance_ref = WorthUiSemanticProvenanceRef(provenance_table.len());
                provenance_table.push(input_node_provenance(declaration).clone());
                match seal_declaration(declaration, provenance_ref) {
                    Ok(declaration) => declarations.push(declaration),
                    Err(diagnostic) => diagnostics.push(diagnostic),
                }
            }
            modules.insert(
                module_id.clone(),
                WorthUiSemanticModule {
                    module_id: module_id.clone(),
                    declarations,
                },
            );
        }

        if !diagnostics.is_empty() {
            return Err(WorthUiDslCompileReport::new(diagnostics));
        }
        let identity = WorthUiSemanticPackageIdentity::from_modules(
            canonical_module_order.iter().map(|module_id| {
                (
                    module_id,
                    modules
                        .get(module_id)
                        .expect("sealed package contains every canonical module"),
                )
            }),
        );
        Ok(Self {
            modules,
            canonical_module_order,
            provenance_table: provenance_table.into_boxed_slice(),
            identity,
            protocol: WorthUiDslProtocolIdentity::current(),
            authored_mode,
            _seal: WorthUiSemanticPackageSeal,
        })
    }

    pub fn module_ids(&self) -> &[WorthUiSourceModuleId] {
        &self.canonical_module_order
    }

    pub fn module(&self, module_id: &WorthUiSourceModuleId) -> Option<&WorthUiSemanticModule> {
        self.modules.get(module_id)
    }

    pub fn declaration_views(
        &self,
        module_id: &WorthUiSourceModuleId,
    ) -> Option<impl ExactSizeIterator<Item = WorthUiSemanticDeclarationView<'_>>> {
        self.modules.get(module_id).map(|module| {
            module.declarations.iter().map(|declaration| {
                let provenance_ref = declaration.provenance_ref();
                WorthUiSemanticDeclarationView {
                    declaration,
                    provenance_ref,
                    provenance: &self.provenance_table[provenance_ref.0],
                }
            })
        })
    }

    pub fn identity(&self) -> &WorthUiSemanticPackageIdentity {
        &self.identity
    }

    pub fn protocol(&self) -> WorthUiDslProtocolIdentity {
        self.protocol
    }

    pub fn authored_mode(&self) -> WorthUiAuthoredMode {
        self.authored_mode
    }

    /// Affine compiler evidence for runtime declaration admission.
    ///
    /// The receipts contain only declarations sealed into this package.
    /// Runtime-owned bootstrap declarations are deliberately excluded.
    pub fn declaration_lowering_receipts(&self) -> Vec<UiDslLoweringReceipt> {
        super::semantic_package_lowering_receipts::lower(self)
    }

    #[cfg(feature = "certification-support")]
    pub(super) fn with_protocol_for_certification(
        mut self,
        protocol: WorthUiDslProtocolIdentity,
    ) -> Self {
        self.protocol = protocol;
        self
    }
}

impl WorthUiSemanticModule {
    pub fn module_id(&self) -> &WorthUiSourceModuleId {
        &self.module_id
    }

    pub fn declarations(&self) -> &[WorthUiSemanticDeclaration] {
        &self.declarations
    }
}

impl WorthUiSemanticImport {
    pub fn target(&self) -> &WorthUiArtifactInputReference {
        &self.target
    }

    pub fn provenance_ref(&self) -> WorthUiSemanticProvenanceRef {
        self.provenance_ref
    }
}

impl WorthUiSemanticBlock {
    pub fn name_text(&self) -> &str {
        &self.name_text
    }

    pub fn authored_identity(&self) -> Option<&str> {
        self.authored_identity.as_deref()
    }

    pub fn structure(&self) -> &WorthUiAuthoredStructuralBody {
        &self.structure
    }

    pub fn provenance_ref(&self) -> WorthUiSemanticProvenanceRef {
        self.provenance_ref
    }
}

impl WorthUiSemanticToken {
    pub fn name_text(&self) -> &str {
        &self.name_text
    }

    pub fn authored_identity(&self) -> Option<&str> {
        self.authored_identity.as_deref()
    }

    pub fn value_text(&self) -> &str {
        &self.value_text
    }

    pub fn provenance_ref(&self) -> WorthUiSemanticProvenanceRef {
        self.provenance_ref
    }
}

impl WorthUiSemanticDeclaration {
    pub fn provenance_ref(&self) -> WorthUiSemanticProvenanceRef {
        match self {
            Self::Import(declaration) => declaration.provenance_ref(),
            Self::Component(declaration)
            | Self::Surface(declaration)
            | Self::Binding(declaration) => declaration.provenance_ref(),
            Self::Token(declaration) => declaration.provenance_ref(),
            Self::SemanticArtifact(declaration) => declaration.provenance_ref(),
        }
    }
}

impl<'package> WorthUiSemanticDeclarationView<'package> {
    pub fn declaration(&self) -> &'package WorthUiSemanticDeclaration {
        self.declaration
    }

    pub fn provenance_ref(&self) -> WorthUiSemanticProvenanceRef {
        self.provenance_ref
    }

    pub fn provenance(&self) -> &'package WorthUiArtifactInputProvenance {
        self.provenance
    }
}

fn seal_declaration(
    declaration: &WorthUiArtifactInputNode,
    provenance_ref: WorthUiSemanticProvenanceRef,
) -> Result<WorthUiSemanticDeclaration, WorthUiDslCompileDiagnostic> {
    match declaration {
        WorthUiArtifactInputNode::Import(import) => {
            Ok(WorthUiSemanticDeclaration::Import(WorthUiSemanticImport {
                target: import.target().clone(),
                provenance_ref,
            }))
        }
        WorthUiArtifactInputNode::Component(block) => {
            seal_block(block, provenance_ref).map(WorthUiSemanticDeclaration::Component)
        }
        WorthUiArtifactInputNode::Surface(block) => {
            seal_block(block, provenance_ref).map(WorthUiSemanticDeclaration::Surface)
        }
        WorthUiArtifactInputNode::Binding(block) => {
            seal_block(block, provenance_ref).map(WorthUiSemanticDeclaration::Binding)
        }
        WorthUiArtifactInputNode::Token(token) => {
            Ok(WorthUiSemanticDeclaration::Token(WorthUiSemanticToken {
                name_text: token.name_text().to_owned(),
                authored_identity: token.authored_identity().map(str::to_owned),
                value_text: token.value_text().to_owned(),
                provenance_ref,
            }))
        }
        WorthUiArtifactInputNode::SemanticArtifact(node) => {
            Ok(WorthUiSemanticDeclaration::SemanticArtifact(
                WorthUiSealedSemanticArtifact::new(node.declaration().clone(), provenance_ref),
            ))
        }
    }
}

fn seal_block(
    block: &crate::source::WorthUiArtifactInputBlockNode,
    provenance_ref: WorthUiSemanticProvenanceRef,
) -> Result<WorthUiSemanticBlock, WorthUiDslCompileDiagnostic> {
    let structure = WorthUiStructuralBodyParser::parse(block.body_atoms())
        .map_err(|failure| structural_diagnostic(failure, block.provenance()))?;
    Ok(WorthUiSemanticBlock {
        name_text: block.name_text().to_owned(),
        authored_identity: block.authored_identity().map(str::to_owned),
        structure,
        provenance_ref,
    })
}

fn input_node_provenance(
    declaration: &WorthUiArtifactInputNode,
) -> &WorthUiArtifactInputProvenance {
    match declaration {
        WorthUiArtifactInputNode::Import(declaration) => declaration.provenance(),
        WorthUiArtifactInputNode::Component(declaration)
        | WorthUiArtifactInputNode::Surface(declaration)
        | WorthUiArtifactInputNode::Binding(declaration) => declaration.provenance(),
        WorthUiArtifactInputNode::Token(declaration) => declaration.provenance(),
        WorthUiArtifactInputNode::SemanticArtifact(declaration) => declaration.provenance(),
    }
}

fn structural_diagnostic(
    failure: WorthUiStructuralParseFailure,
    provenance: &WorthUiArtifactInputProvenance,
) -> WorthUiDslCompileDiagnostic {
    let code = match failure.code {
        WorthUiStructuralLanguageDiagnosticCode::InvalidStructuralSyntax => {
            WorthUiDslCompileDiagnosticCode::InvalidStructuralSyntax
        }
        WorthUiStructuralLanguageDiagnosticCode::DuplicateRegionSizingDeclaration => {
            WorthUiDslCompileDiagnosticCode::DuplicateRegionSizingDeclaration
        }
        WorthUiStructuralLanguageDiagnosticCode::DuplicateRegionStateDeclaration => {
            WorthUiDslCompileDiagnosticCode::DuplicateRegionStateDeclaration
        }
        WorthUiStructuralLanguageDiagnosticCode::DuplicateMountPlacementDeclaration => {
            WorthUiDslCompileDiagnosticCode::DuplicateMountPlacementDeclaration
        }
        WorthUiStructuralLanguageDiagnosticCode::DuplicateMountStateDeclaration => {
            WorthUiDslCompileDiagnosticCode::DuplicateMountStateDeclaration
        }
        WorthUiStructuralLanguageDiagnosticCode::IllegalRootStructuralStatement => {
            WorthUiDslCompileDiagnosticCode::IllegalRootStructuralStatement
        }
    };
    let (module_id, span) = diagnostic_location(provenance);
    WorthUiDslCompileDiagnostic::new(
        code,
        WorthUiDslCompileStopClass::LanguageLegality,
        format!("{} at {}", failure.authored_text, failure.structural_locus),
        Some(module_id),
        span,
    )
}

fn diagnostic_location(
    provenance: &WorthUiArtifactInputProvenance,
) -> (String, Option<WorthUiDslSourceSpan>) {
    match provenance {
        WorthUiArtifactInputProvenance::ParsedSourceDeclaration {
            declaration_span, ..
        } => (
            declaration_span.module_id().as_str().to_owned(),
            Some(WorthUiDslSourceSpan::new(
                declaration_span.module_id().as_str(),
                declaration_span.start_byte(),
                declaration_span.end_byte(),
            )),
        ),
        WorthUiArtifactInputProvenance::RustAuthoredDeclaration {
            authored_module_path,
            ..
        } => (authored_module_path.clone(), None),
    }
}
