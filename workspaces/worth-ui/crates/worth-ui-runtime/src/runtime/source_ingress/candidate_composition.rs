use crate::facade::prepared_application_authority::WorthUiPreparedDeclarationSourceIdentity;
use crate::runtime::{
    WorthUiCandidateAuthoringLane, WorthUiReplacementCandidate, WorthUiReplacementCandidateBasis,
};

use super::WorthUiSourceBackedDslPackage;

#[derive(Clone, Debug, Eq, PartialEq)]
enum WorthUiCandidateDeclarationSource {
    FileAuthored(WorthUiSourceBackedDslPackage),
    RustAuthored(WorthUiSourceBackedDslPackage),
}

/// Comparison evidence binding candidate artifact identity to the distinct
/// declaration-source identity produced by the same structured input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCandidateCompositionBasis {
    candidate: WorthUiReplacementCandidateBasis,
    declaration_source: WorthUiPreparedDeclarationSourceIdentity,
}

/// Sealed pre-authority input. Artifact and declaration source can only move
/// together into application preparation.
#[derive(Debug, Eq, PartialEq)]
pub struct WorthUiCandidateComposition {
    candidate: WorthUiReplacementCandidate,
    declaration_source: WorthUiCandidateDeclarationSource,
    basis: WorthUiCandidateCompositionBasis,
}

pub(crate) struct WorthUiCandidatePreparationHandoff {
    candidate: WorthUiReplacementCandidate,
    declaration_source: WorthUiSourceBackedDslPackage,
    declaration_source_identity: WorthUiPreparedDeclarationSourceIdentity,
}

impl WorthUiCandidateComposition {
    pub(super) fn file_authored(
        candidate: WorthUiReplacementCandidate,
        declaration_source: WorthUiSourceBackedDslPackage,
    ) -> Self {
        Self::new(
            candidate,
            WorthUiCandidateDeclarationSource::FileAuthored(declaration_source),
        )
    }

    pub(super) fn rust_authored(
        candidate: WorthUiReplacementCandidate,
        declaration_source: WorthUiSourceBackedDslPackage,
    ) -> Self {
        Self::new(
            candidate,
            WorthUiCandidateDeclarationSource::RustAuthored(declaration_source),
        )
    }

    fn new(
        candidate: WorthUiReplacementCandidate,
        declaration_source: WorthUiCandidateDeclarationSource,
    ) -> Self {
        let package = declaration_source.package();
        let declaration_source_identity = WorthUiPreparedDeclarationSourceIdentity::derive(
            package.dsl_package(),
            Some(package.declaration_witness()),
        );
        let basis = WorthUiCandidateCompositionBasis {
            candidate: candidate.basis(),
            declaration_source: declaration_source_identity,
        };
        Self {
            candidate,
            declaration_source,
            basis,
        }
    }

    pub fn basis(&self) -> &WorthUiCandidateCompositionBasis {
        &self.basis
    }

    pub fn authoring_lane(&self) -> WorthUiCandidateAuthoringLane {
        self.candidate.authoring_lane()
    }

    pub(super) fn snapshot_digest(&self) -> u64 {
        self.candidate.lowering_basis().snapshot_digest()
    }

    pub(super) fn into_preparation_handoff(self) -> WorthUiCandidatePreparationHandoff {
        WorthUiCandidatePreparationHandoff {
            candidate: self.candidate,
            declaration_source: self.declaration_source.into_package(),
            declaration_source_identity: self.basis.declaration_source,
        }
    }
}

impl WorthUiCandidatePreparationHandoff {
    pub(crate) fn into_parts(
        self,
    ) -> (
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationArtifact,
        WorthUiSourceBackedDslPackage,
        WorthUiPreparedDeclarationSourceIdentity,
    ) {
        let canonical_artifact =
            crate::facade::prepared_application_authority::WorthUiPreparedApplicationArtifact::source_backed(
                &self.candidate,
            );
        (
            canonical_artifact,
            self.declaration_source,
            self.declaration_source_identity,
        )
    }

    pub(crate) fn into_replacement_parts(
        self,
    ) -> (
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationArtifact,
        WorthUiReplacementCandidate,
        WorthUiSourceBackedDslPackage,
        WorthUiPreparedDeclarationSourceIdentity,
    ) {
        let canonical_artifact =
            crate::facade::prepared_application_authority::WorthUiPreparedApplicationArtifact::source_backed(
                &self.candidate,
            );
        (
            canonical_artifact,
            self.candidate,
            self.declaration_source,
            self.declaration_source_identity,
        )
    }
}

impl WorthUiCandidateCompositionBasis {
    pub fn candidate_basis(&self) -> WorthUiReplacementCandidateBasis {
        self.candidate
    }

    pub fn declaration_source_identity(&self) -> &WorthUiPreparedDeclarationSourceIdentity {
        &self.declaration_source
    }
}

impl WorthUiCandidateDeclarationSource {
    fn package(&self) -> &WorthUiSourceBackedDslPackage {
        match self {
            Self::FileAuthored(package) | Self::RustAuthored(package) => package,
        }
    }

    fn into_package(self) -> WorthUiSourceBackedDslPackage {
        match self {
            Self::FileAuthored(package) | Self::RustAuthored(package) => package,
        }
    }
}
