use worth_proof::{Artifact, TransitionOutcome};

use crate::aspects::{AspectKey, AspectMask, DiagnosticMask, MutationMask, ProjectionMask};

use super::{
    prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisReadyArtifact, CanonicalBasisValue,
    CanonicalDigestMaskMode, CanonicalDigestPreparationEntry, CanonicalizationRuleVersion,
    DigestPreparationReadyAspectMask, DigestPreparationReadyAspectMaskArtifact,
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

pub fn prepare_aspect_mask_for_canonical_basis<Mode>(
    version: CanonicalizationRuleVersion,
    aspect_key: AspectKey,
    mask: AspectMask<Mode>,
) -> TransitionOutcome<CanonicalBasisReadyArtifact, super::CanonicalBasisConstructionDenial>
where
    Mode: DigestPreparationMaskMode,
{
    prepare_canonical_basis_sequence(
        version,
        CanonicalBasisDomain::AspectMask,
        canonical_basis_for_aspect_mask(&aspect_key, &mask),
    )
}

fn canonical_basis_for_aspect_mask<Mode>(
    aspect_key: &AspectKey,
    mask: &AspectMask<Mode>,
) -> Vec<CanonicalBasisEntry>
where
    Mode: DigestPreparationMaskMode,
{
    digest_basis_for_aspect_mask(aspect_key, mask)
        .into_iter()
        .map(canonical_entry_for_mask_digest_entry)
        .collect()
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

fn canonical_entry_for_mask_digest_entry(
    entry: CanonicalDigestPreparationEntry,
) -> CanonicalBasisEntry {
    match entry {
        CanonicalDigestPreparationEntry::MaskWholeAspect { key, mode } => mask_entry(
            key.as_str(),
            format!("{}.whole", digest_mask_mode_name(mode)),
            CanonicalBasisValue::ExactText("whole".into()),
        ),
        CanonicalDigestPreparationEntry::MaskFieldPath { key, mode, path } => {
            let path_text = path
                .fields()
                .iter()
                .map(|field| field.as_str())
                .collect::<Vec<_>>()
                .join(".");
            mask_entry(
                key.as_str(),
                format!("{}.field.{path_text}", digest_mask_mode_name(mode)),
                CanonicalBasisValue::ExactText(path_text.into()),
            )
        }
        _ => unreachable!("mask canonical basis only consumes mask digest entries"),
    }
}

fn mask_entry(
    aspect_key: &str,
    locus: impl Into<String>,
    value: CanonicalBasisValue,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::AspectMask,
        CanonicalBasisLocus::Named(format!("{aspect_key}.{}", locus.into()).into()),
        CanonicalBasisEntryKind::Mask,
        value,
    )
}

fn digest_mask_mode_name(mode: CanonicalDigestMaskMode) -> &'static str {
    match mode {
        CanonicalDigestMaskMode::Projection => "projection",
        CanonicalDigestMaskMode::Mutation => "mutation",
        CanonicalDigestMaskMode::Diagnostic => "diagnostic",
    }
}
