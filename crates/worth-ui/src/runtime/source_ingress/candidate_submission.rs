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
use crate::runtime::WorthUiReplacementCause;
use crate::runtime::{
    WorthUiReplacementCandidate, WorthUiReplacementCandidateDenial, WorthUiRuntimeAuthoringSnapshot,
};
use crate::source::{
    build_content_slot_catalog, build_layout_topology_catalog, WorthUiArtifact,
    WorthUiArtifactInputResolver, WorthUiBindingSemanticsLowerer,
    WorthUiCanonicalArtifactAssembler, WorthUiIdentitySeedLowerer, WorthUiParsedSourcePackage,
    WorthUiParsedSourceToArtifactInputLowerer, WorthUiSourcePackageLoader, WorthUiSourceParser,
    WorthUiStructuralLegalityLowerer,
};

#[derive(Debug, Eq, PartialEq)]
pub struct WorthUiWatchedCandidateSubmission {
    candidate: WorthUiReplacementCandidate,
    authoring_snapshot: Option<WorthUiRuntimeAuthoringSnapshot>,
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
        let (candidate, authoring_snapshot) =
            lower_provider_to_candidate(&provider, snapshot, &revision, &ordering_receipt)?;
        counters.emit_candidate_submission();
        Ok(WorthUiWatchedCandidateSubmission {
            candidate,
            authoring_snapshot,
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

    pub fn into_candidate(self) -> WorthUiReplacementCandidate {
        self.candidate
    }

    pub fn into_parts(
        self,
    ) -> (
        WorthUiReplacementCandidate,
        Option<WorthUiRuntimeAuthoringSnapshot>,
        WorthUiSourcePackageRevision,
        WorthUiCandidateOrderingReceipt,
        WorthUiSourceIngressCounters,
    ) {
        (
            self.candidate,
            self.authoring_snapshot,
            self.revision,
            self.ordering_receipt,
            self.counters,
        )
    }
}

fn lower_provider_to_candidate(
    provider: &WorthUiSourceProvider,
    snapshot: &CapabilitySnapshot,
    revision: &WorthUiSourcePackageRevision,
    ordering_receipt: &WorthUiCandidateOrderingReceipt,
) -> Result<
    (
        WorthUiReplacementCandidate,
        Option<WorthUiRuntimeAuthoringSnapshot>,
    ),
    WorthUiWatchedCandidateSubmissionDenial,
> {
    if !ordering_receipt.matches_revision(revision) {
        return Err(source_denial(
            WorthUiSourceIngressDenialReason::OrderingReceiptDrift,
        ));
    }
    reject_ambiguous_candidate_material(provider)?;
    if let Some(input) = provider.artifact_inputs().first() {
        let artifact = input.artifact().cloned().ok_or_else(|| {
            WorthUiWatchedCandidateSubmissionDenial::SourceIngress(WorthUiSourceIngressDenial::new(
                WorthUiSourceIngressDenialReason::NoCandidateMaterial,
            ))
        })?;
        return rust_authored_candidate(artifact, snapshot.digest(), revision)
            .map(|candidate| (candidate, None));
    }
    if !provider.source_modules().is_empty() {
        let (artifact, authoring_snapshot) = file_authored_material(provider, snapshot)?;
        return file_authored_candidate(artifact, snapshot.digest(), provider, revision)
            .map(|candidate| (candidate, Some(authoring_snapshot)));
    }
    Err(source_denial(
        WorthUiSourceIngressDenialReason::NoCandidateMaterial,
    ))
}

fn reject_ambiguous_candidate_material(
    provider: &WorthUiSourceProvider,
) -> Result<(), WorthUiWatchedCandidateSubmissionDenial> {
    if !provider.source_modules().is_empty() && !provider.artifact_inputs().is_empty() {
        return Err(source_denial(
            WorthUiSourceIngressDenialReason::MixedCandidateMaterial,
        ));
    }
    if provider.artifact_inputs().len() > 1 {
        return Err(source_denial(
            WorthUiSourceIngressDenialReason::MultipleArtifactInputs,
        ));
    }
    Ok(())
}

fn file_authored_material(
    provider: &WorthUiSourceProvider,
    snapshot: &CapabilitySnapshot,
) -> Result<
    (WorthUiArtifact, WorthUiRuntimeAuthoringSnapshot),
    WorthUiWatchedCandidateSubmissionDenial,
> {
    let mut loader = WorthUiSourcePackageLoader::from_workspace_root(provider.workspace_root());
    for module in provider.source_modules() {
        loader = loader.register_module_with_source(module.relative_path(), module.source_text());
    }
    let source_package = loader
        .compile()
        .map_err(|_| source_denial(WorthUiSourceIngressDenialReason::SourcePackageRejected))?;
    let parsed = WorthUiSourceParser::parse_package(&source_package)
        .map_err(|_| source_denial(WorthUiSourceIngressDenialReason::SourceParseRejected))?;
    let authoring_snapshot = file_authored_authoring_snapshot(&parsed)?;
    let artifact_input = WorthUiParsedSourceToArtifactInputLowerer::lower(&parsed)
        .map_err(|_| source_denial(WorthUiSourceIngressDenialReason::AuthoringEntryRejected))?;
    canonical_artifact_from_input(artifact_input, snapshot)
        .map(|artifact| (artifact, authoring_snapshot))
}

fn file_authored_authoring_snapshot(
    parsed: &WorthUiParsedSourcePackage,
) -> Result<WorthUiRuntimeAuthoringSnapshot, WorthUiWatchedCandidateSubmissionDenial> {
    let layout_topology = build_layout_topology_catalog(parsed)
        .map_err(|_| source_denial(WorthUiSourceIngressDenialReason::AuthoringEntryRejected))?;
    let content_slots = build_content_slot_catalog(parsed, &layout_topology)
        .map_err(|_| source_denial(WorthUiSourceIngressDenialReason::AuthoringEntryRejected))?;
    Ok(WorthUiRuntimeAuthoringSnapshot::new(
        layout_topology,
        content_slots,
    ))
}

fn canonical_artifact_from_input(
    artifact_input: crate::source::WorthUiArtifactInput,
    snapshot: &CapabilitySnapshot,
) -> Result<WorthUiArtifact, WorthUiWatchedCandidateSubmissionDenial> {
    let resolved = WorthUiArtifactInputResolver::resolve(&artifact_input, snapshot)
        .map_err(|_| source_denial(WorthUiSourceIngressDenialReason::ArtifactResolutionRejected))?;
    let structured = WorthUiStructuralLegalityLowerer::lower(&resolved, snapshot)
        .map_err(|_| source_denial(WorthUiSourceIngressDenialReason::StructuralLegalityRejected))?;
    let bound = WorthUiBindingSemanticsLowerer::lower(&structured, snapshot)
        .map_err(|_| source_denial(WorthUiSourceIngressDenialReason::BindingSemanticsRejected))?;
    let identity_seeded = WorthUiIdentitySeedLowerer::lower(&bound)
        .map_err(|_| source_denial(WorthUiSourceIngressDenialReason::IdentitySeedingRejected))?
        .0;
    WorthUiCanonicalArtifactAssembler::assemble(&identity_seeded)
        .map_err(|_| source_denial(WorthUiSourceIngressDenialReason::CanonicalAssemblyRejected))
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
