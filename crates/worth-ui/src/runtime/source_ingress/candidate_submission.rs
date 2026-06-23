use crate::capability::{CapabilitySnapshot, CapabilitySnapshotDigest};
use crate::runtime::authored_delta::lower_authored_delta_summary;
use crate::runtime::candidate::{
    file_authored_replacement_candidate, rust_authored_replacement_candidate,
};
use crate::runtime::source_ingress::authored_submission::{
    WorthUiSourceAuthoredCandidateSubmission, WorthUiSourceAuthoredCandidateSubmissionDenial,
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
    WorthUiAuthoredDeltaSummary, WorthUiCandidateRuntimeAuthoringSnapshot,
    WorthUiReplacementCandidate, WorthUiReplacementCandidateDenial,
    WorthUiRuntimeAuthoringSnapshot, WorthUiRuntimeAuthoringSnapshotBuilder,
    WorthUiSemanticSliceInventory,
};
use crate::source::{
    build_content_slot_catalog, build_layout_topology_catalog, WorthUiArtifact,
    WorthUiArtifactInputResolver, WorthUiBindingSemanticsLowerer,
    WorthUiCanonicalArtifactAssembler, WorthUiIdentitySeedLowerer,
    WorthUiParsedSourceToArtifactInputLowerer, WorthUiSourcePackageLoader, WorthUiSourceParser,
    WorthUiStructuralLegalityLowerer,
};

#[derive(Debug, Eq, PartialEq)]
pub struct WorthUiWatchedCandidateSubmission {
    candidate: WorthUiReplacementCandidate,
    authoring_snapshot: Option<WorthUiCandidateRuntimeAuthoringSnapshot>,
    authored_delta_summary: Option<WorthUiAuthoredDeltaSummary>,
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
        active_authoring_snapshot: Option<&WorthUiRuntimeAuthoringSnapshot>,
    ) -> Result<WorthUiWatchedCandidateSubmission, WorthUiWatchedCandidateSubmissionDenial> {
        let (provider, revision, ordering_receipt, mut counters) = self.into_parts();
        let (candidate, authoring_snapshot, authored_delta_summary) = lower_provider_to_candidate(
            &provider,
            snapshot,
            active_authoring_snapshot,
            &revision,
            &ordering_receipt,
            &mut counters,
        )?;
        counters.emit_candidate_submission();
        Ok(WorthUiWatchedCandidateSubmission {
            candidate,
            authoring_snapshot,
            authored_delta_summary,
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

    pub fn authored_delta_summary(&self) -> Option<&WorthUiAuthoredDeltaSummary> {
        self.authored_delta_summary.as_ref()
    }

    pub fn into_source_authored_submission(
        self,
    ) -> Result<
        WorthUiSourceAuthoredCandidateSubmission,
        WorthUiSourceAuthoredCandidateSubmissionDenial,
    > {
        if self.authoring_snapshot.is_none() || self.authored_delta_summary.is_none() {
            return Err(WorthUiSourceAuthoredCandidateSubmissionDenial::MissingAuthoredDeltaProof);
        }
        Ok(WorthUiSourceAuthoredCandidateSubmission::new(self))
    }

    pub fn into_parts(
        self,
    ) -> (
        WorthUiReplacementCandidate,
        Option<WorthUiCandidateRuntimeAuthoringSnapshot>,
        Option<WorthUiAuthoredDeltaSummary>,
        WorthUiSourcePackageRevision,
        WorthUiCandidateOrderingReceipt,
        WorthUiSourceIngressCounters,
    ) {
        (
            self.candidate,
            self.authoring_snapshot,
            self.authored_delta_summary,
            self.revision,
            self.ordering_receipt,
            self.counters,
        )
    }
}

fn lower_provider_to_candidate(
    provider: &WorthUiSourceProvider,
    snapshot: &CapabilitySnapshot,
    active_authoring_snapshot: Option<&WorthUiRuntimeAuthoringSnapshot>,
    revision: &WorthUiSourcePackageRevision,
    ordering_receipt: &WorthUiCandidateOrderingReceipt,
    counters: &mut WorthUiSourceIngressCounters,
) -> Result<
    (
        WorthUiReplacementCandidate,
        Option<WorthUiCandidateRuntimeAuthoringSnapshot>,
        Option<WorthUiAuthoredDeltaSummary>,
    ),
    WorthUiWatchedCandidateSubmissionDenial,
> {
    if !ordering_receipt.matches_revision(revision) {
        return Err(source_denial(
            WorthUiSourceIngressDenialReason::OrderingReceiptDrift,
        ));
    }
    reject_ambiguous_candidate_material(provider)?;
    counters.record_observed_modules(provider.source_modules().len());
    if let Some(input) = provider.artifact_inputs().first() {
        let artifact = input.artifact().cloned().ok_or_else(|| {
            WorthUiWatchedCandidateSubmissionDenial::SourceIngress(WorthUiSourceIngressDenial::new(
                WorthUiSourceIngressDenialReason::NoCandidateMaterial,
            ))
        })?;
        return rust_authored_candidate(artifact, snapshot.digest(), revision)
            .map(|candidate| (candidate, None, None));
    }
    if !provider.source_modules().is_empty() {
        let (artifact, authoring_snapshot, authored_delta_summary) =
            file_authored_material(provider, snapshot, active_authoring_snapshot, counters)?;
        return file_authored_candidate(artifact, snapshot.digest(), provider, revision).map(
            |candidate| {
                (
                    candidate,
                    Some(authoring_snapshot),
                    Some(authored_delta_summary),
                )
            },
        );
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
    active_authoring_snapshot: Option<&WorthUiRuntimeAuthoringSnapshot>,
    counters: &mut WorthUiSourceIngressCounters,
) -> Result<
    (
        WorthUiArtifact,
        WorthUiCandidateRuntimeAuthoringSnapshot,
        WorthUiAuthoredDeltaSummary,
    ),
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
    counters.record_parsed_modules(parsed.module_ids().len());
    let layout_topology = build_layout_topology_catalog(&parsed)
        .map_err(|_| source_denial(WorthUiSourceIngressDenialReason::AuthoringEntryRejected))?;
    let content_slots = build_content_slot_catalog(&parsed, &layout_topology)
        .map_err(|_| source_denial(WorthUiSourceIngressDenialReason::AuthoringEntryRejected))?;
    let artifact_input = WorthUiParsedSourceToArtifactInputLowerer::lower(&parsed)
        .map_err(|_| source_denial(WorthUiSourceIngressDenialReason::AuthoringEntryRejected))?;
    let artifact = canonical_artifact_from_input(artifact_input, snapshot)?;
    let content_slots = content_slots
        .verify_canonical_mount_order(&artifact)
        .map_err(|_| source_denial(WorthUiSourceIngressDenialReason::AuthoringEntryRejected))?;
    let authoring_snapshot = WorthUiRuntimeAuthoringSnapshotBuilder::from_source_package(
        &parsed,
        snapshot,
        layout_topology,
        content_slots,
    )
    .map_err(|_| source_denial(WorthUiSourceIngressDenialReason::AuthoringEntryRejected))?;
    let authored_delta_summary = lower_authored_delta_summary(
        provider.source_modules().len(),
        &parsed,
        active_authoring_snapshot,
        &authoring_snapshot,
        &WorthUiSemanticSliceInventory::current(),
    );
    counters.record_authored_declarations_inspected(
        authored_delta_summary
            .counters()
            .authored_declarations_inspected(),
    );
    counters.record_authored_declarations_touched(
        authored_delta_summary
            .counters()
            .authored_declarations_touched(),
    );
    counters.record_semantic_slices_emitted(
        authored_delta_summary.counters().semantic_slices_emitted(),
    );
    Ok((artifact, authoring_snapshot, authored_delta_summary))
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
