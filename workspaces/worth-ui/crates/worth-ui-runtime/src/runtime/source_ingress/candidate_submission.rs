use crate::capability::{CapabilitySnapshot, CapabilitySnapshotDigest};
use crate::runtime::candidate::{
    file_authored_replacement_candidate, rust_authored_replacement_candidate,
};
use crate::runtime::source_ingress::counters::WorthUiSourceIngressCounters;
use crate::runtime::source_ingress::debounce::WorthUiDebouncedWatcherBatch;
use crate::runtime::source_ingress::denial::{
    WorthUiSourceIngressDenial, WorthUiSourceIngressDenialReason,
};
use crate::runtime::source_ingress::ordering_receipt::WorthUiCandidateOrderingReceipt;
use crate::runtime::source_ingress::provider::WorthUiSourceProvider;
use crate::runtime::source_ingress::revision::WorthUiSourcePackageRevision;
use crate::runtime::source_ingress::source_backed_declaration_projection::project_source_backed_declaration_witness;
use crate::runtime::source_ingress::source_backed_package_lowering::source_backed_package;
use crate::runtime::source_ingress::{
    WorthUiCandidateComposition, WorthUiCandidateCompositionBasis,
    WorthUiCandidatePreparationHandoff, WorthUiSourceBackedDslPackage,
};
use crate::runtime::WorthUiReplacementCause;
use crate::runtime::{WorthUiReplacementCandidate, WorthUiReplacementCandidateDenial};
use crate::source::{
    WorthUiArtifact, WorthUiArtifactInputResolver, WorthUiBindingSemanticsLowerer,
    WorthUiCanonicalArtifactAssembler, WorthUiIdentitySeedLowerer,
    WorthUiParsedSourceToArtifactInputLowerer, WorthUiRustAuthoredToArtifactInputLowerer,
    WorthUiSourcePackageLoader, WorthUiSourceParser, WorthUiStructuralLegalityLowerer,
};

#[derive(Debug, Eq, PartialEq)]
pub struct WorthUiWatchedCandidateSubmission {
    composition: WorthUiCandidateComposition,
    revision: WorthUiSourcePackageRevision,
    ordering_receipt: WorthUiCandidateOrderingReceipt,
    counters: WorthUiSourceIngressCounters,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiWatchedCandidateSubmissionDenial {
    SourceIngress(WorthUiSourceIngressDenial),
    Candidate(WorthUiReplacementCandidateDenial),
}

impl WorthUiDebouncedWatcherBatch {
    pub fn lower_to_candidate_submission(
        self,
        snapshot: &CapabilitySnapshot,
    ) -> Result<WorthUiWatchedCandidateSubmission, WorthUiWatchedCandidateSubmissionDenial> {
        let (provider, revision, ordering_receipt, mut counters) = self.into_parts();
        let lowered =
            lower_provider_to_candidate(&provider, snapshot, &revision, &ordering_receipt)?;
        counters.emit_candidate_submission();
        Ok(WorthUiWatchedCandidateSubmission {
            composition: lowered,
            revision,
            ordering_receipt,
            counters,
        })
    }
}

impl WorthUiWatchedCandidateSubmission {
    pub fn ordering_receipt(&self) -> &WorthUiCandidateOrderingReceipt {
        &self.ordering_receipt
    }

    pub fn source_revision(&self) -> &WorthUiSourcePackageRevision {
        &self.revision
    }

    pub fn counters(&self) -> WorthUiSourceIngressCounters {
        self.counters
    }

    pub fn composition_basis(&self) -> &WorthUiCandidateCompositionBasis {
        self.composition.basis()
    }

    pub fn authoring_lane(&self) -> crate::runtime::WorthUiCandidateAuthoringLane {
        self.composition.authoring_lane()
    }

    pub(crate) fn candidate_snapshot_digest(&self) -> u64 {
        self.composition.snapshot_digest()
    }

    pub(crate) fn into_preparation_handoff(self) -> WorthUiCandidatePreparationHandoff {
        self.composition.into_preparation_handoff()
    }

    pub(crate) fn into_replacement_handoff(self) -> WorthUiCandidatePreparationHandoff {
        self.composition.into_preparation_handoff()
    }
}

fn lower_provider_to_candidate(
    provider: &WorthUiSourceProvider,
    snapshot: &CapabilitySnapshot,
    revision: &WorthUiSourcePackageRevision,
    ordering_receipt: &WorthUiCandidateOrderingReceipt,
) -> Result<WorthUiCandidateComposition, WorthUiWatchedCandidateSubmissionDenial> {
    if !ordering_receipt.matches_revision(revision) {
        return Err(source_denial(
            WorthUiSourceIngressDenialReason::OrderingReceiptDrift,
        ));
    }
    reject_ambiguous_candidate_material(provider)?;
    if let Some(input) = provider.rust_authored_inputs().first() {
        let material = rust_authored_material(input, snapshot)?;
        return rust_authored_candidate(material.artifact, snapshot.digest(), revision).map(
            |candidate| {
                WorthUiCandidateComposition::rust_authored(
                    candidate,
                    material.source_backed_dsl_package,
                )
            },
        );
    }
    if !provider.source_modules().is_empty() {
        let material = file_authored_material(provider, snapshot)?;
        return file_authored_candidate(material.artifact, snapshot.digest(), provider, revision)
            .map(|candidate| {
                WorthUiCandidateComposition::file_authored(
                    candidate,
                    material.source_backed_dsl_package,
                )
            });
    }
    Err(source_denial(
        WorthUiSourceIngressDenialReason::NoCandidateMaterial,
    ))
}

fn reject_ambiguous_candidate_material(
    provider: &WorthUiSourceProvider,
) -> Result<(), WorthUiWatchedCandidateSubmissionDenial> {
    if !provider.source_modules().is_empty() && !provider.rust_authored_inputs().is_empty() {
        return Err(source_denial(
            WorthUiSourceIngressDenialReason::MixedCandidateMaterial,
        ));
    }
    if provider.rust_authored_inputs().len() > 1 {
        return Err(source_denial(
            WorthUiSourceIngressDenialReason::MultipleRustAuthoredInputs,
        ));
    }
    Ok(())
}

struct WorthUiLoweredCandidateMaterial {
    artifact: WorthUiArtifact,
    source_backed_dsl_package: WorthUiSourceBackedDslPackage,
}

fn file_authored_material(
    provider: &WorthUiSourceProvider,
    snapshot: &CapabilitySnapshot,
) -> Result<WorthUiLoweredCandidateMaterial, WorthUiWatchedCandidateSubmissionDenial> {
    let mut loader = WorthUiSourcePackageLoader::from_workspace_root(provider.workspace_root());
    for module in provider.source_modules() {
        loader = loader.register_module_with_source(module.relative_path(), module.source_text());
    }
    let source_package = loader
        .compile()
        .map_err(|_| source_denial(WorthUiSourceIngressDenialReason::SourcePackageRejected))?;
    let parsed = WorthUiSourceParser::parse_package(&source_package)
        .map_err(|_| source_denial(WorthUiSourceIngressDenialReason::SourceParseRejected))?;
    let artifact_input = WorthUiParsedSourceToArtifactInputLowerer::lower(&parsed);
    canonical_artifact_from_input(artifact_input, snapshot)
}

fn rust_authored_material(
    input: &crate::source::WorthUiRustAuthoredArtifactInput,
    snapshot: &CapabilitySnapshot,
) -> Result<WorthUiLoweredCandidateMaterial, WorthUiWatchedCandidateSubmissionDenial> {
    let artifact_input = WorthUiRustAuthoredToArtifactInputLowerer::try_lower(input)
        .map_err(|_| source_denial(WorthUiSourceIngressDenialReason::SourcePackageRejected))?;
    canonical_artifact_from_input(artifact_input, snapshot)
}

fn canonical_artifact_from_input(
    artifact_input: crate::source::WorthUiArtifactInput,
    snapshot: &CapabilitySnapshot,
) -> Result<WorthUiLoweredCandidateMaterial, WorthUiWatchedCandidateSubmissionDenial> {
    let resolved = WorthUiArtifactInputResolver::resolve(&artifact_input, snapshot)
        .map_err(|_| source_denial(WorthUiSourceIngressDenialReason::ArtifactResolutionRejected))?;
    let structured = WorthUiStructuralLegalityLowerer::lower(&resolved, snapshot)
        .map_err(|_| source_denial(WorthUiSourceIngressDenialReason::StructuralLegalityRejected))?;
    let declaration_witness = project_source_backed_declaration_witness(&structured)?;
    let source_backed_dsl_package =
        WorthUiSourceBackedDslPackage::new(source_backed_package(&structured), declaration_witness);
    let bound = WorthUiBindingSemanticsLowerer::lower(&structured, snapshot)
        .map_err(|_| source_denial(WorthUiSourceIngressDenialReason::BindingSemanticsRejected))?;
    let identity_seeded = WorthUiIdentitySeedLowerer::lower(&bound)
        .map_err(|_| source_denial(WorthUiSourceIngressDenialReason::IdentitySeedingRejected))?
        .0;
    let artifact = WorthUiCanonicalArtifactAssembler::assemble(&identity_seeded)
        .map_err(|_| source_denial(WorthUiSourceIngressDenialReason::CanonicalAssemblyRejected))?;
    Ok(WorthUiLoweredCandidateMaterial {
        artifact,
        source_backed_dsl_package,
    })
}

fn rust_authored_candidate(
    artifact: WorthUiArtifact,
    snapshot_digest: CapabilitySnapshotDigest,
    revision: &WorthUiSourcePackageRevision,
) -> Result<WorthUiReplacementCandidate, WorthUiWatchedCandidateSubmissionDenial> {
    rust_authored_replacement_candidate(
        artifact,
        snapshot_digest,
        WorthUiReplacementCause::rust_authored_input_change(revision.final_package_digest()),
    )
    .map_err(WorthUiWatchedCandidateSubmissionDenial::Candidate)
}

fn file_authored_candidate(
    artifact: WorthUiArtifact,
    snapshot_digest: CapabilitySnapshotDigest,
    provider: &WorthUiSourceProvider,
    revision: &WorthUiSourcePackageRevision,
) -> Result<WorthUiReplacementCandidate, WorthUiWatchedCandidateSubmissionDenial> {
    let module_id = primary_source_module_id(provider)?;
    file_authored_replacement_candidate(
        artifact,
        snapshot_digest,
        WorthUiReplacementCause::file_source_change(module_id, revision.final_package_digest()),
    )
    .map_err(WorthUiWatchedCandidateSubmissionDenial::Candidate)
}

fn primary_source_module_id(
    provider: &WorthUiSourceProvider,
) -> Result<crate::source::WorthUiSourceModuleId, WorthUiWatchedCandidateSubmissionDenial> {
    let primary_source_module = provider
        .source_modules()
        .first()
        .ok_or_else(|| source_denial(WorthUiSourceIngressDenialReason::NoCandidateMaterial))?;
    crate::source::WorthUiSourceModuleId::from_relative_path(std::path::Path::new(
        primary_source_module.relative_path(),
    ))
    .map_err(|_| source_denial(WorthUiSourceIngressDenialReason::SourcePackageRejected))
}

fn source_denial(
    reason: WorthUiSourceIngressDenialReason,
) -> WorthUiWatchedCandidateSubmissionDenial {
    WorthUiWatchedCandidateSubmissionDenial::SourceIngress(WorthUiSourceIngressDenial::new(reason))
}
