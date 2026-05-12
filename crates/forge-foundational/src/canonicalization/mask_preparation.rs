use forge_proof::{Artifact, TransitionOutcome};

use crate::aspects::{AspectKey, AspectMask, DiagnosticMask, MutationMask, ProjectionMask};

use super::{
    CanonicalDigestMaskMode, CanonicalDigestPreparationEntry, DigestPreparationReadyAspectMask,
    DigestPreparationReadyAspectMaskArtifact,
};

pub trait DigestPreparationMaskMode: sealed::DigestPreparationMaskModeSeal {
    fn digest_mask_mode() -> CanonicalDigestMaskMode;
}

impl DigestPreparationMaskMode for ProjectionMask {
    fn digest_mask_mode() -> CanonicalDigestMaskMode {
        CanonicalDigestMaskMode::Projection
    }
}

impl DigestPreparationMaskMode for MutationMask {
    fn digest_mask_mode() -> CanonicalDigestMaskMode {
        CanonicalDigestMaskMode::Mutation
    }
}

impl DigestPreparationMaskMode for DiagnosticMask {
    fn digest_mask_mode() -> CanonicalDigestMaskMode {
        CanonicalDigestMaskMode::Diagnostic
    }
}

mod sealed {
    use crate::aspects::{DiagnosticMask, MutationMask, ProjectionMask};

    pub trait DigestPreparationMaskModeSeal {}

    impl DigestPreparationMaskModeSeal for ProjectionMask {}
    impl DigestPreparationMaskModeSeal for MutationMask {}
    impl DigestPreparationMaskModeSeal for DiagnosticMask {}
}

pub fn prepare_aspect_mask_for_digest<Mode>(
    aspect_key: AspectKey,
    mask: AspectMask<Mode>,
) -> TransitionOutcome<DigestPreparationReadyAspectMaskArtifact<Mode>>
where
    Mode: DigestPreparationMaskMode,
{
    let basis = digest_basis_for_aspect_mask(&aspect_key, &mask);

    TransitionOutcome::success(Artifact::new(DigestPreparationReadyAspectMask::new(
        aspect_key, mask, basis,
    )))
}

pub fn aspect_mask_digest_preparation_basis<Mode>(
    ready: &DigestPreparationReadyAspectMaskArtifact<Mode>,
) -> &[CanonicalDigestPreparationEntry] {
    ready.payload().basis()
}

fn digest_basis_for_aspect_mask<Mode>(
    aspect_key: &AspectKey,
    mask: &AspectMask<Mode>,
) -> Vec<CanonicalDigestPreparationEntry>
where
    Mode: DigestPreparationMaskMode,
{
    let mode = Mode::digest_mask_mode();
    if mask.is_whole_aspect() {
        return vec![CanonicalDigestPreparationEntry::MaskWholeAspect {
            key: aspect_key.clone(),
            mode,
        }];
    }

    mask.paths()
        .iter()
        .cloned()
        .map(|path| CanonicalDigestPreparationEntry::MaskFieldPath {
            key: aspect_key.clone(),
            mode,
            path,
        })
        .collect()
}
