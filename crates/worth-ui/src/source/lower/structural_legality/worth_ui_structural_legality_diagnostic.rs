use std::cmp::Ordering;

use crate::source::{WorthUiArtifactInputProvenance, WorthUiSourceModuleId};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum WorthUiStructuralLegalityDiagnosticCode {
    InvalidStructuralSyntax,
    DuplicateRegionSizingDeclaration,
    DuplicateRegionStateDeclaration,
    DuplicateMountPlacementDeclaration,
    DuplicateMountStateDeclaration,
    InvalidStructuralSurfaceReferenceId,
    MissingStructuralSurfaceReference,
    DeferredStructuralSurfaceReference,
    UnsupportedStructuralSurfaceReference,
    PlatformInternalStructuralSurfaceReference,
    InvalidMosaicRegionReferenceId,
    MissingMosaicRegionReference,
    DeferredMosaicRegionReference,
    UnsupportedMosaicRegionReference,
    PlatformInternalMosaicRegionReference,
    InvalidMosaicPlacementPolicyReferenceId,
    MissingMosaicPlacementPolicyReference,
    DeferredMosaicPlacementPolicyReference,
    UnsupportedMosaicPlacementPolicyReference,
    PlatformInternalMosaicPlacementPolicyReference,
    InvalidMosaicSizingContractReferenceId,
    MissingMosaicSizingContractReference,
    DeferredMosaicSizingContractReference,
    UnsupportedMosaicSizingContractReference,
    PlatformInternalMosaicSizingContractReference,
    InvalidMosaicStateSlotReferenceId,
    MissingMosaicStateSlotReference,
    DeferredMosaicStateSlotReference,
    UnsupportedMosaicStateSlotReference,
    PlatformInternalMosaicStateSlotReference,
    IllegalRootStructuralStatement,
    IllegalRegionChildMix,
    IllegalLeafRegionChildren,
    IllegalSurfaceMountInRegion,
    IllegalSizingContractForRegion,
    IllegalPlacementPolicyForMount,
    IllegalRegionOwnedScrollState,
    IllegalSurfaceOwnedScrollState,
    IllegalPinnedStateSlotForRegionRole,
    IllegalRegionStateOwner,
    IllegalMountStateOwner,
    IllegalMountStateSlotKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiStructuralLegalityDiagnostic {
    code: WorthUiStructuralLegalityDiagnosticCode,
    module_id: WorthUiSourceModuleId,
    authored_text: String,
    structural_locus: String,
    provenance: WorthUiArtifactInputProvenance,
}

impl WorthUiStructuralLegalityDiagnostic {
    pub(crate) fn new(
        code: WorthUiStructuralLegalityDiagnosticCode,
        module_id: WorthUiSourceModuleId,
        authored_text: impl Into<String>,
        structural_locus: impl Into<String>,
        provenance: WorthUiArtifactInputProvenance,
    ) -> Self {
        Self {
            code,
            module_id,
            authored_text: authored_text.into(),
            structural_locus: structural_locus.into(),
            provenance,
        }
    }

    pub(crate) fn code(&self) -> WorthUiStructuralLegalityDiagnosticCode {
        self.code
    }

    pub(crate) fn authored_text(&self) -> &str {
        &self.authored_text
    }

    pub(crate) fn module_id(&self) -> &WorthUiSourceModuleId {
        &self.module_id
    }

    pub(crate) fn structural_locus(&self) -> &str {
        &self.structural_locus
    }

    pub(crate) fn provenance(&self) -> &WorthUiArtifactInputProvenance {
        &self.provenance
    }

    pub(crate) fn stable_cmp(&self, other: &Self) -> Ordering {
        self.code
            .cmp(&other.code)
            .then_with(|| self.module_id.cmp(&other.module_id))
            .then_with(|| self.structural_locus.cmp(&other.structural_locus))
            .then_with(|| self.authored_text.cmp(&other.authored_text))
            .then_with(|| stable_provenance_cmp(&self.provenance, &other.provenance))
    }
}

fn stable_provenance_cmp(
    left: &WorthUiArtifactInputProvenance,
    right: &WorthUiArtifactInputProvenance,
) -> Ordering {
    match (left, right) {
        (
            WorthUiArtifactInputProvenance::ParsedSourceDeclaration {
                declaration_span: left_declaration,
                detail_span: left_detail,
            },
            WorthUiArtifactInputProvenance::ParsedSourceDeclaration {
                declaration_span: right_declaration,
                detail_span: right_detail,
            },
        ) => stable_span_cmp(left_declaration, right_declaration)
            .then_with(|| stable_optional_span_cmp(left_detail.as_ref(), right_detail.as_ref())),
        (
            WorthUiArtifactInputProvenance::RustAuthoredDeclaration {
                authored_module_path: left_path,
                declaration_index: left_index,
            },
            WorthUiArtifactInputProvenance::RustAuthoredDeclaration {
                authored_module_path: right_path,
                declaration_index: right_index,
            },
        ) => left_path
            .cmp(right_path)
            .then_with(|| left_index.cmp(right_index)),
        (WorthUiArtifactInputProvenance::ParsedSourceDeclaration { .. }, _) => Ordering::Less,
        (_, WorthUiArtifactInputProvenance::ParsedSourceDeclaration { .. }) => Ordering::Greater,
    }
}

fn stable_optional_span_cmp(
    left: Option<&crate::source::WorthUiSourceSpan>,
    right: Option<&crate::source::WorthUiSourceSpan>,
) -> Ordering {
    match (left, right) {
        (Some(left), Some(right)) => stable_span_cmp(left, right),
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}

fn stable_span_cmp(
    left: &crate::source::WorthUiSourceSpan,
    right: &crate::source::WorthUiSourceSpan,
) -> Ordering {
    left.module_id()
        .cmp(right.module_id())
        .then_with(|| left.start_byte().cmp(&right.start_byte()))
        .then_with(|| left.end_byte().cmp(&right.end_byte()))
}
