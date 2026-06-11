#![cfg_attr(not(test), allow(dead_code))]

mod artifact_input;
mod bound;
mod canonical;
mod identity_seeded;
mod import_graph;
mod lower;
mod module;
mod package;
mod parse;
mod resolved;
mod structured;

pub(crate) use artifact_input::{
    WorthUiArtifactInput, WorthUiArtifactInputBlockNode, WorthUiArtifactInputBodyAtom,
    WorthUiArtifactInputEquivalentShape, WorthUiArtifactInputImportNode,
    WorthUiArtifactInputModule, WorthUiArtifactInputNode, WorthUiArtifactInputNodeKind,
    WorthUiArtifactInputNormalizer, WorthUiArtifactInputProvenance, WorthUiArtifactInputReference,
    WorthUiArtifactInputTokenNode,
};
pub(crate) use bound::{
    WorthUiBoundArtifactInput, WorthUiBoundArtifactInputBindingNode,
    WorthUiBoundArtifactInputComponentNode, WorthUiBoundArtifactInputEquivalentShape,
    WorthUiBoundArtifactInputModule, WorthUiBoundArtifactInputNode,
    WorthUiBoundArtifactInputSurfaceNode, WorthUiBoundArtifactInputThemeTokenNode,
    WorthUiBoundCommandProjectionReference, WorthUiBoundCommandReference,
    WorthUiBoundCommandSemantics, WorthUiBoundIconReference, WorthUiBoundQueryViewSemantics,
    WorthUiBoundSurfaceSemantics, WorthUiBoundThemeTokenSemantics,
    WorthUiBoundViewBindingReference,
};
pub(crate) use canonical::WorthUiCanonicalModuleOrder;
#[allow(unused_imports)]
pub(crate) use identity_seeded::{
    WorthUiArtifactIdentitySeed, WorthUiArtifactIdentitySeedKind, WorthUiDurableStateEligibility,
    WorthUiDurableStateIneligibilityReason, WorthUiIdentityReplacementClass,
    WorthUiIdentitySeededArtifactInput, WorthUiIdentitySeededArtifactInputBindingNode,
    WorthUiIdentitySeededArtifactInputComponentNode,
    WorthUiIdentitySeededArtifactInputEquivalentShape,
    WorthUiIdentitySeededArtifactInputImportNode, WorthUiIdentitySeededArtifactInputModule,
    WorthUiIdentitySeededArtifactInputNode, WorthUiIdentitySeededArtifactInputSurfaceNode,
    WorthUiIdentitySeededArtifactInputThemeTokenNode,
};
pub(crate) use import_graph::{WorthUiSourceImport, WorthUiSourceImportGraph};
#[allow(unused_imports)]
pub(crate) use lower::{
    WorthUiArtifactInputResolver, WorthUiBindingDiagnostic, WorthUiBindingDiagnosticCode,
    WorthUiBindingSemanticsLowerer, WorthUiBindingSemanticsMetrics, WorthUiBindingSemanticsReport,
    WorthUiIdentityReplacementClassifier, WorthUiIdentitySeedLowerer,
    WorthUiIdentitySeedingMetrics, WorthUiParsedSourceToArtifactInputLowerer,
    WorthUiResolutionDiagnostic, WorthUiResolutionDiagnosticCode, WorthUiResolutionMetrics,
    WorthUiResolutionReport, WorthUiRustAuthoredArtifactInput,
    WorthUiRustAuthoredArtifactInputModule, WorthUiRustAuthoredToArtifactInputLowerer,
    WorthUiStructuralLegalityDiagnostic, WorthUiStructuralLegalityDiagnosticCode,
    WorthUiStructuralLegalityLowerer, WorthUiStructuralLegalityMetrics,
    WorthUiStructuralLegalityReport,
};
pub(crate) use module::{WorthUiSourceModuleId, WorthUiSourceModuleRecord};
#[allow(unused_imports)]
pub(crate) use package::{
    WorthUiSourcePackage, WorthUiSourcePackageDiagnostic, WorthUiSourcePackageDiagnosticCode,
    WorthUiSourcePackageDigest, WorthUiSourcePackageLoader, WorthUiSourcePackagePlan,
    WorthUiSourcePackageReport, WorthUiValidatedSourcePackagePlan,
};
#[allow(unused_imports)]
pub(crate) use parse::{
    WorthUiParseDiagnostic, WorthUiParseDiagnosticCode, WorthUiParseReport, WorthUiParsedBlockBody,
    WorthUiParsedBlockDeclaration, WorthUiParsedImportDeclaration, WorthUiParsedSourceDeclaration,
    WorthUiParsedSourceModule, WorthUiParsedSourcePackage, WorthUiParsedTokenDeclaration,
    WorthUiSourceParser, WorthUiSourceSpan, WorthUiSourceToken, WorthUiSourceTokenKind,
};
pub(crate) use resolved::{
    WorthUiResolvedArtifactInput, WorthUiResolvedArtifactInputBindingNode,
    WorthUiResolvedArtifactInputComponentNode, WorthUiResolvedArtifactInputEquivalentShape,
    WorthUiResolvedArtifactInputModule, WorthUiResolvedArtifactInputNode,
    WorthUiResolvedArtifactInputSurfaceNode, WorthUiResolvedArtifactInputThemeTokenNode,
};
pub(crate) use structured::{
    WorthUiLegallyStructuredArtifactInput, WorthUiLegallyStructuredArtifactInputBindingNode,
    WorthUiLegallyStructuredArtifactInputComponentNode,
    WorthUiLegallyStructuredArtifactInputEquivalentShape,
    WorthUiLegallyStructuredArtifactInputModule, WorthUiLegallyStructuredArtifactInputNode,
    WorthUiLegallyStructuredArtifactInputSurfaceNode,
    WorthUiLegallyStructuredArtifactInputThemeTokenNode, WorthUiMosaicMountFacts,
    WorthUiMosaicRegionFacts, WorthUiMosaicStructureFacts,
};

#[cfg(test)]
mod tests;
