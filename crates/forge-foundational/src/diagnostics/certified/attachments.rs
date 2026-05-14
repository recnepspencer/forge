use forge_proof::{Artifact, Proof, TransitionOutcome};

use super::authority::FoundationalDiagnosticCertifiedAttachmentAuthority;
use super::surfaces::{
    FoundationalCertifiedDiagnosticBundle, FoundationalCertifiedDiagnosticPayload,
    FoundationalCertifiedDiagnosticSourceDigest,
};
use super::vocabulary::{
    FoundationalCertifiedDiagnosticProvenanceHook, FoundationalCertifiedDiagnosticSourceKind,
    FoundationalDiagnosticCertifiedAttachmentDenial, FoundationalDiagnosticCertifiedCoverageClass,
    FoundationalDiagnosticCertifiedCoverageDenial, FoundationalDiagnosticCoverageFamilyStatus,
    FoundationalDiagnosticCoverageMatrix,
};
use crate::boundary_artifacts::{CurrentBasisBoundaryArtifact, CurrentBasisBoundaryBundle};
use crate::canonicalization::{
    CanonicalBasisConstructionDenial, CanonicalBasisReadyArtifact, CanonicalizationRuleVersion,
};
use crate::diagnostics::{
    prepare_diagnostic_explanation_bundle_for_canonical_basis,
    prepare_diagnostic_support_report_for_canonical_basis, FoundationalDiagnosticExplanationBundle,
    FoundationalDiagnosticNamedGap, FoundationalDiagnosticPartiality, FoundationalDiagnosticRow,
    FoundationalDiagnosticRowFamily, FoundationalDiagnosticSupportReport,
};
use crate::transitions::{
    CurrentBasisCommitReceiptArtifact, CurrentBasisCommittedAuthorityArtifact,
};

mod sealed {
    pub trait Sealed {}
}

pub trait FoundationalCertifiedDiagnosticSource: sealed::Sealed + Sized {
    fn source_kind(&self) -> FoundationalCertifiedDiagnosticSourceKind;
    fn provenance_hook(&self) -> FoundationalCertifiedDiagnosticProvenanceHook;
    fn source_basis(&self) -> &CanonicalBasisReadyArtifact;
}

impl<T> sealed::Sealed for CurrentBasisCommittedAuthorityArtifact<T> {}
impl<T> FoundationalCertifiedDiagnosticSource for CurrentBasisCommittedAuthorityArtifact<T> {
    fn source_kind(&self) -> FoundationalCertifiedDiagnosticSourceKind {
        FoundationalCertifiedDiagnosticSourceKind::CurrentBasisCommittedAuthority
    }

    fn provenance_hook(&self) -> FoundationalCertifiedDiagnosticProvenanceHook {
        FoundationalCertifiedDiagnosticProvenanceHook::TransitionEvidenceOriginAttachment
    }

    fn source_basis(&self) -> &CanonicalBasisReadyArtifact {
        self.strong_basis()
    }
}

impl sealed::Sealed for CurrentBasisCommitReceiptArtifact {}
impl FoundationalCertifiedDiagnosticSource for CurrentBasisCommitReceiptArtifact {
    fn source_kind(&self) -> FoundationalCertifiedDiagnosticSourceKind {
        FoundationalCertifiedDiagnosticSourceKind::CurrentBasisCommitReceipt
    }

    fn provenance_hook(&self) -> FoundationalCertifiedDiagnosticProvenanceHook {
        FoundationalCertifiedDiagnosticProvenanceHook::TransitionEvidenceOriginAttachment
    }

    fn source_basis(&self) -> &CanonicalBasisReadyArtifact {
        self.strong_basis()
    }
}

impl<Surface> sealed::Sealed for CurrentBasisBoundaryArtifact<Surface> {}
impl<Surface> FoundationalCertifiedDiagnosticSource for CurrentBasisBoundaryArtifact<Surface> {
    fn source_kind(&self) -> FoundationalCertifiedDiagnosticSourceKind {
        FoundationalCertifiedDiagnosticSourceKind::CurrentBasisBoundaryArtifact
    }

    fn provenance_hook(&self) -> FoundationalCertifiedDiagnosticProvenanceHook {
        FoundationalCertifiedDiagnosticProvenanceHook::BoundaryArtifactEvidenceOriginAttachment
    }

    fn source_basis(&self) -> &CanonicalBasisReadyArtifact {
        self.strong_basis()
    }
}

impl<Primary, ReportRow> sealed::Sealed for CurrentBasisBoundaryBundle<Primary, ReportRow> {}
impl<Primary, ReportRow> FoundationalCertifiedDiagnosticSource
    for CurrentBasisBoundaryBundle<Primary, ReportRow>
{
    fn source_kind(&self) -> FoundationalCertifiedDiagnosticSourceKind {
        FoundationalCertifiedDiagnosticSourceKind::CurrentBasisBoundaryBundle
    }

    fn provenance_hook(&self) -> FoundationalCertifiedDiagnosticProvenanceHook {
        FoundationalCertifiedDiagnosticProvenanceHook::BoundaryArtifactEvidenceOriginAttachment
    }

    fn source_basis(&self) -> &CanonicalBasisReadyArtifact {
        self.strong_basis()
    }
}

pub trait FoundationalCertifiedDiagnosticBundleTarget: sealed::Sealed + Sized {
    fn rows(&self) -> &[FoundationalDiagnosticRow];
    fn partiality(&self) -> &FoundationalDiagnosticPartiality;
    fn prepare_bundle_basis(
        version: CanonicalizationRuleVersion,
        bundle: &Self,
    ) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial>;
}

impl sealed::Sealed for FoundationalDiagnosticSupportReport {}
impl FoundationalCertifiedDiagnosticBundleTarget for FoundationalDiagnosticSupportReport {
    fn rows(&self) -> &[FoundationalDiagnosticRow] {
        self.rows()
    }

    fn partiality(&self) -> &FoundationalDiagnosticPartiality {
        self.partiality()
    }

    fn prepare_bundle_basis(
        version: CanonicalizationRuleVersion,
        bundle: &Self,
    ) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial> {
        prepare_diagnostic_support_report_for_canonical_basis(version, bundle)
    }
}

impl sealed::Sealed for FoundationalDiagnosticExplanationBundle {}
impl FoundationalCertifiedDiagnosticBundleTarget for FoundationalDiagnosticExplanationBundle {
    fn rows(&self) -> &[FoundationalDiagnosticRow] {
        self.rows()
    }

    fn partiality(&self) -> &FoundationalDiagnosticPartiality {
        self.partiality()
    }

    fn prepare_bundle_basis(
        version: CanonicalizationRuleVersion,
        bundle: &Self,
    ) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial> {
        prepare_diagnostic_explanation_bundle_for_canonical_basis(version, bundle)
    }
}

pub type FoundationalCertifiedDiagnosticAttachmentOutcome<Source, Bundle> = TransitionOutcome<
    FoundationalCertifiedDiagnosticBundle<Source, Bundle>,
    FoundationalDiagnosticCertifiedAttachmentDenial,
>;

pub fn certify_current_basis_diagnostic_bundle<Source, Bundle>(
    version: CanonicalizationRuleVersion,
    source: Source,
    bundle: Bundle,
    coverage_matrix: FoundationalDiagnosticCoverageMatrix,
    authority: forge_proof::AuthorityWitness<FoundationalDiagnosticCertifiedAttachmentAuthority>,
) -> FoundationalCertifiedDiagnosticAttachmentOutcome<Source, Bundle>
where
    Source: FoundationalCertifiedDiagnosticSource,
    Bundle: FoundationalCertifiedDiagnosticBundleTarget,
{
    let source_digest =
        FoundationalCertifiedDiagnosticSourceDigest::from_basis(source.source_basis());
    certify_diagnostic_bundle_with_source_basis(
        version,
        source.source_kind(),
        source.provenance_hook(),
        Some(source_digest),
        source,
        bundle,
        coverage_matrix,
        authority,
    )
}

pub fn certify_diagnostic_bundle_with_source_basis<Source, Bundle>(
    version: CanonicalizationRuleVersion,
    source_kind: FoundationalCertifiedDiagnosticSourceKind,
    provenance_hook: FoundationalCertifiedDiagnosticProvenanceHook,
    source_digest: Option<FoundationalCertifiedDiagnosticSourceDigest>,
    source: Source,
    bundle: Bundle,
    coverage_matrix: FoundationalDiagnosticCoverageMatrix,
    authority: forge_proof::AuthorityWitness<FoundationalDiagnosticCertifiedAttachmentAuthority>,
) -> FoundationalCertifiedDiagnosticAttachmentOutcome<Source, Bundle>
where
    Bundle: FoundationalCertifiedDiagnosticBundleTarget,
{
    let source_digest = match source_digest {
        Some(basis) => basis,
        None => {
            return TransitionOutcome::denied(
                FoundationalDiagnosticCertifiedAttachmentDenial::MissingSourceDigest,
            );
        }
    };

    let bundle_basis = match Bundle::prepare_bundle_basis(version, &bundle) {
        TransitionOutcome::Success(ready) => ready,
        TransitionOutcome::Denied(_) => {
            return TransitionOutcome::denied(
                FoundationalDiagnosticCertifiedAttachmentDenial::MissingSourceDigest,
            );
        }
        TransitionOutcome::Deferred(_)
        | TransitionOutcome::Stale(_)
        | TransitionOutcome::RebindRequired(_)
        | TransitionOutcome::Failed(_) => {
            unreachable!("diagnostic certified bundle preparation uses only denied")
        }
    };

    let coverage_class =
        match classify_coverage(bundle.rows(), bundle.partiality(), &coverage_matrix) {
            Ok(class) => class,
            Err(denial) => return TransitionOutcome::denied(denial),
        };

    let proof = Proof::from_authority_witness(&authority);
    TransitionOutcome::success(FoundationalCertifiedDiagnosticBundle::new(
        Artifact::with_proofs_and_current_basis(
            FoundationalCertifiedDiagnosticPayload::new(
                source,
                source_kind,
                source_digest,
                bundle,
                coverage_class,
                coverage_matrix,
                provenance_hook,
            ),
            proof,
            bundle_basis,
            authority,
        ),
    ))
}

fn classify_coverage(
    rows: &[FoundationalDiagnosticRow],
    partiality: &FoundationalDiagnosticPartiality,
    coverage_matrix: &FoundationalDiagnosticCoverageMatrix,
) -> Result<
    FoundationalDiagnosticCertifiedCoverageClass,
    FoundationalDiagnosticCertifiedAttachmentDenial,
> {
    let families = [
        FoundationalDiagnosticRowFamily::Decision,
        FoundationalDiagnosticRowFamily::Failure,
        FoundationalDiagnosticRowFamily::Comparison,
        FoundationalDiagnosticRowFamily::Support,
        FoundationalDiagnosticRowFamily::ProvenanceReady,
    ];
    let has_partial_bundle = matches!(
        partiality,
        FoundationalDiagnosticPartiality::PartialWithNamedGaps(gaps) if !gaps.is_empty()
    );
    let bundle_gaps = partiality.named_gaps();
    let mut saw_partial = false;

    for family in families {
        let actual_count = rows.iter().filter(|row| row.family() == family).count() as u32;
        match coverage_matrix.for_family(family) {
            FoundationalDiagnosticCoverageFamilyStatus::AbsentFromBundle => {
                if actual_count != 0 {
                    return Err(
                        FoundationalDiagnosticCertifiedAttachmentDenial::CoveredFamilyMustExposeHostileRows,
                    );
                }
            }
            FoundationalDiagnosticCoverageFamilyStatus::HostileRowsPresent { row_count } => {
                if actual_count == 0 {
                    return Err(
                        FoundationalDiagnosticCertifiedAttachmentDenial::CoveredFamilyCannotBeAbsentFromBundle,
                    );
                }
                if *row_count == 0 {
                    return Err(
                        FoundationalDiagnosticCertifiedAttachmentDenial::CoveredFamilyMustExposeHostileRows,
                    );
                }
            }
            FoundationalDiagnosticCoverageFamilyStatus::PartialWithNamedGap(gap) => {
                if actual_count == 0 {
                    return Err(
                        FoundationalDiagnosticCertifiedAttachmentDenial::CoveredFamilyCannotBeAbsentFromBundle,
                    );
                }
                if !has_partial_bundle {
                    return Err(
                        FoundationalDiagnosticCertifiedAttachmentDenial::PartialCoverageRequiresNamedBundleGaps,
                    );
                }
                if !bundle_contains_named_gap(bundle_gaps, gap) {
                    return Err(
                        FoundationalDiagnosticCertifiedAttachmentDenial::TypedNamedGapMustBelongToBundle,
                    );
                }
                saw_partial = true;
            }
            FoundationalDiagnosticCoverageFamilyStatus::Denied(denial) => {
                return Err(match denial {
                    FoundationalDiagnosticCertifiedCoverageDenial::CoverageIncompleteDenied => {
                        FoundationalDiagnosticCertifiedAttachmentDenial::CoverageIncompleteDenied
                    }
                    FoundationalDiagnosticCertifiedCoverageDenial::HappyPathOnlyDenied => {
                        FoundationalDiagnosticCertifiedAttachmentDenial::HappyPathOnlyDenied
                    }
                });
            }
        }
    }

    if saw_partial {
        Ok(FoundationalDiagnosticCertifiedCoverageClass::PartialWithNamedGaps)
    } else {
        Ok(FoundationalDiagnosticCertifiedCoverageClass::HostileCoveragePresent)
    }
}

fn bundle_contains_named_gap(
    bundle_gaps: &[FoundationalDiagnosticNamedGap],
    candidate: &FoundationalDiagnosticNamedGap,
) -> bool {
    bundle_gaps.iter().any(|gap| gap == candidate)
}
