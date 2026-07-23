mod artifact;
mod artifact_input;
mod bound;
mod canonical;
mod dependency;
mod equivalence;
mod identity_seeded;
mod import_graph;
#[cfg(test)]
mod inspection;
mod lexical;
mod lower;
mod module;
mod package;
mod parse;
mod resolved;
mod structured;

pub(crate) use artifact::{
    WorthUiArtifact, WorthUiArtifactBindingHandle, WorthUiArtifactBindingNode,
    WorthUiArtifactComponentHandle, WorthUiArtifactComponentNode, WorthUiArtifactHandle,
    WorthUiArtifactImportHandle, WorthUiArtifactImportNode, WorthUiArtifactModule,
    WorthUiArtifactNode, WorthUiArtifactNodeKind, WorthUiArtifactSurfaceHandle,
    WorthUiArtifactSurfaceNode, WorthUiArtifactSurfaceNodeInput, WorthUiArtifactThemeTokenHandle,
    WorthUiArtifactThemeTokenNode,
};
pub use artifact_input::WorthUiArtifactInputBodyAtom;
#[cfg(test)]
pub(crate) use artifact_input::WorthUiArtifactInputEquivalentShape;
pub(crate) use artifact_input::{
    WorthUiArtifactInput, WorthUiArtifactInputBlockNode, WorthUiArtifactInputImportNode,
    WorthUiArtifactInputModule, WorthUiArtifactInputNode, WorthUiArtifactInputNodeKind,
    WorthUiArtifactInputNormalizer, WorthUiArtifactInputProvenance, WorthUiArtifactInputReference,
    WorthUiArtifactInputTokenNode,
};
#[cfg(test)]
pub(crate) use bound::WorthUiBoundArtifactInputEquivalentShape;
pub(crate) use bound::{
    WorthUiBoundArtifactInput, WorthUiBoundArtifactInputBindingNode,
    WorthUiBoundArtifactInputComponentNode, WorthUiBoundArtifactInputModule,
    WorthUiBoundArtifactInputNode, WorthUiBoundArtifactInputSurfaceNode,
    WorthUiBoundArtifactInputThemeTokenNode, WorthUiBoundCommandProjectionReference,
    WorthUiBoundCommandReference, WorthUiBoundCommandSemantics, WorthUiBoundIconReference,
    WorthUiBoundQueryViewSemantics, WorthUiBoundSurfaceSemantics, WorthUiBoundThemeTokenSemantics,
    WorthUiBoundViewBindingReference,
};
pub(crate) use canonical::WorthUiCanonicalModuleOrder;
pub use dependency::WorthUiArtifactSubtreeDigest;
#[cfg(test)]
pub(crate) use dependency::WorthUiRuntimeQuerySurface;
pub(crate) use dependency::{
    WorthUiArtifactDependencyEdge, WorthUiArtifactDependencyEdgeKind,
    WorthUiArtifactDependencyGraph, WorthUiArtifactDependencyTarget, WorthUiArtifactImpactMetadata,
    WorthUiIncrementalInvalidationBasis, WorthUiRuntimeDependencyHook,
    WorthUiRuntimeDependencyHookKind,
};
pub(crate) use equivalence::{
    WorthUiArtifactDifference, WorthUiArtifactDigest, WorthUiArtifactDigestReport,
    WorthUiArtifactEquivalence, WorthUiArtifactEquivalenceBasis, WorthUiArtifactSemanticDelta,
};
#[cfg(test)]
pub(crate) use identity_seeded::WorthUiIdentityReplacementClass;
#[cfg(test)]
pub(crate) use identity_seeded::WorthUiIdentitySeededArtifactInputEquivalentShape;
pub(crate) use identity_seeded::{
    WorthUiArtifactIdentitySeed, WorthUiArtifactIdentitySeedKind, WorthUiDurableStateEligibility,
    WorthUiDurableStateIneligibilityReason, WorthUiIdentitySeededArtifactInput,
    WorthUiIdentitySeededArtifactInputBindingNode, WorthUiIdentitySeededArtifactInputComponentNode,
    WorthUiIdentitySeededArtifactInputImportNode, WorthUiIdentitySeededArtifactInputModule,
    WorthUiIdentitySeededArtifactInputNode, WorthUiIdentitySeededArtifactInputSurfaceNode,
    WorthUiIdentitySeededArtifactInputThemeTokenNode,
};
pub(crate) use import_graph::{WorthUiSourceImport, WorthUiSourceImportGraph};
#[cfg(test)]
pub(crate) use inspection::{
    WorthUiArtifactCapabilityReference, WorthUiArtifactCapabilityReferenceInspection,
    WorthUiArtifactCapabilityReferenceRole, WorthUiArtifactInspection,
    WorthUiArtifactNodeInspection, WorthUiArtifactProvenanceMap, WorthUiArtifactSourceOrigin,
    WorthUiQueryInspectionLink, WorthUiQueryInspectionLinkRole,
};
pub(crate) use lexical::{tokenize_module_source, WorthUiSourceToken, WorthUiSourceTokenKind};
pub(crate) use lower::WorthUiRustAuthoredToArtifactInputLowerer;
pub(crate) use lower::{
    WorthUiArtifactAssemblyDiagnostic, WorthUiArtifactAssemblyMetrics,
    WorthUiArtifactAssemblyReport, WorthUiArtifactDependencyDeriver,
    WorthUiArtifactDependencyMetrics, WorthUiArtifactDependencyReport, WorthUiArtifactDigestor,
    WorthUiArtifactEquivalenceComparator, WorthUiArtifactEquivalenceMetrics,
    WorthUiArtifactInputResolver, WorthUiBindingDiagnostic, WorthUiBindingDiagnosticCode,
    WorthUiBindingSemanticsLowerer, WorthUiBindingSemanticsMetrics, WorthUiBindingSemanticsReport,
    WorthUiCanonicalArtifactAssembler, WorthUiIdentitySeedLowerer,
    WorthUiIdentitySeedingDiagnostic, WorthUiIdentitySeedingMetrics, WorthUiIdentitySeedingReport,
    WorthUiParsedSourceToArtifactInputLowerer, WorthUiResolutionDiagnostic,
    WorthUiResolutionDiagnosticCode, WorthUiResolutionMetrics, WorthUiResolutionReport,
    WorthUiStructuralLegalityDiagnostic, WorthUiStructuralLegalityDiagnosticCode,
    WorthUiStructuralLegalityLowerer, WorthUiStructuralLegalityMetrics,
    WorthUiStructuralLegalityReport,
};
#[cfg(test)]
pub(crate) use lower::{
    WorthUiArtifactAssemblyDiagnosticCode, WorthUiArtifactInspectionBasisBuilder,
    WorthUiArtifactInspectionDeriver, WorthUiArtifactInspectionDiagnosticCode,
    WorthUiIdentityReplacementClassifier, WorthUiIdentitySeedingDiagnosticCode,
    WorthUiRustCompositionToArtifactInputLowerer,
};
#[cfg(test)]
pub(crate) use lower::{
    WorthUiArtifactInspectionBasis, WorthUiArtifactInspectionDiagnostic,
    WorthUiArtifactInspectionMetrics, WorthUiArtifactInspectionReport, WorthUiRustCompositionInput,
    WorthUiRustCompositionMetrics, WorthUiRustCompositionModule, WorthUiRustCompositionReport,
};
pub use lower::{WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule};
pub(crate) use module::{WorthUiSourceModuleId, WorthUiSourceModuleRecord};
pub(crate) use package::{
    WorthUiSourcePackage, WorthUiSourcePackageDiagnostic, WorthUiSourcePackageDiagnosticCode,
    WorthUiSourcePackageDigest, WorthUiSourcePackageLoader, WorthUiSourcePackageReport,
};
pub(crate) use parse::{
    WorthUiParseDiagnostic, WorthUiParseDiagnosticCode, WorthUiParseReport, WorthUiParsedBlockBody,
    WorthUiParsedBlockDeclaration, WorthUiParsedImportDeclaration, WorthUiParsedSourceDeclaration,
    WorthUiParsedSourceModule, WorthUiParsedSourcePackage, WorthUiParsedTokenDeclaration,
    WorthUiSourceParser, WorthUiSourceSpan,
};
#[cfg(test)]
pub(crate) use resolved::WorthUiResolvedArtifactInputEquivalentShape;
pub(crate) use resolved::{
    WorthUiResolvedArtifactInput, WorthUiResolvedArtifactInputBindingNode,
    WorthUiResolvedArtifactInputComponentNode, WorthUiResolvedArtifactInputModule,
    WorthUiResolvedArtifactInputNode, WorthUiResolvedArtifactInputSurfaceNode,
    WorthUiResolvedArtifactInputThemeTokenNode,
};
#[cfg(test)]
pub(crate) use structured::WorthUiLegallyStructuredArtifactInputEquivalentShape;
pub(crate) use structured::{
    WorthUiLegallyStructuredArtifactInput, WorthUiLegallyStructuredArtifactInputBindingNode,
    WorthUiLegallyStructuredArtifactInputComponentNode,
    WorthUiLegallyStructuredArtifactInputModule, WorthUiLegallyStructuredArtifactInputNode,
    WorthUiLegallyStructuredArtifactInputSurfaceNode,
    WorthUiLegallyStructuredArtifactInputThemeTokenNode, WorthUiMosaicMountFacts,
    WorthUiMosaicRegionFacts, WorthUiMosaicStructureFacts,
};

#[cfg(test)]
mod tests;
