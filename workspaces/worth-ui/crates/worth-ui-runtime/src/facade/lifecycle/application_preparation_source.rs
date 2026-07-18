use crate::capability::CapabilitySnapshotDigest;
use crate::facade::prepared_application_authority::{
    WorthUiPreparedApplicationArtifact, WorthUiPreparedDeclarationSourceIdentity,
};
use crate::runtime::WorthUiSourceBackedDslPackage;
use worth_ui_dsl::WorthUiDslPackage;

use super::WorthUiApplicationPreparationDenial;

pub(crate) enum WorthUiApplicationPreparationSource {
    DeclarationAuthored {
        dsl_package: WorthUiDslPackage,
        declaration_source_identity: WorthUiPreparedDeclarationSourceIdentity,
    },
    SourceBacked {
        canonical_artifact: WorthUiPreparedApplicationArtifact,
        declaration_source: WorthUiSourceBackedDslPackage,
        declaration_source_identity: WorthUiPreparedDeclarationSourceIdentity,
    },
}

pub(crate) enum WorthUiApplicationDeclarationSource<'source> {
    Declared(&'source WorthUiDslPackage),
    SourceBacked(&'source WorthUiSourceBackedDslPackage),
}

impl WorthUiApplicationPreparationSource {
    pub(crate) fn declared(dsl_package: WorthUiDslPackage) -> Self {
        let declaration_source_identity =
            WorthUiPreparedDeclarationSourceIdentity::derive(&dsl_package, None);
        Self::DeclarationAuthored {
            dsl_package,
            declaration_source_identity,
        }
    }

    pub(crate) fn watched_submission(
        submission: crate::runtime::WorthUiWatchedCandidateSubmission,
        snapshot_digest: CapabilitySnapshotDigest,
    ) -> Result<Self, WorthUiApplicationPreparationDenial> {
        let candidate_snapshot_digest = submission.candidate_snapshot_digest();
        let handoff = submission.into_preparation_handoff();
        let (canonical_artifact, declaration_source, declaration_source_identity) =
            handoff.into_parts();
        if candidate_snapshot_digest != snapshot_digest.as_u64() {
            return Err(
                WorthUiApplicationPreparationDenial::CandidateSnapshotMismatch {
                    candidate_snapshot_digest,
                    prepared_snapshot_digest: snapshot_digest.as_u64(),
                },
            );
        }
        Ok(Self::SourceBacked {
            canonical_artifact,
            declaration_source,
            declaration_source_identity,
        })
    }

    pub(crate) fn declaration_source(&self) -> WorthUiApplicationDeclarationSource<'_> {
        match self {
            Self::DeclarationAuthored { dsl_package, .. } => {
                WorthUiApplicationDeclarationSource::Declared(dsl_package)
            }
            Self::SourceBacked {
                declaration_source, ..
            } => WorthUiApplicationDeclarationSource::SourceBacked(declaration_source),
        }
    }

    pub(crate) fn into_prepared_parts(
        self,
    ) -> (
        WorthUiPreparedApplicationArtifact,
        WorthUiPreparedDeclarationSourceIdentity,
    ) {
        match self {
            Self::DeclarationAuthored {
                dsl_package,
                declaration_source_identity,
            } => (
                WorthUiPreparedApplicationArtifact::DeclarationAuthored(dsl_package),
                declaration_source_identity,
            ),
            Self::SourceBacked {
                canonical_artifact,
                declaration_source_identity,
                ..
            } => (canonical_artifact, declaration_source_identity),
        }
    }
}
