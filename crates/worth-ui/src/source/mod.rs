#![cfg_attr(not(test), allow(dead_code))]

mod artifact;
mod artifact_input;
mod bound;
mod canonical;
mod dependency;
mod equivalence;
mod identity_seeded;
mod import_graph;
mod inspection;
mod layout_topology;
mod lower;
mod module;
mod package;
mod parse;
mod resolved;
mod structured;

#[allow(unused_imports)]
pub(crate) use artifact::{
    WorthUiArtifact, WorthUiArtifactBindingHandle, WorthUiArtifactBindingNode,
    WorthUiArtifactComponentHandle, WorthUiArtifactComponentNode, WorthUiArtifactEquivalentShape,
    WorthUiArtifactHandle, WorthUiArtifactImportHandle, WorthUiArtifactImportNode,
    WorthUiArtifactModule, WorthUiArtifactNode, WorthUiArtifactNodeKind,
    WorthUiArtifactSurfaceHandle, WorthUiArtifactSurfaceNode, WorthUiArtifactThemeTokenHandle,
    WorthUiArtifactThemeTokenNode,
};
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
pub use dependency::WorthUiArtifactSubtreeDigest;
#[allow(unused_imports)]
pub(crate) use dependency::{
    WorthUiArtifactDependencyEdge, WorthUiArtifactDependencyEdgeKind,
    WorthUiArtifactDependencyGraph, WorthUiArtifactDependencyTarget, WorthUiArtifactImpact,
    WorthUiArtifactImpactMetadata, WorthUiIncrementalInvalidationBasis,
    WorthUiRuntimeDependencyHook, WorthUiRuntimeDependencyHookKind, WorthUiRuntimeQuerySurface,
};
pub(crate) use equivalence::{
    WorthUiArtifactDifference, WorthUiArtifactDigest, WorthUiArtifactDigestReport,
    WorthUiArtifactEquivalence, WorthUiArtifactEquivalenceBasis, WorthUiArtifactSemanticDelta,
};
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
pub(crate) use inspection::{
    WorthUiArtifactCapabilityReference, WorthUiArtifactCapabilityReferenceInspection,
    WorthUiArtifactCapabilityReferenceRole, WorthUiArtifactInspection,
    WorthUiArtifactNodeInspection, WorthUiArtifactProvenanceMap, WorthUiArtifactSourceOrigin,
    WorthUiQueryInspectionLink, WorthUiQueryInspectionLinkRole,
};
pub use layout_topology::{
    WorthUiLayoutAxis, WorthUiLayoutDimension, WorthUiLayoutSizingSpec, WorthUiLayoutSizingValue,
    WorthUiLayoutSlotNode, WorthUiLayoutTopologyCatalog, WorthUiLayoutTopologyChild,
    WorthUiLayoutTopologyDiagnostic, WorthUiLayoutTopologyDiagnosticCode,
    WorthUiLayoutTopologyNode, WorthUiLayoutTopologyReport, WorthUiPageLayoutTopology,
};
#[allow(unused_imports)]
pub(crate) use lower::{
    build_layout_topology_catalog, validate_layout_topology_tokens,
    WorthUiArtifactAssemblyDiagnostic, WorthUiArtifactAssemblyDiagnosticCode,
    WorthUiArtifactAssemblyMetrics, WorthUiArtifactAssemblyReport,
    WorthUiArtifactDependencyDeriver, WorthUiArtifactDependencyMetrics,
    WorthUiArtifactDependencyReport, WorthUiArtifactDigestor, WorthUiArtifactEquivalenceComparator,
    WorthUiArtifactEquivalenceMetrics, WorthUiArtifactInputResolver,
    WorthUiArtifactInspectionBasis, WorthUiArtifactInspectionBasisBuilder,
    WorthUiArtifactInspectionDeriver, WorthUiArtifactInspectionDiagnostic,
    WorthUiArtifactInspectionDiagnosticCode, WorthUiArtifactInspectionMetrics,
    WorthUiArtifactInspectionReport, WorthUiAuthoringEntryDiagnostic,
    WorthUiAuthoringEntryDiagnosticCode, WorthUiAuthoringEntryReport, WorthUiBindingDiagnostic,
    WorthUiBindingDiagnosticCode, WorthUiBindingSemanticsLowerer, WorthUiBindingSemanticsMetrics,
    WorthUiBindingSemanticsReport, WorthUiCanonicalArtifactAssembler,
    WorthUiIdentityReplacementClassifier, WorthUiIdentitySeedLowerer,
    WorthUiIdentitySeedingDiagnostic, WorthUiIdentitySeedingDiagnosticCode,
    WorthUiIdentitySeedingMetrics, WorthUiIdentitySeedingReport,
    WorthUiParsedSourceToArtifactInputLowerer, WorthUiResolutionDiagnostic,
    WorthUiResolutionDiagnosticCode, WorthUiResolutionMetrics, WorthUiResolutionReport,
    WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule,
    WorthUiRustAuthoredToArtifactInputLowerer, WorthUiRustCompositionInput,
    WorthUiRustCompositionMetrics, WorthUiRustCompositionModule, WorthUiRustCompositionReport,
    WorthUiRustCompositionToArtifactInputLowerer, WorthUiStructuralLegalityDiagnostic,
    WorthUiStructuralLegalityDiagnosticCode, WorthUiStructuralLegalityLowerer,
    WorthUiStructuralLegalityMetrics, WorthUiStructuralLegalityReport,
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
    WorthUiParseDiagnostic, WorthUiParseDiagnosticCode, WorthUiParseReport,
    WorthUiParsedAuthoringDeclaration, WorthUiParsedBlockBody, WorthUiParsedBlockDeclaration,
    WorthUiParsedImportDeclaration, WorthUiParsedPageDeclaration, WorthUiParsedSourceDeclaration,
    WorthUiParsedSourceModule, WorthUiParsedSourcePackage, WorthUiParsedTemplateParameter,
    WorthUiParsedTokenDeclaration, WorthUiSourceParser, WorthUiSourceSpan, WorthUiSourceToken,
    WorthUiSourceTokenKind,
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
